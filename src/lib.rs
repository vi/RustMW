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
    block_type_sp: TileTypeEnum,
    block_type_x: TileTypeEnum,
    block_type_a: TileTypeEnum,
    block_type_b: TileTypeEnum,
}

// 8x4 block of rooms
type RoomBlock = [RoomData; 32];

pub struct Area {
    rooms: [RoomData; 32],
    meta: [RoomMetadata; 32],
    player_starting_point: Option<(u16, u16)>,
}

impl Area {
    const fn new(s: &'static [u8]) -> Area {
        let char_lookup = [CharDescription {
            chr: b'S',
            upper: LowlevelCellType::Special,
            lower: LowlevelCellType::Empty,
        },
        CharDescription{
            chr: b'J',
            upper: LowlevelCellType::Empty,
            lower: LowlevelCellType::CustomA,
        },
        CharDescription{
            chr: b'j',
            upper: LowlevelCellType::CustomA,
            lower: LowlevelCellType::Solid,
        },
        CharDescription{
            chr: b'l',
            upper: LowlevelCellType::Empty,
            lower: LowlevelCellType::CustomB,
        },
        CharDescription{
            chr: b'L',
            upper: LowlevelCellType::CustomB,
            lower: LowlevelCellType::Solid,
        },
        ];
        let (rooms, specials) = utils::makearea(s, char_lookup);
        let meta = [RoomMetadata {
            block_type_sp: TileTypeEnum::EmptyTile(tiles::EmptyTile),
            block_type_x: TileTypeEnum::UsualArea1Tile(tiles::UsualArea1Tile),
            block_type_a: TileTypeEnum::JumpyTile(tiles::JumpyTile),
            block_type_b: TileTypeEnum::Ladder1Tile(tiles::Ladder1Tile),
        }; 32];
        let mut player_starting_point = None;

        let mut i = 0;
        while i < specials.len() {
            if let Some(s) = specials[i] {
                if s.chr == b'S' {
                    player_starting_point = Some((s.x, s.y));
                }
            }
            i += 1;
        }

        Area {
            rooms,
            meta,
            player_starting_point,
        }
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
pub struct SpecialPosition {
    chr: u8,
    x: u16,
    y: u16,
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
            if let Some(pos) = level::AREA1.player_starting_point {
                self.player.pos = World::from_world_coords(pos);
                self.camera.pos = self.player.pos;
            } else {
                return;
            }
        }

        self.player.control(self.prevpad, gamepad);

        for _ in 0..10 {
            let epsilon = 1.0;
            self.player.grounded = false;

            self.player.ground_force_direction = self.player.ground_force_direction.scale(0.5);
            //self.player.ground_force_direction += cf32::new(0.0, -0.02);

            let mut acceleration = cf32::new(0.0, 0.0);
            self.player.handle_collisions(&self.room, &mut acceleration);
            self.player.movement(&mut acceleration);

            self.player.vel += epsilon * acceleration;
            self.player.pos += epsilon * self.player.vel / 2000.0;
            self.player.ground_force_direction = self.player.ground_force_direction.unscale(self.player.ground_force_direction.norm());
        }

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
