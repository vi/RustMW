mod wasm4;
use mapview::MapViewer;
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
mod mapview;

use camera::Camera;
use world::World;
use player::Player;

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

pub struct Level {
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

pub const LEVEL : Level = Level::new();

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


pub struct Game {
    frame: u8,

    pub camera: Camera,
    pub player: Player,

    pub world: World,
}

impl Game {
    pub const fn new() -> Game {
        let s = Game {
            frame: 0,
            camera: Camera::new(),
            player: Player::new(),
            world: World::new(),
        };
        s
    }

    pub fn tick(&mut self, gamepad: u8, prev_gamepad: u8) -> MainState {
        unsafe {
            *PALETTE = [0, 0x808080, 0xFFFFFF, 0x8080FF];
        }

        let mut inhibit_drawing_player = false;

        if !self.player.pos.is_normal() {
            self.player.pos = World::from_world_coords(LEVEL.unique_item_pos(UniqueItem::PlayerStart));
            self.camera.pos = self.player.pos;
        }

        let newstate = self.player.control(prev_gamepad, gamepad);

        #[allow(unused_variables)]
        let mut iterations_counter = 0;
        let mut remaining_movement_units = 10.0;
        while remaining_movement_units > 0.0 {
            self.player.grounded = false;
            
            //self.player.ground_force_direction += cf32::new(0.0, -0.02);
            
            let mut acceleration = cf32::new(0.0, 0.0);
            self.player.handle_collisions(&mut acceleration);
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

        self.camera.update(&self.player, gamepad);
        World::draw(self.frame, self.player.my_world_coords(), &self.camera);
        
        let campos = World::to_world_coords(self.camera.pos);
        let playpos = World::to_world_coords(self.player.pos);
        for item in World::get_unique_items_around_tile(campos) {
            if let Some(item) = item {
                unique_items::draw_unique(item, self.frame, playpos, &self.camera, &mut inhibit_drawing_player);
            }
        }

        if ! inhibit_drawing_player {
            self.player.draw(self.frame, gamepad, &self.camera);
        }

        self.frame = self.frame.wrapping_add(1);
        newstate
    }
}


pub enum MainState {
    Game,
    Map,
}

pub struct GlobalState {
    main_state: MainState,
    game: Game,
    map_viewer: MapViewer,

    previous_gamepad: u8,
}

impl GlobalState {
    pub const fn new() -> GlobalState {
        GlobalState {
            main_state: MainState::Game,
            game: Game::new(),
            map_viewer: MapViewer::new(),
            previous_gamepad: 0,
        }
    }

    pub fn tick(&mut self, gamepad_state: u8) {
        self.main_state = match self.main_state {
            MainState::Game => self.game.tick(gamepad_state, self.previous_gamepad),
            MainState::Map => self.map_viewer.tick(gamepad_state, self.previous_gamepad, &self.game),
        };
        self.previous_gamepad = gamepad_state;
    }
}


static mut GLOBAL_STATE : GlobalState = GlobalState::new();

#[no_mangle]
fn update() {
    unsafe {
        GLOBAL_STATE.tick(*GAMEPAD1);
    }
}
