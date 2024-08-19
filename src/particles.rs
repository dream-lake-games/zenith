use crate::prelude::*;

/// The set that contains all physics related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParticlesSet;

/// Kind of like sound effects. Anyone anywhere can spawn one of these.
/// Then there's a system that turns these into ParticleInternals, which actually do stuff
#[derive(Component, Clone, Reflect)]
pub struct Particle {
    pos: Vec2,
    vel: Option<Vec2>,
    render_layers: RenderLayers,
    internal: ParticleInternal,
}
impl Particle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: None,
            render_layers: SpriteLayer::render_layers(),
            internal: default(),
        }
    }

    pub fn with_pos(mut self, pos: Vec2) -> Self {
        self.pos = pos;
        self
    }

    pub fn with_vel(mut self, vel: Vec2) -> Self {
        self.vel = Some(vel);
        self
    }

    pub fn with_render_layers(mut self, layers: RenderLayers) -> Self {
        self.render_layers = layers;
        self
    }

    pub fn with_lifespan(mut self, time: f32) -> Self {
        self.internal.lifespan = Timer::from_seconds(time, TimerMode::Once);
        self
    }

    pub fn with_spleen(mut self, spleen: Spleen) -> Self {
        self.internal.spleen = spleen;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.internal.start_color = color.into();
        self.internal.end_color = color.into();
        self
    }

    pub fn with_colors(mut self, start: Color, end: Color) -> Self {
        self.internal.start_color = start.into();
        self.internal.end_color = end.into();
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.internal.start_size = size;
        self.internal.end_size = size;
        self
    }

    pub fn with_sizes(mut self, start: f32, end: f32) -> Self {
        self.internal.start_size = start;
        self.internal.end_size = end;
        self
    }
}

/// A particle spawner that should be attached to things with DynoTrans AND either a static or trigger receiver.
/// This will make it so that during physics, it will spawn particles every "unit" it travels
/// This makes it so a fast-travelling thing can still create a smooth streak
#[derive(Component, Reflect)]
pub struct DynoAwareParticleSpawner {
    pub poses: Vec<Vec2>,
    pub references: Vec<Particle>,
}
impl DynoAwareParticleSpawner {
    pub fn new(references: Vec<Particle>) -> Self {
        Self {
            poses: vec![Vec2::ZERO],
            references,
        }
    }

    pub fn with_pos(mut self, pos: Vec2) -> Self {
        self.poses = vec![pos];
        self
    }

    pub fn with_poses(mut self, poses: Vec<Vec2>) -> Self {
        self.poses = poses;
        self
    }

    pub fn do_spawn(&self, base_pos: Vec2, commands: &mut Commands, proot: &ParticlesRoot) {
        for offset in &self.poses {
            for reference in &self.references {
                commands
                    .spawn(reference.clone().with_pos(base_pos + *offset))
                    .set_parent(proot.eid());
            }
        }
    }
}

/// A simple particle spawner that spawns one particle every update
/// Must be on something with GlobalTransform
#[derive(Component, Reflect)]
pub struct SimpleParticleSpawner {
    pub poses: Vec<Vec2>,
    pub references: Vec<Particle>,
}
impl SimpleParticleSpawner {
    pub fn new(references: Vec<Particle>) -> Self {
        Self {
            poses: vec![Vec2::ZERO],
            references,
        }
    }

    pub fn with_pos(mut self, pos: Vec2) -> Self {
        self.poses = vec![pos];
        self
    }

    pub fn with_poses(mut self, poses: Vec<Vec2>) -> Self {
        self.poses = poses;
        self
    }

    pub fn do_spawn(
        &self,
        gtran: &GlobalTransform,
        commands: &mut Commands,
        proot: &ParticlesRoot,
    ) {
        let (pos, angle) = gtran.pos_n_angle();
        for offset in &self.poses {
            let correct_offset = offset.my_rotate(angle);
            for reference in &self.references {
                commands
                    .spawn(reference.clone().with_pos(pos + correct_offset))
                    .set_parent(proot.eid());
            }
        }
    }
}

#[derive(Component, Clone, Reflect)]
struct ParticleInternal {
    lifespan: Timer,
    spleen: Spleen,
    start_color: Srgba,
    end_color: Srgba,
    start_size: f32,
    end_size: f32,
}
impl Default for ParticleInternal {
    fn default() -> Self {
        Self {
            lifespan: Timer::from_seconds(0.5, TimerMode::Once),
            spleen: Spleen::EaseOutQuad,
            start_color: Color::WHITE.into(),
            end_color: Color::WHITE.into(),
            start_size: 1.0,
            end_size: 1.0,
        }
    }
}

#[derive(Bundle)]
struct ParticleInternalBundle {
    particle: ParticleInternal,
    sprite: SpriteBundle,
    render_layers: RenderLayers,
}
impl ParticleInternalBundle {
    fn spawn(pos: Vec2, particle: &Particle, parent: Entity, commands: &mut Commands) {
        let mut ent_comm = commands.spawn(Self {
            particle: particle.internal.clone(),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::ZERO),
                    ..default()
                },
                transform: Transform::from_translation(pos.extend(0.0)),
                ..default()
            },
            render_layers: SpriteLayer::render_layers(),
        });
        ent_comm.set_parent(parent);
        if let Some(vel) = particle.vel {
            ent_comm.insert(DynoTran { vel });
        }
    }
}

fn materialize_particles(
    mut commands: Commands,
    proot: Res<ParticlesRoot>,
    data: Query<(Entity, &Particle)>,
) {
    for (eid, particle) in &data {
        ParticleInternalBundle::spawn(particle.pos, particle, proot.eid(), &mut commands);
        commands.entity(eid).despawn_recursive();
    }
}

fn update_particles_internal(
    mut particles: Query<(Entity, &mut ParticleInternal, &mut Sprite, &mut Transform)>,
    mut commands: Commands,
    bullet_time: Res<BulletTime>,
    simple_spawners: Query<(&SimpleParticleSpawner, &GlobalTransform)>,
    proot: Res<ParticlesRoot>,
) {
    for (eid, mut internal, mut sprite, mut tran) in &mut particles {
        if internal.lifespan.finished() {
            commands.entity(eid).despawn_recursive();
            continue;
        }
        internal.lifespan.tick(bullet_time.delta());
        let new_color = Srgba::new(
            internal.spleen.bound_interp(
                internal.lifespan.fraction(),
                internal.start_color.red,
                internal.end_color.red,
            ),
            internal.spleen.bound_interp(
                internal.lifespan.fraction(),
                internal.start_color.green,
                internal.end_color.green,
            ),
            internal.spleen.bound_interp(
                internal.lifespan.fraction(),
                internal.start_color.blue,
                internal.end_color.blue,
            ),
            internal.spleen.bound_interp(
                internal.lifespan.fraction(),
                internal.start_color.alpha,
                internal.end_color.alpha,
            ),
        );
        sprite.color = new_color.into();
        let new_size = internal.spleen.bound_interp(
            internal.lifespan.fraction(),
            internal.start_size,
            internal.end_size,
        );
        sprite.custom_size = Some(Vec2::ONE * new_size);
        // Particles age to the back
        tran.translation.z -= bullet_time.delta_seconds();
    }
    for (simple_spawner, gtran) in &simple_spawners {
        simple_spawner.do_spawn(&gtran, &mut commands, &proot);
    }
}

pub(super) struct ParticlesPlugin;
impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            materialize_particles
                .run_if(in_state(PhysicsState::Active))
                .in_set(ParticlesSet),
        );
        app.add_systems(
            Update,
            update_particles_internal
                .run_if(in_state(PhysicsState::Active))
                .in_set(ParticlesSet),
        );

        app.register_type::<ParticleInternal>();
        app.register_type::<DynoAwareParticleSpawner>();
        app.register_type::<SimpleParticleSpawner>();
    }
}
