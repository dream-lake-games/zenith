use crate::prelude::*;

/// When the camera is in "Follow" mode, it will move to show the entity with this
/// component at the center of the screen every frame iff:
/// - There is exactly one such entity
/// - This entity has a GlobalTransform
#[derive(Component, Debug, Clone, Reflect)]
pub struct DynamicCameraLeader;

/// States that the dynamic camera can be in with respect to movement
#[derive(Component, Debug, Clone, Reflect)]
enum DynamicCameraMode {
    /// The camera is still and not moving
    Fixed,
    /// If there is one entity with a `DynamicCameraLeader` component, it will ensure that
    /// is always exactly in the center of the screen.
    /// If there is no such component, or for some reason multiple, nothing will happen
    Follow,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct DynamicCameraMarker;

#[derive(Bundle)]
pub struct DynamicCameraBundle {
    name: Name,
    marker: DynamicCameraMarker,
    mode: DynamicCameraMode,
    spatial: SpatialBundle,
}

fn setup_camera(mut commands: Commands, root: Res<LayerRoot>) {
    commands
        .spawn(DynamicCameraBundle {
            name: Name::new("dynamic_camera"),
            marker: DynamicCameraMarker,
            mode: DynamicCameraMode::Follow,
            spatial: default(),
        })
        .set_parent(root.eid());
}

fn move_dynamic_camera(
    mut camera_q: Query<(&DynamicCameraMode, &mut Transform), With<DynamicCameraMarker>>,
    leader_q: Query<&Transform, (Without<DynamicCameraMarker>, With<DynamicCameraLeader>)>,
) {
    let (mode, mut cam_tran) = camera_q.single_mut();
    match mode {
        DynamicCameraMode::Fixed => {
            // Do nothing
        }
        DynamicCameraMode::Follow => {
            let Ok(leader_gtran) = leader_q.get_single() else {
                return;
            };
            cam_tran.translation = leader_gtran.translation;
        }
    }
}

/// The dynamic camera is really just a special spatial bundle. In order to really
/// move the camera, you have to move the layer cameras.
fn move_layer_cameras(
    dynamic_camera_q: Query<&Transform, With<DynamicCameraMarker>>,
    mut layer_camera_q: Query<
        &mut Transform,
        (
            Without<DynamicCameraMarker>,
            Or<(
                With<BgSpriteLayer>,
                With<BgLightLayer>,
                With<SpriteLayer>,
                With<LightLayer>,
            )>,
        ),
    >,
) {
    let dynamic_tran = dynamic_camera_q.single();
    for mut layer_tran in &mut layer_camera_q {
        *layer_tran = dynamic_tran.clone();
    }
}

pub(super) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DynamicCameraLeader>();
        app.register_type::<DynamicCameraMode>();
        app.register_type::<DynamicCameraMarker>();

        app.add_systems(Startup, setup_camera.after(RootInit));
        app.add_systems(
            FixedPostUpdate,
            (move_dynamic_camera, move_layer_cameras)
                .chain()
                .before(PhysicsSet),
        );
        // app.add_systems(
        //     PostUpdate,
        //     (move_dynamic_camera, move_layer_cameras.chain()),
        // );
    }
}
