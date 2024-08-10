//! I've found this kind of thing useful to have in the past.
//! Basically you put a Birthing/Dying timer on entities.
//! When the timer expires, there will be one pass of the schedule where this runs (Main)
//! where `Birth` or `Death` is observable. After that, the component will be removed/entity despawned.

use crate::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
pub struct Birthing {
    birthspan: f32,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct Birthed;

#[derive(Component, Debug, Clone, Reflect)]
pub struct Dying {
    deathspan: f32,
}
impl Dying {
    pub fn new(deathspan: f32) -> Self {
        Self { deathspan }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct Dead;

fn update_final_states(
    births: Query<Entity, With<Birthed>>,
    deaths: Query<Entity, With<Dead>>,
    mut commands: Commands,
) {
    for eid in &births {
        commands.entity(eid).remove::<Birthing>();
        commands.entity(eid).remove::<Birthed>();
    }
    for eid in &deaths {
        commands.entity(eid).despawn_recursive();
    }
}

fn update_transition_states(
    mut birthing: Query<(Entity, &mut Birthing)>,
    mut dying: Query<(Entity, &mut Dying)>,
    mut commands: Commands,
    bullet_time: Res<BulletTime>,
) {
    for (eid, mut birthing) in &mut birthing {
        birthing.birthspan -= bullet_time.delta_seconds();
        if birthing.birthspan <= 0.0 {
            commands.entity(eid).insert(Birthed);
        }
    }
    for (eid, mut dying) in &mut dying {
        dying.deathspan -= bullet_time.delta_seconds();
        if dying.deathspan <= 0.0 {
            commands.entity(eid).insert(Dead);
        }
    }
}

pub(super) struct LifecyclePlugin;
impl Plugin for LifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Dying>();
        app.register_type::<Birthing>();
        app.add_systems(
            PostUpdate,
            (update_final_states, update_transition_states)
                .chain()
                .after(AnimationSet)
                .after(ParticlesSet),
        );
    }
}
