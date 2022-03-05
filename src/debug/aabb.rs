#![allow(unused_imports)]

use bevy::{
    core_pipeline::Transparent3d,
    ecs::system::{
        lifetimeless::{Read, SQuery, SRes},
        SystemParamItem,
    },
    pbr::{
        MaterialPipeline, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
        SetMeshViewBindGroup,
    },
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::GpuBufferInfo,
        render_asset::{PrepareAssetError, RenderAsset, RenderAssetPlugin, RenderAssets},
        render_component::{ExtractComponent, ExtractComponentPlugin},
        render_phase::{
            AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
            SetItemPipeline, TrackedRenderPass,
        },
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer,
            BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages, PrimitiveTopology,
            RenderPipelineCache, RenderPipelineDescriptor, ShaderStages, SpecializedPipeline,
            SpecializedPipelines, VertexAttribute, VertexBufferLayout, VertexFormat,
            VertexStepMode,
        },
        renderer::RenderDevice,
        view::ExtractedView,
        RenderApp, RenderStage, RenderWorld,
    },
};
use bytemuck::{Pod, Zeroable};

use crate::GlobalAabb;

pub struct GlobalAabbMaterialPlugin;

impl Plugin for GlobalAabbMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<AabbMaterial>::default())
            .init_resource::<AabbMaterial>();

        app.sub_app_mut(RenderApp)
            .add_system_to_stage(RenderStage::Extract, extract_aabb_material);
        //         .add_render_command::<Transparent3d, DrawCustom>()
        //         .init_resource::<AabbPipeline>()
        //         .init_resource::<SpecializedPipelines<AabbPipeline>>()
        //         .add_system_to_stage(RenderStage::Queue, queue_custom)
        //         .add_system_to_stage(RenderStage::Prepare, prepare_instance_buffers);
    }
}

// This is the struct that will be passed to your shader
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "5d5c4565-2537-49d8-87a5-88ec96c4e3ec"]
pub struct AabbMaterial {
    color: Color,
    //width: f32,
}

impl Default for AabbMaterial {
    fn default() -> Self {
        Self {
            color: Color::RED,
            //width: 1.0,
        }
    }
}

pub fn extract_aabb_material(
    mut commands: Commands,
    aabb_material: Res<AabbMaterial>,
) {
    // If the aabb material has changed
    if aabb_material.is_changed() {
        // Update the aabb_material resource in the render world
        commands.insert_resource(aabb_material.clone())
    }
}

#[derive(Clone)]
pub struct GpuAabbMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

// The implementation of [`Material`] needs this impl to work properly.
impl RenderAsset for AabbMaterial {
    type ExtractedAsset = AabbMaterial;
    type PreparedAsset = GpuAabbMaterial;
    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let color = Vec4::from_slice(&extracted_asset.color.as_linear_rgba_f32());
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: color.as_std140().as_bytes(),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuAabbMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl Material for AabbMaterial {
    // When creating a custom material, you need to define either a vertex shader, a fragment shader or both.
    // If you don't define one of them it will use the default mesh shader which can be found at
    // <https://github.com/bevyengine/bevy/blob/latest/crates/bevy_pbr/src/render/mesh.wgsl>

    // For this example we don't need a vertex shader
    // fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
    //     // Use the same path as the fragment shader since wgsl let's you define both shader in the same file
    //     Some(asset_server.load("shaders/custom_material.wgsl"))
    // }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/aabb.wgsl"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
                },
                count: None,
            }],
            label: None,
        })
    }
}
