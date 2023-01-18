use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, ImageLayout, ImageView, StructureType},
};
use std::ffi::CStr;
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
#[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
pub type FNCmdBindInvocationMaskHuawei =
    unsafe extern "system" fn(command_buffer: CommandBuffer, image_view: ImageView, image_layout: ImageLayout);
