mod wasm4;
use wasm4::*;

pub mod utils;

use utils::{draw_colours, sprite8x8, sprite16x16, room16x16,  UfmtBuf};
use ufmt::uwrite;

use num_complex::Complex32 as cf32;

const _WHEEL: [u8; 8] = sprite8x8(
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

const WHEEL1: [u8; 32] = sprite16x16(
    "
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . X . . . X . . . . . . .  
    . . . . . X . . X . . . X . . .
    . . . . . . X X X X . X . . . .
    . . . . . X . . . . X . . . . .
    . . . X X X . . . . X . . . . .
    . . . . . X . . . . X X X . . .
    . . . . . X . . . . X . . . . .
    . . . . X . X X X X . . . . . .
    . . . . . . X . . . X . . . . .
    . . . . . X . . . . . X . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
",
);

const WHEEL2: [u8; 32] = sprite16x16(
    "
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . X . . . X . . . . .  
    . . . . . . X . . X . . . . . .
    . . . X . . . X X X . . . . . .
    . . . . X . X . . . X X . . . .
    . . . . . X . . . . . X X X . .
    . . . X X X . . . . . X . . . .
    . . . . . X . . . . X . . . . .
    . . . . X . X X X X . X . . . .
    . . . X . . . X . . . . X . . .
    . . . . . . . X . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
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
   |XXXXXX,XXXXXXXXX|
");

struct Player {
    pos: cf32,
    vel: cf32,
    anim_timer: std::num::Wrapping<u8>,

    power: f32,
    jump_dir: Option<f32>,
    grounded: bool,
}

impl Player {
    const fn new() -> Player {
        Player {
            pos: cf32::new(70.0, 70.0),
            vel: cf32::new(0.0, 0.0),
            power: 50.0,
            anim_timer: std::num::Wrapping(0),
            grounded: false,
            jump_dir: None,
        }
    }
    fn jump_strength(cur: u8) -> f32 {
        let (down,up) = (cur & BUTTON_DOWN != 0, cur & BUTTON_UP != 0);
        let strength = match (down,up) {
            (true, false) => 0.7,
            (true, true) => 0.85,
            (false, false) => 0.9,
            (false, true) => 1.0,
        };
        strength
    }
    fn control(&mut self, prev: u8, cur: u8) {
        let mut dir = cf32::new(0.0, 0.0);
        let mut movpower : f32 = 0.0;

        if self.grounded && prev & BUTTON_2 != 0 {
            self.jump_dir = Some(self.jump_dir.unwrap_or_default());
            let jump_dir = self.jump_dir.as_mut().unwrap();
            if cur & BUTTON_2 != 0 {
                if cur & BUTTON_LEFT != 0 {
                    *jump_dir -= 0.02;
                }
                if cur & BUTTON_RIGHT != 0 {
                    *jump_dir += 0.02;
                }
                *jump_dir = jump_dir.clamp(-1.0, 1.0);
            } else {
                dir = cf32::new(self.jump_dir.unwrap(), -1.0);
                let strength = Player::jump_strength(cur);
                movpower = self.power * strength;
                self.jump_dir = None;
            }
        } else {
            self.jump_dir = None;

            let mut brake = false;

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
                if self.grounded {
                    brake = true;
                } else {
                    dir.im += 1.0;
                }
            }

            movpower = if self.grounded {
                self.power / 20.0
            } else {
                self.power / 200.0
            };

            if brake {
                dir = -self.vel;
                movpower = self.power / 5.0;
            }
        }

        let dirnorm = dir.norm();
        if movpower > 0.1 && dirnorm > 0.1 {
            dir = dir.unscale(dir.norm());

            if self.grounded {
                self.vel += dir.scale(movpower)
            } else {
                self.vel += dir.scale(movpower)
            }
            self.power -= movpower
        }


        self.power = 0.95*self.power + 0.05*300.0;

        if self.grounded && self.vel.re.abs() > 5.0 {
            self.anim_timer += std::num::Wrapping(1);
        }
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

        let radius = 5.0;

        if vn < radius+3.0 {
            v = v.unscale(vn);
            let veldir = self.vel.unscale(self.vel.norm());
            let accelerating = (veldir / v).re;
            //traceln!("accel {}", (accelerating*100.0) as i32);
            let fade = 1.0 - (vn - radius)/3.0;
            if fade > 0.01 && v.im < -0.5 {
                self.grounded = true;
            }
            //traceln!("fade {}", (fade*100.0) as i32);
            let mut scale = if vn <= 5.0 { 1.0 } else { fade * fade  };
            if accelerating < 0.0 {
                scale *= 10.0;
            } 
            self.vel += v.scale(scale);
        }
    }
    fn handle_collisions(&mut self, r: &Room) {
        //self.repel_point(cf32::new(70.0, 100.0));
        //return;
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
        self.vel += cf32::new(0.0, 0.5);

        if self.grounded {
            // friction
            self.vel.re -= self.vel.re * 0.002;
            if self.vel.re.abs() < 0.001 {
                self.vel.re = 0.0;
            }
        }

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
    fn draw(&self, _global_frame: u8, keys: u8) {
        draw_colours(3, 0, 0, 0);
        if self.anim_timer.0 & 0x1F < 16 {
            blit(&WHEEL1, self.pos.re as i32 - 8, self.pos.im as i32 - 8, 16, 16, BLIT_1BPP);
        } else {
            blit(&WHEEL2, self.pos.re as i32 - 8, self.pos.im as i32 - 8, 16, 16, BLIT_1BPP);
        };
        if let Some(jump_dir) = self.jump_dir {
            draw_colours(4, 0, 0, 0);
            let mut v = cf32::new(jump_dir, -1.0);
            v = v.unscale(v.norm());
            let strength = Player::jump_strength(keys);
            v = self.pos + v * 1.0 * ((2.0*strength).exp()*2.0 - 0.5); 
            line(self.pos.re as i32 , self.pos.im as i32, v.re as i32, v.im as i32)
        }
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
            self.player.grounded = false;
            self.player.handle_collisions(&self.room);
            self.player.movement();
        }

        self.textbox.control(self.prevpad, gamepad);

        self.room.draw(self.frame, self.player.my_world_coords());
        self.player.draw(self.frame, gamepad);
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
