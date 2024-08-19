use crate::prelude::*;

#[derive(Resource, Debug, Clone, Reflect)]
pub struct BulletTime {
    time_factor: f32,
    main_duration: Duration,
}
impl BulletTime {
    const NORMAL: f32 = 1.0;
    const SLOW: f32 = 0.2;

    pub fn new() -> Self {
        Self {
            time_factor: 1.0,
            main_duration: Duration::default(),
        }
    }

    pub fn delta(&self) -> Duration {
        self.main_duration
    }

    pub fn delta_seconds(&self) -> f32 {
        self.main_duration.as_secs_f32()
    }

    pub fn set_normal(&mut self) {
        self.set_time_factor(Self::NORMAL);
    }

    pub fn set_slow(&mut self) {
        self.set_time_factor(Self::SLOW);
    }

    pub fn set_time_factor(&mut self, factor: f32) {
        self.time_factor = factor;
    }
}

fn update_bullet_time(mut bullet_time: ResMut<BulletTime>, time: Res<Time>) {
    bullet_time.main_duration = time.delta().mul_f32(bullet_time.time_factor);
}

fn drive_bullet_time(
    mut bullet_time: ResMut<BulletTime>,
    ships: Query<&ShipLaunchState, With<Ship>>,
) {
    let any_ship_launching = ships
        .iter()
        .any(|launch_state| launch_state.current_launch.is_some());
    if any_ship_launching {
        bullet_time.set_slow();
    } else {
        bullet_time.set_normal();
    }
}

pub(super) struct BulletTimePlugin;
impl Plugin for BulletTimePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BulletTime>();
        app.insert_resource(BulletTime::new());
        app.add_systems(First, update_bullet_time);
        app.add_systems(Update, drive_bullet_time);
    }
}
