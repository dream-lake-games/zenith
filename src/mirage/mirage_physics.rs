// use bevy::sprite::Mesh2dHandle;

// use crate::prelude::*;

// #[derive(Component, Debug, Clone, Reflect)]
// pub struct MirageStaticTxManager {
//     offsets: Vec<Vec2>,
// }
// impl MirageStaticTxManager {
//     pub fn room_offsets(room_state: &RoomState) -> Self {
//         Self {
//             offsets: room_state.mirage_offsets(),
//         }
//     }
// }

// #[derive(Component)]
// struct MirageStaticTx;

// #[derive(Bundle)]
// struct MirageStaticRxBundle {
//     name: Name,
//     marker: MirageStaticTx,
//     spatial: SpatialBundle,
//     static_tx: StaticTx,
// }

// pub fn spawn_static_tx_mirages(
//     mut commands: Commands,
//     managers: Query<(Entity, &Children, &StaticTx), (Changed<StaticTx>, With<MirageStaticTxManager>)>,
//     ditto_q: Query<(
//         Entity,
//         &Mesh2dHandle,
//         &Handle<AnimationMaterial>,
//         &RenderLayers,
//     )>,
// ) {
//     for (manager_eid, children) in &managers {
//         for (ix, child) in children.iter().enumerate() {
//             let (ditto_eid, mesh, mat, render_layers) = ditto_q.get(*child).unwrap();
//             commands
//                 .spawn(MirageMeshMatBundle {
//                     name: Name::new(format!("mirage_{ix}")),
//                     marker: MirageMeshMat {
//                         following: ditto_eid,
//                         offset: Vec2::new(20.0, 20.0),
//                     },
//                     spatial: default(),
//                     mesh: mesh.clone(),
//                     mat: mat.clone(),
//                     render_layers: render_layers.clone(),
//                 })
//                 .set_parent(manager_eid);
//         }
//     }
// }

// pub fn spawn_texture_manager_mirages<StateMachine: TextureStateMachine>(
//     mut commands: Commands,
//     managers: Query<
//         (Entity, &MirageTextureManager, &Children),
//         (Changed<TextureManager<StateMachine>>,),
//     >,
//     ditto_q: Query<(
//         Entity,
//         &Mesh2dHandle,
//         &Handle<AnimationMaterial>,
//         &RenderLayers,
//     )>,
// ) {
//     for (manager_eid, mirage, children) in &managers {
//         for (ix, child) in children.iter().enumerate() {
//             let (ditto_eid, mesh, mat, render_layers) = ditto_q.get(*child).unwrap();
//             for (jx, offset) in mirage.offsets.iter().enumerate() {
//                 commands
//                     .spawn(MirageMeshMatBundle {
//                         name: Name::new(format!("mirage_{ix}_{jx}")),
//                         marker: MirageMeshMat {
//                             following: ditto_eid,
//                             offset: *offset,
//                         },
//                         spatial: default(),
//                         mesh: mesh.clone(),
//                         mat: mat.clone(),
//                         render_layers: render_layers.clone(),
//                     })
//                     .set_parent(manager_eid);
//             }
//         }
//     }
// }

// fn update_mirage_mesh_mats(
//     mut commands: Commands,
//     reference: Query<
//         &Transform,
//         (
//             With<Mesh2dHandle>,
//             With<Handle<AnimationMaterial>>,
//             Without<MirageMeshMat>,
//         ),
//     >,
//     mut followers_q: Query<(Entity, &MirageMeshMat, &mut Transform)>,
// ) {
//     for (eid, info, mut tran) in &mut followers_q {
//         let Ok(ref_tran) = reference.get(info.following) else {
//             commands.entity(eid).despawn_recursive();
//             continue;
//         };
//         *tran = ref_tran.clone();
//         tran.translation.x += info.offset.x;
//         tran.translation.y += info.offset.y;
//     }
// }

// pub(super) fn register_mirage_drawing(app: &mut App) {
//     app.register_type::<MirageMeshMat>();

//     app.add_systems(
//         FixedPostUpdate,
//         update_mirage_mesh_mats
//             .after(AnimationSet)
//             .in_set(MirageSet),
//     );
// }
