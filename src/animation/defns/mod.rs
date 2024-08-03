use super::manager::*;
use crate::prelude::*;

defn_animation!(
    AnimationLenny,
    bodies: [
        egg: {
            path: "play/egg.png",
            size: (24, 24),
        },
        damage: {
            path: "play/fly_damage.png",
            size: (24, 24),
            length: 3,
            fps: 1.0,
        },
        fly: {
            path: "play/fly.png",
            size: (24, 24),
            length: 3,
        },
        light: {
            path: "play/spotlight.png",
            size: (48, 48),
            scale: (4.0, 4.0),
            render_layers: LightLayer::render_layers(),
        },
    ],
    states: [
        Fly: {
            parts: [
                fly,
                light,
            ],
        },
        Hurt: {
            parts: [
                damage,
                light,
            ],
            next: Fly,
        },
        DieDamage: {
            parts: [
                damage,
            ],
            next: DieEgg,
        },
        DieEgg: {
            parts: [
                egg,
            ],
            #[special]
            next: HideThenDie(1.0),
        }
    ],
);

fn test_startup(mut commands: Commands) {
    commands.spawn((
        Name::new("test_entity"),
        SpatialBundle::default(),
        AnimationManagerBundle::<AnimationLenny>::new(),
    ));
}

fn test_update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut managers: Query<&mut AnimationManager<AnimationLenny>>,
) {
    for mut manager in &mut managers {
        if keyboard.just_pressed(KeyCode::KeyF) {
            manager.set_state(AnimationLenny::Fly);
        }
        if keyboard.just_pressed(KeyCode::KeyE) {
            manager.set_state(AnimationLenny::DieEgg);
        }
    }
}

pub(super) struct AnimationDefnsPlugin;
impl Plugin for AnimationDefnsPlugin {
    fn build(&self, app: &mut App) {
        // TESTING
        app.add_systems(Startup, test_startup);
        app.add_systems(Update, test_update);

        register_animation_manager::<AnimationLenny>(app);
    }
}
