use crate::{cf32, player::Player, wasm4::{BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP}};

pub struct Camera {
    pub pos: cf32,
    inertia: cf32,
    limit: u8,
    look_down_ctr: u8,
}

impl Camera {
    pub const fn new() -> Camera {
        Camera {
            pos: cf32::new(70.0, 70.0),
            inertia: cf32::new(0.0, 0.0),
            limit: 0,
            look_down_ctr: 0,
        }
    }

    pub fn update(&mut self, p : &Player, keys: u8) {
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
