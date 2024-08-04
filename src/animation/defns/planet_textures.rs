use crate::prelude::*;

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
