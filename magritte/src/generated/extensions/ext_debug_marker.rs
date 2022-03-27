use crate::vulkan1_0::{BaseInStructure, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum DebugReportObjectTypeEXT {
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeUnknownExt = 0,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeInstanceExt = 1,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypePhysicalDeviceExt = 2,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeDeviceExt = 3,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeQueueExt = 4,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeSemaphoreExt = 5,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeCommandBufferExt = 6,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeFenceExt = 7,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeDeviceMemoryExt = 8,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeBufferExt = 9,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeImageExt = 10,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeEventExt = 11,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeQueryPoolExt = 12,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeBufferViewExt = 13,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeImageViewExt = 14,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeShaderModuleExt = 15,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypePipelineCacheExt = 16,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypePipelineLayoutExt = 17,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeRenderPassExt = 18,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypePipelineExt = 19,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeDescriptorSetLayoutExt = 20,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeSamplerExt = 21,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeDescriptorPoolExt = 22,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeDescriptorSetExt = 23,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeFramebufferExt = 24,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeCommandPoolExt = 25,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeSurfaceKhrExt = 26,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeSwapchainKhrExt = 27,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeDebugReportCallbackExtExt = 28,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeDisplayKhrExt = 29,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeDisplayModeKhrExt = 30,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeValidationCacheExtExt = 33,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    DebugReportObjectTypeSamplerYcbcrConversionExt = 1000156000,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    DebugReportObjectTypeDescriptorUpdateTemplateExt = 1000085000,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nvx_binary_import`]
    DebugReportObjectTypeCuModuleNvxExt = 1000029000,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nvx_binary_import`]
    DebugReportObjectTypeCuFunctionNvxExt = 1000029001,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_acceleration_structure`]
    DebugReportObjectTypeAccelerationStructureKhrExt = 1000150000,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    DebugReportObjectTypeAccelerationStructureNvExt = 1000165000,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::fuchsia_buffer_collection`]
    DebugReportObjectTypeBufferCollectionFuchsiaExt = 1000366000,
}
impl const Default for DebugReportObjectTypeEXT {
    fn default() -> Self {
        DebugReportObjectTypeUnknownExt
    }
}
impl DebugReportObjectTypeEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkDebugMarkerObjectNameInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerObjectNameInfoEXT.html) - Specify parameters of a name to give to an object
///# C Specifications
///The [`DebugMarkerObjectNameInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_debug_marker
///typedef struct VkDebugMarkerObjectNameInfoEXT {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkDebugReportObjectTypeEXT    objectType;
///    uint64_t                      object;
///    const char*                   pObjectName;
///} VkDebugMarkerObjectNameInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`object_type`] is a [`DebugReportObjectTypeEXT`] specifying the type of the object to be
///   named.
/// - [`object`] is the object to be named.
/// - [`p_object_name`] is a null-terminated UTF-8 string specifying the name to apply to
///   [`object`].
///# Description
///Applications **may** change the name associated with an object simply by
///calling [`DebugMarkerSetObjectNameEXT`] again with a new string.
///To remove a previously set name, [`p_object_name`]**should** be set to an
///empty string.Valid Usage
/// - [`object_type`]**must** not be `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT`
/// - [`object`]**must** not be [`crate::utils::Handle::null`]
/// -  [`object`]**must** be a Vulkan object of the type associated with [`object_type`] as defined in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`object_type`]**must** be a valid [`DebugReportObjectTypeEXT`] value
/// - [`p_object_name`]**must** be a null-terminated UTF-8 string
///# Related
/// - [`VK_EXT_debug_marker`]
/// - [`DebugReportObjectTypeEXT`]
/// - [`StructureType`]
/// - [`DebugMarkerSetObjectNameEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DebugMarkerObjectNameInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`object_type`] is a [`DebugReportObjectTypeEXT`] specifying the
    ///type of the object to be named.
    object_type: DebugReportObjectTypeEXT,
    ///[`object`] is the object to be named.
    object: u64,
    ///[`p_object_name`] is a null-terminated UTF-8 string specifying the name
    ///to apply to [`object`].
    p_object_name: &'lt CStr,
}
///[VkDebugMarkerObjectTagInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerObjectTagInfoEXT.html) - Specify parameters of a tag to attach to an object
///# C Specifications
///The [`DebugMarkerObjectTagInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_debug_marker
///typedef struct VkDebugMarkerObjectTagInfoEXT {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkDebugReportObjectTypeEXT    objectType;
///    uint64_t                      object;
///    uint64_t                      tagName;
///    size_t                        tagSize;
///    const void*                   pTag;
///} VkDebugMarkerObjectTagInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`object_type`] is a [`DebugReportObjectTypeEXT`] specifying the type of the object to be
///   named.
/// - [`object`] is the object to be tagged.
/// - [`tag_name`] is a numerical identifier of the tag.
/// - [`tag_size`] is the number of bytes of data to attach to the object.
/// - [`p_tag`] is a pointer to an array of [`tag_size`] bytes containing the data to be associated
///   with the object.
///# Description
///The [`tag_name`] parameter gives a name or identifier to the type of data
///being tagged.
///This can be used by debugging layers to easily filter for only data that can
///be used by that implementation.Valid Usage
/// - [`object_type`]**must** not be `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT`
/// - [`object`]**must** not be [`crate::utils::Handle::null`]
/// -  [`object`]**must** be a Vulkan object of the type associated with [`object_type`] as defined in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`object_type`]**must** be a valid [`DebugReportObjectTypeEXT`] value
/// - [`p_tag`]**must** be a valid pointer to an array of [`tag_size`] bytes
/// - [`tag_size`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_debug_marker`]
/// - [`DebugReportObjectTypeEXT`]
/// - [`StructureType`]
/// - [`DebugMarkerSetObjectTagEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DebugMarkerObjectTagInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`object_type`] is a [`DebugReportObjectTypeEXT`] specifying the
    ///type of the object to be named.
    object_type: DebugReportObjectTypeEXT,
    ///[`object`] is the object to be tagged.
    object: u64,
    ///[`tag_name`] is a numerical identifier of the tag.
    tag_name: u64,
    ///[`tag_size`] is the number of bytes of data to attach to the object.
    tag_size: usize,
    ///[`p_tag`] is a pointer to an array of [`tag_size`] bytes containing
    ///the data to be associated with the object.
    p_tag: *mut c_void,
}
///[VkDebugMarkerMarkerInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerMarkerInfoEXT.html) - Specify parameters of a command buffer marker region
///# C Specifications
///The [`DebugMarkerMarkerInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_debug_marker
///typedef struct VkDebugMarkerMarkerInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    const char*        pMarkerName;
///    float              color[4];
///} VkDebugMarkerMarkerInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`p_marker_name`] is a pointer to a null-terminated UTF-8 string containing the name of the
///   marker.
/// - [`color`] is an **optional** RGBA color value that can be associated with the marker. A
///   particular implementation **may** choose to ignore this color value. The values contain RGBA
///   values in order, in the range 0.0 to 1.0. If all elements in [`color`] are set to 0.0 then it
///   is ignored.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`p_marker_name`]**must** be a null-terminated UTF-8 string
///# Related
/// - [`VK_EXT_debug_marker`]
/// - [`StructureType`]
/// - [`CmdDebugMarkerBeginEXT`]
/// - [`CmdDebugMarkerInsertEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DebugMarkerMarkerInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`p_marker_name`] is a pointer to a null-terminated UTF-8 string
    ///containing the name of the marker.
    p_marker_name: &'lt CStr,
    ///[`color`] is an **optional** RGBA color value that can be associated with
    ///the marker.
    ///A particular implementation **may** choose to ignore this color value.
    ///The values contain RGBA values in order, in the range 0.0 to 1.0.
    ///If all elements in [`color`] are set to 0.0 then it is ignored.
    color: [f32; 4],
}
