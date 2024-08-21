use super::*;

defn_animation!(
    AnimationRingShrink,
    bodies: [
        core: {
            path: "juice/ring_shrink.png",
            size: (16, 28),
            length: 6,
            fps: 16.0,
            scale: (0.5, 0.5),
        },
        light: {
            path: "juice/ring_shrink_light.png",
            size: (16, 28),
            length: 6,
            fps: 16.0,
            scale: (0.5, 0.5),
            render_layers: LightLayer::render_layers(),
        },
    ],
    states: [
        Shrinking: {
            parts: [
                core,
                light,
            ],
            #[special]
            next: HideThenDie(0.0),
        },
    ],
);
