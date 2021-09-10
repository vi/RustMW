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

use camera::Camera;
use world::World;
use player::Player;

use ufmt::uwrite;
use utils::{draw_colours, UfmtBuf};

use num_complex::Complex32 as cf32;


type RoomData = [u32; 16];

#[derive(Clone, Copy)]
struct RoomMetadata {
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
}

pub type TilePos = (u16, u16);

struct Level {
    the_area: Area,
    unique_items: [(UniqueItem, TilePos); UniqueItem::VARIANT_COUNT],
}

impl Level {
    pub const fn new() -> Level {
        let mut unique_items = [(UniqueItem::PlayerStart, (0,0)); UniqueItem::VARIANT_COUNT];
        let mut prioritized = [false; UniqueItem::VARIANT_COUNT];

        let mut i = 0;

        let mut j = 0;
        let specials = level::AREA1.1;
        while j < specials.len() {
            if let Some(UniqueItemPosition { item, pos, priority}) = specials[j] {
                let mut insert_at_the_end = true;

                let mut k = 0;
                while k < i {
                    if unique_items[k].0 as u8== item as u8 {
                        insert_at_the_end = false;
                        match (priority, prioritized[k]) {
                            (false, false) => {b"Duplicate position for an unique item"[999];}
                            (false, true) => (), // silently ignore non-priority position when priority one is already set
                            (true, false) => {
                                unique_items[k].1 = pos;
                                prioritized[k] = true;
                            }
                            (true, true) => {b"Duplicate priority position for an unique item"[999];}
                        }
                    }
                    k+=1;
                }
                
                if insert_at_the_end {
                    unique_items[i].0 = item;
                    unique_items[i].1 = pos;
                    prioritized[i] = priority;
                    i+=1;
                }
            }
            j += 1;
        }
        if i != UniqueItem::VARIANT_COUNT {
            b"There is a missing unique item on the level"[999];
        }

        Level {
            the_area: level::AREA1.0,
            unique_items,
        }
    }

    pub const fn unique_item_pos(&self, item: UniqueItem) -> TilePos {
        let mut i=0;
        while i < self.unique_items.len() {
            if item as u8 == self.unique_items[i].0 as u8 {
                return self.unique_items[i].1;
            }
            i+=1;
        }
        #[allow(unconditional_panic)]
        b"Internal error: Level::new should have caught missing item position"[999];
        (0,0)
    }
}

pub const MAX_UNIQUE_ITEM_POSITIONS : usize = 16;
pub type UniqueItemPositions = [Option<UniqueItemPosition>; MAX_UNIQUE_ITEM_POSITIONS];

pub struct Area {
    rooms: [RoomData; 32],
    meta: [RoomMetadata; 32],
}

static LEVEL : Level = Level::new();

impl Area {
    const fn build(s: &'static [u8]) -> (Area, UniqueItemPositions) {
        let char_lookup = utils::ll_char_descriptions::<6>(b"S!.      J.A      jAX      l.B      LBX     !!.  ");
        let item_lookup: [MappingBetweenCharAndItem; 2] = [c2i!(S PlayerStart), c2i!(b'!' ! PlayerStart)];
       
        let (rooms, specials_ll) = utils::makearea(s, char_lookup);
        let meta = [RoomMetadata {
            block_type_sp: Some(TileTypeEnum::EmptyTile(tiles::EmptyTile)),
            block_type_x: Some(TileTypeEnum::UsualArea1Tile(tiles::UsualArea1Tile)),
            block_type_a: Some(TileTypeEnum::JumpyTile(tiles::JumpyTile)),
            block_type_b: Some(TileTypeEnum::Ladder1Tile(tiles::Ladder1Tile)),
        }; 32];

        let mut specials = [None; MAX_UNIQUE_ITEM_POSITIONS];

        let mut i = 0;
        let mut j: usize = 0;
        while i < specials_ll.len() {
            if let Some(spcll) = specials_ll[i] {
                let chr = spcll.chr;
                let mut k = 0;
                let mut found = false;

                while k < item_lookup.len() {
                    let MappingBetweenCharAndItem { chr: m_chr, item, priority } = item_lookup[k];
                    if m_chr == chr {
                        found = true;

                        specials[j] = Some(UniqueItemPosition{item, pos:spcll.pos, priority});
                        j+=1;

                        break;
                    }
                    k += 1;
                }

                if !found {
                    b"Encountered unique item character that is not mapped to UniquItem"[999];
                }
            }
            i+=1;
        }

        (Area {
            rooms,
            meta,
        }, specials)
    }
}

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

    /// For temporary position overrides during development
    priority: bool,
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
