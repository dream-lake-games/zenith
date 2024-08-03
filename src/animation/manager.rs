use bevy::{reflect::GetTypeRegistration, sprite::Mesh2dHandle};

use crate::prelude::*;

use super::mat::AnimationMaterial;

#[derive(Debug, Clone, Reflect)]
pub struct BodyData {
    pub(super) path: String,
    pub(super) size: UVec2,
    pub(super) length: u32,
    pub(super) fps: f32,
    pub(super) color: Color,
    pub(super) offset: Vec3,
    pub(super) scale: Vec2,
    pub(super) render_layers: RenderLayers,
}
impl BodyData {
    fn with_overrides(mut self, overrides: BodyDataOverrides) -> Self {
        self.offset = overrides.override_offset.unwrap_or(self.offset);
        self.scale = overrides.override_scale.unwrap_or(self.scale);
        self.color = overrides.override_color.unwrap_or(self.color);
        self
    }
}

#[derive(Default, Debug, Clone, Reflect)]
pub(super) struct BodyDataOverrides {
    pub(super) override_offset: Option<Vec3>,
    pub(super) override_scale: Option<Vec2>,
    pub(super) override_color: Option<Color>,
}

pub trait AnimationBody {
    fn to_body_data(&self) -> BodyData;
}

#[derive(Debug, Clone, Reflect, PartialEq)]
pub(super) enum AnimationNextState<NextType> {
    None,
    Some(NextType),
    HideThenDie(f32),
}

#[derive(Debug, Clone, Reflect)]
pub struct StateData<NextType, BodyType: AnimationBody> {
    pub(super) overwritten_bodies: Vec<(BodyType, BodyDataOverrides)>,
    pub(super) next: AnimationNextState<NextType>,
}

pub trait AnimationStateMachine:
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
    type FileType: AnimationBody;

    fn to_state_data(&self) -> StateData<Self, Self::FileType>;
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct AnimationManager<StateMachine: AnimationStateMachine> {
    pub(super) state: StateMachine,
    pub(super) hidden: bool,
    pub(super) flip_x: bool,
    pub(super) flip_y: bool,
}
impl<StateMachine: AnimationStateMachine> AnimationManager<StateMachine> {
    pub fn new() -> Self {
        Self {
            state: default(),
            hidden: false,
            flip_x: false,
            flip_y: false,
        }
    }

    impl_get!(state, StateMachine);
    impl_with!(state, StateMachine);
    impl_get!(hidden, bool);
    impl_with!(hidden, bool);
    impl_get!(flip_x, bool);
    impl_with!(flip_x, bool);
    impl_get!(flip_y, bool);
    impl_with!(flip_y, bool);
}
macro_rules! impl_mutable_animation_manager_field {
    ($field:ident, $type:ty) => {
        paste::paste! {
            fn [<set_ $field>](&mut self, val: $type) {
                if val == self.$field {
                    return;
                }
                self.$field = val;
            }
            fn [<reset_ $field>](&mut self, val: $type) {
                self.$field = val;
            }
        }
    };
}
pub trait MutableAnimationManagerActions<StateMachine: AnimationStateMachine> {
    /// Sets the currently value of the animation manager state, doing nothing if the value is the same
    fn set_state(&mut self, state: StateMachine);
    /// Resets the currently value of the animation manager state, triggering change even if the value is the same
    fn reset_state(&mut self, state: StateMachine);
    /// Sets the currently value of the animation manager hidden, doing nothing if the value is the same
    fn set_hidden(&mut self, hidden: bool);
    /// Resets the currently value of the animation manager hidden, triggering change even if the value is the same
    fn reset_hidden(&mut self, hidden: bool);
    /// Sets the currently value of the animation manager flip_x, doing nothing if the value is the same
    fn set_flip_x(&mut self, flip_x: bool);
    /// Resets the currently value of the animation manager flip_x, triggering change even if the value is the same
    fn reset_flip_x(&mut self, flip_x: bool);
    /// Sets the currently value of the animation manager flip_y, doing nothing if the value is the same
    fn set_flip_y(&mut self, flip_y: bool);
    /// Resets the currently value of the animation manager flip_y, triggering change even if the value is the same
    fn reset_flip_y(&mut self, flip_y: bool);
}
impl<'w, StateMachine: AnimationStateMachine> MutableAnimationManagerActions<StateMachine>
    for Mut<'w, AnimationManager<StateMachine>>
{
    impl_mutable_animation_manager_field!(state, StateMachine);
    impl_mutable_animation_manager_field!(hidden, bool);
    impl_mutable_animation_manager_field!(flip_x, bool);
    impl_mutable_animation_manager_field!(flip_y, bool);
}
#[derive(Bundle)]
pub struct AnimationManagerBundle<StateMachine: AnimationStateMachine> {
    manager: AnimationManager<StateMachine>,
}
impl<StateMachine: AnimationStateMachine> AnimationManagerBundle<StateMachine> {
    pub fn new() -> Self {
        Self {
            manager: AnimationManager::new(),
        }
    }
}

/// For tracking animations that play
#[derive(Component, Debug, Clone, Reflect)]
struct AnimationIndex<StateMachine: AnimationStateMachine> {
    ix: u32,
    length: u32,
    time: f32,
    /// Seconds per frame
    spf: f32,
    /// The state to transition to after this state. Note that this has a None variant inside it.
    next: AnimationNextState<StateMachine>,
}

/// Attached to the body of the animation that (when finished) triggers the state transition
#[derive(Component, Debug, Clone, Reflect)]
struct AnimationNextBurden<StateMachine: AnimationStateMachine> {
    next_state: AnimationNextState<StateMachine>,
}

#[derive(Bundle)]
struct BodyDataBundle<StateMachine: AnimationStateMachine> {
    name: Name,
    mesh: Mesh2dHandle,
    material: Handle<AnimationMaterial>,
    spatial: SpatialBundle,
    render_layers: RenderLayers,
    index: AnimationIndex<StateMachine>,
}
impl<StateMachine: AnimationStateMachine> BodyDataBundle<StateMachine> {
    fn new(
        data: BodyData,
        next: AnimationNextState<StateMachine>,
        ass: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        mats: &mut ResMut<Assets<AnimationMaterial>>,
    ) -> Self {
        let mesh = Mesh::from(Rectangle::new(data.size.x as f32, data.size.y as f32));
        Self {
            name: Name::new("body_data_bundle"),
            mesh: meshes.add(mesh).into(),
            material: mats.add(AnimationMaterial::new(
                ass.load(data.path),
                data.length,
                false,
                false,
                Vec2::ONE,
                Color::WHITE,
            )),
            spatial: SpatialBundle::from_transform(Transform {
                translation: data.offset,
                scale: data.scale.extend(1.0),
                ..default()
            }),
            render_layers: data.render_layers,
            index: AnimationIndex {
                ix: 0,
                length: data.length,
                time: 0.0,
                spf: 1.0 / data.fps,
                next,
            },
        }
    }
}

fn handle_manager_changes<StateMachine: AnimationStateMachine>(
    mut commands: Commands,
    managers: Query<
        (Entity, &AnimationManager<StateMachine>),
        Changed<AnimationManager<StateMachine>>,
    >,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<AnimationMaterial>>,
) {
    for (eid, manager) in &managers {
        println!("got changed event");
        commands.entity(eid).despawn_descendants();
        let state_data = manager.get_state().to_state_data();
        for (ix, (body, overwrite)) in state_data.overwritten_bodies.into_iter().enumerate() {
            let data = body.to_body_data().with_overrides(overwrite);
            let next = if ix == 0 {
                state_data.next.clone()
            } else {
                AnimationNextState::None
            };
            let body_bund = BodyDataBundle::new(data, next, &ass, &mut meshes, &mut mats);
            commands.spawn(body_bund).set_parent(eid);
        }
    }
}

fn play_animations<StateMachine: AnimationStateMachine>(
    mut commands: Commands,
    mut managers: Query<(Entity, &mut AnimationManager<StateMachine>, Option<&Dying>)>,
    mut bodies: Query<(
        &mut AnimationIndex<StateMachine>,
        &Handle<AnimationMaterial>,
        &Parent,
    )>,
    mut mats: ResMut<Assets<AnimationMaterial>>,
    bullet_time: Res<BulletTime>,
) {
    for (mut index, hand, parent) in &mut bodies {
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
            match index.next {
                AnimationNextState::None => {
                    // Looping the animation
                    if index.length <= 1 {
                        // Degen animations don't need to do anything
                        continue;
                    }
                    index.ix = 0;
                    let mat = mats.get_mut(hand.id()).unwrap();
                    mat.set_ix(index.ix);
                }
                AnimationNextState::Some(variant) => {
                    // Transitioning to a new state
                    let (_eid, mut manager, _) = managers.get_mut(parent.get()).unwrap();
                    manager.reset_state(variant);
                }
                AnimationNextState::HideThenDie(dying_time) => {
                    // Triggering the death process for this entity
                    let (eid, _, already_dying) = managers.get(parent.get()).unwrap();
                    if !already_dying.is_some() {
                        commands.entity(eid).insert(Dying::new(dying_time));
                    }
                }
            }
        }
    }
}

pub(super) fn register_animation_manager<StateMachine: AnimationStateMachine>(app: &mut App) {
    app.register_type::<AnimationManager<StateMachine>>();
    app.add_systems(FixedPostUpdate, handle_manager_changes::<StateMachine>);
    app.add_systems(
        FixedUpdate,
        play_animations::<StateMachine>.in_set(AnimationSet),
    );
}
