use std::{error::Error, mem::size_of};

use bytemuck::offset_of;
use log::info;
use magritte::{
    vulkan1_0::{
        BlendFactor, BlendOp, ColorComponentFlags, CompareOp, Extent2D, Format, FrontFace, GraphicsPipelineCreateInfo,
        LogicOp, Offset2D, Pipeline as VkPipeline, PipelineColorBlendAttachmentState,
        PipelineColorBlendStateCreateInfo, PipelineDepthStencilStateCreateInfo, PipelineDynamicStateCreateInfo,
        PipelineInputAssemblyStateCreateInfo, PipelineLayoutCreateInfo, PipelineMultisampleStateCreateInfo,
        PipelineRasterizationStateCreateInfo, PipelineVertexInputStateCreateInfo, PipelineViewportStateCreateInfo,
        PolygonMode, PrimitiveTopology, Rect2D, SampleCountFlagBits, ShaderStageFlagBits, StencilOp, StencilOpState,
        VertexInputAttributeDescription, VertexInputBindingDescription, VertexInputRate, Viewport,
    },
    AsRaw, Unique,
};
use magritte_samples::{renderpass::RenderPass, shader::Shader, surface::Surface, vulkan::Vulkan};

use crate::Vertex;

/// The SPIR-V of the compiled vertex shader
static VERTEX_SHADER: &[u8] = include_bytes!("./shaders/texture.vert.spv");

/// The SPIR-V of the compiled fragment shader
static FRAGMENT_SHADER: &[u8] = include_bytes!("./shaders/texture.frag.spv");

/// Container for a simple graphics pipeline
pub struct Pipeline {
    pub pipeline: Unique<VkPipeline>,
}

impl Pipeline {
    /// Creates a new very simple pipeline
    pub fn new(vulkan: &Vulkan, renderpass: &RenderPass, surface: &Surface) -> Result<Self, Box<dyn Error>> {
        let vertex = Shader::new(vulkan, VERTEX_SHADER, ShaderStageFlagBits::VERTEX)?;
        let fragment = Shader::new(vulkan, FRAGMENT_SHADER, ShaderStageFlagBits::FRAGMENT)?;

        // We don't have any information to put in the layout
        let layout_create_info = PipelineLayoutCreateInfo::default();

        // We create the layout (even though it empty)
        let (pipeline_layout, _) = unsafe { vulkan.device().create_pipeline_layout(&layout_create_info, None)? };

        info!("Created pipeline layout: {:?}", pipeline_layout.as_raw());

        // Here we prepare the shader stages that are part of the pipeline
        let shader_stage_create_infos = [
            vertex.as_shader_stage_create_info(),
            fragment.as_shader_stage_create_info(),
        ];

        // Here we set the input sized, in our case, a single vertex.
        // If we had multiple vertex buffers (for instancing for example), then we would specify more than
        // one. However, in most cases, you only have one vertex input.
        let vertex_input_binding_description = VertexInputBindingDescription {
            binding: 0,
            stride: size_of::<Vertex>() as u32,
            input_rate: VertexInputRate::VERTEX,
        };

        // Here, we define the inputs of the **vertex** shader, in our case, one vec4<f32> and one vec4<u8>
        // For each, we need to define four attributes:
        // - the location, that is the location within one binding (one binding is one
        //   `VertexInputBindingDescription`)
        // - the binding, that is the binding index
        // - the format that tells Vulkan the size and how to interpret it
        // - the offset, that tells Vulkan **where** is is in the structure
        let vertex_input_attribute_descriptions = [
            VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: Format::R32G32_SFLOAT,
                offset: offset_of!(Vertex, position) as u32,
            },
            VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: Format::R32G32_SFLOAT,
                offset: offset_of!(Vertex, uv) as u32,
            },
        ];

        // Here, we gather all of the information about the vertex shader inputs
        let vertex_input_state_info = PipelineVertexInputStateCreateInfo::default()
            .with_vertex_attribute_descriptions(&vertex_input_attribute_descriptions)
            .with_vertex_binding_descriptions(std::slice::from_ref(&vertex_input_binding_description));

        // We define the topology of our primitives, in this case a list of triangles.
        // This structure is very powerful and can be use for a lot more.
        let vertex_input_assembly_state_info =
            PipelineInputAssemblyStateCreateInfo::default().with_topology(PrimitiveTopology::TRIANGLE_LIST);

        // We define the viewport we will use.
        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: surface.extent().width() as f32,
            height: surface.extent().height() as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        // We define where on screen we want to draw
        let scissor = Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent: Extent2D {
                width: surface.extent().width(),
                height: surface.extent().height(),
            },
        };

        // We collect the viewport and the scissor
        let viewport_state_info = PipelineViewportStateCreateInfo::default()
            .with_scissors(std::slice::from_ref(&scissor))
            .with_viewports(std::slice::from_ref(&viewport));

        // We configure the rasterization
        let rasterization_info = PipelineRasterizationStateCreateInfo::default()
            .with_front_face(FrontFace::COUNTER_CLOCKWISE)
            .with_line_width(1.0)
            .with_polygon_mode(PolygonMode::FILL);

        // We don't do multisampling
        let multisample_state_info =
            PipelineMultisampleStateCreateInfo::default().with_rasterization_samples(SampleCountFlagBits::_1);

        // We don't care about the stencil, we create a NOOP stencil state.
        let depth_stencil_state = StencilOpState::default()
            .with_fail_op(StencilOp::KEEP)
            .with_pass_op(StencilOp::KEEP)
            .with_depth_fail_op(StencilOp::KEEP)
            .with_compare_op(CompareOp::ALWAYS);

        let depth_state_info = PipelineDepthStencilStateCreateInfo::default()
            .with_depth_test_enable(true)
            .with_depth_write_enable(true)
            .with_depth_compare_op(CompareOp::LESS_OR_EQUAL)
            .with_front(depth_stencil_state)
            .with_back(depth_stencil_state)
            .with_max_depth_bounds(1.0);

        // How to blend color
        // One per output attachment
        let color_blend_attachment_state = PipelineColorBlendAttachmentState::default()
            .with_blend_enable(false)
            .with_src_color_blend_factor(BlendFactor::SRC_COLOR)
            .with_dst_color_blend_factor(BlendFactor::ONE_MINUS_DST_COLOR)
            .with_color_blend_op(BlendOp::ADD)
            .with_src_alpha_blend_factor(BlendFactor::ZERO)
            .with_dst_alpha_blend_factor(BlendFactor::ZERO)
            .with_alpha_blend_op(BlendOp::ADD)
            .with_color_write_mask(ColorComponentFlags::all());

        // We gather the blending information for each output
        let color_blend_state = PipelineColorBlendStateCreateInfo::default()
            .with_logic_op(LogicOp::CLEAR)
            .with_attachments(std::slice::from_ref(&color_blend_attachment_state));

        // What state is dynamic (in our case nothing)
        // A typical use case for dynamic state would be a dynamic viewport and scissor
        // to avoid having to recreated the pipeline when resizing the window.
        let dynamic_state = [];

        let dynamic_state_info = PipelineDynamicStateCreateInfo::default().with_dynamic_states(&dynamic_state);

        // Gather all of the information for creating the pipeline
        let graphics_pipeline_info = GraphicsPipelineCreateInfo::default()
            .with_stages(&shader_stage_create_infos)
            .with_vertex_input_state(&vertex_input_state_info)
            .with_input_assembly_state(&vertex_input_assembly_state_info)
            .with_viewport_state(&viewport_state_info)
            .with_rasterization_state(&rasterization_info)
            .with_multisample_state(&multisample_state_info)
            .with_depth_stencil_state(&depth_state_info)
            .with_color_blend_state(&color_blend_state)
            .with_dynamic_state(&dynamic_state_info)
            .with_layout(pipeline_layout.as_raw())
            .with_render_pass(renderpass.renderpass().as_raw());

        // Create the pipeline
        let (mut pipelines, _) = unsafe {
            vulkan
                .device()
                .create_graphics_pipelines(None, std::slice::from_ref(&graphics_pipeline_info), None)?
        };

        let pipeline = pipelines.pop().unwrap();

        info!("Created pipeline: {:?}", pipeline.as_raw());

        Ok(Self { pipeline })
    }

    /// Get a reference to the pipeline's pipeline.
    pub fn pipeline(&self) -> &Unique<VkPipeline> {
        &self.pipeline
    }
}
