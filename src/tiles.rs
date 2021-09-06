use crate::{sprites, cf32};

pub struct CollisionCenter {
    /// Relative position against center of the tile,
    pub rp: cf32,
    /// Radius to add
    pub rad: f32,
    /// elasticity. 0.9 - jumpy, 0.1 - usual
    pub el: f32,
}


#[enum_dispatch::enum_dispatch(TileTypes)]
pub trait TileType {
    fn collision_configuration(&self) -> &'static [CollisionCenter];
    fn sprite(&self) -> &'static [u8; 8];
}

pub struct UsualArea1Tile;

static SQUARE_TILE : [CollisionCenter; 16] = [
    CollisionCenter{rp: cf32::new(-3.0,  4.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new(-1.0,  4.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new( 1.0,  4.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new( 3.0,  4.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new( 4.0,  3.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new( 4.0,  1.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new( 4.0, -1.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new( 4.0, -3.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new( 3.0, -4.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new( 1.0, -4.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new(-1.0, -4.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new(-3.0, -4.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new(-4.0, -3.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new(-4.0, -1.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new(-4.0,  1.0), rad:1.2, el: 0.01},
    CollisionCenter{rp: cf32::new(-4.0,  3.0), rad:1.2, el: 0.01},
];

impl TileType for UsualArea1Tile {
    fn collision_configuration(&self) -> &'static [CollisionCenter] {
        &SQUARE_TILE
    }

    fn sprite(&self) -> &'static [u8; 8] {
        &sprites::SOLIDTILE
    }
}

#[enum_dispatch::enum_dispatch]
pub enum TileTypes {
    UsualArea1Tile,
}
