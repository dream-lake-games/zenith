use super::*;

defn_animation!(
    AnimationSuicido,
    bodies: [
        ship: {
            path: "play/egg.png",
            size: (24, 24),
            scale: (0.5, 0.5),
        },
        spotlight: {
            path: "play/spotlight.png",
            size: (48, 48),
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
