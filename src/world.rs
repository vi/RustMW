
use crate::{camera::Camera, cf32, level, tiles::{self, TileTypeEnum, TileType}, utils::draw_colours, wasm4::{SCREEN_SIZE, blit}};

pub struct World {   
}

impl World {
    pub const fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&self, _global_frame: u8, player_coords:(u16,u16), cam: &Camera) {
        let (camx, camy) = World::to_world_coords(cam.pos);
        let minx = camx.saturating_sub(9);
        let miny = camy.saturating_sub(9);
        for y in miny..(miny+19) {
            for x in minx..(minx+19) {
                if let Some(sprite) =  self.get_tile(x,y).sprite() {
                    let mut col = 2;
                    if (player_coords.0 as i32 - x as i32).abs() <= 1 && (player_coords.1 as i32 - y as i32).abs() <= 1  {
                        col = 4;
                    }
                    let upperleft = (8.0 * cf32::new(x as f32, y as f32)) - cam.pos + cf32::new(0.5, 0.5) * SCREEN_SIZE as f32;
                    if upperleft.re < 0.5 || upperleft.im < 0.5 || upperleft.re + 8.5 > SCREEN_SIZE as f32  || upperleft.im + 8.5 >= SCREEN_SIZE as f32 {
                        continue;
                    }
                    draw_colours(col, 0, 0, 0);
                    blit(sprite, upperleft.re as i32, upperleft.im as i32, 8, 8, 0);
                }
            }
        }
    }

    pub fn get_tile(&self, x: u16, y: u16) -> TileTypeEnum {
        if x >= 16*8 || y >= 16*4 {
            return tiles::EmptyTile.into();
        }

        let room_x = x >> 4;
        let room_y = y >> 4;
        let within_room_x = x & 0xF;
        let within_room_y = y & 0xF;


        let lowlevel_tile_type = ((level::AREA1.rooms[(room_y*8+room_x) as usize][within_room_y as usize] >> (within_room_x as usize*2)) & 0b11) as u8;
        match lowlevel_tile_type {
            0 => tiles::EmptyTile.into(),
            1 => tiles::UsualArea1Tile.into(),
            _ => todo!(),
        }
    }

    pub fn from_world_coords((x,y): (u16, u16)) -> cf32 {
        cf32::new(
            8.0 * x as f32 + 4.0,
            8.0 * y as f32 + 4.0,
        )
    }

    pub fn to_world_coords(pos: cf32) -> (u16, u16) {
        let x = match pos.re {
            t if t <= 4.0 => 0,
            t if t >= 4.0 + 16.0*8.0*8.0 => 127,
            t => {
                ((t - 0.0) / 8.0) as u16
            }
        };
        let y = match pos.im {
            t if t <= 4.0 => 0,
            t if t >= 4.0 + 16.0*8.0*4.0 => 63,
            t => {
                ((t - 0.0) / 8.0) as u16
            }
        };
        (x,y)
    }
}
