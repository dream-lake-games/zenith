use super::manager::*;
use crate::prelude::*;

pub mod ship_animation;

pub use ship_animation::*;

fn test_startup(mut commands: Commands) {
    commands.spawn(ShipBundle::new(default()));

    // commands.spawn((
    //     Name::new("static_rx_entity"),
    //     spat_tran!(100.0, 100.0),
    //     StaticRx::from_kind_n_shape(StaticRxKind::Normal, Shape::Circle { radius: 30.0 }),
    // ));
}

fn test_update(
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

pub(super) struct AnimationDefnsPlugin;
impl Plugin for AnimationDefnsPlugin {
    fn build(&self, app: &mut App) {
        // TESTING
        app.add_systems(Startup, test_startup);
        app.add_systems(Update, test_update);

        register_animation_manager::<AnimationShip>(app);
    }
}
