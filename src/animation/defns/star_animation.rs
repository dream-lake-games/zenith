use super::*;

defn_animation!(
    AnimationStar,
    bodies: [
        core: {
            path: "play/7a.png",
            size: (7, 7),
            render_layers: BgSpriteLayer::render_layers(),
        },
        light: {
            path: "play/7aL.png",
            size: (9, 9),
            // scale: (2.0, 2.0),
            render_layers: BgLightLayer::render_layers(),
        },
    ],
    states: [
        Cruise: {
            parts: [
                core,
                light,
            ],
        },
    ],
);
