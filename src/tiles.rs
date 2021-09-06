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


#[derive(Clone, Copy)]
pub struct JumpyTile;
static JUMPYTILE : [CollisionCenter; 16] = [
    CollisionCenter{rp: cf32::new(-3.0,  4.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new(-1.0,  4.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new( 1.0,  4.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new( 3.0,  4.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new( 4.0,  3.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new( 4.0,  1.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new( 4.0, -1.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new( 4.0, -3.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new( 3.0, -4.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new( 1.0, -4.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new(-1.0, -4.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new(-3.0, -4.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new(-4.0, -3.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new(-4.0, -1.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new(-4.0,  1.0), rad:1.2, el: 0.95},
    CollisionCenter{rp: cf32::new(-4.0,  3.0), rad:1.2, el: 0.95},
];
impl TileType for JumpyTile {
    fn collision_configuration(self) -> &'static [CollisionCenter] {
        &JUMPYTILE
    }
    fn sprite(self) -> Option<&'static [u8; 8]> {
        Some(&sprites::JUMPYTILE)
    }
}



#[derive(Clone, Copy)]
pub struct Ladder1Tile;
static LADDER1_TILE : [CollisionCenter; 16] = [
    CollisionCenter{rp: cf32::new(   3.5,  3.5), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new(   3.0,  3.0), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new(   2.5,  2.5), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new(   2.0,  2.0), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new(   1.5,  1.5), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new(   1.0,  1.0), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new (  0.5,  0.5), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new (  0.0,  0.0), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new ( -0.5, -0.5), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new ( -1.0, -1.0), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new ( -1.5, -1.5), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new ( -2.0, -2.0), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new ( -2.5, -2.5), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new ( -3.0, -3.0), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new ( -3.5, -3.5), rad:1.5, el: 0.01},
    CollisionCenter{rp: cf32::new ( -4.0, -4.0), rad:1.5, el: 0.01},
];
impl TileType for Ladder1Tile {
    fn collision_configuration(self) -> &'static [CollisionCenter] {
        &LADDER1_TILE
    }
    fn sprite(self) -> Option<&'static [u8; 8]> {
        Some(&sprites::LADDER1)
    }
}


#[enum_dispatch::enum_dispatch]
#[derive(Clone, Copy)]
pub enum TileTypeEnum {
    EmptyTile,
    UsualArea1Tile,
    JumpyTile,
    Ladder1Tile,
}

