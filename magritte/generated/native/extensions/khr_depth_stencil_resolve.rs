use crate::native::vulkan1_2::{
    PhysicalDeviceDepthStencilResolveProperties, ResolveModeFlagBits, ResolveModeFlags,
    SubpassDescriptionDepthStencilResolve,
};
///See [`ResolveModeFlags`]
#[doc(alias = "VkResolveModeFlagsKHR")]
pub type ResolveModeFlagsKHR = ResolveModeFlags;
///See [`ResolveModeFlagBits`]
#[doc(alias = "VkResolveModeFlagBitsKHR")]
pub type ResolveModeFlagBitsKHR = ResolveModeFlagBits;
///See [`PhysicalDeviceDepthStencilResolveProperties`]
#[doc(alias = "VkPhysicalDeviceDepthStencilResolvePropertiesKHR")]
pub type PhysicalDeviceDepthStencilResolvePropertiesKHR = PhysicalDeviceDepthStencilResolveProperties;
///See [`SubpassDescriptionDepthStencilResolve`]
#[doc(alias = "VkSubpassDescriptionDepthStencilResolveKHR")]
pub type SubpassDescriptionDepthStencilResolveKHR = SubpassDescriptionDepthStencilResolve;
pub use crate::common::extensions::khr_depth_stencil_resolve::{
    KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME, KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION,
};
