use std::marker::PhantomData;

use bevy::{reflect::GetTypeRegistration, sprite::Mesh2dHandle};

use crate::prelude::*;

/// Again, let's start by just imagining the perfect macro
///
///
/// defn_texture!(
///     TextureNormalPlanet,
///     textures: [
///         {(similar to animation, with offset and scale removed)
///         ^ actually offset should be replaced with zoffset, think about bordered mesh
///         ^ double actually, you should just label here, without the part thing
///         ^ undo that last actually, we still want textures to be able to have animation states
///     ],
///     parts: [
///         Inner,
///         Outer,
///     ]
///     states: [
///         // Then basically the manager will have a hashmap from these states (as enums) to points, will update on change
///         Undamaged: [
///             Inner: inner_body_id {
///                 next: Damaged // TODO
///             },
///             Outer: outer_body_id,
///         ],
///         Damaged: [...]
///     ],
/// )
///
///
///
use super::{mat::AnimationMaterial, mesh::points_to_mesh, ManagersSet};

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Default)]
pub enum TextureDimGrowth {
    Scale,
    #[default]
    Repeat,
}

#[derive(Debug, Clone, Reflect, Default)]
pub struct TextureGrowth {
    pub x: TextureDimGrowth,
    pub y: TextureDimGrowth,
}

#[derive(Debug, Clone, Reflect)]
pub struct TextureBodyData {
    pub(super) path: String,
    pub(super) size: UVec2,
    pub(super) length: u32,
    pub(super) fps: f32,
    pub(super) growth: TextureGrowth,
    pub(super) z_offset: f32,
    pub(super) color: Color,
    pub(super) render_layers: RenderLayers,
}

pub trait TextureBody {
    fn to_body_data(&self) -> TextureBodyData;
}

pub trait TexturePart:
    Sized
    + Copy
    + Send
    + Sync
    + 'static
    + std::fmt::Debug
    + std::hash::Hash
    + PartialEq
    + Eq
    + Reflect
    + FromReflect
    + TypePath
    + GetTypeRegistration
{
    fn all() -> Vec<Self>;
}

/// TODO: If needed, add the ability to not just specify body here, but next
/// That way a textures animations can transition (and especially HideThenDie) like normal
pub trait TextureStateMachine:
    Sized
    + Copy
    + Send
    + Sync
    + 'static
    + std::fmt::Debug
    + Default
    + PartialEq
    + Eq
    + Reflect
    + FromReflect
    + TypePath
    + GetTypeRegistration
{
    type BodyType: TextureBody;
    type PartType: TexturePart;

    fn part_to_body(&self, part: Self::PartType) -> Self::BodyType;
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct TextureManager<StateMachine: TextureStateMachine> {
    pub(super) state: StateMachine,
    pub(super) point_map: HashMap<StateMachine::PartType, Vec<Vec2>>,
}
impl<StateMachine: TextureStateMachine> TextureManager<StateMachine> {
    pub fn new() -> Self {
        Self {
            state: default(),
            point_map: default(),
        }
    }

    impl_get!(state, StateMachine);
    impl_with!(state, StateMachine);
    impl_get_ref!(point_map, HashMap<StateMachine::PartType, Vec<Vec2>>);

    pub fn get_part_points(&self, part: StateMachine::PartType) -> &[Vec2] {
        &self.point_map.get(&part).unwrap()
    }

    pub fn with_part_points(mut self, part: StateMachine::PartType, points: Vec<Vec2>) -> Self {
        self.point_map.insert(part, points);
        self
    }
}
pub trait MutableTextureManagerActions<StateMachine: TextureStateMachine> {
    /// Sets the state of this texture, only actually mutating the underlying reference if the state is changing
    fn set_state(&mut self, state: StateMachine);
    /// Resets the state of this texture, mutating the underlying reference (triggering Change) regardless of current state
    fn reset_state(&mut self, state: StateMachine);
    /// Sets the points for a given part
    fn set_part_points(&mut self, part: StateMachine::PartType, points: Vec<Vec2>);
    /// Resets the points for a given part
    fn reset_part_points(&mut self, part: StateMachine::PartType, points: Vec<Vec2>);
}
impl<StateMachine: TextureStateMachine> MutableTextureManagerActions<StateMachine>
    for TextureManager<StateMachine>
{
    fn set_state(&mut self, state: StateMachine) {
        if state == self.state {
            return;
        }
        self.state = state;
    }
    fn reset_state(&mut self, state: StateMachine) {
        self.state = state;
    }

    fn set_part_points(&mut self, part: StateMachine::PartType, points: Vec<Vec2>) {
        if self.point_map.get(&part).unwrap() == &points {
            return;
        }
        self.point_map.insert(part, points);
    }
    fn reset_part_points(&mut self, part: StateMachine::PartType, points: Vec<Vec2>) {
        self.point_map.insert(part, points);
    }
}

#[derive(Component, Debug, Clone, Reflect)]
struct TextureIndex<StateMachine: TextureStateMachine> {
    ix: u32,
    length: u32,
    time: f32,
    /// Seconds per frame
    spf: f32,
    _pd: PhantomData<StateMachine>,
}

#[derive(Bundle)]
struct TextureBodyDataBundle<StateMachine: TextureStateMachine> {
    name: Name,
    mesh: Mesh2dHandle,
    material: Handle<AnimationMaterial>,
    spatial: SpatialBundle,
    render_layers: RenderLayers,
    index: TextureIndex<StateMachine>,
}
impl<StateMachine: TextureStateMachine> TextureBodyDataBundle<StateMachine> {
    fn new(
        data: TextureBodyData,
        points: &[Vec2],
        ass: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        mats: &mut ResMut<Assets<AnimationMaterial>>,
    ) -> Self {
        let bound = uvec2_bound(&points);
        let mesh = points_to_mesh(&points);
        let mut repetitions = Vec2::ONE;
        if data.growth.x == TextureDimGrowth::Repeat {
            repetitions.x = bound.x as f32 / data.size.x as f32;
        }
        if data.growth.y == TextureDimGrowth::Repeat {
            repetitions.y = bound.y as f32 / data.size.y as f32;
        }
        Self {
            name: Name::new("body_data_bundle"),
            mesh: meshes.add(mesh).into(),
            material: mats.add(AnimationMaterial::new(
                ass.load(data.path),
                data.length,
                false,
                false,
                repetitions,
                data.color,
            )),
            spatial: spat_tran!(0.0, 0.0, data.z_offset),
            render_layers: data.render_layers,
            index: TextureIndex {
                ix: 0,
                length: data.length,
                time: 0.0,
                spf: 1.0 / data.fps,
                _pd: default(),
            },
        }
    }
}

fn handle_manager_changes<StateMachine: TextureStateMachine>(
    mut commands: Commands,
    managers: Query<(Entity, &TextureManager<StateMachine>), Changed<TextureManager<StateMachine>>>,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<AnimationMaterial>>,
) {
    for (eid, manager) in &managers {
        commands.entity(eid).despawn_descendants();
        for part in StateMachine::PartType::all() {
            let data = manager.state.part_to_body(part);
            let body_bund = TextureBodyDataBundle::<StateMachine>::new(
                data.to_body_data(),
                manager.point_map.get(&part).unwrap(),
                &ass,
                &mut meshes,
                &mut mats,
            );
            commands.spawn(body_bund).set_parent(eid);
        }
    }
}

fn play_animations<StateMachine: TextureStateMachine>(
    mut bodies: Query<(&mut TextureIndex<StateMachine>, &Handle<AnimationMaterial>)>,
    mut mats: ResMut<Assets<AnimationMaterial>>,
    bullet_time: Res<BulletTime>,
) {
    for (mut index, hand) in &mut bodies {
        index.time += bullet_time.delta_seconds();
        if index.time < index.spf {
            // No update is happening to this body, can just continue
            continue;
        }
        index.time = 0.0;
        if index.ix + 1 < index.length {
            // Progressing to the next frame of the animation
            index.ix += 1;
            let mat = mats.get_mut(hand.id()).unwrap();
            mat.set_ix(index.ix);
        } else {
            // Looping the animation
            if index.length <= 1 {
                // Degen animations don't need to do anything
                continue;
            }
            index.ix = 0;
            let mat = mats.get_mut(hand.id()).unwrap();
            mat.set_ix(index.ix);
        }
    }
}

pub(super) fn register_texture_manager<StateMachine: TextureStateMachine>(app: &mut App) {
    app.register_type::<TextureManager<StateMachine>>();
    app.add_systems(
        FixedPostUpdate,
        handle_manager_changes::<StateMachine>
            .in_set(AnimationSet)
            .in_set(ManagersSet),
    );
    app.add_systems(
        FixedUpdate,
        play_animations::<StateMachine>
            .in_set(AnimationSet)
            .in_set(ManagersSet),
    );
    app.add_systems(
        FixedPostUpdate,
        spawn_texture_manager_mirages::<StateMachine>
            .in_set(AnimationSet)
            .in_set(MirageSet)
            .after(ManagersSet),
    );
}
