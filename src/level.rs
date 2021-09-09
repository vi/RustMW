use crate::utils::{room16x16};
use crate::{Area, RoomData, SpecialPositions};


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

pub const AREA1: (Area, SpecialPositions) = Area::build(b"                                                                                                       <
|` ```           ` ```           ` ```           ` ```           ` ```           ` ```           ` ```           ` ```           |
|        `               `               `               `               `               `               `               `       |
|XXXX       ,     XXX                            XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    |
|XXXX                                            XXXX            XXXX            XXXX            XXXX            XXXX            |
|X                                                              XX              XX              XX              XX              X|
|X   ,``  `,    XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX````X  X  XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    X|
|X ,`           XX ,`           XX ,`           XX ,`     X  X  XX ,`           XX ,`           XX ,`           XX ,`           X|
|XXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXX  XJJXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXX|
|` ```           ` ```           ` ```           ` ```    X  X   ` ```           ` ```           ` ```           ` ```           |
|        `               `               `               `X  X           `     XXX       `               `               `       |
|XXXX       ,    XXXX       ,    XXXX       ,    XXXX     X  X   XXXX       ,  XXXXXX       ,    XXXX       ,    XXXX       ,    |
|XXXX            XXXX            XXXX            XXXX     X  X   XXXX    XXXXXXXXXXXX  X XXXX    XXXX            XXXX            |
|X              XX              XX              XX        X  X  XX       XXXXXXXXXXX`  X XXXX   XX              XX              X|
|X   ,``  `,    XX   ,``  `,    XX   ,`               ``  X  X  XX   ,`` XXXXXXXXXX   jX XXXX   XX   ,``  `,    XX   ,``  `,    X|
|X ,`           XX ,`           XX ,`                     X  X  XX ,`    XXXXXXXXXX  XXX XXXX   XX ,`           XX ,`           X|
|XXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXX,                 X,XXX  X XXXXXXXX,XXXXXXXXXXX    X XXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXX|
|` ```           ` ```           XXX`            ` ```    X  X   ` ```   XXXXXXXXXXXX  X XXXX    ` ```           ` ```           |
|        `               `       XXXXLl                S  X  X           XXXXXXXXXX    X XXXX            `               `       |
|XXXX       ,    XXXX       ,    XXXXXXLl                 X  X                  `X  ,XXX XXXX    XXXX       ,    XXXX       ,    |
|XXXX            XXXX            XXXXXXXXLl                      XXXX    XXXXXX     XXXX XXXX    XXXX            XXXX            |
|X              XX              XX   XXXXXXLl                   XX       XXXXXXXXXX XXXX XXXX   XX              XX              X|
|X   ,``  `,    XX   ,``  `,    XX   XXXXXXXXXLl          jjj   XX   ,`` XXXXXXXXX  XXXX XXXX   XX   ,``  `,    XX   ,``  `,    X|
|X ,`           XX ,`           XX ,`XXXXXXXXXXXXXXl            XX ,`    XXXXXXXXX XXXXX XXXX   XX ,`           XX ,`           X|
|XXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXX XXXXX XXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXX|
|` ```           ` ```           ` ```           ` ```           ` ```   XXXXXX  ` XXXXX XXXX    ` ```           ` ```           |
|        `               `               `               `               `               `               `               `       |
|XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    XXXX       ,    |
|XXXX            XXXX            XXXX            XXXX            XXXX            XXXX            XXXX            XXXX            |
|X              XX              XX              XX              XX              XX              XX              XX              X|
|X   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    XX   ,``  `,    X|
|X ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`           XX ,`           X|
|XXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXXXXXXXX,XXXXXXXXX|
");
