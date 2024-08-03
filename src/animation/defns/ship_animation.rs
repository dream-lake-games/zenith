use super::*;

defn_animation!(
    AnimationShip,
    bodies: [
        ship: {
            path: "play/egg.png",
            size: (24, 24),
        },
        spotlight: {
            path: "play/spotlight.png",
            size: (48, 48),
            scale: (2.0, 2.0),
            render_layers: LightLayer::render_layers(),
        },
    ],
    states: [
        Cruise: {
            parts: [
                ship,
                spotlight,
            ],
        },
    ],
);
