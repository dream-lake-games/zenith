use crate::prelude::*;

defn_texture!(
    TextureNormalPlanet,
    textures: [
        egg: {
            path: "play/egg.png",
            size: (24, 24),
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
