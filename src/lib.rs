mod wasm4;
use tiles::TileTypeEnum;
use wasm4::*;

pub mod utils;
mod sprites;
mod level;
mod camera;
mod player;
mod tiles;
mod world;
mod unique_items;

use camera::Camera;
use world::World;
use player::Player;

use ufmt::uwrite;
use utils::{draw_colours, UfmtBuf};

use num_complex::Complex32 as cf32;


type RoomData = [u32; 16];

#[derive(Clone, Copy)]
pub struct RoomMetadata {
    block_type_sp: Option<TileTypeEnum>,
    block_type_x: Option<TileTypeEnum>,
    block_type_a: Option<TileTypeEnum>,
    block_type_b: Option<TileTypeEnum>,
}

// 8x4 block of rooms
type RoomBlock = [RoomData; 32];

#[derive(variant_count::VariantCount, PartialEq, Eq, Copy, Clone)]
pub enum UniqueItem {
    PlayerStart,
    Info1,
}

pub type TilePos = (u16, u16);

struct Level {
    the_area: Area,
    unique_items: [(UniqueItem, TilePos); UniqueItem::VARIANT_COUNT],
}

pub const MAX_UNIQUE_ITEM_POSITIONS : usize = 16;
pub const MAX_UNIQUE_ITEMS_PER_ROOM: usize = 2;
pub type UniqueItemPositions = [Option<UniqueItemPosition>; MAX_UNIQUE_ITEM_POSITIONS];
pub type UniqueItemsInThisRoom = [Option<UniqueItem>; MAX_UNIQUE_ITEMS_PER_ROOM];

pub struct Area {
    rooms: [RoomData; 32],
    meta: [RoomMetadata; 32],
    uniques: [UniqueItemsInThisRoom; 32],
}

pub struct AreaSource<const C: usize, const T:usize, const I:usize> {
    pub cells: &'static [u8],
    pub empty_tile_style: TileTypeEnum,
    pub solid_tile_style: TileTypeEnum,
    char_lookup: [CharDescription; C],
    tile_lookup: [MappingBetweenCharAndTileType; T],
    item_lookup: [MappingBetweenCharAndItem; I],
}

static LEVEL : Level = Level::new();

#[derive(Clone, Copy)]
pub enum LowlevelCellType {
    Empty,
    Solid,
    CustomA,
    CustomB,
    Special,
}

impl LowlevelCellType {
    pub const fn ll_code(&self) -> u8 {
        match self {
            LowlevelCellType::Empty => 0b00,
            LowlevelCellType::Solid => 0b01,
            LowlevelCellType::CustomA => 0b10,
            LowlevelCellType::CustomB => 0b11,
            LowlevelCellType::Special => 0b00,
        }
    }
}

#[derive(Clone, Copy)]
pub struct CharDescription {
    chr: u8,
    upper: LowlevelCellType,
    lower: LowlevelCellType,
}
#[derive(Clone, Copy)]
pub struct UniqueItemPositionLowlevel {
    chr: u8,
    pos : TilePos,
}

#[derive(Clone, Copy)]
pub struct UniqueItemPosition {
    item: UniqueItem,
    pos : TilePos,

    /// For temporary position overrides during development
    priority: bool,
}

#[derive(Clone, Copy)]
pub struct MappingBetweenCharAndItem {
    chr: u8,
    item: UniqueItem,
    priority: bool,
}

#[derive(Clone, Copy)]
pub struct MappingBetweenCharAndTileType {
    chr: u8,
    tt: TileTypeEnum,
}

struct TextBox {
    c: u8,
}

impl TextBox {
    const fn new() -> TextBox {
        TextBox { c: 2 }
    }

    fn control(&mut self, prev: u8, cur: u8) {
        if (cur & !prev) & BUTTON_1 != 0 {
            self.c += 1;
            if self.c > 4 {
                self.c = 2;
            }
        }
    }

    fn draw(&self, _global_frame: u8) {
        draw_colours(self.c, 0, 0, 0);
        let mut buf = UfmtBuf::<11>::new();
        let _ = uwrite!(buf, "{}", 33);
        text(buf.as_str(), 10, 10);
    }
}


struct State {
    prevpad: u8,
    frame: u8,

    camera: Camera,
    player: Player,
    textbox: TextBox,

    room: World,
}

impl State {
    pub const fn new() -> State {
        let s = State {
            prevpad: 0,
            frame: 0,
            camera: Camera::new(),
            player: Player::new(),
            textbox: TextBox::new(),
            room: World::new(),
        };
        s
    }

    pub fn tick(&mut self) {
        unsafe {
            *PALETTE = [0, 0x808080, 0xFFFFFF, 0x8080FF];
        }

        let gamepad = unsafe { *GAMEPAD1 };

        if !self.player.pos.is_normal() {
            self.player.pos = World::from_world_coords(LEVEL.unique_item_pos(UniqueItem::PlayerStart));
            self.camera.pos = self.player.pos;
        }

        self.player.control(self.prevpad, gamepad);

        #[allow(unused_variables)]
        let mut iterations_counter = 0;
        let mut remaining_movement_units = 10.0;
        while remaining_movement_units > 0.0 {
            self.player.grounded = false;
            
            //self.player.ground_force_direction += cf32::new(0.0, -0.02);
            
            let mut acceleration = cf32::new(0.0, 0.0);
            self.player.handle_collisions(&self.room, &mut acceleration);
            self.player.movement(&mut acceleration);
            
            let vel_estimate1 = self.player.vel.norm();
            let vel_estimate2 = (self.player.vel + acceleration*2.0).norm();
            let vel_estimate = vel_estimate1.max(vel_estimate2  ) / 2000.0;

            // aim to move do 5 iterations of collision calculations per pixel of movement
            let mut epsilon = 0.2 / vel_estimate;

            epsilon = epsilon.min(1.0/acceleration.norm());
            epsilon = epsilon.max(0.1);
            epsilon = epsilon.min(remaining_movement_units);

            //crate::traceln!("  accel {} epsilon {}", (acceleration.norm() * 100.0) as i32, (epsilon * 100.0) as i32);

            self.player.vel += epsilon * acceleration;
            self.player.pos += epsilon * self.player.vel / 2000.0;

            remaining_movement_units -= epsilon;
            iterations_counter += 1;
        }
        //crate::traceln!("iters {}", iterations_counter);

        self.textbox.control(self.prevpad, gamepad);

        self.camera.update(&self.player, gamepad);
        self.room
            .draw(self.frame, self.player.my_world_coords(), &self.camera);
        self.player.draw(self.frame, gamepad, &self.camera);
        self.textbox.draw(self.frame);

        self.prevpad = gamepad;
        self.frame = self.frame.wrapping_add(1);
    }
}

static mut STATE: State = State::new();

#[no_mangle]
fn update() {
    unsafe {
        STATE.tick();
    }
}
