use super::*;

fn rotate_ship_tail(
    drag_input: Res<DragInput>,
    ships: Query<(&ShipLaunchState, &Transform), (With<Ship>, Without<ShipTail>)>,
    mut tails: Query<(&mut Transform, &Parent), With<ShipTail>>,
) {
    for (mut tran, parent) in &mut tails {
        let (launch_state, ship_tran) = ships.get(parent.get()).unwrap();
        if launch_state.current_launch.is_some() {
            if let Some(start_pos) = drag_input.get_left_drag_start() {
                let diff = drag_input.get_screen_pos() - start_pos;
                if diff.length_squared() > 0.1 {
                    let prot = ship_tran.pos_n_angle().1;
                    tran.set_angle(-prot + diff.to_angle() + PI);
                }
            }
        } else {
            // Fuck I don't want to refactor this if/else rn
            tran.set_angle(0.0);
        }
    }
}

#[derive(Component)]
struct LaunchParticleSpawner;

#[derive(Bundle)]
struct LaunchParticleSpawnerBundle {
    name: Name,
    marker: LaunchParticleSpawner,
    spatial: SpatialBundle,
    spawner: SimpleParticleSpawner,
}
impl LaunchParticleSpawnerBundle {
    fn new() -> Self {
        let particle = Particle::new(Vec2::ZERO)
            .with_sizes(3.0, 1.0)
            .with_colors(Color::WHITE, Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 0.0)))
            .with_lifespan(0.05);
        let spawner = SimpleParticleSpawner::new(vec![particle])
            .with_poses(vec![Vec2::new(-5.0, 5.0), Vec2::new(-5.0, -5.0)]);
        Self {
            name: Name::new("launch_particle_spawner"),
            marker: LaunchParticleSpawner,
            spatial: SpatialBundle::default(),
            spawner,
        }
    }
}

fn manage_launch_particle_spawners(
    mut commands: Commands,
    ships: Query<&ShipLaunchState, With<Ship>>,
    tails: Query<(Entity, &Parent, Option<&Children>), With<ShipTail>>,
    drag_input: Res<DragInput>,
    mut relevant_spawners: Query<
        (Entity, &mut SimpleParticleSpawner, &Parent),
        With<LaunchParticleSpawner>,
    >,
    base_consts: Res<ShipBaseConstants>,
) {
    // Spawn in new spawners if needed
    for (eid, parent, ochildren) in &tails {
        let no_spawner = match ochildren {
            Some(children) => children
                .iter()
                .all(|child| !relevant_spawners.contains(*child)),
            None => true,
        };
        let launch_state = ships.get(parent.get()).unwrap();
        if launch_state.current_launch.is_some() && no_spawner {
            commands
                .spawn(LaunchParticleSpawnerBundle::new())
                .set_parent(eid);
        }
    }
    // Update the active spawners, despawning if needed
    for (eid, mut spawner, parent) in &mut relevant_spawners {
        let (_, tail_parent, _) = tails.get(parent.get()).unwrap();
        let launch_state = ships.get(tail_parent.get()).unwrap();
        if let Some(launch_time) = launch_state.current_launch {
            let Some(drag_start) = drag_input.get_left_drag_start() else {
                continue;
            };
            let diff = drag_input.get_screen_pos() - drag_start;
            for pref in spawner.references.iter_mut() {
                *pref = pref.clone().with_vel(diff * 10.0).with_colors(
                    Color::Srgba(Srgba::new(
                        1.0,
                        1.0 - launch_time / base_consts.max_launch_time,
                        1.0 - launch_time / base_consts.max_launch_time,
                        1.0,
                    )),
                    Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 0.0)),
                );
            }
        } else {
            commands.entity(eid).despawn_recursive();
        }
    }
}

pub(super) fn register_launch_juice(app: &mut App) {
    app.add_systems(PostUpdate, rotate_ship_tail.in_set(ShipSet));
    app.add_systems(Update, manage_launch_particle_spawners);
}
