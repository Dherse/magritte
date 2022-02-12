#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_MARKER_SPEC_VERSION")]
pub const EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_MARKER_EXTENSION_NAME")]
pub const EXT_DEBUG_MARKER_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_debug_marker");
///[VkDebugReportObjectTypeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportObjectTypeEXT.html) - Specify the type of an object handle
///# C Specifications
///Possible values passed to the `objectType` parameter of the callback
///function specified by
///[`DebugReportCallbackCreateInfoEXT::pfn_callback`], specifying the
///type of object handle being reported, are:
///```c
///// Provided by VK_EXT_debug_marker, VK_EXT_debug_report
///typedef enum VkDebugReportObjectTypeEXT {
///    VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT = 0,
///    VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT = 1,
///    VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT = 2,
///    VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT = 3,
///    VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT = 4,
///    VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT = 5,
///    VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT = 6,
///    VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT = 7,
///    VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT = 8,
///    VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT = 9,
///    VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT = 10,
///    VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT = 11,
///    VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT = 12,
///    VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT = 13,
///    VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT = 14,
///    VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT = 15,
///    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT = 16,
///    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT = 17,
///    VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT = 18,
///    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT = 19,
///    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT = 20,
///    VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT = 21,
///    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT = 22,
///    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT = 23,
///    VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT = 24,
///    VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT = 25,
///    VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT = 26,
///    VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT = 27,
///    VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT = 28,
///    VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT = 29,
///    VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT = 30,
///    VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT = 33,
///  // Provided by VK_VERSION_1_1 with VK_EXT_debug_report, VK_KHR_sampler_ycbcr_conversion with
/// VK_EXT_debug_report
///    VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT = 1000156000,
///  // Provided by VK_VERSION_1_1 with VK_EXT_debug_report
///    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT = 1000085000,
///  // Provided by VK_NVX_binary_import
///    VK_DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX_EXT = 1000029000,
///  // Provided by VK_NVX_binary_import
///    VK_DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX_EXT = 1000029001,
///  // Provided by VK_KHR_acceleration_structure
///    VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR_EXT = 1000150000,
///  // Provided by VK_NV_ray_tracing
///    VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV_EXT = 1000165000,
///  // Provided by VK_FUCHSIA_buffer_collection
///    VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA_EXT = 1000366000,
///    VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT =
/// VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT,
///    VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT =
/// VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT,
///  // Provided by VK_KHR_descriptor_update_template with VK_EXT_debug_report
///    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT =
/// VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT,
///  // Provided by VK_KHR_sampler_ycbcr_conversion
///    VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR_EXT =
/// VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT,
///} VkDebugReportObjectTypeEXT;
///```
///# Related
/// - [`VK_EXT_debug_marker`]
/// - [`VK_EXT_debug_report`]
/// - [`DebugMarkerObjectNameInfoEXT`]
/// - [`DebugMarkerObjectTagInfoEXT`]
/// - [`DebugReportMessageEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugReportObjectTypeEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DebugReportObjectTypeEXT(i32);
impl const Default for DebugReportObjectTypeEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DebugReportObjectTypeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("DebugReportObjectTypeEXT")
            .field(match *self {
                Self::DEBUG_REPORT_OBJECT_TYPE_UNKNOWN => &"DEBUG_REPORT_OBJECT_TYPE_UNKNOWN",
                Self::DEBUG_REPORT_OBJECT_TYPE_INSTANCE => &"DEBUG_REPORT_OBJECT_TYPE_INSTANCE",
                Self::DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE => &"DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE",
                Self::DEBUG_REPORT_OBJECT_TYPE_DEVICE => &"DEBUG_REPORT_OBJECT_TYPE_DEVICE",
                Self::DEBUG_REPORT_OBJECT_TYPE_QUEUE => &"DEBUG_REPORT_OBJECT_TYPE_QUEUE",
                Self::DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE => &"DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE",
                Self::DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER => &"DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER",
                Self::DEBUG_REPORT_OBJECT_TYPE_FENCE => &"DEBUG_REPORT_OBJECT_TYPE_FENCE",
                Self::DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY => &"DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY",
                Self::DEBUG_REPORT_OBJECT_TYPE_BUFFER => &"DEBUG_REPORT_OBJECT_TYPE_BUFFER",
                Self::DEBUG_REPORT_OBJECT_TYPE_IMAGE => &"DEBUG_REPORT_OBJECT_TYPE_IMAGE",
                Self::DEBUG_REPORT_OBJECT_TYPE_EVENT => &"DEBUG_REPORT_OBJECT_TYPE_EVENT",
                Self::DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL => &"DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL",
                Self::DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW => &"DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW",
                Self::DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW => &"DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW",
                Self::DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE => &"DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE",
                Self::DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE => &"DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE",
                Self::DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT => &"DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT",
                Self::DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS => &"DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS",
                Self::DEBUG_REPORT_OBJECT_TYPE_PIPELINE => &"DEBUG_REPORT_OBJECT_TYPE_PIPELINE",
                Self::DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT => {
                    &"DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT"
                },
                Self::DEBUG_REPORT_OBJECT_TYPE_SAMPLER => &"DEBUG_REPORT_OBJECT_TYPE_SAMPLER",
                Self::DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL => &"DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL",
                Self::DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET => &"DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET",
                Self::DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER => &"DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER",
                Self::DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL => &"DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL",
                Self::DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR => &"DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR",
                Self::DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR => &"DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR",
                Self::DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK => {
                    &"DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK"
                },
                Self::DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR => &"DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR",
                Self::DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR => &"DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR",
                Self::DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE => &"DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE",
                Self::DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION => {
                    &"DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION"
                },
                Self::DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE => {
                    &"DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE"
                },
                Self::DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX => &"DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX",
                Self::DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX => &"DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX",
                Self::DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR => {
                    &"DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR"
                },
                Self::DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV => {
                    &"DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV"
                },
                Self::DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA => {
                    &"DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA"
                },
                other => unreachable!("invalid value for `DebugReportObjectTypeEXT`: {:?}", other),
            })
            .finish()
    }
}
impl DebugReportObjectTypeEXT {
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_UNKNOWN: Self = Self(0);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_INSTANCE: Self = Self(1);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE: Self = Self(2);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_DEVICE: Self = Self(3);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_QUEUE: Self = Self(4);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE: Self = Self(5);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER: Self = Self(6);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_FENCE: Self = Self(7);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY: Self = Self(8);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_BUFFER: Self = Self(9);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_IMAGE: Self = Self(10);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_EVENT: Self = Self(11);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL: Self = Self(12);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW: Self = Self(13);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW: Self = Self(14);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE: Self = Self(15);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE: Self = Self(16);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT: Self = Self(17);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS: Self = Self(18);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE: Self = Self(19);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_SAMPLER: Self = Self(21);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL: Self = Self(22);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET: Self = Self(23);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER: Self = Self(24);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL: Self = Self(25);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR: Self = Self(26);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR: Self = Self(27);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK: Self = Self(28);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR: Self = Self(29);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR: Self = Self(30);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE: Self = Self(33);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    pub const DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nvx_binary_import`]
    pub const DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX: Self = Self(1000029000);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nvx_binary_import`]
    pub const DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX: Self = Self(1000029001);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_acceleration_structure`]
    pub const DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::fuchsia_buffer_collection`]
    pub const DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366000);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT: Self = Self::DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE: Self = Self::DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_descriptor_update_template`]
    pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self =
        Self::DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    pub const DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR: Self =
        Self::DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION;
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
