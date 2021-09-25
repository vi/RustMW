use crate::{sprites, cf32};

pub struct CollisionSegment {
    /// Relative position against center of the tile, start of the segment
    pub rp1: cf32,
     /// Relative position against center of the tile, end of the segment
    pub rp2: cf32,
    /// Radius to add
    pub rad: f32,
    /// elasticity. 0.9 - jumpy, 0.1 - usual
    pub el: f32,
}


#[enum_dispatch::enum_dispatch(TileTypeEnum)]
pub trait TileType {
    fn collision_configuration(self) -> &'static [CollisionSegment];
    fn sprite(self) -> Option<&'static [u8; 8]>;
    fn map_viewer_colour(self) -> u8;
}


#[derive(Clone, Copy)]
pub struct EmptyTile;
static EMRTY_TILE : [CollisionSegment; 0] = [];
impl TileType for EmptyTile {
    fn collision_configuration(self) -> &'static [CollisionSegment] {
        &EMRTY_TILE
    }
    fn sprite(self) -> Option<&'static [u8; 8]> {
        None
    }
    fn map_viewer_colour(self) -> u8 {
        0
    }
}


#[derive(Clone, Copy)]
pub struct UsualArea1Tile;
static SQUARE_TILE : [CollisionSegment; 4] = [
    CollisionSegment{rp1: cf32::new(-3.0,  -3.0), rp2: cf32::new(3.0,  -3.0), rad:1.0, el: 0.01},
    CollisionSegment{rp1: cf32::new( 3.0,  -3.0), rp2: cf32::new(3.0,   3.0), rad:1.0, el: 0.01},
    CollisionSegment{rp1: cf32::new( 3.0,   3.0), rp2: cf32::new(-3.0,  3.0), rad:1.0, el: 0.01},
    CollisionSegment{rp1: cf32::new(-3.0,   3.0), rp2: cf32::new(-3.0, -3.0), rad:1.0, el: 0.01},
];
impl TileType for UsualArea1Tile {
    fn collision_configuration(self) -> &'static [CollisionSegment] {
        &SQUARE_TILE
    }
    fn sprite(self) -> Option<&'static [u8; 8]> {
        Some(&sprites::SOLIDTILE)
    }
    fn map_viewer_colour(self) -> u8 {
        1
    }
}


#[derive(Clone, Copy)]
pub struct JumpyTile;
static JUMPYTILE : [CollisionSegment; 4] = [
    CollisionSegment{rp1: cf32::new(-3.5,  -3.5), rp2: cf32::new( 3.5,  -3.5), rad:1.5, el: 0.95},
    CollisionSegment{rp1: cf32::new( 3.5,  -3.5), rp2: cf32::new( 3.5,   3.5), rad:1.5, el: 0.95},
    CollisionSegment{rp1: cf32::new( 3.5,   3.5), rp2: cf32::new(-3.5,   3.5), rad:1.5, el: 0.95},
    CollisionSegment{rp1: cf32::new(-3.5,   3.5), rp2: cf32::new(-3.5,  -3.5), rad:1.5, el: 0.95},
];
impl TileType for JumpyTile {
    fn collision_configuration(self) -> &'static [CollisionSegment] {
        &JUMPYTILE
    }
    fn sprite(self) -> Option<&'static [u8; 8]> {
        Some(&sprites::JUMPYTILE)
    }
    fn map_viewer_colour(self) -> u8 {
        1
    }
}



#[derive(Clone, Copy)]
pub struct Ladder1Tile;
static LADDER1_TILE : [CollisionSegment; 3] = [
    CollisionSegment{rp1: cf32::new(-4.0,  -4.0), rp2: cf32::new(4.0,   4.0), rad:1.5, el: 0.01},
    CollisionSegment{rp1: cf32::new( 4.0,   4.0), rp2: cf32::new(-4.0,  4.0), rad:1.5, el: 0.01},
    CollisionSegment{rp1: cf32::new(-4.0,   4.0), rp2: cf32::new(-4.0, -4.0), rad:1.5, el: 0.01},
];
impl TileType for Ladder1Tile {
    fn collision_configuration(self) -> &'static [CollisionSegment] {
        &LADDER1_TILE
    }
    fn sprite(self) -> Option<&'static [u8; 8]> {
        Some(&sprites::LADDER1)
    }
    fn map_viewer_colour(self) -> u8 {
        1
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

/// TODO: make this automatic somehow
pub const fn tile_type_enum_eq(a: TileTypeEnum, b: TileTypeEnum) -> bool {
    use TileTypeEnum::*;
    match (a, b) {
        (EmptyTile(..), EmptyTile(..)) => true,
        (UsualArea1Tile(..), UsualArea1Tile(..)) => true,
        (JumpyTile(..), JumpyTile(..)) => true,
        (Ladder1Tile(..), Ladder1Tile(..)) => true,
        _ => false,
    }
}
