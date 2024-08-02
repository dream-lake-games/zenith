use crate::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::{
    AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState, RenderPipelineDescriptor,
    ShaderRef, SpecializedMeshPipelineError,
};
use bevy::sprite::{Material2d, Material2dKey};

pub const BLEND_ADD: BlendState = BlendState {
    color: BlendComponent {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::OneMinusSrcAlpha,
        operation: BlendOperation::Add,
    },
    alpha: BlendComponent {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::OneMinusSrcAlpha,
        operation: BlendOperation::Add,
    },
};

/// The material doing the heavy lifting for our layering/lighting.
#[derive(AsBindGroup, TypePath, Asset, Debug, Clone)]
pub struct BlendTexturesMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub sprite_texture: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub light_texture: Handle<Image>,
}

impl Material2d for BlendTexturesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/blend_light.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(fragment) = &mut descriptor.fragment {
            if let Some(target_state) = &mut fragment.targets[0] {
                target_state.blend = Some(BLEND_ADD);
            }
        }

        Ok(())
    }
}
