use crate::{LEVEL, TilePos, UniqueItem, camera::Camera, cf32, sprites::{INFOBOX1, INFOBOX2}, utils::draw_colours, wasm4::{SCREEN_SIZE, blit, rect, text}, world::World};


pub fn draw_unique(item: UniqueItem, frame: u8, player_pos: TilePos, cam: &Camera, inhibit_drawing_player: &mut bool) {
    let posraw = LEVEL.unique_item_pos(item);
    let pos = World::from_world_coords(posraw);

    let touched_now = posraw == player_pos;

    let center = pos - cam.pos + cf32::new(0.5, 0.5) * SCREEN_SIZE as f32;
    if center.re < 4.5 || center.im < 4.5 || center.re + 4.5 > SCREEN_SIZE as f32  || center.im + 4.5 >= SCREEN_SIZE as f32 {
        return;
    }
    let (x, y) = (center.re as i32, center.im as i32);

    let blinker = frame % 60 < 30;

    draw_colours(3,0,0,0);

    use UniqueItem::*;
    match (item, blinker, touched_now) {
        (Info1, false, false) => blit(&INFOBOX1, x-4, y-4, 8, 8, 0),
        (Info1, true, false) => blit(&INFOBOX2, x-4, y-4, 8, 8, 0),
        (Info1, _, true) => {
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
