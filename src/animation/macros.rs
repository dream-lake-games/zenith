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
            pub enum [<AnimationBody_ $name>] {
                $(
                    $body_id,
                )+
            }
            impl AnimationBody for [<AnimationBody_ $name>] {
                fn to_body_data(&self) -> AnimationBodyData {
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

                                AnimationBodyData {
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
                type BodyType = [<AnimationBody_ $name>];

                fn to_state_data(&self) -> AnimationStateData<Self, Self::BodyType> {
                    match &self {
                        $(
                            Self::$state_id => {
                                let mut overwritten_bodies = vec![];

                                $(
                                    let part_id = Self::BodyType::$part_id;
                                    #[allow(unused, unused_mut)]
                                    let mut overwrite = AnimationBodyDataOverrides::default();
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

                                AnimationStateData {
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

#[macro_export]
macro_rules! defn_texture {
    (
        $name:ident $(,)?
        textures: [
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
                        growth: ($growth_x:expr, $growth_y:expr),
                    )?
                    $(
                        z_offset: $z_offset:expr,
                    )?
                    $(
                        color: $color:expr,
                    )?
                    $(
                        render_layers: $render_layers:expr,
                    )?
                } $(,)?
            )+
        ] $(,)?
        parts: [
            $(
                $part_id:ident,
            )+
        ] $(,)?
        states: [
            $(
                $state_id:ident: [
                    $(
                        $assign_id:ident: $assign_val:ident,
                    )+
                ] $(,)?
            )+
        ] $(,)?
    ) => {
        paste::paste! {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
            #[allow(nonstandard_style)]
            pub enum [<TextureBody_ $name>] {
                $(
                    $body_id,
                )+
            }
            impl TextureBody for [<TextureBody_ $name>] {
                fn to_body_data(&self) -> TextureBodyData {
                    match &self {
                        $(
                            Self::$body_id => {
                                #[allow(unused, unused_mut)]
                                let mut length = 1;
                                #[allow(unused, unused_mut)]
                                let mut fps = DEFAULT_ANIMATION_FPS;
                                #[allow(unused, unused_mut)]
                                let mut growth = TextureGrowth::default();
                                #[allow(unused, unused_mut)]
                                let mut z_offset = 0.0;
                                #[allow(unused, unused_mut)]
                                let mut color = Color::WHITE;
                                #[allow(unused, unused_mut)]
                                let mut render_layers = SpriteLayer::render_layers();

                                $(
                                    length = $length;
                                )?
                                $(
                                    fps = $fps;
                                )?
                                $(
                                    growth.x = $growth_x;
                                    growth.y = $growth_y;
                                )?
                                $(
                                    z_offset = $z_offset;
                                )?
                                $(
                                    color = $color;
                                )?
                                $(
                                    render_layers = $render_layers;
                                )?

                                TextureBodyData {
                                    path: $path.into(),
                                    size: UVec2::new($w, $h),
                                    length,
                                    fps,
                                    growth,
                                    z_offset,
                                    color,
                                    render_layers,
                                }
                            }
                        )+
                    }
                }
            }
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
            #[allow(nonstandard_style)]
            pub enum [<$name Part>] {
                $(
                    $part_id,
                )+
            }
            impl TexturePart for [<$name Part>] {
                fn all() -> Vec<Self> {
                    vec![
                        $(
                            Self::$part_id,
                        )+
                    ]
                }
            }
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect, Default)]
            #[allow(nonstandard_style)]
            pub enum [<$name State>] {
                #[default]
                $(
                    $state_id,
                )+
            }
            impl TextureStateMachine for [<$name State>] {
                type BodyType = [<TextureBody_ $name>];
                type PartType = [<$name Part>];

                fn part_to_body(&self, part: Self::PartType) -> Self::BodyType {
                    match self {
                        $(
                            Self::$state_id => {
                                match part {
                                    $(
                                        Self::PartType::$assign_id => Self::BodyType::$assign_val,
                                    )+
                                }
                            }
                        )+
                    }
                }
            }
        }
    };
}
pub use defn_texture;
