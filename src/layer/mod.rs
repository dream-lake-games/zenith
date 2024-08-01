use bevy::render::camera::RenderTarget;
use bevy::render::{
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    texture::BevyDefault,
    view::RenderLayers,
};
use bevy::sprite::MaterialMesh2dBundle;
use mats::BlendTexturesMaterial;

use crate::{prelude::*, tran_tran};

mod mats;

#[derive(Component, Debug, Reflect, Default)]
pub struct BgSpriteLayer;
#[derive(Component, Debug, Reflect, Default)]
pub struct BgLightLayer;
#[derive(Component, Debug, Reflect, Default)]
pub struct SpriteLayer;
#[derive(Component, Debug, Reflect, Default)]
pub struct LightLayer;
#[derive(Component, Debug, Reflect, Default)]
pub struct MenuLayer;

trait CameraLayerInternal {
    const RENDER_LAYER: usize;
}
pub trait CameraLayer: CameraLayerInternal {
    fn layer() -> usize {
        Self::RENDER_LAYER
    }

    fn render_layers() -> RenderLayers {
        RenderLayers::from_layers(&[Self::RENDER_LAYER])
    }
}

impl CameraLayerInternal for BgSpriteLayer {
    const RENDER_LAYER: usize = 1;
}
impl CameraLayer for BgSpriteLayer {}
impl CameraLayerInternal for BgLightLayer {
    const RENDER_LAYER: usize = 2;
}
impl CameraLayer for BgLightLayer {}
impl CameraLayerInternal for SpriteLayer {
    const RENDER_LAYER: usize = 3;
}
impl CameraLayer for SpriteLayer {}
impl CameraLayerInternal for LightLayer {
    const RENDER_LAYER: usize = 4;
}
impl CameraLayer for LightLayer {}
impl CameraLayerInternal for MenuLayer {
    const RENDER_LAYER: usize = 5;
}
impl CameraLayer for MenuLayer {}

/// Grows all of the layers by a given scale.
/// Makes it easy for the game to fill the screen in a satisfying way.
#[derive(Resource)]
pub struct LayerGrowth {
    scale: f32,
}
impl LayerGrowth {
    impl_get_set_with!(scale, f32);
}
impl Default for LayerGrowth {
    fn default() -> Self {
        Self { scale: 1.0 }
    }
}

/// Configures the clear colors and ambient light of the layers.
#[derive(Resource)]
pub struct LayerColors {
    bg_clear_color: ClearColorConfig,
    bg_ambient_light: ClearColorConfig,
    clear_color: ClearColorConfig,
    ambient_light: ClearColorConfig,
    menu_clear_color: ClearColorConfig,
}
impl LayerColors {
    impl_get_set_with!(bg_clear_color, ClearColorConfig);
    impl_get_set_with!(bg_ambient_light, ClearColorConfig);
    impl_get_set_with!(clear_color, ClearColorConfig);
    impl_get_set_with!(ambient_light, ClearColorConfig);
    impl_get_set_with!(menu_clear_color, ClearColorConfig);
}
impl Default for LayerColors {
    fn default() -> Self {
        Self {
            bg_clear_color: ClearColorConfig::Default,
            bg_ambient_light: ClearColorConfig::Custom(Color::srgb(0.5, 0.5, 0.5)),
            clear_color: ClearColorConfig::Custom(Color::srgba(0.1, 0.1, 0.1, 0.05)),
            ambient_light: ClearColorConfig::Custom(Color::srgb(0.6, 0.6, 0.6)),
            menu_clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        }
    }
}

#[derive(Resource)]
struct CameraTargets {
    ideal_size: UVec2,
    menu_growth: u32,
    bg_sprite_target: Handle<Image>,
    bg_light_target: Handle<Image>,
    sprite_target: Handle<Image>,
    light_target: Handle<Image>,
    menu_target: Handle<Image>,
}
impl Default for CameraTargets {
    fn default() -> Self {
        Self {
            ideal_size: IDEAL_VEC,
            menu_growth: MENU_GROWTH,
            bg_sprite_target: default(),
            bg_light_target: default(),
            sprite_target: default(),
            light_target: default(),
            menu_target: default(),
        }
    }
}
impl CameraTargets {
    /// Creates actually images that the various layers can write to to place on quads.
    pub fn initialize(&mut self, images: &mut Assets<Image>) {
        macro_rules! make_layer_image {
            ($label:expr, $unique_u128:expr, $size:expr) => {{
                let target_extent = Extent3d {
                    width: $size.x,
                    height: $size.y,
                    ..default()
                };

                // Makes the image
                let mut image = Image {
                    texture_descriptor: TextureDescriptor {
                        label: Some($label),
                        size: target_extent,
                        dimension: TextureDimension::D2,
                        format: TextureFormat::bevy_default(),
                        mip_level_count: 1,
                        sample_count: 1,
                        usage: TextureUsages::TEXTURE_BINDING
                            | TextureUsages::COPY_DST
                            | TextureUsages::RENDER_ATTACHMENT,
                        view_formats: &[],
                    },
                    ..default()
                };
                // Fills it with zeros
                image.resize(target_extent);
                let handle: Handle<Image> = Handle::weak_from_u128($unique_u128);
                images.insert(handle.id(), image);
                handle
            }};
        }

        self.bg_light_target =
            make_layer_image!("target_bg_light", 84562364042238462870, self.ideal_size);
        self.bg_sprite_target =
            make_layer_image!("target_bg_sprite", 81297563682952991276, self.ideal_size);
        self.light_target =
            make_layer_image!("target_light", 84562364042238462871, self.ideal_size);
        self.sprite_target =
            make_layer_image!("target_sprite", 81297563682952991277, self.ideal_size);
        self.menu_target = make_layer_image!(
            "target_menu",
            51267563632952991278,
            self.ideal_size * self.menu_growth
        );
    }
}

macro_rules! impl_layer_quad_n_mat {
    ($prefix:ident, $mat_type:ty, $unique_u128s:expr) => {
        paste::paste! {
            const [<$prefix _QUAD>]: Handle<Mesh> = Handle::weak_from_u128($unique_u128s);
            const [<$prefix _MATERIAL>]: Handle<$mat_type> = Handle::weak_from_u128($unique_u128s + 1);
        }
    };
}

impl_layer_quad_n_mat!(BG_PP, BlendTexturesMaterial, 23467206864860343677);
impl_layer_quad_n_mat!(PP, BlendTexturesMaterial, 53466206864860343678);
const MENU_MATERIAL: Handle<Image> = Handle::weak_from_u128(36467206864860383170);

fn remake_layering_materials(
    camera_targets: &CameraTargets,
    blend_materials: &mut ResMut<Assets<BlendTexturesMaterial>>,
) {
    let bg_material = BlendTexturesMaterial {
        sprite_texture: camera_targets.bg_sprite_target.clone(),
        light_texture: camera_targets.bg_light_target.clone(),
    };
    let material = BlendTexturesMaterial {
        sprite_texture: camera_targets.sprite_target.clone(),
        light_texture: camera_targets.light_target.clone(),
    };
    blend_materials.insert(BG_PP_MATERIAL.id(), bg_material);
    blend_materials.insert(PP_MATERIAL.id(), material);
}

fn setup_layer_materials(
    root: Res<LayerRoot>,
    mut commands: Commands,
    mut camera_targets: ResMut<CameraTargets>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut blend_materials: ResMut<Assets<BlendTexturesMaterial>>,
) {
    // Initialize the materials needed for the camers
    camera_targets.initialize(&mut images);
    remake_layering_materials(&camera_targets, &mut blend_materials);
    let combined_layer = RenderLayers::from_layers(&[30]);

    // Create the meshes
    let ideal_quad = Mesh::from(Rectangle::new(
        camera_targets.ideal_size.x as f32,
        camera_targets.ideal_size.y as f32,
    ));
    meshes.insert(BG_PP_QUAD.id(), ideal_quad.clone());
    meshes.insert(PP_QUAD.id(), ideal_quad.clone());

    // Bg and fg layers are the same
    macro_rules! spawn_layer_mat_mesh {
        ($name:expr, $quad:expr, $mat:expr, $z:expr) => {{
            commands
                .spawn((
                    Name::new($name),
                    MaterialMesh2dBundle {
                        mesh: $quad.clone().into(),
                        material: $mat,
                        transform: Transform {
                            translation: Vec3::Z * $z,
                            ..default()
                        },
                        ..default()
                    },
                    combined_layer.clone(),
                ))
                .set_parent(root.eid());
        }};
    }
    spawn_layer_mat_mesh!("bg_layer_quad", BG_PP_QUAD, BG_PP_MATERIAL, 1.0);
    spawn_layer_mat_mesh!("fg_layer_quad", PP_QUAD, PP_MATERIAL, 2.0);

    // Menu layer is special
    commands.spawn((
        Name::new("menu_layer_quad"),
        SpriteBundle {
            transform: tran_tran!(Vec3::Z * 3.0),
            texture: MENU_MATERIAL,
            ..default()
        },
        combined_layer.clone(),
    ));

    // This is the camera that sees all of the layer quads and squashes them into one image
    commands
        .spawn((
            Name::new("final_camera"),
            Camera2dBundle {
                camera: Camera {
                    order: 6,
                    ..default()
                },
                ..default()
            },
            InheritedVisibility::VISIBLE,
            combined_layer,
        ))
        .set_parent(root.eid());
}

fn setup_layer_cameras(
    mut commands: Commands,
    camera_targets: Res<CameraTargets>,
    layer_colors: Res<LayerColors>,
    layer_root: Res<LayerRoot>,
) {
    macro_rules! spawn_layer_camera {
        ($comp:ty, $name:expr, $order:expr, $image:expr, $clear_color:expr) => {{
            commands
                .spawn((
                    Name::new($name),
                    Camera2dBundle {
                        camera: Camera {
                            order: $order,
                            target: RenderTarget::Image($image),
                            clear_color: $clear_color,
                            ..default()
                        },
                        projection: OrthographicProjection {
                            near: ZIX_MIN,
                            far: ZIX_MAX,
                            ..default()
                        },
                        ..Default::default()
                    },
                    <$comp>::default(),
                    <$comp>::render_layers(),
                ))
                .set_parent(layer_root.eid());
        }};
    }
    spawn_layer_camera!(
        BgLightLayer,
        "bg_light_camera",
        0,
        camera_targets.bg_light_target.clone(),
        layer_colors.bg_ambient_light
    );
    spawn_layer_camera!(
        BgSpriteLayer,
        "bg_sprite_camera",
        1,
        camera_targets.bg_sprite_target.clone(),
        layer_colors.bg_clear_color
    );
    spawn_layer_camera!(
        LightLayer,
        "fg_light_camera",
        2,
        camera_targets.light_target.clone(),
        layer_colors.ambient_light
    );
    spawn_layer_camera!(
        SpriteLayer,
        "fg_sprite_camera",
        3,
        camera_targets.sprite_target.clone(),
        layer_colors.clear_color
    );
    spawn_layer_camera!(
        MenuLayer,
        "menu_camera",
        4,
        camera_targets.menu_target.clone(),
        layer_colors.menu_clear_color
    );
}

#[derive(Default)]
pub(super) struct LayeringPlugin {
    ideal_size: UVec2,
    menu_growth: u32,
    layer_colors: LayerColors,
    layer_growth: LayerGrowth,
}
impl LayeringPlugin {
    pub fn new(ideal_size: UVec2, menu_growth: u32) -> Self {
        Self {
            ideal_size,
            menu_growth,
            ..default()
        }
    }
    impl_get_set_with!(layer_colors, LayerColors);
    impl_get_set_with!(layer_growth, LayerGrowth);
}
impl Plugin for LayeringPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LayerColors::default());
        app.insert_resource(CameraTargets::default());

        app.add_systems(
            Startup,
            (setup_layer_materials, setup_layer_cameras)
                .chain()
                .after(RootInit),
        );
    }
}
