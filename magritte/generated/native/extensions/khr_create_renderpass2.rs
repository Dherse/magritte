use crate::native::vulkan1_2::{
    AttachmentDescription2, AttachmentReference2, FNCmdBeginRenderPass2, FNCmdEndRenderPass2, FNCmdNextSubpass2,
    FNCreateRenderPass2, RenderPassCreateInfo2, SubpassBeginInfo, SubpassDependency2, SubpassDescription2,
    SubpassEndInfo,
};
///See [`AttachmentDescription2`]
#[doc(alias = "VkAttachmentDescription2KHR")]
pub type AttachmentDescription2KHR = AttachmentDescription2;
///See [`AttachmentReference2`]
#[doc(alias = "VkAttachmentReference2KHR")]
pub type AttachmentReference2KHR = AttachmentReference2;
///See [`SubpassDescription2`]
#[doc(alias = "VkSubpassDescription2KHR")]
pub type SubpassDescription2KHR = SubpassDescription2;
///See [`SubpassDependency2`]
#[doc(alias = "VkSubpassDependency2KHR")]
pub type SubpassDependency2KHR = SubpassDependency2;
///See [`RenderPassCreateInfo2`]
#[doc(alias = "VkRenderPassCreateInfo2KHR")]
pub type RenderPassCreateInfo2KHR = RenderPassCreateInfo2;
///See [`SubpassBeginInfo`]
#[doc(alias = "VkSubpassBeginInfoKHR")]
pub type SubpassBeginInfoKHR = SubpassBeginInfo;
///See [`SubpassEndInfo`]
#[doc(alias = "VkSubpassEndInfoKHR")]
pub type SubpassEndInfoKHR = SubpassEndInfo;
pub use crate::common::extensions::khr_create_renderpass2::{
    KHR_CREATE_RENDERPASS_2_EXTENSION_NAME, KHR_CREATE_RENDERPASS_2_SPEC_VERSION,
};
///See [`create_render_pass2`]
#[doc(alias = "vkCreateRenderPass2KHR")]
pub type FNCreateRenderPass2Khr = FNCreateRenderPass2;
///See [`cmd_begin_render_pass2`]
#[doc(alias = "vkCmdBeginRenderPass2KHR")]
pub type FNCmdBeginRenderPass2Khr = FNCmdBeginRenderPass2;
///See [`cmd_next_subpass2`]
#[doc(alias = "vkCmdNextSubpass2KHR")]
pub type FNCmdNextSubpass2Khr = FNCmdNextSubpass2;
///See [`cmd_end_render_pass2`]
#[doc(alias = "vkCmdEndRenderPass2KHR")]
pub type FNCmdEndRenderPass2Khr = FNCmdEndRenderPass2;
