use crate::{sprites, cf32};

pub struct CollisionCenter {
    /// Relative position against center of the tile,
    pub rp: cf32,
    /// Radius to add
    pub rad: f32,
    /// elasticity. 0.9 - jumpy, 0.1 - usual
    pub el: f32,
}

pub trait TileType {
    fn collision_configuration() -> &'static [CollisionCenter];
    fn sprite() -> &'static [u8; 8];
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
    fn collision_configuration() -> &'static [CollisionCenter] {
        &SQUARE_TILE
    }

    fn sprite() -> &'static [u8; 8] {
        &sprites::SOLIDTILE
    }
}
