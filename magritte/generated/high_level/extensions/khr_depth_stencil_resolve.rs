pub use crate::common::extensions::khr_depth_stencil_resolve::{
    KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME, KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION,
};
use crate::vulkan1_2::{
    PhysicalDeviceDepthStencilResolveProperties, ResolveModeFlagBits, ResolveModeFlags,
    SubpassDescriptionDepthStencilResolve,
};
#[doc(alias = "VkResolveModeFlagsKHR")]
pub type ResolveModeFlagsKHR = ResolveModeFlags;
#[doc(alias = "VkResolveModeFlagBitsKHR")]
pub type ResolveModeFlagBitsKHR = ResolveModeFlagBits;
#[doc(alias = "VkPhysicalDeviceDepthStencilResolvePropertiesKHR")]
pub type PhysicalDeviceDepthStencilResolvePropertiesKHR = PhysicalDeviceDepthStencilResolveProperties;
#[doc(alias = "VkSubpassDescriptionDepthStencilResolveKHR")]
pub type SubpassDescriptionDepthStencilResolveKHR = SubpassDescriptionDepthStencilResolve;
