use crate::prelude::*;

defn_texture!(
    TextureTestPlanet,
    textures: [
        blue: {
            path: "play/texture_blue.png",
            size: (36, 36),
            z_offset: 1.0,
        }
        red: {
            path: "play/texture_red.png",
            size: (36, 36),
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
            Inner: red,
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
