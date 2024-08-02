use crate::prelude::*;

/// Let's map out ideal usage first
///
/// IDEA TO TRY: Separate the use cases of textures and animations
/// -> Problem: Think of the spring, need to support animated textures
/// -> Maybe solution: keep the same (pretty) powerful animation mat but just
///    make separate macros for the use cases?
/// -> Maybe solution: keep the same (pretty) powerful animation mat, but make separete
///    macros and also separate components to mange ecs? (I think I like this more)
/// -> Design goal: Let's seee how dead simple we can make animations
///
/// animation_bundle!(
///     AnimationLenny,
///     bodies: [
///         fly: {
///             path: "lenny/fly.png", [REQUIRED]
///             size: (16, 16), [REQUIRED]
///             length: 3, [OPTIONAL (assumed 1)]
///             fps: 12.0, [OPTIONAL (assumed DEFAULT_ANIMATION_FPS)]
///             color: Color::WHITE, [OPTIONAL (assumed Color::WHITE)]
///             offset: (x, y, z) [OPTIONAL (assumed (0, 0, 0))]
///             scale: (x, y) [OPTIONAL (assumed (1, 1))]
///             render_layers: expr [OPTIONAL (assumed SpriteLayer::render_layers())]
///         },
///         light: {
///             path: "lenny/light.png",
///             size: (64, 64),
///         }
///         fall: {
///             ...assume this existed...
///         }
///     ],
///     states: [
///         Fly: {
///             bodies: [
///                 fly: {
///                     override_offset: (x, y, z) [OPTIONAL, if set will override what is set in bodies],
///                     override_scale: (x, y) [OPTIONAL, if set will override what is set in bodies],
///                     override_color: expr [OPTIONAL, if set will override what is set in bodies],
///                 },
///                 light,
///             ],
///             next: Fall,
///         },
///         Fall: {
///             bodies: [
///                 fall,
///                 light,
///             ],
///             next: HideAndDie(f32), (after the animation of the FIRST BODY finishes, hide this animation and insert Dying(f32),
///         }
///         Stable: {
///             bodies: [
///                 fly,
///                 light,
///             ]
///         }
///     ],
///     properties: { // This whole field is optional
///         hidden: bool, [OPTIONAl (assumed false)]
///         flip_x: bool, [OPTIONAL (assumed false)]
///         flip_y: bool, [OPTIONAL (assumed false)]
///         flip_y: bool, [OPTIONAL (assumed false)]
///     }
/// )
///
/// Now going to write (below DUmmy) what code this should produce
///
struct DUmmy;

struct BodyData {
    path: String,
    size: UVec2,
    length: u32,
    fps: f32,
    color: Color,
    offset: Vec3,
    scale: Vec2,
    render_layers: RenderLayers,
}

#[derive(Default)]
struct BodyDataOverrides {
    override_offset: Option<Vec3>,
    override_scale: Option<Vec2>,
    override_color: Option<Color>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum AnimationLennyFile {
    #[allow(nonstandard_style)]
    fly,
    #[allow(nonstandard_style)]
    light,
    #[allow(nonstandard_style)]
    fall,
}
impl AnimationLennyFile {
    fn to_body_data(&self) -> BodyData {
        todo!("boilerplate")
    }
}

struct StateData<NextType, BodyType> {
    overwritten_bodies: Vec<(BodyType, BodyDataOverrides)>,
    next: Option<NextType>,
}

trait AnimationStateMachine: Sized {
    type FileType;

    fn to_state_data(&self) -> StateData<Self, Self::FileType>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AnimationLenny {
    Fly,
    Fall,
    Stable,
}

impl AnimationStateMachine for AnimationLenny {
    type FileType = AnimationLennyFile;

    fn to_state_data(&self) -> StateData<Self, Self::FileType> {
        StateData {
            overwritten_bodies: todo!("boilerplate?"),
            next: todo!("boilerplate!?"),
        }
    }
}

struct AnimationManager<StateMachine: AnimationStateMachine> {
    current_state: StateMachine,
}
