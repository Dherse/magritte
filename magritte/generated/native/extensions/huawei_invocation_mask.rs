use crate::native::vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, ImageLayout, ImageView, StructureType};
#[doc(alias = "VkPhysicalDeviceInvocationMaskFeaturesHUAWEI")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "invocationMask")]
    pub invocation_mask: Bool32,
}
impl Default for PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceInvocationMaskFeaturesHuawei,
            p_next: unsafe { std::mem::zeroed() },
            invocation_mask: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::huawei_invocation_mask::{
    HUAWEI_INVOCATION_MASK_EXTENSION_NAME, HUAWEI_INVOCATION_MASK_SPEC_VERSION,
};
#[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
pub type FNCmdBindInvocationMaskHuawei =
    unsafe extern "system" fn(command_buffer: CommandBuffer, image_view: ImageView, image_layout: ImageLayout);
