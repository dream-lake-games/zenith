use super::*;

defn_animation!(
    AnimationSuicido,
    bodies: [
        wander: {
            path: "enemies/suicido/wander.png",
            size: (16, 16),
            length: 4,
        },
        charge: {
            path: "enemies/suicido/charge.png",
            size: (16, 16),
            length: 4,
            fps: 12.0,
        },
        explode: {
            path: "enemies/suicido/explode.png",
            size: (16, 16),
            length: 4,
        },
    ],
    states: [
        Charge: {
            parts: [
                charge,
            ],
        },
        Wander: {
            parts: [
                wander,
            ],
        },
        Explode: {
            parts: [
                explode,
            ],
        }
    ],
);
