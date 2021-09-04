mod wasm4;
use wasm4::*;

pub mod utils;

use utils::{draw_colours, sprite8x8, room16x16,  UfmtBuf};
use ufmt::uwrite;

use num_complex::Complex32 as cf32;

const WHEEL: [u8; 8] = sprite8x8(
    "
    . X . . X . . .
    . . X X X X . X
    . X . . . . X .
    X X . . . . X .
    . X . . . . X X
    . X . . . . X .
    X . X X X X . .
    . . . X . . X .
",
);

const SOLIDTILE: [u8; 8] = sprite8x8(
    "
    . X . X . X . X
    X . X . X . X .
    . X . X . X . X
    X . X . X . X .
    . X . X . X . X
    X . X . X . X .
    . X . X . X . X
    X . X . X . X .
",
);

type RoomData = [u32; 16];

const MAP: RoomData = room16x16( "
   |` ```           |
   |        `       |
   |XXXX       ,    |
   |XXXX            |
   |X              X|
   |X   ,``  `,    X|
   |X ,`           X|
   |XXXXXXXXXXXXXXXX|
");

struct Player {
    pos: cf32,
    vel: cf32,
    anim_timer: std::num::Wrapping<u8>,

    power: f32,
}

impl Player {
    const fn new() -> Player {
        Player {
            pos: cf32::new(70.0, 70.0),
            vel: cf32::new(0.0, 0.0),
            power: 50.0,
            anim_timer: std::num::Wrapping(0),
        }
    }
    fn control(&mut self, _prev: u8, cur: u8) {
        let mut dir = cf32::new(0.0, 0.0);

        if cur & BUTTON_LEFT != 0 {
            dir.re -= 1.0;
        }
        if cur & BUTTON_RIGHT != 0 {
            dir.re += 1.0;
        }
        if cur & BUTTON_UP != 0 {
            dir.im -= 1.0;
        }
        if cur & BUTTON_DOWN != 0 {
            dir.im += 1.0;
        }

        let dirnorm = dir.norm();
        if dirnorm > 0.5 {
            self.anim_timer += std::num::Wrapping(1);

            // from 0.5 to 2.2 KiB to wasm size just for this line
            dir = dir.unscale(dirnorm);

            self.vel += dir * self.power / 20.0;
            self.power -= self.power / 20.0;
        }

        self.power += (200.0-self.power)/100.0;

    }
    fn repel_point(&mut self, p : cf32) {
        let mut v =  self.pos - p;
        let vn = v.norm();
        
        /*
        traceln!(
            "repel {} {}   {} {}   {} {} n={}",
            (self.pos.re * 10.0) as i32,
            (self.pos.im * 10.0) as i32,
            (p.re * 10.0) as i32,
            (p.im * 10.0) as i32,
            (v.re * 10.0) as i32,
            (v.im * 10.0) as i32,
            (   vn * 10.0) as i32
        );
        */

        if vn < 7.0 {
            v = v.unscale(vn);
            //self.vel -= self.vel * 1.0 / vn;
            let scale = if vn <= 6.0 { 60.0 } else { 60.0 / (vn - 5.0) };
            self.vel += v.scale(scale);
        }
    }
    fn handle_collisions(&mut self, r: &Room) {
        let (x,y) = self.my_world_coords();
        if x > 0 && y > 0 && r.get_tile(x-1, y-1) != 0 {
            self.repel_point(self.from_world_coords((x-1, y-1))+ cf32::new(2.0, 4.0));
            self.repel_point(self.from_world_coords((x-1, y-1))+ cf32::new(4.0, 2.0));
            self.repel_point(self.from_world_coords((x-1, y-1))+ cf32::new(4.0, 4.0));
        }
        if y > 0 && r.get_tile(x, y-1) != 0 {
            self.repel_point(self.from_world_coords((x, y-1))+ cf32::new(-4.0, 4.0));
            self.repel_point(self.from_world_coords((x, y-1))+ cf32::new(0.0, 4.0));
            self.repel_point(self.from_world_coords((x, y-1))+ cf32::new(4.0, 4.0));
        }
        if y > 0 && r.get_tile(x+1, y-1) != 0 {
            self.repel_point(self.from_world_coords((x+1, y-1))+ cf32::new(-1.0, 4.0));
            self.repel_point(self.from_world_coords((x+1, y-1))+ cf32::new(-3.0, 2.0));
            self.repel_point(self.from_world_coords((x+1, y-1))+ cf32::new(-3.0, 4.0));
        }

        if x > 0 && r.get_tile(x-1, y) != 0 {
            self.repel_point(self.from_world_coords((x-1, y))+ cf32::new( 4.0, -4.0));
            self.repel_point(self.from_world_coords((x-1, y))+ cf32::new( 4.0, 0.0));
            self.repel_point(self.from_world_coords((x-1, y))+ cf32::new( 4.0, 4.0));
        }
        if  r.get_tile(x+1, y) != 0 {
            self.repel_point(self.from_world_coords((x+1, y))+ cf32::new( -2.0, -4.0));
            self.repel_point(self.from_world_coords((x+1, y))+ cf32::new( -2.0, 0.0));
            self.repel_point(self.from_world_coords((x+1, y))+ cf32::new( -2.0, 4.0));
        }


        if x > 0 && r.get_tile(x-1, y+1) != 0 {
            self.repel_point(self.from_world_coords((x-1, y+1))+ cf32::new(2.0, -2.0));
            self.repel_point(self.from_world_coords((x-1, y+1))+ cf32::new(4.0, -0.0));
            self.repel_point(self.from_world_coords((x-1, y+1))+ cf32::new(4.0, -2.0));
        }
        if r.get_tile(x, y+1) != 0 {
            self.repel_point(self.from_world_coords((x, y+1))+ cf32::new(-4.0, -2.0));
            self.repel_point(self.from_world_coords((x, y+1))+ cf32::new(0.0, -2.0));
            self.repel_point(self.from_world_coords((x, y+1))+ cf32::new(4.0, -2.0));
        }
        if r.get_tile(x+1, y+1) != 0 {
            self.repel_point(self.from_world_coords((x+1, y+1))+ cf32::new(-1.0, -2.0));
            self.repel_point(self.from_world_coords((x+1, y+1))+ cf32::new(-3.0,  -0.0));
            self.repel_point(self.from_world_coords((x+1, y+1))+ cf32::new(-3.0, -2.0));
        }
    }
    fn movement(&mut self) {
        self.vel += cf32::new(0.0, 0.1);

        self.pos += self.vel / 2000.0;
        self.vel -= self.vel / 2000.0;
        
        if self.pos.re < 4.0 {
            self.pos.re = 4.0;
            if self.vel.re < 0.0 { self.vel.re = 0.0; }
        }
        if self.pos.re > 1.0*SCREEN_SIZE as f32 - 10.0 {
            self.pos.re = 1.0*SCREEN_SIZE as f32- 10.0;
            if self.vel.re > 0.0 { self.vel.re = 0.0; }
        }
        if self.pos.im < 4.0 {
            self.pos.im = 4.0;
            if self.vel.im < 0.0 { self.vel.im = 0.0; }
        }
        if self.pos.im > 1.0*SCREEN_SIZE as f32 - 10.0 {
            self.pos.im  = 1.0*SCREEN_SIZE as f32 - 10.0;
            if self.vel.im > 0.0 { self.vel.im = 0.0; }
        }
    }
    fn draw(&self, _global_frame: u8) {
        draw_colours(3, 0, 0, 0);
        let bf = if self.anim_timer.0 & 0x1F < 16 {
            0
        } else {
            BLIT_FLIP_X
        };
        blit(&WHEEL, self.pos.re as i32 - 4, self.pos.im as i32 - 4, 8, 8, BLIT_1BPP | bf);
    }

    fn my_world_coords(&self) -> (u16, u16) {
        let x = match self.pos.re {
            t if t <= 24.0 => 0,
            t if t >= 24.0 + 16.0*8.0 => 15,
            t => {
                ((t - 20.0) / 8.0) as u16
            }
        };
        let y = match self.pos.im {
            t if t <= 24.0 => 0,
            t if t >= 24.0 + 16.0*8.0 => 15,
            t => {
                ((t - 20.0) / 8.0) as u16
            }
        };
        (x,y)
    }
    fn from_world_coords(&self, (x,y): (u16, u16)) -> cf32 {
        cf32::new(
            8.0 * x as f32 + 24.0,
            8.0 * y as f32 + 24.0,
        )
    }
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

struct Room {   
}

impl Room {
    const fn new() -> Self {
        Self {

        }
    }

    fn draw(&self, _global_frame: u8, _player_coords:(u16,u16)) {
        for y in 0..16 {
            for x in 0..16 {
                if self.get_tile(x,y) != 0 {
                    let mut col = 2;
                    if (_player_coords.0 as i32 - x as i32).abs() <= 1 && (_player_coords.1 as i32 - y as i32).abs() <= 1  {
                        col = 4;
                    }
                    draw_colours(col, 0, 0, 0);
                    blit(&SOLIDTILE, 20+8*x as i32, 20+8*y as i32, 8, 8, 0);
                }
            }
        }
    }

    fn get_tile(&self, x: u16, y: u16) -> u8 {
        if x >= 16 || y >= 16 {
            return 0;
        }
        ((MAP[y as usize] >> (x as usize*2)) & 0b11) as u8
    }
}

struct State {
    prevpad: u8,
    frame: u8,

    player: Player,
    textbox: TextBox,

    room: Room,
}

impl State {
    pub const fn new() -> State {
        State {
            prevpad: 0,
            frame: 0,
            player: Player::new(),
            textbox: TextBox::new(),
            room: Room::new(),
        }
    }

    pub fn tick(&mut self) {
        unsafe {
            *PALETTE = [0, 0x808080, 0xFFFFFF, 0x8080FF];
        }

        let gamepad = unsafe { *GAMEPAD1 };
        self.player.control(self.prevpad, gamepad);

        for _ in 0..10 {
            self.player.handle_collisions(&self.room);
            self.player.movement();
        }

        self.textbox.control(self.prevpad, gamepad);

        self.room.draw(self.frame, self.player.my_world_coords());
        self.player.draw(self.frame);
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
