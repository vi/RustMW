mod wasm4;
use wasm4::*;

pub mod utils;

use utils::{draw_colours, sprite8x8, sprite16x16, room16x16,  UfmtBuf};
use ufmt::uwrite;

use num_complex::Complex32 as cf32;

static _WHEEL: [u8; 8] = sprite8x8(
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

static WHEEL1: [u8; 32] = sprite16x16(
    "
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
    . . . . . . . . . . . . . . . .
",
);

static WHEEL2: [u8; 32] = sprite16x16(
    "
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
    . . . . . . . . . . . . . . . .
",
);

static SOLIDTILE: [u8; 8] = sprite8x8(
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


#[derive(Clone, Copy)]
struct RoomMetadata {
    block_type_sp: u8,
    block_type_x: u8,
    block_type_a: u8,
    block_type_b: u8,
}

// 8x4 block of rooms
type RoomBlock = [RoomData; 32];

struct Area {
    rooms: [RoomData; 32],
    meta: [RoomMetadata; 32],
    player_starting_point: Option<(u16,u16)>,
}

impl Area {
    const fn new(s: &'static [u8]) -> Area {
        let char_lookup = [CharDescription {
            chr: b'S',
            upper: LowlevelCellType::Special,
            lower: LowlevelCellType::Empty,
        }];
        let (rooms, specials) = utils::makearea(s, char_lookup);
        let meta = [RoomMetadata {
            block_type_sp: 0,
            block_type_x: 1,
            block_type_a: 2,
            block_type_b: 3,
        }; 32];
        let mut player_starting_point = None;

        let mut i = 0;
        while i < specials.len() {
            if let Some(s) = specials[i] {
                if s.chr == b'S' {
                    player_starting_point = Some((s.x, s.y));
                }
            }
            i+=1;
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


const MAP: RoomData = room16x16( b"
   |` ```           |
   |        `       |
   |XXXX       ,    |
   |XXXX            |
   |X              X|
   |X   ,``  `,    X|
   |X ,`           X|
   |XXXXXX,XXXXXXXXX|
");

static AREA1: Area = Area::new(b"                                                                                                       <
   |` ```           ` ```           ` ```           ` ```           ` ```           ` ```           ` ```           ` ```           |
   |        `               `               `               `               `               `               `               `       |
   |XXXX       ,     XXX       ,     XXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    |
   |XXXX                            XXXX            XXXX            XXXX            XXXX            XXXX            XXXX            |
   |X                                                              XX              XX              XX              XX              X|
   |X   ,``  `,    XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX`````     XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    X|
   |X ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`           X|
   |XXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXX  XXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXX|
   |` ```           ` ```           ` ```           ` ```           ` ```           ` ```           ` ```           ` ```           |
   |        `               `               `               `               `               `               `               `       |
   |XXXX       ,    XXXX       ,    XXXX       ,    XXXX            XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    |
   |XXXX            XXXX            XXXX            XXXX            XXXX            XXXX            XXXX            XXXX            |
   |X              XX              XX              XX              XX              XX              XX              XX              X|
   |X   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``        XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    X|
   |X ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`           X|
   |XXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XX   XXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXX|
   |` ```           ` ```           ` ```           ` ```           ` ```           ` ```           ` ```           ` ```           |
   |        `               `               `                               `               `               `               `       |
   |XXXX       ,    XXXX       ,    XXXX       ,               ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    |
   |XXXX            XXXX            XXXXXXXXX,                      XXXX            XXXX            XXXX            XXXX            |
   |X              XX              XX   XXXXXXX,                   XX              XX              XX              XX              X|
   |X   ,``  `,    XX   ,``  `,    XX   XXXXXXXXXX,          `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    X|
   |X ,`           XX ,`           XX ,`XXXXXXXXXXXXXX,            XX ,`           XX ,`           XX ,`           XX ,`           X|
   |XXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXX|
   |` ```           ` ```           ` ```           ` ```           ` ```           ` ```           ` ```           ` ```           |
   |        `               `               `               `               `               `               `               `       |
   |XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    |
   |XXXX            XXXX            XXXX            XXXX            XXXX            XXXX            XXXX            XXXX            |
   |X              XX              XX              XX              XX              XX              XX              XX              X|
   |X   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    X|
   |X ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`           X|
   |XXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXX|
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
                    *jump_dir -= 0.03;
                }
                if cur & BUTTON_RIGHT != 0 {
                    *jump_dir += 0.03;
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
    fn handle_collisions(&mut self, r: &World) {
        //self.repel_point(cf32::new(70.0, 100.0));
        //return;
        let fwc = World::from_world_coords;
        let (x,y) = self.my_world_coords();
        if x > 0 && y > 0 && r.get_tile(x-1, y-1) != 0 {
            self.repel_point(fwc((x-1, y-1))+ cf32::new(2.0, 4.0));
            self.repel_point(fwc((x-1, y-1))+ cf32::new(4.0, 2.0));
            self.repel_point(fwc((x-1, y-1))+ cf32::new(4.0, 4.0));
        }
        if y > 0 && r.get_tile(x, y-1) != 0 {
            self.repel_point(fwc((x, y-1))+ cf32::new(-4.0, 4.0));
            self.repel_point(fwc((x, y-1))+ cf32::new(0.0, 4.0));
            self.repel_point(fwc((x, y-1))+ cf32::new(4.0, 4.0));
        }
        if y > 0 && r.get_tile(x+1, y-1) != 0 {
            self.repel_point(fwc((x+1, y-1))+ cf32::new(-1.0, 4.0));
            self.repel_point(fwc((x+1, y-1))+ cf32::new(-3.0, 2.0));
            self.repel_point(fwc((x+1, y-1))+ cf32::new(-3.0, 4.0));
        }

        if x > 0 && r.get_tile(x-1, y) != 0 {
            self.repel_point(fwc((x-1, y))+ cf32::new( 4.0, -4.0));
            self.repel_point(fwc((x-1, y))+ cf32::new( 4.0, 0.0));
            self.repel_point(fwc((x-1, y))+ cf32::new( 4.0, 4.0));
        }
        if  r.get_tile(x+1, y) != 0 {
            self.repel_point(fwc((x+1, y))+ cf32::new( -2.0, -4.0));
            self.repel_point(fwc((x+1, y))+ cf32::new( -2.0, 0.0));
            self.repel_point(fwc((x+1, y))+ cf32::new( -2.0, 4.0));
        }


        if x > 0 && r.get_tile(x-1, y+1) != 0 {
            self.repel_point(fwc((x-1, y+1))+ cf32::new(2.0, -2.0));
            self.repel_point(fwc((x-1, y+1))+ cf32::new(4.0, -0.0));
            self.repel_point(fwc((x-1, y+1))+ cf32::new(4.0, -2.0));
        }
        if r.get_tile(x, y+1) != 0 {
            self.repel_point(fwc((x, y+1))+ cf32::new(-4.0, -2.0));
            self.repel_point(fwc((x, y+1))+ cf32::new(0.0, -2.0));
            self.repel_point(fwc((x, y+1))+ cf32::new(4.0, -2.0));
        }
        if r.get_tile(x+1, y+1) != 0 {
            self.repel_point(fwc((x+1, y+1))+ cf32::new(-1.0, -2.0));
            self.repel_point(fwc((x+1, y+1))+ cf32::new(-3.0,  -0.0));
            self.repel_point(fwc((x+1, y+1))+ cf32::new(-3.0, -2.0));
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
         
        /*
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
        */
    }
    fn draw(&self, _global_frame: u8, keys: u8, cam: &Camera) {
        draw_colours(3, 0, 0, 0);
        let onscreen = self.pos - cam.pos + cf32::new(0.5, 0.5) * SCREEN_SIZE as f32;
        if self.anim_timer.0 & 0x1F < 16 {
            blit(&WHEEL1, onscreen.re as i32 - 8, onscreen.im as i32 - 8, 16, 16, BLIT_1BPP);
        } else {
            blit(&WHEEL2, onscreen.re as i32 - 8, onscreen.im as i32 - 8, 16, 16, BLIT_1BPP);
        };
        if let Some(jump_dir) = self.jump_dir {
            draw_colours(4, 0, 0, 0);
            let mut v = cf32::new(jump_dir, -1.0);
            v = v.unscale(v.norm());
            let strength = Player::jump_strength(keys);
            v = onscreen + v * 1.0 * ((2.0*strength).exp()*2.0 - 0.5); 
            line(onscreen.re as i32 , onscreen.im as i32, v.re as i32, v.im as i32)
        }
    }

    fn my_world_coords(&self) -> (u16, u16) {
        World::to_world_coords(self.pos)
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

struct World {   
}

impl World {
    const fn new() -> Self {
        Self {

        }
    }

    fn draw(&self, _global_frame: u8, player_coords:(u16,u16), cam: &Camera) {
        let (camx, camy) = World::to_world_coords(cam.pos);
        let minx = camx.saturating_sub(9);
        let miny = camy.saturating_sub(9);
        for y in miny..(miny+19) {
            for x in minx..(minx+19) {
                if self.get_tile(x,y) != 0 {
                    let mut col = 2;
                    if (player_coords.0 as i32 - x as i32).abs() <= 1 && (player_coords.1 as i32 - y as i32).abs() <= 1  {
                        col = 4;
                    }
                    let upperleft = (8.0 * cf32::new(x as f32, y as f32)) - cam.pos + cf32::new(0.5, 0.5) * SCREEN_SIZE as f32;
                    if upperleft.re < 0.5 || upperleft.im < 0.5 || upperleft.re + 8.5 > SCREEN_SIZE as f32  || upperleft.im + 8.5 >= SCREEN_SIZE as f32 {
                        continue;
                    }
                    draw_colours(col, 0, 0, 0);
                    blit(&SOLIDTILE, upperleft.re as i32, upperleft.im as i32, 8, 8, 0);
                }
            }
        }
    }

    fn get_tile(&self, x: u16, y: u16) -> u8 {
        if x >= 16*8 || y >= 16*4 {
            return 0;
        }

        let room_x = x >> 4;
        let room_y = y >> 4;
        let within_room_x = x & 0xF;
        let within_room_y = y & 0xF;


        ((AREA1.rooms[(room_y*8+room_x) as usize][within_room_y as usize] >> (within_room_x as usize*2)) & 0b11) as u8
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

struct Camera {
    pos: cf32,
    inertia: cf32,
    limit: u8,
    look_down_ctr: u8,
}

impl Camera {
    const fn new() -> Camera {
        Camera {
            pos: cf32::new(70.0, 70.0),
            inertia: cf32::new(0.0, 0.0),
            limit: 0,
            look_down_ctr: 0,
        }
    }

    fn update(&mut self, p : &Player, keys: u8) {
        let mut futurepos = p.pos + p.vel*0.1;
        if p.jump_dir.is_some() {
            futurepos += cf32::new(0.0, -30.0);
        }

        if !p.grounded && p.jump_dir.is_none() {
            if keys & BUTTON_UP != 0 {
                futurepos += cf32::new(0.0, -20.0);
            }
            if keys & BUTTON_LEFT != 0 {
                futurepos += cf32::new(-20.0, 0.0);
            }
            if keys & BUTTON_RIGHT != 0 {
                futurepos += cf32::new(20.0, 0.0);
            }
            if keys & BUTTON_DOWN != 0 {
                futurepos += cf32::new(00.0, 50.0);
            }
        } else if p.jump_dir.is_none() {
            if keys & BUTTON_DOWN != 0 {
                if self.look_down_ctr > 100 {
                    if keys & BUTTON_UP != 0 {
                        futurepos += cf32::new(0.0, -48.0);
                    } else
                    if keys & BUTTON_LEFT != 0 {
                        futurepos += cf32::new(-48.0, 0.0);
                    } else 
                    if keys & BUTTON_RIGHT != 0 {
                        futurepos += cf32::new(48.0, 0.0);
                    } else {
                        futurepos += cf32::new(0.0, 48.0);
                    }
                } else {
                    self.look_down_ctr += 1;
                }
            } else {
                self.look_down_ctr = 0;
            }
        }


        let discr = (self.pos - futurepos).norm();
        let inertia_norm = self.inertia.norm();
        if discr < 20.0 || (discr < 28.0 && inertia_norm < 0.1) {
            if self.limit == 0 {
                self.inertia = cf32::new(0.0, 0.0);
            } else {
                //traceln!("lim {}", self.limit);
                self.limit -= 1;
            }
        } else {
            let delta = futurepos - self.pos;
            if inertia_norm > 0.01 {
                let delta_normalized = delta.unscale(delta.norm());
                let inertia_normalized = self.inertia.unscale(inertia_norm);
                if (delta_normalized/inertia_normalized).re < 0.0 {
                    // angle > 90 deg
                    self.inertia=cf32::new(0.0,0.0);
                    return;
                }
            }
            self.inertia = 0.98 * self.inertia + 0.02*delta;
            self.limit = 5;
        }
        self.pos += 0.05 * self.inertia;
        self.inertia *= 0.99;
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
        State {
            prevpad: 0,
            frame: 0,
            camera: Camera::new(),
            player: Player::new(),
            textbox: TextBox::new(),
            room: World::new(),
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
        
        self.camera.update(&self.player, gamepad);
        self.room.draw(self.frame, self.player.my_world_coords(), &self.camera);
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
