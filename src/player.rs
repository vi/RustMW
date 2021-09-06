
use crate::{Camera, cf32};
use crate::wasm4::{BLIT_1BPP, BUTTON_2, BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, SCREEN_SIZE, blit, line};
use crate::World;
use crate::utils::draw_colours;
use crate::sprites;

pub struct Player {
    pub pos: cf32,
    pub vel: cf32,
    pub anim_timer: std::num::Wrapping<u8>,

    pub power: f32,
    pub jump_dir: Option<f32>,
    pub grounded: bool,
}

impl Player {
    pub const fn new() -> Player {
        Player {
            pos: cf32::new(f32::NAN, f32::NAN),
            vel: cf32::new(0.0, 0.0),
            power: 50.0,
            anim_timer: std::num::Wrapping(0),
            grounded: false,
            jump_dir: None,
        }
    }
    pub fn jump_strength(cur: u8) -> f32 {
        let (down,up) = (cur & BUTTON_DOWN != 0, cur & BUTTON_UP != 0);
        let strength = match (down,up) {
            (true, false) => 0.7,
            (true, true) => 0.85,
            (false, false) => 0.9,
            (false, true) => 1.0,
        };
        strength
    }
    pub fn control(&mut self, prev: u8, cur: u8) {
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
    fn repel_point(&mut self, p : cf32, acceleration: &mut cf32) {
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
            *acceleration += v.scale(scale);
        }
    }
    pub fn handle_collisions(&mut self, r: &World, acceleration: &mut cf32 ) {
        //rp(cf32::new(70.0, 100.0));
        //return;
        let fwc = World::from_world_coords;
        let (x,y) = self.my_world_coords();
        let mut rp = |p|self.repel_point(p, acceleration);
        if x > 0 && y > 0 && r.get_tile(x-1, y-1) != 0 {
            rp(fwc((x-1, y-1))+ cf32::new(2.0, 4.0));
            rp(fwc((x-1, y-1))+ cf32::new(4.0, 2.0));
            rp(fwc((x-1, y-1))+ cf32::new(4.0, 4.0));
        }
        if y > 0 && r.get_tile(x, y-1) != 0 {
            rp(fwc((x, y-1))+ cf32::new(-4.0, 4.0));
            rp(fwc((x, y-1))+ cf32::new(0.0, 4.0));
            rp(fwc((x, y-1))+ cf32::new(4.0, 4.0));
        }
        if y > 0 && r.get_tile(x+1, y-1) != 0 {
            rp(fwc((x+1, y-1))+ cf32::new(-1.0, 4.0));
            rp(fwc((x+1, y-1))+ cf32::new(-3.0, 2.0));
            rp(fwc((x+1, y-1))+ cf32::new(-3.0, 4.0));
        }

        if x > 0 && r.get_tile(x-1, y) != 0 {
            rp(fwc((x-1, y))+ cf32::new( 4.0, -4.0));
            rp(fwc((x-1, y))+ cf32::new( 4.0, 0.0));
            rp(fwc((x-1, y))+ cf32::new( 4.0, 4.0));
        }
        if  r.get_tile(x+1, y) != 0 {
            rp(fwc((x+1, y))+ cf32::new( -2.0, -4.0));
            rp(fwc((x+1, y))+ cf32::new( -2.0, 0.0));
            rp(fwc((x+1, y))+ cf32::new( -2.0, 4.0));
        }


        if x > 0 && r.get_tile(x-1, y+1) != 0 {
            rp(fwc((x-1, y+1))+ cf32::new(2.0, -2.0));
            rp(fwc((x-1, y+1))+ cf32::new(4.0, -0.0));
            rp(fwc((x-1, y+1))+ cf32::new(4.0, -2.0));
        }
        if r.get_tile(x, y+1) != 0 {
            rp(fwc((x, y+1))+ cf32::new(-4.0, -2.0));
            rp(fwc((x, y+1))+ cf32::new(0.0, -2.0));
            rp(fwc((x, y+1))+ cf32::new(4.0, -2.0));
        }
        if r.get_tile(x+1, y+1) != 0 {
            rp(fwc((x+1, y+1))+ cf32::new(-1.0, -2.0));
            rp(fwc((x+1, y+1))+ cf32::new(-3.0,  -0.0));
            rp(fwc((x+1, y+1))+ cf32::new(-3.0, -2.0));
        }
    }
    pub fn movement(&mut self, acceleration: &mut cf32) {
        *acceleration += cf32::new(0.0, 0.5);

        if self.grounded {
            // friction
            *acceleration -= cf32::new(self.vel.re * 0.002, 0.0);
            if self.vel.re.abs() < 0.001 {
                self.vel.re = 0.0;
            }
        }

        *acceleration -= self.vel / 2000.0;
         
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
    pub fn draw(&self, _global_frame: u8, keys: u8, cam: &Camera) {
        draw_colours(3, 0, 0, 0);
        let onscreen = self.pos - cam.pos + cf32::new(0.5, 0.5) * SCREEN_SIZE as f32;
        if self.anim_timer.0 & 0x1F < 16 {
            blit(&sprites::WHEEL1, onscreen.re as i32 - 8, onscreen.im as i32 - 8, 16, 16, BLIT_1BPP);
        } else {
            blit(&sprites::WHEEL2, onscreen.re as i32 - 8, onscreen.im as i32 - 8, 16, 16, BLIT_1BPP);
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

    pub fn my_world_coords(&self) -> (u16, u16) {
        World::to_world_coords(self.pos)
    }
}
