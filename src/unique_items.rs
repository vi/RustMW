use crate::{LEVEL, TilePos, camera::Camera, cf32, sprites::{CRATE, INFOBOX1, INFOBOX2, STAR1, STAR2}, utils::draw_colours, wasm4::{SCREEN_SIZE, blit, rect, text}, world::World};


#[derive(variant_count::VariantCount, PartialEq, Eq, Copy, Clone, enum_iterator::IntoEnumIterator)]
pub enum UniqueItem {
    PlayerStart,
    InfoWelcome,
    FeatureSmallSize,
    CrateLog,
}

pub enum UniqueItemType {
    Other,
    Infobox,
    Feature,
    Crate,
}

impl UniqueItem {
    pub const fn get_pos(self) -> TilePos {
        LEVEL.unique_item_pos(self)
    }

    pub const fn visible(self) -> bool {
        match self.r#type() {
            UniqueItemType::Other => false,
            UniqueItemType::Infobox => true,
            UniqueItemType::Feature => true,
            UniqueItemType::Crate => true,
        }
    }

    pub const fn r#type(self) -> UniqueItemType {
        match self as usize {
            x if x >= UniqueItem::InfoWelcome as usize && x < UniqueItem::FeatureSmallSize as usize => UniqueItemType::Infobox,
            x if x >= UniqueItem::FeatureSmallSize as usize && x < UniqueItem::CrateLog as usize => UniqueItemType::Feature,
            x if x >= UniqueItem::CrateLog as usize => UniqueItemType::Crate,
            _ => UniqueItemType::Other,
        }
    }

    pub const fn text(self) -> &'static str {
        use UniqueItem::*;
        match self {
            InfoWelcome => "Welcome to\nRustMW",
            CrateLog => "log",
            _ => "",
        }
    }
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

    let mut blinker = frame % 60 < 30;


    if touched {
        blinker = false;
        draw_colours(2,0,0,0);
    } else {
        draw_colours(3,0,0,0);
    }

    let r#type = item.r#type();

    if matches!(r#type, Infobox) && touched_now {
        *inhibit_drawing_player = true;
        draw_colours(2, 0,0,0);
        rect(10, 10, SCREEN_SIZE-20, SCREEN_SIZE-20);
        draw_colours(1, 0,0,0);
        rect(11, 11, SCREEN_SIZE-22, SCREEN_SIZE-22);

        draw_colours(3, 0, 0, 0);
        text(item.text(), 14, 14);
    }


    if matches!(r#type, Crate) && touched_now {
        draw_colours(2, 0,0,0);
        rect(60, 6, 94, 16);
        draw_colours(1, 0,0,0);
        rect(61, 7, 92, 14);

        draw_colours(3, 0, 0, 0);
        text(item.text(), 63, 10);
    }

    if *inhibit_drawing_player {
        return;
    }

    use UniqueItemType::*;
    match (r#type, touched, blinker) {
        (Infobox, _, false) => blit(&INFOBOX1, x-4, y-4, 8, 8, 0),
        (Infobox, _, true) => blit(&INFOBOX2, x-4, y-4, 8, 8, 0),
        (Feature, _, false) => blit(&STAR1, x-4, y-4, 8, 8, 0),
        (Feature, _, true) => blit(&STAR2, x-4, y-4, 8, 8, 0),
        (Crate,   _, _) => blit(&CRATE, x-4, y-4, 8, 8, 0),
        _ => (),
    }
}
