use std::ops::Range;

use crate::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
pub struct Star;

#[derive(Bundle)]
pub struct StarBundle {
    name: Name,
    star: Star,
    plx: ParallaxBundle,
    animation: AnimationManager<AnimationStar>,
}
impl StarBundle {
    pub fn random(wrap: f32, dist_range: &Range<f32>) -> Self {
        let mut rng = thread_rng();
        let distance = rng.gen_range(dist_range.clone());
        let wrap_size = IDEAL_VEC_f32 * wrap;
        Self {
            name: Name::new("star"),
            star: Star,
            plx: ParallaxBundle::new(
                Vec3::new(
                    rng.gen_range((-wrap_size.x / 2.0)..(wrap_size.x / 2.0)),
                    rng.gen_range((-wrap_size.y / 2.0)..(wrap_size.y / 2.0)),
                    -distance,
                ),
                Parallax {
                    wrap,
                    distance,
                    apply_scale: true,
                },
            ),
            animation: AnimationManager::new(),
        }
    }
}

pub fn spawn_stars(
    commands: &mut Commands,
    num_stars: u32,
    wrap: f32,
    dist_range: Range<f32>,
    parent: Entity,
) {
    for _ in 0..num_stars {
        let bund = StarBundle::random(wrap, &dist_range);
        commands.spawn(bund).set_parent(parent);
    }
}

pub(super) struct StarPlugin;
impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Star>();
    }
}
