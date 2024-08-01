//! It's pet peeve of mine to have a disorganized hierarchy in the debugger.
//! It looks bad, and when stuff goes wrong it makes it much harder to actually
//! figure out what's wrong.
//! These components and resources should hopefully make it easy to always spawn
//! things under a component with default transform (or with some offset, see menu root + transition root + maybe others idk).
//! This also is useful for levels and the like, which can then be cleaned up by just running
//! `despawn_descendants`.

use crate::prelude::*;

macro_rules! impl_root_types {
    ($name:ident) => {
        paste::paste! {
            #[derive(Component, Debug, Reflect)]
            pub struct[<$name Component>];

            #[derive(Bundle)]
            pub struct[<$name Bundle>] {
                name: Name,
                marker: [<$name Component>],
                spatial: SpatialBundle,
            }
            impl [<$name Bundle>] {
                fn new(offset: Vec3) -> Self {
                    Self {
                        name: Name::new(stringify!($name)),
                        marker: [<$name Component>],
                        spatial: SpatialBundle::from_transform(Transform::from_translation(offset)),
                    }
                }
            }

            #[derive(Resource, Debug, Reflect)]
            pub struct $name {
                eid: Entity,
            }
            impl $name {
                pub fn eid(&self) -> Entity {
                    self.eid
                }
            }
        }
    };
}

macro_rules! impl_root_init {
    ($($name:ident$({
        $(offset: $offset:expr,)?
    })?),*) => {
        $(
            impl_root_types!($name);
        )*

        paste::paste! {
            fn setup_roots(
                mut commands: Commands,
                $(
                    #[allow(nonstandard_style)]
                    mut [<$name _res>]: ResMut<$name>,
                )*
            ) {
                $(
                    #[allow(unused_mut)]
                    let mut root_pos = Vec3::ZERO;
                    $($(
                        root_pos.x = $offset.x;
                        root_pos.y = $offset.y;
                        root_pos.z = $offset.z;
                    )?)?
                    #[allow(nonstandard_style)]
                    let [<$name _eid>] = commands.spawn([<$name Bundle>]::new(root_pos)).id();
                    [<$name _res>].eid = [<$name _eid>];
                )*
            }
        }

        #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
        pub struct RootInit;

        pub(super) struct RootPlugin;
        impl Plugin for RootPlugin {
            fn build(&self, app: &mut App) {
                $(
                    app.insert_resource($name {
                        eid: Entity::PLACEHOLDER,
                    });
                )*

                app.add_systems(Startup, setup_roots.in_set(RootInit));
            }
        }
    };
}

impl_root_init!(
    CollisionRoot,
    ConvoRoot,
    CutsceneRoot,
    DebugRoot {
        offset: Vec3::new(0.0, 0.0, ZIX_DEBUG),
    },
    LayerRoot,
    MenuRoot {
        offset: Vec3::new(0.0, 0.0, ZIX_MENU),
    },
    ParticlesRoot {
        offset: Vec3::new(0.0, 0.0, ZIX_PARTICLES),
    },
    PauseRoot {
        offset: Vec3::new(0.0, 0.0, ZIX_PAUSE),
    },
    SoundRoot,
    TransitionRoot {
        offset: Vec3::new(0.0, 0.0, ZIX_TRANSITION),
    },
    TutorialRoot
);
