//!# [VK_EXT_debug_marker](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_marker.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_debug_marker/VK_EXT_debug_marker.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, CommandBuffer, Device, StructureType, VulkanResultCodes},
};
use std::ffi::{c_char, CStr};
///# [VkDebugMarkerObjectNameInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerObjectNameInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_marker/VkDebugMarkerObjectNameInfoEXT.md")]
#[doc(alias = "VkDebugMarkerObjectNameInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugMarkerObjectNameInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "objectType")]
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    #[doc(alias = "pObjectName")]
    object_name: *const c_char,
}
///# [VkDebugMarkerObjectTagInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerObjectTagInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_marker/VkDebugMarkerObjectTagInfoEXT.md")]
#[doc(alias = "VkDebugMarkerObjectTagInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugMarkerObjectTagInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "objectType")]
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    #[doc(alias = "tagName")]
    tag_name: u64,
    #[doc(alias = "tagSize")]
    tag_size: usize,
    #[doc(alias = "pTag")]
    tag: *const std::ffi::c_void,
}
///# [VkDebugMarkerMarkerInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerMarkerInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_marker/VkDebugMarkerMarkerInfoEXT.md")]
#[doc(alias = "VkDebugMarkerMarkerInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugMarkerMarkerInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "pMarkerName")]
    marker_name: *const c_char,
    color: [f32; 4 as usize],
}
#[doc(alias = "VK_EXT_DEBUG_MARKER_SPEC_VERSION")]
pub const EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;
#[doc(alias = "VK_EXT_DEBUG_MARKER_EXTENSION_NAME")]
pub const EXT_DEBUG_MARKER_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_debug_marker");
///# [VkDebugReportObjectTypeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportObjectTypeEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_marker/VkDebugReportObjectTypeEXT.md")]
#[doc(alias = "VkDebugReportObjectTypeEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DebugReportObjectTypeEXT(i32);
impl DebugReportObjectTypeEXT {
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT")]
    pub const UNKNOWN: Self = Self(0);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT")]
    pub const INSTANCE: Self = Self(1);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT")]
    pub const PHYSICAL_DEVICE: Self = Self(2);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT")]
    pub const DEVICE: Self = Self(3);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT")]
    pub const QUEUE: Self = Self(4);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT")]
    pub const SEMAPHORE: Self = Self(5);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT")]
    pub const COMMAND_BUFFER: Self = Self(6);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT")]
    pub const FENCE: Self = Self(7);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT")]
    pub const DEVICE_MEMORY: Self = Self(8);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT")]
    pub const BUFFER: Self = Self(9);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT")]
    pub const IMAGE: Self = Self(10);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT")]
    pub const EVENT: Self = Self(11);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT")]
    pub const QUERY_POOL: Self = Self(12);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT")]
    pub const BUFFER_VIEW: Self = Self(13);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT")]
    pub const IMAGE_VIEW: Self = Self(14);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT")]
    pub const SHADER_MODULE: Self = Self(15);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT")]
    pub const PIPELINE_CACHE: Self = Self(16);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT")]
    pub const PIPELINE_LAYOUT: Self = Self(17);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT")]
    pub const RENDER_PASS: Self = Self(18);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT")]
    pub const PIPELINE: Self = Self(19);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT")]
    pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT")]
    pub const SAMPLER: Self = Self(21);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT")]
    pub const DESCRIPTOR_POOL: Self = Self(22);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT")]
    pub const DESCRIPTOR_SET: Self = Self(23);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT")]
    pub const FRAMEBUFFER: Self = Self(24);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT")]
    pub const COMMAND_POOL: Self = Self(25);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT")]
    pub const SURFACE_KHR: Self = Self(26);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT")]
    pub const SWAPCHAIN_KHR: Self = Self(27);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT")]
    pub const DEBUG_REPORT_CALLBACK: Self = Self(28);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT")]
    pub const DISPLAY_KHR: Self = Self(29);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT")]
    pub const DISPLAY_MODE_KHR: Self = Self(30);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT")]
    pub const VALIDATION_CACHE: Self = Self(33);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT")]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT")]
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX_EXT")]
    #[cfg(feature = "VK_NVX_binary_import")]
    pub const CU_MODULE_NVX: Self = Self(1000029000);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX_EXT")]
    #[cfg(feature = "VK_NVX_binary_import")]
    pub const CU_FUNCTION_NVX: Self = Self(1000029001);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR_EXT")]
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV_EXT")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA_EXT")]
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366000);
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT")]
    pub const DEBUG_REPORT: Self = Self::DEBUG_REPORT_CALLBACK;
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT")]
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE;
    #[doc(alias = "VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR_EXT")]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::UNKNOWN.bits() => Some(Self(x)),
            x if x == Self::INSTANCE.bits() => Some(Self(x)),
            x if x == Self::PHYSICAL_DEVICE.bits() => Some(Self(x)),
            x if x == Self::DEVICE.bits() => Some(Self(x)),
            x if x == Self::QUEUE.bits() => Some(Self(x)),
            x if x == Self::SEMAPHORE.bits() => Some(Self(x)),
            x if x == Self::COMMAND_BUFFER.bits() => Some(Self(x)),
            x if x == Self::FENCE.bits() => Some(Self(x)),
            x if x == Self::DEVICE_MEMORY.bits() => Some(Self(x)),
            x if x == Self::BUFFER.bits() => Some(Self(x)),
            x if x == Self::IMAGE.bits() => Some(Self(x)),
            x if x == Self::EVENT.bits() => Some(Self(x)),
            x if x == Self::QUERY_POOL.bits() => Some(Self(x)),
            x if x == Self::BUFFER_VIEW.bits() => Some(Self(x)),
            x if x == Self::IMAGE_VIEW.bits() => Some(Self(x)),
            x if x == Self::SHADER_MODULE.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_CACHE.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_LAYOUT.bits() => Some(Self(x)),
            x if x == Self::RENDER_PASS.bits() => Some(Self(x)),
            x if x == Self::PIPELINE.bits() => Some(Self(x)),
            x if x == Self::DESCRIPTOR_SET_LAYOUT.bits() => Some(Self(x)),
            x if x == Self::SAMPLER.bits() => Some(Self(x)),
            x if x == Self::DESCRIPTOR_POOL.bits() => Some(Self(x)),
            x if x == Self::DESCRIPTOR_SET.bits() => Some(Self(x)),
            x if x == Self::FRAMEBUFFER.bits() => Some(Self(x)),
            x if x == Self::COMMAND_POOL.bits() => Some(Self(x)),
            x if x == Self::SURFACE_KHR.bits() => Some(Self(x)),
            x if x == Self::SWAPCHAIN_KHR.bits() => Some(Self(x)),
            x if x == Self::DEBUG_REPORT_CALLBACK.bits() => Some(Self(x)),
            x if x == Self::DISPLAY_KHR.bits() => Some(Self(x)),
            x if x == Self::DISPLAY_MODE_KHR.bits() => Some(Self(x)),
            x if x == Self::VALIDATION_CACHE.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
            x if x == Self::SAMPLER_YCBCR_CONVERSION.bits() => Some(Self(x)),
            x if x == Self::DESCRIPTOR_UPDATE_TEMPLATE.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NVX_binary_import")]
            x if x == Self::CU_MODULE_NVX.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NVX_binary_import")]
            x if x == Self::CU_FUNCTION_NVX.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_acceleration_structure")]
            x if x == Self::ACCELERATION_STRUCTURE_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_ray_tracing")]
            x if x == Self::ACCELERATION_STRUCTURE_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            x if x == Self::BUFFER_COLLECTION_FUCHSIA.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkDebugMarkerSetObjectNameEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_marker/vkDebugMarkerSetObjectNameEXT.md")]
#[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
pub type FNDebugMarkerSetObjectNameExt =
    unsafe extern "system" fn(device: Device, p_name_info: *const DebugMarkerObjectNameInfoEXT) -> VulkanResultCodes;
///# [vkDebugMarkerSetObjectTagEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_marker/vkDebugMarkerSetObjectTagEXT.md")]
#[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
pub type FNDebugMarkerSetObjectTagExt =
    unsafe extern "system" fn(device: Device, p_tag_info: *const DebugMarkerObjectTagInfoEXT) -> VulkanResultCodes;
///# [vkCmdDebugMarkerBeginEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerBeginEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_marker/vkCmdDebugMarkerBeginEXT.md")]
#[doc(alias = "vkCmdDebugMarkerBeginEXT")]
pub type FNCmdDebugMarkerBeginExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_marker_info: *const DebugMarkerMarkerInfoEXT);
///# [vkCmdDebugMarkerEndEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerEndEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_marker/vkCmdDebugMarkerEndEXT.md")]
#[doc(alias = "vkCmdDebugMarkerEndEXT")]
pub type FNCmdDebugMarkerEndExt = unsafe extern "system" fn(command_buffer: CommandBuffer);
///# [vkCmdDebugMarkerInsertEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerInsertEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_marker/vkCmdDebugMarkerInsertEXT.md")]
#[doc(alias = "vkCmdDebugMarkerInsertEXT")]
pub type FNCmdDebugMarkerInsertExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_marker_info: *const DebugMarkerMarkerInfoEXT);
