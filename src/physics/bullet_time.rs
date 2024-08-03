use crate::prelude::*;

#[derive(Resource, Debug, Clone, Reflect)]
pub struct BulletTime {
    time_factor: f32,
    last_delta: Duration,
}
impl BulletTime {
    const NORMAL: f32 = 1.0;
    const SLOW: f32 = 0.2;

    pub fn new() -> Self {
        Self {
            time_factor: 1.0,
            last_delta: default(),
        }
    }

    pub fn delta(&self) -> Duration {
        self.last_delta.mul_f32(self.time_factor)
    }

    pub fn delta_seconds(&self) -> f32 {
        self.last_delta.as_secs_f32() * self.time_factor
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

fn update_bullet_time_delta(mut bullet_time: ResMut<BulletTime>, time: Res<Time>) {
    bullet_time.last_delta = time.delta();
}

pub(super) struct BulletTimePlugin;
impl Plugin for BulletTimePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BulletTime>();
        app.insert_resource(BulletTime::new());
        app.add_systems(FixedPreUpdate, update_bullet_time_delta);
    }
}
