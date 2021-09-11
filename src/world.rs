
use crate::{LEVEL, Level, MAX_UNIQUE_ITEMS_PER_ROOM, TilePos, UniqueItem, camera::Camera, cf32, tiles::{self, TileTypeEnum, TileType}, utils::draw_colours, wasm4::{SCREEN_SIZE, blit}};

pub struct World {   
}

static THE_LEVEL : Level = LEVEL; 

impl World {
    pub const fn new() -> Self {
        Self {

        }
    }

    pub const BOTTOM_RIGHT_TILE: TilePos = (16*8, 16*4);

    pub fn draw(_global_frame: u8, player_coords:TilePos, cam: &Camera) {
        let (camx, camy) = World::to_world_coords(cam.pos);
        let minx = camx.saturating_sub(9);
        let miny = camy.saturating_sub(9);
        for y in miny..(miny+19) {
            for x in minx..(minx+19) {
                if let Some(sprite) =  World::get_tile((x,y)).sprite() {
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

    pub fn get_tile((x,y): TilePos) -> TileTypeEnum {
        if x >= World::BOTTOM_RIGHT_TILE.0 || y >= World::BOTTOM_RIGHT_TILE.1 {
            return tiles::EmptyTile.into();
        }

        let room_x = x >> 4;
        let room_y = y >> 4;
        let within_room_x = x & 0xF;
        let within_room_y = y & 0xF;

        let lowlevel_tile_type = ((THE_LEVEL.the_area.rooms[(room_y*8+room_x) as usize][within_room_y as usize] >> (within_room_x as usize*2)) & 0b11) as u8;
        let meta = THE_LEVEL.the_area.meta[(room_y*8+room_x) as usize];
        match lowlevel_tile_type {
            0 => meta.block_type_sp.unwrap(),
            1 => meta.block_type_x.unwrap(),
            2 => meta.block_type_a.unwrap(),
            3 => meta.block_type_b.unwrap(),
            _ => unreachable!(),
        }
    }

    pub const fn get_unique_items_around_tile((x,y):TilePos) -> [Option<UniqueItem>; MAX_UNIQUE_ITEMS_PER_ROOM*9] {
        let mut found_items = [None; MAX_UNIQUE_ITEMS_PER_ROOM*9];
        let mut i = 0;

        if x >= World::BOTTOM_RIGHT_TILE.0 || y >= World::BOTTOM_RIGHT_TILE.1 {
            return found_items;
        }

        let room_x = x >> 4;
        let room_y = y >> 4;

        let minrx = room_x.saturating_sub(1);
        let minry = room_y.saturating_sub(1);
        let mut maxrx = room_x + 1; if maxrx >= 8 { maxrx = 7; }
        let mut maxry = room_y + 1; if maxry >= 4 { maxry = 3; }

        let mut y : u16 = minry;
        while y <= maxry {
            let mut x : u16 = minrx;
            while x <= maxrx {
                let items = LEVEL.the_area.uniques[(y*8+x) as usize];
                let mut k = 0;
                while k < items.len() {
                    if let Some(item) = items[k] {
                        found_items[i] = Some(item);
                        i+=1;
                    }
                    k += 1;
                }
                x+=1;
            }
            y += 1;
        }

        found_items
    }

    pub fn from_world_coords((x,y): TilePos) -> cf32 {
        cf32::new(
            8.0 * x as f32 + 4.0,
            8.0 * y as f32 + 4.0,
        )
    }

    pub fn to_world_coords(pos: cf32) -> TilePos {
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
