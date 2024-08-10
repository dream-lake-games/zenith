use crate::prelude::*;

/// An object that is following another entity
/// This works by updating it's DynoTran, subject to a given accelleration and max speed
#[derive(Component, Debug, Clone, Reflect)]
pub struct Follow {
    eid: Entity,
    accel: f32,
    max_speed: f32,
    /// If provided, will not do anything when target is in this range
    acceptable_dist_range_sq: Option<(f32, f32)>,
}
impl Follow {
    pub fn new(eid: Entity, accel: f32, max_speed: f32) -> Self {
        Self {
            eid,
            accel,
            max_speed,
            acceptable_dist_range_sq: None,
        }
    }

    pub fn set_accel(&mut self, accel: f32) {
        self.accel = accel;
    }

    pub fn set_max_speed(&mut self, max_speed: f32) {
        self.max_speed = max_speed;
    }

    pub fn set_acceptable_dist_range(&mut self, range: (f32, f32)) {
        self.acceptable_dist_range_sq = Some((range.0.powi(2), range.1.powi(2)));
    }

    pub fn with_acceptable_dist_range(mut self, range: (f32, f32)) -> Self {
        self.set_acceptable_dist_range(range);
        self
    }
}

fn update_follow(
    mut follow: Query<(&Follow, &mut DynoTran, &GlobalTransform)>,
    shared_data: Query<&GlobalTransform>,
    bullet_time: Res<BulletTime>,
    meta_state: Res<State<MetaState>>,
) {
    let wrap_size = meta_state
        .get()
        .get_room_state()
        .map(|room_state| room_state.room_size.as_vec2())
        .unwrap_or(IDEAL_VEC_f32);
    for (follow, mut dyno_tran, gtran) in &mut follow {
        let Ok(target_gtran) = shared_data.get(follow.eid) else {
            continue;
        };
        let target_tran = target_gtran.translation().truncate();
        let my_tran = gtran.translation().truncate();
        let dist_left = (target_tran.x - my_tran.x).rem_euclid(wrap_size.x);
        let dist_right = (my_tran.x - target_tran.x).rem_euclid(wrap_size.x);
        let dist_up = (target_tran.y - my_tran.y).rem_euclid(wrap_size.y);
        let dist_down = (my_tran.y - target_tran.y).rem_euclid(wrap_size.y);
        let diff = Vec2 {
            x: if dist_left < dist_right {
                dist_left
            } else {
                -dist_right
            },
            y: if dist_up < dist_down {
                dist_up
            } else {
                -dist_down
            },
        };
        let dist_sq = diff.length_squared();
        if let Some((min_dist_sq, max_dist_sq)) = follow.acceptable_dist_range_sq {
            if dist_sq >= min_dist_sq && dist_sq <= max_dist_sq {
                // We're in the acceptable range, chill
                continue;
            }
        }
        let dir = diff.normalize_or_zero();
        let accel = if let Some((min_dist_sq, _)) = follow.acceptable_dist_range_sq {
            if diff.length_squared() < min_dist_sq {
                dir * -follow.accel
            } else {
                dir * follow.accel
            }
        } else {
            dir * follow.accel
        };
        // TODO: This basically functions as a global speed cap.
        // It really should like only slow down this object or something
        dyno_tran.vel += accel * bullet_time.delta_seconds();
        dyno_tran.vel = dyno_tran.vel.clamp_length_max(follow.max_speed);
    }
}

pub(super) fn register_follow(app: &mut App) {
    app.register_type::<Follow>();
    app.add_systems(
        Update,
        update_follow
            .in_set(PhysicsSet)
            .after(super::CollisionsSet)
            .run_if(in_state(PhysicsState::Active)),
    );
}
