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
    config.line_width = 2.0;
    config.render_layers = SpriteLayer::render_layers();
}

fn debug_startup(mut commands: Commands, camera_root: Res<DynamicCameraRoot>) {
    let room_state = RoomState::xth_encounter(EncounterKind::SimpOnly, 1);

    let ship_id = commands.spawn(ShipBundle::new(default(), &room_state)).id();

    commands.spawn(PlanetBundle::new(
        "wrap1",
        StaticTxKind::Normal,
        Vec2::new(0.0, room_state.room_size.y as f32 / 2.0),
        Shape::Circle {
            center: Vec2::ZERO,
            radius: 15.0,
        },
        &room_state,
    ));

    let freestyle_shape = Shape::Circle {
        center: default(),
        radius: 10.0,
    };
    let mut freestyle_shapes = vec![freestyle_shape.clone()];
    for offset in room_state.mirage_offsets() {
        freestyle_shapes.push(freestyle_shape.clone().with_offset(offset));
    }
    commands.spawn((
        Name::new("freestyle_trigger_tx"),
        TriggerTx::from_kind_n_shapes(TriggerKind::Ship, freestyle_shapes),
        spat_tran!(-80.0, room_state.room_size.y as f32 / 2.0),
    ));

    spawn_stars(&mut commands, 100, 2.0, 2.0..10.0, camera_root.eid());

    commands.spawn(SuicidoBundle::new(
        Vec2::new(0.0, -10.0),
        ship_id,
        &room_state,
    ));
}

fn debug_update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut bullet_time: ResMut<BulletTime>,
    mut launch: EventReader<Launch>,
    mut fire: EventReader<Fire>,
    mut ship: Query<(Entity, &mut DynoTran, &mut Transform), With<Ship>>,
    mut planet_textures: Query<&mut TextureManager<TextureTestPlanetState>>,
) {
    if keyboard.just_pressed(KeyCode::BracketLeft) {
        bullet_time.set_normal();
    }
    if keyboard.just_pressed(KeyCode::BracketRight) {
        bullet_time.set_slow();
    }
    if keyboard.just_pressed(KeyCode::Space) {
        for mut planet_texture in &mut planet_textures {
            let next_color = match planet_texture.get_state() {
                TextureTestPlanetState::BlueInner => TextureTestPlanetState::RedInner,
                TextureTestPlanetState::RedInner => TextureTestPlanetState::BlueInner,
            };
            planet_texture.set_state(next_color);
        }
    }
    for evt in launch.read() {
        for (eid, mut dyno_tran, mut tran) in &mut ship {
            commands.entity(eid).remove::<Stuck>();
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
        app.add_systems(Startup, debug_startup.after(CameraSet));
        app.add_systems(Update, debug_update);
    }
}
