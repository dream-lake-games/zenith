use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::prelude::*;

mod dphysics;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
pub struct DebugState {
    show_physics_bounds: bool,
}
impl Default for DebugState {
    fn default() -> Self {
        Self {
            show_physics_bounds: true,
        }
    }
}

#[derive(Resource, Reflect)]
struct DebugInteractive(DebugState);
fn update_debug_state(
    interactive_state: Res<DebugInteractive>,
    debug_state: Res<State<DebugState>>,
    mut next_debug_state: ResMut<NextState<DebugState>>,
) {
    if &interactive_state.0 != debug_state.get() {
        next_debug_state.set(interactive_state.0.clone());
    }
}

fn set_gizmo_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 1.0;
    config.render_layers = SpriteLayer::render_layers();
}

fn debug_startup(mut commands: Commands, ass: Res<AssetServer>) {
    commands.spawn(ShipBundle::new(default()));
    let room_size = (IDEAL_VEC * 2).as_ivec2();

    for xmul in [-1, 0, 1] {
        for ymul in [-1, 0, 1] {
            commands.spawn((
                Name::new("test-grid({xmul},{ymul})"),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(640.0, 360.0)),
                        ..default()
                    },
                    texture: ass.load("play/test-grid.png"),
                    transform: tran_tran!(
                        (room_size.x * xmul) as f32,
                        (room_size.y * ymul) as f32,
                        0.0,
                    ),
                    ..default()
                },
                BgSpriteLayer::render_layers(),
            ));
        }
    }

    // commands.spawn((
    //     Name::new("static_rx_entity"),
    //     spat_tran!(100.0, 100.0),
    //     StaticRx::from_kind_n_shape(StaticRxKind::Normal, Shape::Circle { radius: 30.0 }),
    // ));
}

fn debug_update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut bullet_time: ResMut<BulletTime>,
    mut launch: EventReader<Launch>,
    mut fire: EventReader<Fire>,
    mut ship: Query<(&mut DynoTran, &mut Transform), With<Ship>>,
) {
    if keyboard.just_pressed(KeyCode::BracketLeft) {
        bullet_time.set_normal();
    }
    if keyboard.just_pressed(KeyCode::BracketRight) {
        bullet_time.set_slow();
    }
    for evt in launch.read() {
        println!("launch!");
        for (mut dyno_tran, mut tran) in &mut ship {
            dyno_tran.vel = evt.0;
            tran.set_angle(evt.0.to_angle());
        }
    }
    for _ in fire.read() {
        println!("fire!");
    }
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Tab)),
        );
        app.insert_state(DebugState::default());
        app.add_systems(Startup, set_gizmo_config);

        // Debug
        app.insert_resource(DebugInteractive(DebugState::default()));
        app.add_plugins(
            ResourceInspectorPlugin::<DebugInteractive>::new()
                .run_if(input_toggle_active(false, KeyCode::Tab)),
        );
        app.add_systems(Update, update_debug_state.run_if(in_state(AppMode::Dev)));
        app.add_plugins(
            ResourceInspectorPlugin::<State<MetaState>>::new()
                .run_if(input_toggle_active(false, KeyCode::Tab)),
        );

        // Physics
        dphysics::register_dphysics(app);

        // Random testing
        app.add_systems(Startup, debug_startup);
        app.add_systems(Update, debug_update);
    }
}
