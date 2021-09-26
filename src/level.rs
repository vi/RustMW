use crate::utils::{ll_char_descriptions, room16x16};
use crate::{Area, AreaSource, RoomData, UniqueItemPositions, tile_type, tile_types_mapping, unique_items_mapping};


const FIRST_MAP: RoomData = room16x16( b"
|` ```           |
|        `       |
|XXXX       ,    |
|XXXX            |
|X              X|
|X   ,``  `,    X|
|X ,`           X|
|XXXXXX,XXXXXXXXX|
");

pub const AREA1: (Area, UniqueItemPositions) = Area::build(AreaSource {
    empty_tile_style: tile_type!(EmptyTile),
    solid_tile_style: tile_type!(UsualArea1Tile),
    // First char of triplet is identifier. Second one is upper cell type, third one is lower cell type.
    // The same character is also used in tile types mapping and unique items mapping
    //    `X` means solid tile,
    //    `.` means empty tile,
    //    `A` means custom tile A, where specific tile type is determined y tile types mapping
    //    `B` ...               B, ...
    //        Same character cannot have both A and B in it. Individual room cannot assign different tile mappings to A or B.
    //    `!` - position of a unique item. In area itself it is an empty tile.
    char_lookup: ll_char_descriptions::<9>(b"s!. J.A jAX l.B LBX S!. i.!  M!. G.!"),
    tile_lookup: tile_types_mapping![(JumpyTile J j) (Ladder1Tile L l)],
    item_lookup: unique_items_mapping![(PlayerStart s) (PlayerStart! S) (InfoWelcome i) (FeatureSmallSize M) (CrateLog G)],
    cells: b"                                                                                                       <
|` ```           ` ```           ` ```           ` ```           ` ```           ` ```           ` ```           ` ```           |
|        `               `               `               `               `               `               `               `       |
|XXXX       ,     XXX                            XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    |
|XXXX                                            XXXX            XXXX            XXXX            XXXX            XXXX            |
|X                                                              XX              XX              XX              XX              X|
|X   ,``  `,    XXXXXXXXXXXXXXXXXX   XXXXXXXXXXXXXXXXX````X  X  XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    X|
|X ,`           XX ,`           XX  `           XX ,`     X  X  XX ,`           XX ,`                                           X|
|XXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXX  XXX,XXXXXXXXXXXXXXX,XXX  XJJXXXXXXX,XXXXXXXXXXXXXXX,X                             XXXXXXXXXXX|
|` ```           ` ```           `               ` ```    X  X   ` ```           ` ```                                XXXXXXXXXXX|
|        `               `        jj    ``               `X  X           `     XXX           ,,,XX                    XXXXXXXXXXX|
|XXXX       ,    XXXX       ,    XXXX       ,    XXXX     X  X   XXXX       ,  XXXXXX  M                              XXXXXXXXXXX|
|XXXX            XXXX            XXXX            XXXX     X  X   XXXX    XXXXXXXXXXXX  X           XXX                XXXXXXXXXXX|
|X              XX              XX              XX        X  X  XX       XXXXXXXXXXX`  X                              XXXXXXXXXXX|
|X   ,``  `,    XX   ,``  `,    XX   ,`               ``  X  X  XX   ,`` XXXXXXXXXX   jX       XXX                    XXXXXXXXXXX|
|X ,`           XX ,`           XX ,`                     X  X  XX ,`    XXXXXXXXXX  XXX                              XXXXXXXXXXX|
|XXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXX,                 X,XXX  X XXXXXXXX,XXXXXXXXXXX    X    XXX                       XXXXXXXXXXX|
|` ```           ` ```           XXX`            ` ```    X  X   ` ```   XXXXXXXXXXXX  X                              XXXXXXXXXXX|
|        `               `       XXXXLl                   X  X           XXXXXXXXXX    X  XXX                         XXXXXXXXXXX|
|XXXX       ,    XXXX       ,    XXXXXXLl                 X  X     G            `X  ,XXX                          i s XXXXXXXXXXX|
|XXXX            XXXX            XXXXXXXXLl                      XXXX    XXXXXX     XXXX XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|
|X              XX              XX   XXXXXXLl                   XX       XXXXXXXXXX XXXX XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|
|X   ,``  `,    XX   ,``  `,    XX   XXXXXXXXXLl          jjj   XX   ,`` XXXXXXXXX  XXXX XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|
|X ,`           XX ,`           XX ,`XXXXXXXXXXXXXXl            XX ,`    XXXXXXXXX XXXXX XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|
|XXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXX XXXXX XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|
|` ```           ` ```           ` ```           ` ```           ` ```   XXXXXX  ` XXXXX XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|
|        `               `               `               `               `               XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|
|XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX    XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|
|XXXX            XXXX            XXXX            XXXX            XXXX            XXXX    XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|
|X              XX              XX              XX              XX              XX       XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|
|X   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,`` XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|
|X ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`    XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|
|XXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXX|
",
});
