//!# [VK_KHR_depth_stencil_resolve](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_depth_stencil_resolve.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_depth_stencil_resolve/VK_KHR_depth_stencil_resolve.md")]
use crate::{
    cstr,
    vulkan1_2::{
        PhysicalDeviceDepthStencilResolveProperties, ResolveModeFlagBits, ResolveModeFlags,
        SubpassDescriptionDepthStencilResolve,
    },
};
use std::ffi::CStr;
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
#[doc(alias = "VK_KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION")]
pub const KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME")]
pub const KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_depth_stencil_resolve");
