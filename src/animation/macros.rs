#[macro_export]
macro_rules! defn_animation {
    (
        $name:ident $(,)?
        bodies: [
            $(
                $body_id:ident $(:)? {
                    path: $path:expr,
                    size: ($w:expr, $h:expr),
                    $(
                        length: $length:expr,
                    )?
                    $(
                        fps: $fps:expr,
                    )?
                    $(
                        color: $color:expr,
                    )?
                    $(
                        offset: $offset:expr,
                    )?
                    $(
                        scale: ($scale_w:expr, $scale_h:expr),
                    )?
                    $(
                        render_layers: $render_layers:expr,
                    )?
                } $(,)?
            )+
        ] $(,)?
        states: [
            $(
                $state_id:ident $(:)? {
                    parts: [
                        $(
                            $part_id:ident
                            $(
                                : {
                                    $(
                                        override_offset: ($oox:expr, $ooy:expr, $ooz:expr),
                                    )?
                                    $(
                                        override_scale: ($osx:expr, $osy:expr, $osz:expr),
                                    )?
                                    $(
                                        override_color: $osc:expr,
                                    )?
                                }
                            )?
                            $(,)?
                        )+
                    ],
                    $(
                        #[special]
                        next: HideThenDie($hide_then_die_time:expr),
                    )?
                    $(
                        next: $next_id:ident,
                    )?
                } $(,)?
            )+
        ] $(,)?
    ) => {
        paste::paste! {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
            #[allow(nonstandard_style)]
            pub enum [<Body_ $name>] {
                $(
                    $body_id,
                )+
            }
            impl AnimationBody for [<Body_ $name>] {
                fn to_body_data(&self) -> BodyData {
                    match &self {
                        $(
                            Self::$body_id => {
                                #[allow(unused, unused_mut)]
                                let mut length = 1;
                                #[allow(unused, unused_mut)]
                                let mut fps = DEFAULT_ANIMATION_FPS;
                                #[allow(unused, unused_mut)]
                                let mut color = Color::WHITE;
                                #[allow(unused, unused_mut)]
                                let mut offset = Vec3::ZERO;
                                #[allow(unused, unused_mut)]
                                let mut scale = Vec2::ONE;
                                #[allow(unused, unused_mut)]
                                let mut render_layers = SpriteLayer::render_layers();

                                $(
                                    length = $length;
                                )?
                                $(
                                    fps = $fps;
                                )?
                                $(
                                    color = $color;
                                )?
                                $(
                                    offset = $offset;
                                )?
                                $(
                                    scale = Vec2::new($scale_w, $scale_h);
                                )?
                                $(
                                    render_layers = $render_layers;
                                )?

                                BodyData {
                                    path: $path.into(),
                                    size: UVec2::new($w, $h),
                                    length,
                                    fps,
                                    color,
                                    offset,
                                    scale,
                                    render_layers,
                                }
                            }
                        )+
                    }
                }
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Reflect)]
            pub enum $name {
                #[default]
                $($state_id,)+
            }
            impl AnimationStateMachine for $name {
                type FileType = [<Body_ $name>];

                fn to_state_data(&self) -> StateData<Self, Self::FileType> {
                    match &self {
                        $(
                            Self::$state_id => {
                                let mut overwritten_bodies = vec![];

                                $(
                                    let part_id = Self::FileType::$part_id;
                                    #[allow(unused, unused_mut)]
                                    let mut overwrite = BodyDataOverrides::default();
                                    overwritten_bodies.push((part_id, overwrite));
                                )+

                                #[allow(unused, unused_mut)]
                                let mut next_state = AnimationNextState::None;
                                $(
                                    next_state = AnimationNextState::HideThenDie($hide_then_die_time);
                                )?
                                $(
                                    next_state = AnimationNextState::Some(Self::$next_id);
                                )?

                                StateData {
                                    overwritten_bodies,
                                    next: next_state,
                                }
                            }
                        )+
                    }
                }
            }
        }
    };
}
pub use defn_animation;
