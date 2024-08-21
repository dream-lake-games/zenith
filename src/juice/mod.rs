use crate::prelude::*;

trait Juiceable:
    Component + std::fmt::Debug + Clone + Reflect + FromReflect + TypePath + GetTypeRegistration
{
}

trait SimpleVisualJuice: Juiceable {
    type StateMachine: AnimationStateMachine;

    fn get_name(&self) -> String;

    fn get_transform(&self) -> Transform;
}

#[derive(Bundle)]
struct SimpleVisualJuiceBundle<StateMachine: AnimationStateMachine> {
    name: Name,
    spatial: SpatialBundle,
    animation: AnimationManager<StateMachine>,
    mirage: MirageAnimationManager,
}
impl<StateMachine: AnimationStateMachine> SimpleVisualJuiceBundle<StateMachine> {
    fn new(name: String, transform: Transform, wrap: Vec2) -> Self {
        Self {
            name: Name::new(name),
            spatial: SpatialBundle::from_transform(transform),
            animation: AnimationManager::new(),
            mirage: MirageAnimationManager::wrap_offsets(wrap),
        }
    }
}

fn monitor_juice<Trigger: SimpleVisualJuice, StateMachine: AnimationStateMachine>(
    mut commands: Commands,
    triggers: Query<(Entity, &Trigger)>,
    root: Res<ParticlesRoot>,
    room_state: Res<State<MetaState>>,
) {
    let wrap = room_state.wrap_size();
    for (eid, trigger) in &triggers {
        commands.entity(eid).despawn_recursive();
        commands
            .spawn(SimpleVisualJuiceBundle::<StateMachine>::new(
                trigger.get_name(),
                trigger.get_transform(),
                wrap,
            ))
            .set_parent(root.eid());
    }
}

macro_rules! defn_simple_visual_juice {
    ($name:ident, $state_machine:ty) => {
        #[derive(Component, Debug, Clone, Reflect)]
        pub struct $name {
            pub transform: Transform,
        }
        impl $name {
            pub fn new(transform: Transform) -> Self {
                Self { transform }
            }

            fn register(app: &mut App) {
                app.add_systems(Update, monitor_juice::<$name, $state_machine>);
            }
        }
        impl Juiceable for $name {}
        impl SimpleVisualJuice for $name {
            type StateMachine = $state_machine;
            fn get_name(&self) -> String {
                format!("{}", stringify!($name))
            }
            fn get_transform(&self) -> Transform {
                self.transform.clone()
            }
        }
    };
}

defn_simple_visual_juice!(RingShrink, AnimationRingShrink);

pub(super) struct JuicePlugin;
impl Plugin for JuicePlugin {
    fn build(&self, app: &mut App) {
        RingShrink::register(app);
    }
}
