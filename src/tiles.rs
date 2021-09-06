use crate::{sprites, cf32};

pub struct CollisionCenter {
    /// Relative position against center of the tile,
    pub rp: cf32,
    /// Radius to add
    pub rad: f32,
    /// elasticity. 0.9 - jumpy, 0.1 - usual
    pub el: f32,
}


#[enum_dispatch::enum_dispatch(TileTypeEnum)]
pub trait TileType {
    fn collision_configuration(self) -> &'static [CollisionCenter];
    fn sprite(self) -> Option<&'static [u8; 8]>;
}


#[derive(Clone, Copy)]
pub struct EmptyTile;
static EMRTY_TILE : [CollisionCenter; 0] = [];
impl TileType for EmptyTile {
    fn collision_configuration(self) -> &'static [CollisionCenter] {
        &EMRTY_TILE
    }
    fn sprite(self) -> Option<&'static [u8; 8]> {
        None
    }
}


#[derive(Clone, Copy)]
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
    fn collision_configuration(self) -> &'static [CollisionCenter] {
        &SQUARE_TILE
    }
    fn sprite(self) -> Option<&'static [u8; 8]> {
        Some(&sprites::SOLIDTILE)
    }
}

#[enum_dispatch::enum_dispatch]
#[derive(Clone, Copy)]
pub enum TileTypeEnum {
    EmptyTile,
    UsualArea1Tile,
}

