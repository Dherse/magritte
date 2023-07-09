use crate::native::vulkan1_0::{
    BaseOutStructure, CommandBuffer, PipelineBindPoint, PipelineLayout, StructureType, WriteDescriptorSet,
};
#[doc(alias = "VkPhysicalDevicePushDescriptorPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePushDescriptorPropertiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxPushDescriptors")]
    pub max_push_descriptors: u32,
}
impl Default for PhysicalDevicePushDescriptorPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePushDescriptorPropertiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            max_push_descriptors: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_push_descriptor::{
    KHR_PUSH_DESCRIPTOR_EXTENSION_NAME, KHR_PUSH_DESCRIPTOR_SPEC_VERSION,
};
#[doc(alias = "vkCmdPushDescriptorSetKHR")]
pub type FNCmdPushDescriptorSetKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet,
);
