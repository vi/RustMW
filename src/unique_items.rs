use crate::{LEVEL, TilePos, camera::Camera, cf32, sprites::{INFOBOX1, INFOBOX2}, utils::draw_colours, wasm4::{SCREEN_SIZE, blit, rect, text}, world::World};


#[derive(variant_count::VariantCount, PartialEq, Eq, Copy, Clone)]
pub enum UniqueItem {
    PlayerStart,
    Info1,
}

const fn bitfield_len(x: usize) -> usize {
    (x+7)/8
}
const fn bitfield_byte(x: usize) -> usize {
    x / 8
}
const fn bitfield_bitmask(x: usize) -> u8 {
    0x1 << (x & 7)
}

pub struct TouchedUniqueItems {
    data: [u8; bitfield_len(UniqueItem::VARIANT_COUNT)],
}

impl TouchedUniqueItems {
    pub const fn new() -> TouchedUniqueItems {
        TouchedUniqueItems { data: [0;  bitfield_len(UniqueItem::VARIANT_COUNT)]}
    }

    pub fn touch(&mut self, item: UniqueItem) {
        let x = item as usize;
        self.data[bitfield_byte(x)] |= bitfield_bitmask(x);
    }

    pub fn is_touched(&self, item: UniqueItem) -> bool {
        let x = item as usize;
        (self.data[bitfield_byte(x)] & bitfield_bitmask(x)) != 0
    }
}


pub fn draw_unique(item: UniqueItem, frame: u8, player_pos: TilePos, cam: &Camera, inhibit_drawing_player: &mut bool, status: &mut TouchedUniqueItems) {
    let posraw = LEVEL.unique_item_pos(item);
    let pos = World::from_world_coords(posraw);

    let touched_now = posraw == player_pos;
    let mut touched = status.is_touched(item);
    if touched_now && !touched {
        status.touch(item);
        touched = true;
    } 

    let center = pos - cam.pos + cf32::new(0.5, 0.5) * SCREEN_SIZE as f32;
    if center.re < 4.5 || center.im < 4.5 || center.re + 4.5 > SCREEN_SIZE as f32  || center.im + 4.5 >= SCREEN_SIZE as f32 {
        return;
    }
    let (x, y) = (center.re as i32, center.im as i32);

    let blinker = frame % 60 < 30;

    draw_colours(3,0,0,0);

    use UniqueItem::*;
    match (item, touched, touched_now, blinker) {
        (Info1, false, false, false) => blit(&INFOBOX1, x-4, y-4, 8, 8, 0),
        (Info1, true, false, _) => blit(&INFOBOX1, x-4, y-4, 8, 8, 0),
        (Info1, false, false, true) => blit(&INFOBOX2, x-4, y-4, 8, 8, 0),
        (Info1, true, true, _) => {
            *inhibit_drawing_player = true;
            draw_colours(2, 0,0,0);
            rect(10, 10, SCREEN_SIZE-20, SCREEN_SIZE-20);
            draw_colours(1, 0,0,0);
            rect(11, 11, SCREEN_SIZE-22, SCREEN_SIZE-22);

            draw_colours(3, 0, 0, 0);
            text("Welcome to\nRustMW", 14, 14);
        }
        _ => (),
    }
}
