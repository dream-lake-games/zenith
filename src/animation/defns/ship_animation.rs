use super::*;

defn_animation!(
    AnimationShipBody,
    bodies: [
        core: {
            path: "ship/ship_body.png",
            size: (24, 24),
        },
        light: {
            path: "ship/ship_body_light.png",
            size: (24, 24),
            render_layers: LightLayer::render_layers(),
        },
        spotlight: {
            path: "ship/ship_spotlight.png",
            size: (96, 96),
            render_layers: LightLayer::render_layers(),
        }
    ],
    states: [
        Default: {
            parts: [
                core,
                light,
                spotlight,
            ],
        },
    ],
);

defn_animation!(
    AnimationShipGun,
    bodies: [
        core: {
            path: "ship/ship_gun.png",
            size: (24, 24),
            offset: Vec3::new(0.0, 0.0, 0.1),
        },
        light: {
            path: "ship/ship_gun_light.png",
            size: (24, 24),
            render_layers: LightLayer::render_layers(),
        },
    ],
    states: [
        Default: {
            parts: [
                core,
                light,
            ],
        },
    ],
);

defn_animation!(
    AnimationShipTail,
    bodies: [
        core: {
            path: "ship/ship_tail.png",
            size: (24, 24),
            offset: Vec3::new(0.0, 0.0, -0.1),
        },
        light: {
            path: "ship/ship_tail_light.png",
            size: (24, 24),
            render_layers: LightLayer::render_layers(),
        },
    ],
    states: [
        Default: {
            parts: [
                core,
                light,
            ],
        },
    ],
);

defn_animation!(
    AnimationShipBulletDefault,
    bodies: [
        core: {
            path: "ship/weapons/bullet.png",
            size: (10, 10),
            offset: Vec3::new(0.0, 0.0, -0.2),
            scale: (0.5, 0.5),
        },
        light: {
            path: "ship/weapons/bullet.png",
            size: (10, 10),
            scale: (0.5, 0.5),
            render_layers: LightLayer::render_layers(),
        },
    ],
    states: [
        Default: {
            parts: [
                core,
                light,
            ],
        },
    ],
);
