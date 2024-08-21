use super::*;

defn_animation!(
    AnimationSuicidoBody,
    bodies: [
        charge: {
            path: "enemies/suicido/charge.png",
            size: (16, 16),
            length: 4,
            fps: 16.0,
        },
        launch: {
            path: "enemies/suicido/launch.png",
            size: (16, 16),
            length: 4,
            fps: 16.0,
        },
        explode: {
            path: "enemies/suicido/explode.png",
            size: (16, 16),
            length: 4,
            fps: 8.0,
        },
        stable_light: {
            path: "enemies/suicido/pinkball_light_stable.png",
            size: (28, 28),
            render_layers: LightLayer::render_layers(),
        },
        charging_light: {
            path: "enemies/suicido/pinkball_light_charging.png",
            size: (28, 28),
            length: 2,
            render_layers: LightLayer::render_layers(),
        },
        warning_circle: {
            path: "enemies/suicido/warning_circle.png",
            size: (64, 64),
            length: 3,
            fps: 18.0,
        },
        warning_circle_light: {
            path: "enemies/suicido/warning_circle.png",
            size: (64, 64),
            length: 3,
            fps: 18.0,
            render_layers: LightLayer::render_layers(),
        },
    ],
    states: [
        Charge: {
            parts: [
                charge,
                charging_light,
            ],
        },
        Launch: {
            parts: [
                launch,
                stable_light,
            ],
        },
        Explode: {
            parts: [
                explode,
                stable_light,
                warning_circle,
                warning_circle_light,
            ],
            #[special]
            next: HideThenDie(0.1),
        }
    ],
);

defn_animation!(
    AnimationSuicidoExplosionCircle,
    bodies: [
        core: {
            path: "enemies/suicido/explosion_circle.png",
            size: (64, 64),
            length: 4,
            fps: 12.0,
        },
        light: {
            path: "enemies/suicido/explosion_circle.png",
            size: (64, 64),
            length: 4,
            fps: 12.0,
            render_layers: LightLayer::render_layers(),
        },
    ],
    states: [
        Charge: {
            parts: [
                core,
                light,
            ],
            #[special]
            next: HideThenDie(0.1),
        },
    ],
);
