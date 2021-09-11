
use crate::tiles::{CollisionSegment, TileType};
use crate::{Camera, TilePos, cf32};
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
    remembered_jump : u8,
    pub grounded: bool,

    /// To move better diagonally when we are on a slope
    pub ground_level: cf32,
    ground_level_score: f32,
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
            remembered_jump: 0,
            ground_level: cf32::new(1.0, 0.0),
            ground_level_score: 0.3,
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

        let mut do_jump_now = false;
        if prev & BUTTON_2 != 0 {
            self.jump_dir = Some(self.jump_dir.unwrap_or_default());
            let jump_dir = self.jump_dir.as_mut().unwrap();
            if cur & BUTTON_LEFT != 0 {
                *jump_dir -= 0.03;
            }
            if cur & BUTTON_RIGHT != 0 {
                *jump_dir += 0.03;
            }
            *jump_dir = jump_dir.clamp(-1.0, 1.0);
            if cur & BUTTON_2 == 0  {
                if self.grounded {
                    do_jump_now = true;
                } else {
                    self.remembered_jump = 12;
                }
            }
        } else {
            let mut brake = false;

            if !self.grounded {
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
            } else {
                // grounded
                if cur & BUTTON_LEFT != 0 {
                    //dir.re -= 1.0;
                    //crate::traceln!("{} {}", (self.ground_force_direction.re * 10.0) as i32, (self.ground_force_direction.im * 10.0) as i32);
                    dir -= self.ground_level;
                }
                if cur & BUTTON_RIGHT != 0 {
                    //dir.re += 1.0;
                    //crate::traceln!("{} {}", (self.ground_force_direction.re * 10.0) as i32, (self.ground_force_direction.im * 10.0) as i32);
                    dir += self.ground_level;
                }
                if cur & BUTTON_UP != 0 {
                    //dir.im -= 1.0;
                }
                if cur & BUTTON_DOWN != 0 {
                    brake = true;
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

        if self.remembered_jump > 0 {
            if self.grounded && self.vel.im < 0.0 {
                do_jump_now = true;
            }
            self.remembered_jump -= 1;
        } else {
            if prev & BUTTON_2 == 0 {
                self.jump_dir = None;
            }

        }

        if do_jump_now {
            if self.vel.im > 0.0 {
                self.vel.im = 0.0;
            }
            dir = cf32::new(self.jump_dir.unwrap(), -1.0);
            let strength = Player::jump_strength(cur);
            movpower = self.power * strength;
            self.jump_dir = None;
            self.remembered_jump = 0;
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
    fn repel_tile(&mut self, tile_center: cf32, config: &[CollisionSegment], acceleration: &mut cf32) {
        const DEBUG_REPEL : bool = false;

        if config.is_empty() {
            return;
        }

        if DEBUG_REPEL {
            crate::traceln!(
                ":::: {} {}",
                (self.pos.re * 10.0) as i32,
                (self.pos.im * 10.0) as i32,
            );
        }

        let mut attained_distance: f32 = f32::INFINITY;
        // dummy initial values:
        let mut chosen_vector: cf32 = cf32::new(f32::NAN, f32::NAN);
        let mut chosen_vector_norm: f32 = f32::NAN;
        let mut chosen_segment: &CollisionSegment = &config[0];


        for colsegm in config {
            let pos1 = tile_center + colsegm.rp1;
            let pos2 = tile_center + colsegm.rp2;

            let x;
            let projected_forcepoint = {
                let a = self.pos - pos1;
                let b = pos2 - pos1;
                x = (a / b).re.clamp(0.0, 1.0);
                pos1 * (1.0 - x) + pos2 * x
            };

            let v =  self.pos - projected_forcepoint;
            let vn = v.norm();
            
            if DEBUG_REPEL {
                crate::traceln!(
                    "cand @ {}  {} {}",
                    (x*100.0) as i32,
                    (projected_forcepoint.re * 10.0) as i32,
                    (projected_forcepoint.im * 10.0) as i32,
                );
                crate::traceln!(
                    "vector {} {} n={}",
                    (v.re * 10.0) as i32,
                    (v.im * 10.0) as i32,
                    (   vn * 100.0) as i32
                );
            }

            if attained_distance > vn + colsegm.rad {
                chosen_vector = v;
                chosen_vector_norm = vn;
                chosen_segment = colsegm;
                attained_distance = vn + colsegm.rad;
                if DEBUG_REPEL { crate::traceln!("new cand",); }
            }
        }

        let mut radius = 1.5;
        let feather = 3.0;
        radius += chosen_segment.rad;

        if chosen_vector_norm < radius+feather {
            chosen_vector = chosen_vector.unscale(chosen_vector_norm);
            let veldir = self.vel.unscale(self.vel.norm());
            let accelerating = (veldir / chosen_vector).re;
            //traceln!("accel {}", (accelerating*100.0) as i32);
            let fade = 1.0 - (chosen_vector_norm - radius)/feather;

            if DEBUG_REPEL {
                crate::traceln!(
                    "within_range fade={}",
                    (fade * 100.0) as i32,
                );
                crate::traceln!("accel={}", (accelerating*100.0) as i32 );
            }
            if fade > 0.01 && chosen_vector.im < -0.5 {
                if DEBUG_REPEL { crate::traceln!("grounding",); }
                self.grounded = true;

                let mut new_ground_level = chosen_segment.rp2 - chosen_segment.rp1;
                new_ground_level = new_ground_level.unscale(new_ground_level.norm());
                if new_ground_level.re < 0.0 {
                    new_ground_level = - new_ground_level;
                }
                if self.ground_level_score < new_ground_level.re {
                    self.ground_level_score = new_ground_level.re;
                    self.ground_level = new_ground_level;
                    if DEBUG_REPEL { 
                        crate::traceln!(
                            "new grlvl {} {}",
                            (100.0 * self.ground_level.re) as i32,
                            (100.0 * self.ground_level.im) as i32,
                        ); 
                    }
                }
            }

            //traceln!("fade {}", (fade*100.0) as i32);
            let mut scale = if chosen_vector_norm <= radius { 1.0 } else { fade * fade  };
            scale *= 200.0;
            if accelerating > 0.01 {
                scale *= accelerating; // prevent lateral forces
                scale *= chosen_segment.el;
                if scale < 0.3*chosen_segment.el {
                    scale = 0.0;
                }
            } else if accelerating < -0.02 {
                scale *= -accelerating; // prevent lateral forces
                if scale < 0.01 {
                    scale = 0.0;
                }
            }

            if DEBUG_REPEL {
                crate::traceln!(
                    "act dir=({} {}) scale={}",
                    (chosen_vector.re * 10.0) as i32,
                    (chosen_vector.im * 10.0) as i32,
                    (scale * 100.0) as i32
                );
            }
            *acceleration += chosen_vector.scale(scale);
        }
    }
    pub fn handle_collisions(&mut self, acceleration: &mut cf32 ) {
        self.ground_level_score = 0.3; // do not touch ground level if it is detected this steep;
        //rp(cf32::new(70.0, 100.0));
        //return;

        let (myx,myy) = self.my_world_coords();

        let xx = myx.saturating_sub(1);
        let yy = myy.saturating_sub(1);

        for y in yy..(yy+3) {
            for x in xx..(xx+3) {
                if x == myx && y == myy { continue }
                let tiletype = World::get_tile((x, y));
                self.repel_tile(World::from_world_coords((x,y)), tiletype.collision_configuration(), acceleration);
            }
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
            if self.grounded {
                draw_colours(4, 0, 0, 0);
            } else {
                draw_colours(2, 0, 0, 0);
            }
            let mut v = cf32::new(jump_dir, -1.0);
            v = v.unscale(v.norm());
            let strength = Player::jump_strength(keys);
            v = onscreen + v * 1.0 * ((2.0*strength).exp()*2.0 - 0.5); 
            line(onscreen.re as i32 , onscreen.im as i32, v.re as i32, v.im as i32)
        }
        if self.grounded {
            draw_colours(4, 0, 0, 0);
            let p1 = onscreen - self.ground_level*2.0 + cf32::new(0.0, 4.0);
            let p2 = onscreen + self.ground_level*2.0 + cf32::new(0.0, 4.0);
            line(p1.re as i32, p1.im as i32, p2.re as i32, p2.im as i32);
        }
    }

    pub fn my_world_coords(&self) -> TilePos {
        World::to_world_coords(self.pos)
    }
}
