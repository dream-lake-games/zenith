use super::*;

defn_animation!(
    AnimationSuicidoBody,
    bodies: [
        charge: {
            path: "enemies/suicido/charge.png",
            size: (16, 16),
            length: 4,
        },
        launch: {
            path: "enemies/suicido/launch.png",
            size: (16, 16),
            length: 4,
            fps: 12.0,
        },
        explode: {
            path: "enemies/suicido/explode.png",
            size: (16, 16),
            length: 4,
            fps: 12.0,
        },
    ],
    states: [
        Charge: {
            parts: [
                charge,
            ],
        },
        Launch: {
            parts: [
                launch,
            ],
        },
        Explode: {
            parts: [
                explode,
            ],
            #[special]
            next: HideThenDie(0.1),
        }
    ],
);

defn_animation!(
    AnimationSuicidoAnticipation,
    bodies: [
        core: {
            path: "enemies/suicido/anticipation.png",
            size: (64, 64),
            length: 4,
        },
    ],
    states: [
        Charge: {
            parts: [
                core,
            ],
            #[special]
            next: HideThenDie(0.1),
        },
    ],
);
