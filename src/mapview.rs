use crate::{Game, MainState, TilePos, UniqueItem, wasm4::{BUTTON_1, FRAMEBUFFER, PALETTE, SCREEN_SIZE}, world::World};
use crate::tiles::TileType;
use enum_iterator::IntoEnumIterator;

pub struct MapViewer {
    upper_left_tile: TilePos,
    blinker : u8,
}

impl MapViewer {
    pub const fn new() -> MapViewer {
        MapViewer {
            upper_left_tile: (u16::MAX, u16::MAX),
            blinker: 0,
        }
    }

    pub fn clamp_tile(mut pos: (i32, i32)) -> TilePos {
        if pos.0 >= World::BOTTOM_RIGHT_TILE.0 as i32 - SCREEN_SIZE as i32 {
            pos.0 =  World::BOTTOM_RIGHT_TILE.0 as i32 - SCREEN_SIZE as i32 - 1;
        }
        if pos.1 >= World::BOTTOM_RIGHT_TILE.1 as i32 - SCREEN_SIZE as i32{
            pos.1 =  World::BOTTOM_RIGHT_TILE.1 as i32 - SCREEN_SIZE as i32 - 1;
        }
        if pos.0 < 0 { pos.0 = 0; }
        if pos.1 < 0 { pos.1 = 0; }
        (pos.0 as u16 & !0b11, pos.1 as u16 & !0b11)
    }

    pub fn tick(&mut self, cur: u8, prev: u8, game: &Game) -> MainState {
        unsafe {
            *PALETTE = [0x101010, 0x808080, 0xFFFFFF, 0x8080FF];
        }

        if (cur & !prev) & BUTTON_1 != 0{
            self.upper_left_tile = (u16::MAX, u16::MAX);
            return MainState::Game;
        }

        if self.upper_left_tile == (u16::MAX, u16::MAX) {
            let playerpos = World::to_world_coords(game.player.pos);
            let (x,y) = (playerpos.0 as i32, playerpos.1 as i32);
            self.upper_left_tile = MapViewer::clamp_tile((x - (SCREEN_SIZE as i32)/2, y - (SCREEN_SIZE as i32)/2));
        }

        for y in 0..SCREEN_SIZE {
            for x in (0..SCREEN_SIZE).step_by(4) {
                let tt1 = World::get_tile((self.upper_left_tile.0 + x as u16 + 0, self.upper_left_tile.1 + y as u16)).map_viewer_colour();
                let tt2 = World::get_tile((self.upper_left_tile.0 + x as u16 + 1, self.upper_left_tile.1 + y as u16)).map_viewer_colour();
                let tt3 = World::get_tile((self.upper_left_tile.0 + x as u16 + 2, self.upper_left_tile.1 + y as u16)).map_viewer_colour();
                let tt4 = World::get_tile((self.upper_left_tile.0 + x as u16 + 3, self.upper_left_tile.1 + y as u16)).map_viewer_colour();
                let byte = 
                    (((tt1) & 0b11) << 0) | 
                    (((tt2) & 0b11) << 2) | 
                    (((tt3) & 0b11) << 4) | 
                    (((tt4) & 0b11) << 6) | 
                    0;
                let offset = ((SCREEN_SIZE * y >> 2) + (x>>2)) as isize;
                unsafe {
                    let ptr =(FRAMEBUFFER as *mut u8).offset(offset); 
                    ptr.write(byte);
                }
            }
        }

        for item in UniqueItem::into_enum_iter() {
            let itempos = item.get_pos();
            if game.player.status.is_touched(item) || self.blinker < 30 {
                self.set_pixel(itempos, 0b11);
            }
        }

        if self.blinker < 30 {
            let playerpos = World::to_world_coords(game.player.pos);
            self.set_pixel(playerpos, 0b10);
        }


        self.blinker = self.blinker.wrapping_add(1);
        if self.blinker >= 60 {
            self.blinker = 0;
        }
        MainState::Map
    }

    fn set_pixel(&self, pos: TilePos, colour: u8) {
        match (pos.0.checked_sub(self.upper_left_tile.0), pos.1.checked_sub(self.upper_left_tile.1)) {
            (Some(x), Some(y)) if x < SCREEN_SIZE as u16 && y < SCREEN_SIZE as u16 => {
                let offset = ((SCREEN_SIZE * y as u32 >> 2) + (x as u32>>2)) as isize;
                let shift = (x & 0b11) << 1;
                let bitmask = 0b11 << shift;
                unsafe {
                    let ptr = (FRAMEBUFFER as *mut u8).offset(offset);
                    ptr.write((ptr.read() & !bitmask) | (colour << shift) );
                }
            }
            _ => (),
        }
    }
}
