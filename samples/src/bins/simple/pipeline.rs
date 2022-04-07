use std::{mem::size_of, error::Error};

use bytemuck::offset_of;
use log::info;
use magritte::{vulkan1_0::{Pipeline as VkPipeline, PipelineLayoutCreateInfo, ShaderStageFlagBits, VertexInputRate, VertexInputBindingDescription, VertexInputAttributeDescription, Format, PipelineVertexInputStateCreateInfo, PrimitiveTopology, PipelineInputAssemblyStateCreateInfo, Viewport, PipelineViewportStateCreateInfo, Rect2D, Offset2D, Extent2D, PipelineRasterizationStateCreateInfo, FrontFace, PolygonMode, SampleCountFlagBits, PipelineMultisampleStateCreateInfo, StencilOpState, StencilOp, CompareOp, PipelineColorBlendAttachmentState, BlendFactor, BlendOp, ColorComponentFlags, LogicOp, PipelineColorBlendStateCreateInfo, DynamicState, GraphicsPipelineCreateInfo, PipelineDynamicStateCreateInfo, PipelineDepthStencilStateCreateInfo}, AsRaw, Unique};
use magritte_samples::{shader::Shader, vulkan::Vulkan};

use crate::{Vertex, renderpass::RenderPass};

/// The SPIR-V of the compiled vertex shader
static VERTEX_SHADER: &[u8] = include_bytes!("./shaders/triangle.vert.spv");

/// The SPIR-V of the compiled fragment shader
static FRAGMENT_SHADER: &[u8] = include_bytes!("./shaders/triangle.frag.spv");

/// Container for a simple graphics pipeline
pub struct Pipeline {
    pub pipeline: Unique<VkPipeline>,
}

impl Pipeline {
    /// Creates a new very simple pipeline
    pub fn new(vulkan: &Vulkan, renderpass: &RenderPass) -> Result<Self, Box<dyn Error>> {
        let vertex = Shader::new(vulkan, VERTEX_SHADER, ShaderStageFlagBits::VERTEX)?;
        let fragment = Shader::new(vulkan, FRAGMENT_SHADER, ShaderStageFlagBits::FRAGMENT)?;

        // We don't have any information to put in the layout
        let layout_create_info = PipelineLayoutCreateInfo::default();

        // We create the layout (even though it empty)
        let (pipeline_layout, _) = unsafe {
            vulkan.device().create_pipeline_layout(&layout_create_info, None)?
        };

        info!("Created pipeline layout: {:?}", pipeline_layout.as_raw());

        // Here we prepare the shader stages that are part of the pipeline
        let shader_stage_create_infos = [ vertex.as_shader_stage_create_info(), fragment.as_shader_stage_create_info() ];
        
        // Here we set the input sized, in our case, a single vertex.
        // If we had multiple vertex buffers (for instancing for example), then we would specify more than one.
        // However, in most cases, you only have one vertex input.
        let vertex_input_binding_description = VertexInputBindingDescription {
            binding: 0,
            stride: size_of::<Vertex>() as u32,
            input_rate: VertexInputRate::VERTEX,
        };

        // Here, we define the inputs of the **vertex** shader, in our case, one vec4<f32> and one vec4<u8>
        // For each, we need to define four attributes:
        // - the location, that is the location within one binding (one binding is one `VertexInputBindingDescription`)
        // - the binding, that is the binding index
        // - the format that tells Vulkan the size and how to interpret it
        // - the offset, that tells Vulkan **where** is is in the structure
        let vertex_input_attribute_descriptions = [
            VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: Format::R32_G_32_B_32_A_32_SFLOAT,
                offset: offset_of!(Vertex, position) as u32,
            },
            VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: Format::R8_G_8_B_8_A_8_UNORM,
                offset: offset_of!(Vertex, color) as u32,
            },
        ];

        // Here, we gather all of the information about the vertex shader inputs
        let vertex_input_state_info = PipelineVertexInputStateCreateInfo::default()
            .set_vertex_attribute_descriptions(&vertex_input_attribute_descriptions)
            .set_vertex_binding_descriptions(std::slice::from_ref(&vertex_input_binding_description));

        // We define the topology of our primitives, in this case a list of triangles.
        // This structure is very powerful and can be use for a lot more.
        let vertex_input_assembly_state_info = PipelineInputAssemblyStateCreateInfo::default()
            .set_topology(PrimitiveTopology::TRIANGLE_LIST);

        // We define the viewport we will use.
        // In our case, we will use dynamic viewports so that we don't need to re-created the pipeline all the time.
        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: 1920.0,
            height: 1080.0,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        // We define where on screen we want to draw
        let scissor = Rect2D {
            offset: Offset2D {
                x: 0,
                y: 0,
            },
            extent: Extent2D {
                width: 1920,
                height: 1080,
            },
        };

        // We collect the viewport and the scissor
        let viewport_state_info = PipelineViewportStateCreateInfo::default()
            .set_scissors(std::slice::from_ref(&scissor))
            .set_viewports(std::slice::from_ref(&viewport));

        // We configure the rasterization
        let rasterization_info = PipelineRasterizationStateCreateInfo::default()
            .set_front_face(FrontFace::COUNTER_CLOCKWISE)
            .set_line_width(1.0)
            .set_polygon_mode(PolygonMode::FILL);
        
        // We don't do multisampling
        let multisample_state_info = PipelineMultisampleStateCreateInfo::default()
            .set_rasterization_samples(SampleCountFlagBits::_1);

        // We don't care about the stencil, we create a NOOP stencil state.
        let depth_stencil_state = StencilOpState::default()
            .set_fail_op(StencilOp::KEEP)
            .set_pass_op(StencilOp::KEEP)
            .set_depth_fail_op(StencilOp::KEEP)
            .set_compare_op(CompareOp::ALWAYS);

        let depth_state_info = PipelineDepthStencilStateCreateInfo::default()
            .set_depth_test_enable(true)
            .set_depth_write_enable(true)
            .set_depth_compare_op(CompareOp::LESS_OR_EQUAL)
            .set_front(depth_stencil_state)
            .set_back(depth_stencil_state)
            .set_max_depth_bounds(1.0);

        // How to blend color
        // One per output attachment
        let color_blend_attachment_state = PipelineColorBlendAttachmentState::default()
            .set_blend_enable(false)
            .set_src_color_blend_factor(BlendFactor::SRC_COLOR)
            .set_dst_color_blend_factor(BlendFactor::ONE_MINUS_DST_COLOR)
            .set_color_blend_op(BlendOp::ADD)
            .set_src_alpha_blend_factor(BlendFactor::ZERO)
            .set_dst_alpha_blend_factor(BlendFactor::ZERO)
            .set_alpha_blend_op(BlendOp::ADD)
            .set_color_write_mask(ColorComponentFlags::all());

        // We gather the blending information for each output
        let color_blend_state = PipelineColorBlendStateCreateInfo::default()
            .set_logic_op(LogicOp::CLEAR)
            .set_attachments(std::slice::from_ref(&color_blend_attachment_state));

        // What state is dynamic
        let dynamic_state = [DynamicState::VIEWPORT, DynamicState::SCISSOR];

        let dynamic_state_info =
            PipelineDynamicStateCreateInfo::default().set_dynamic_states(&dynamic_state);

        // Gather all of the information for creating the pipeline
        let graphics_pipeline_info = GraphicsPipelineCreateInfo::default()
            .set_stages(&shader_stage_create_infos)
            .set_vertex_input_state(&vertex_input_state_info)
            .set_input_assembly_state(&vertex_input_assembly_state_info)
            .set_viewport_state(&viewport_state_info)
            .set_rasterization_state(&rasterization_info)
            .set_multisample_state(&multisample_state_info)
            .set_depth_stencil_state(&depth_state_info)
            .set_color_blend_state(&color_blend_state)
            .set_dynamic_state(&dynamic_state_info)
            .set_layout(pipeline_layout.as_raw())
            .set_render_pass(renderpass.renderpass().as_raw());

        // Create the pipeline
        let (mut pipelines, _) = unsafe {
            vulkan.device().create_graphics_pipelines(
                None,
                std::slice::from_ref(&graphics_pipeline_info),
                None
            )?
        };

        let pipeline = pipelines.pop().unwrap();

        info!("Created pipeline: {:?}", pipeline.as_raw());

        Ok(Self {
            pipeline
        })
    }

    /// Get a reference to the pipeline's pipeline.
    pub fn pipeline(&self) -> &Unique<VkPipeline> {
        &self.pipeline
    }
}