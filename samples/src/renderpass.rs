use crate::{depth::Depth, surface::Surface, vulkan::Vulkan};
use log::info;
use magritte::{
    vulkan1_0::{
        AccessFlags, AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp, Format,
        Framebuffer, FramebufferCreateInfo, ImageLayout, PipelineBindPoint, PipelineStageFlags,
        RenderPass as VkRenderPass, RenderPassCreateInfo, SampleCountFlagBits, SubpassDependency, SubpassDescription,
        VulkanResultCodes, SUBPASS_EXTERNAL,
    },
    AsRaw, SmallVec, Unique,
};

/// The container for all the renderpass related handles
pub struct RenderPass {
    /// The renderpass
    pub renderpass: Unique<VkRenderPass>,

    /// The framebuffers for this renderpass
    pub framebuffers: SmallVec<Unique<Framebuffer>>,
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        // Making sure that all framebuffers are destroyed before the render pass.
        self.framebuffers.clear();
    }
}

impl RenderPass {
    /// Creates the renderpass we will use in this demo
    pub fn new(vulkan: &Vulkan, surface: &Surface, depth: &Depth) -> Result<Self, VulkanResultCodes> {
        let renderpass = Self::create_renderpass(vulkan, surface)?;

        info!("Created the renderpass: {:?}", renderpass.as_raw());

        let framebuffers = Self::create_framebuffers(surface, depth, &renderpass)?;

        info!("Created {} framebuffers", framebuffers.len());

        Ok(Self {
            renderpass,
            framebuffers,
        })
    }

    pub fn resize(&mut self, surface: &Surface, depth: &Depth) -> Result<(), VulkanResultCodes> {
        self.framebuffers = Self::create_framebuffers(surface, depth, self.renderpass())?;

        info!("Created {} framebuffers", self.framebuffers.len());

        Ok(())
    }

    fn create_framebuffers(
        surface: &Surface,
        depth: &Depth,
        renderpass: &Unique<VkRenderPass>,
    ) -> Result<SmallVec<Unique<Framebuffer>>, VulkanResultCodes> {
        surface
            .swapchain_image_views()
            .iter()
            .map(|view| {
                // We gether the attachments we will pass to create the frame buffer.
                // In the **same** order as in the render pass!
                // Note that we use `as_raw_image_view` to get a regular image view from a `SwapchainImageView`!
                let attachments = [view.as_raw_image_view(), depth.image_view().as_raw()];

                let frame_buffer_create_info = FramebufferCreateInfo::default()
                    .set_render_pass(renderpass.as_raw())
                    .set_attachments(&attachments)
                    .set_width(surface.extent().width)
                    .set_height(surface.extent().height)
                    .set_layers(1);

                unsafe {
                    depth
                        .image()
                        .device()
                        .create_framebuffer(&frame_buffer_create_info, None)
                        .result()
                }
            })
            .collect::<Result<SmallVec<_>, _>>()
    }

    fn create_renderpass(vulkan: &Vulkan, surface: &Surface) -> Result<Unique<VkRenderPass>, VulkanResultCodes> {
        let renderpass_attachments = [
            AttachmentDescription {
                format: surface.format(),
                samples: SampleCountFlagBits::_1,
                load_op: AttachmentLoadOp::CLEAR,
                store_op: AttachmentStoreOp::STORE,
                final_layout: ImageLayout::PRESENT_SRC_KHR,
                ..Default::default()
            },
            AttachmentDescription {
                format: Format::D16_UNORM,
                samples: SampleCountFlagBits::_1,
                load_op: AttachmentLoadOp::CLEAR,
                initial_layout: ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                final_layout: ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                ..Default::default()
            },
        ];

        let color_attachment_ref = AttachmentReference {
            attachment: 0,
            layout: ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let depth_attachment_ref = AttachmentReference {
            attachment: 1,
            layout: ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let dependency = SubpassDependency {
            src_subpass: SUBPASS_EXTERNAL,
            src_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            ..Default::default()
        };

        let subpass = SubpassDescription::default()
            .set_color_attachments(std::slice::from_ref(&color_attachment_ref))
            .set_depth_stencil_attachment(&depth_attachment_ref)
            .set_pipeline_bind_point(PipelineBindPoint::GRAPHICS);

        let renderpass_create_info = RenderPassCreateInfo::default()
            .set_attachments(&renderpass_attachments)
            .set_subpasses(std::slice::from_ref(&subpass))
            .set_dependencies(std::slice::from_ref(&dependency));

        unsafe {
            vulkan
                .device()
                .create_render_pass(&renderpass_create_info, None)
                .result()
        }
    }

    /// Get a reference to the render pass's renderpass.
    pub fn renderpass(&self) -> &Unique<VkRenderPass> {
        &self.renderpass
    }

    /// Get a reference to the render pass's framebuffers.
    pub fn framebuffers(&self) -> &SmallVec<Unique<Framebuffer>> {
        &self.framebuffers
    }
}
