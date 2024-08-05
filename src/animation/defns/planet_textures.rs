use crate::prelude::*;

defn_texture!(
    TextureTestPlanet,
    textures: [
        blue: {
            path: "play/texture_blue.png",
            size: (36, 36),
            z_offset: 0.1,
        }
        red: {
            path: "play/texture_red.png",
            size: (36, 36),
        }
        red_high: {
            path: "play/texture_red.png",
            size: (36, 36),
            z_offset: 0.2,
        }
    ],
    parts: [
        Inner,
        Outer,
    ],
    states: [
        BlueInner: [
            Inner: blue,
            Outer: red,
        ],
        RedInner: [
            Inner: red_high,
            Outer: blue,
        ]
    ],
);

defn_texture!(
    TextureNormalPlanet,
    textures: [
        egg: {
            path: "play/texture.png",
            size: (36, 36),
        },
    ],
    parts: [
        Single,
    ],
    states: [
        Default: [
            Single: egg,
        ],
    ],
);
