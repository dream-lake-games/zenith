use crate::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
pub struct Parallax {
    pub wrap: f32,
    pub distance: f32,
    pub apply_scale: bool,
}
impl Parallax {
    pub fn new(wrap: f32, distance: f32, scale_transform: bool) -> Self {
        Self {
            wrap,
            distance,
            apply_scale: scale_transform,
        }
    }
}

#[derive(Bundle)]
pub struct ParallaxBundle {
    spatial: SpatialBundle,
    parallax: Parallax,
}
impl ParallaxBundle {
    pub fn new(pos: Vec3, parallax: Parallax) -> Self {
        Self {
            spatial: spat_tran!(pos),
            parallax,
        }
    }
}

pub fn move_parallaxes(
    mut parallax_q: Query<(&Parallax, &mut Transform), Without<DynamicCameraMarker>>,
    camera_q: Query<(&Transform, &DynamicCameraMarker)>,
    meta_state: Res<State<MetaState>>,
) {
    let (cam_tran, cam_marker) = camera_q.single();
    let wrap_size = meta_state
        .get()
        .get_room_state()
        .map(|room_state| room_state.room_size.as_vec2())
        .unwrap_or(IDEAL_VEC_f32);
    let cam_pos = cam_tran.translation.truncate();

    let dist_left = (cam_pos.x - cam_marker.first_pos.x).rem_euclid(wrap_size.x);
    let dist_right = (cam_marker.first_pos.x - cam_pos.x).rem_euclid(wrap_size.x);
    let dist_up = (cam_pos.y - cam_marker.first_pos.y).rem_euclid(wrap_size.y);
    let dist_down = (cam_marker.first_pos.y - cam_pos.y).rem_euclid(wrap_size.y);
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

    for (parallax, mut tran) in &mut parallax_q {
        tran.translation.x -= diff.x / parallax.distance;
        tran.translation.y -= diff.y / parallax.distance;

        // Do wrapping
        if tran.translation.x < -parallax.wrap * IDEAL_WIDTH_f32 / 2.0 {
            let offset = -parallax.wrap * IDEAL_WIDTH_f32 / 2.0 - tran.translation.x;
            let offset = offset.rem_euclid(IDEAL_WIDTH_f32);
            tran.translation.x = parallax.wrap * IDEAL_WIDTH_f32 / 2.0 - offset;
        }
        if tran.translation.x > parallax.wrap * IDEAL_WIDTH_f32 / 2.0 {
            let offset = tran.translation.x - parallax.wrap * IDEAL_WIDTH_f32 / 2.0;
            let offset = offset.rem_euclid(IDEAL_WIDTH_f32);
            tran.translation.x = -parallax.wrap * IDEAL_WIDTH_f32 / 2.0 + offset;
        }
        if tran.translation.y < -parallax.wrap * IDEAL_HEIGHT_f32 / 2.0 {
            let offset = -parallax.wrap * IDEAL_HEIGHT_f32 / 2.0 - tran.translation.y;
            let offset = offset.rem_euclid(IDEAL_HEIGHT_f32);
            tran.translation.y = parallax.wrap * IDEAL_HEIGHT_f32 / 2.0 - offset;
        }
        if tran.translation.y > parallax.wrap * IDEAL_HEIGHT_f32 / 2.0 {
            let offset = tran.translation.y - parallax.wrap * IDEAL_HEIGHT_f32 / 2.0;
            let offset = offset.rem_euclid(IDEAL_HEIGHT_f32);
            tran.translation.y = -parallax.wrap * IDEAL_HEIGHT_f32 / 2.0 + offset;
        }

        if parallax.apply_scale {
            tran.scale = Vec3::new(1.0 / parallax.distance, 1.0 / parallax.distance, 1.0);
        }
    }
}

pub(super) struct ParallaxPlugin;
impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Parallax>();

        app.add_systems(PostUpdate, move_parallaxes.after(PhysicsSet));
    }
}
