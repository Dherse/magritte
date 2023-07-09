pub use crate::common::extensions::khr_create_renderpass2::{
    KHR_CREATE_RENDERPASS_2_EXTENSION_NAME, KHR_CREATE_RENDERPASS_2_SPEC_VERSION,
};
use crate::vulkan1_2::{
    AttachmentDescription2, AttachmentReference2, RenderPassCreateInfo2, SubpassBeginInfo, SubpassDependency2,
    SubpassDescription2, SubpassEndInfo,
};
#[doc(alias = "VkAttachmentDescription2KHR")]
pub type AttachmentDescription2KHR = AttachmentDescription2;
#[doc(alias = "VkAttachmentReference2KHR")]
pub type AttachmentReference2KHR = AttachmentReference2;
#[doc(alias = "VkSubpassDescription2KHR")]
pub type SubpassDescription2KHR = SubpassDescription2;
#[doc(alias = "VkSubpassDependency2KHR")]
pub type SubpassDependency2KHR = SubpassDependency2;
#[doc(alias = "VkRenderPassCreateInfo2KHR")]
pub type RenderPassCreateInfo2KHR = RenderPassCreateInfo2;
#[doc(alias = "VkSubpassBeginInfoKHR")]
pub type SubpassBeginInfoKHR = SubpassBeginInfo;
#[doc(alias = "VkSubpassEndInfoKHR")]
pub type SubpassEndInfoKHR = SubpassEndInfo;
