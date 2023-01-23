//!# [VK_KHR_push_descriptor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_push_descriptor.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_push_descriptor/VK_KHR_push_descriptor.md")]
use crate::{
    cstr,
    vulkan1_0::{
        BaseOutStructure, CommandBuffer, PipelineBindPoint, PipelineLayout, StructureType, WriteDescriptorSet,
    },
};
use std::ffi::CStr;
///# [VkPhysicalDevicePushDescriptorPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_push_descriptor/VkPhysicalDevicePushDescriptorPropertiesKHR.md")]
#[doc(alias = "VkPhysicalDevicePushDescriptorPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePushDescriptorPropertiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxPushDescriptors")]
    max_push_descriptors: u32,
}
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION")]
pub const KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME")]
pub const KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_push_descriptor");
///# [vkCmdPushDescriptorSetKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_push_descriptor/vkCmdPushDescriptorSetKHR.md")]
#[doc(alias = "vkCmdPushDescriptorSetKHR")]
pub type FNCmdPushDescriptorSetKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet,
);
