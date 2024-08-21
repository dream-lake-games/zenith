use bevy::sprite::Mesh2dHandle;

use crate::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
pub struct MirageAnimationManager {
    offsets: Vec<Vec2>,
}
impl MirageAnimationManager {
    pub fn room_offsets(room_state: &RoomState) -> Self {
        Self {
            offsets: room_state.mirage_offsets(),
        }
    }

    pub fn wrap_offsets(wrap: Vec2) -> Self {
        Self {
            offsets: wrap_offsets(wrap),
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct MirageTextureManager {
    offsets: Vec<Vec2>,
}
impl MirageTextureManager {
    pub fn room_offsets(room_state: &RoomState) -> Self {
        Self {
            offsets: room_state.mirage_offsets(),
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
struct MirageMeshMat {
    following: Entity,
    offset: Vec2,
}

#[derive(Bundle)]
struct MirageMeshMatBundle {
    name: Name,
    marker: MirageMeshMat,
    spatial: SpatialBundle,
    mesh: Mesh2dHandle,
    mat: Handle<AnimationMaterial>,
    render_layers: RenderLayers,
}

pub fn spawn_animation_manager_mirages<StateMachine: AnimationStateMachine>(
    mut commands: Commands,
    managers: Query<
        (Entity, &Children, &MirageAnimationManager),
        (Changed<AnimationManager<StateMachine>>,),
    >,
    ditto_q: Query<(
        Entity,
        &Mesh2dHandle,
        &Handle<AnimationMaterial>,
        &RenderLayers,
    )>,
    root: Res<MirageRoot>,
) {
    for (_manager_eid, children, mirage) in &managers {
        for (ix, child) in children.iter().enumerate() {
            let (ditto_eid, mesh, mat, render_layers) = ditto_q.get(*child).unwrap();
            for (jx, offset) in mirage.offsets.iter().enumerate() {
                commands
                    .spawn(MirageMeshMatBundle {
                        name: Name::new(format!("mirage_{ix}_{jx}")),
                        marker: MirageMeshMat {
                            following: ditto_eid,
                            offset: *offset,
                        },
                        spatial: default(),
                        mesh: mesh.clone(),
                        mat: mat.clone(),
                        render_layers: render_layers.clone(),
                    })
                    .set_parent(root.eid());
            }
        }
    }
}

pub fn spawn_texture_manager_mirages<StateMachine: TextureStateMachine>(
    mut commands: Commands,
    managers: Query<
        (Entity, &MirageTextureManager, &Children),
        (Changed<TextureManager<StateMachine>>,),
    >,
    ditto_q: Query<(
        Entity,
        &Mesh2dHandle,
        &Handle<AnimationMaterial>,
        &RenderLayers,
    )>,
    root: Res<MirageRoot>,
) {
    for (_manager_eid, mirage, children) in &managers {
        for (ix, child) in children.iter().enumerate() {
            let (ditto_eid, mesh, mat, render_layers) = ditto_q.get(*child).unwrap();
            for (jx, offset) in mirage.offsets.iter().enumerate() {
                commands
                    .spawn(MirageMeshMatBundle {
                        name: Name::new(format!("mirage_{ix}_{jx}")),
                        marker: MirageMeshMat {
                            following: ditto_eid,
                            offset: *offset,
                        },
                        spatial: default(),
                        mesh: mesh.clone(),
                        mat: mat.clone(),
                        render_layers: render_layers.clone(),
                    })
                    .set_parent(root.eid());
            }
        }
    }
}

fn update_mirage_mesh_mats(
    mut commands: Commands,
    reference: Query<
        &GlobalTransform,
        (
            With<Mesh2dHandle>,
            With<Handle<AnimationMaterial>>,
            Without<MirageMeshMat>,
        ),
    >,
    mut followers_q: Query<(Entity, &MirageMeshMat, &mut Transform)>,
) {
    for (eid, info, mut tran) in &mut followers_q {
        let Ok(ref_tran) = reference.get(info.following) else {
            commands.entity(eid).despawn_recursive();
            continue;
        };
        *tran = ref_tran.compute_transform();
        tran.translation.x += info.offset.x;
        tran.translation.y += info.offset.y;
        let new_global: GlobalTransform = (tran.clone()).into();
        commands.entity(eid).insert(new_global);
    }
}

pub(super) fn register_mirage_drawing(app: &mut App) {
    app.register_type::<MirageMeshMat>();

    app.add_systems(
        PostUpdate,
        update_mirage_mesh_mats
            .after(AnimationSet)
            .after(TransformSystem::TransformPropagate)
            .in_set(MirageSet),
    );
}
