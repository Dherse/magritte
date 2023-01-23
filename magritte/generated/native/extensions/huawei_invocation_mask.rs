//!# [VK_HUAWEI_invocation_mask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_HUAWEI_invocation_mask.html)
# ! [doc = include_str ! ("../../../../doc/extensions/huawei_invocation_mask/VK_HUAWEI_invocation_mask.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, ImageLayout, ImageView, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceInvocationMaskFeaturesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInvocationMaskFeaturesHUAWEI.html)
# [doc = include_str ! ("../../../../doc/extensions/huawei_invocation_mask/VkPhysicalDeviceInvocationMaskFeaturesHUAWEI.md")]
#[doc(alias = "VkPhysicalDeviceInvocationMaskFeaturesHUAWEI")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "invocationMask")]
    invocation_mask: Bool32,
}
#[doc(alias = "VK_HUAWEI_INVOCATION_MASK_SPEC_VERSION")]
pub const HUAWEI_INVOCATION_MASK_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_HUAWEI_INVOCATION_MASK_EXTENSION_NAME")]
pub const HUAWEI_INVOCATION_MASK_EXTENSION_NAME: &'static CStr = cstr!("VK_HUAWEI_invocation_mask");
///# [vkCmdBindInvocationMaskHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html)
# [doc = include_str ! ("../../../../doc/extensions/huawei_invocation_mask/vkCmdBindInvocationMaskHUAWEI.md")]
#[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
pub type FNCmdBindInvocationMaskHuawei =
    unsafe extern "system" fn(command_buffer: CommandBuffer, image_view: ImageView, image_layout: ImageLayout);
