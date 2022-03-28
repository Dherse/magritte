//![VK_EXT_debug_marker](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_marker.html) - device extension
//!# Description
//!The [`VK_EXT_debug_marker`] extension is a device extension.
//!It introduces concepts of object naming and tagging, for better tracking of
//!Vulkan objects, as well as additional commands for recording annotations of
//!named sections of a workload to aid organization and offline analysis in
//!external tools.
//!# Revision
//!4
//!# Dependencies
//! - *Promoted* to `[`VK_EXT_debug_utils`]` extension
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_EXT_debug_report`]`
//!# Contacts
//! - Baldur Karlsson [baldurk](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_debug_marker]
//!   @baldurk%0A<<Here describe the issue or question you have about the VK_EXT_debug_marker
//!   extension>>)
//!# New functions & commands
//! - [`CmdDebugMarkerBeginEXT`]
//! - [`CmdDebugMarkerEndEXT`]
//! - [`CmdDebugMarkerInsertEXT`]
//! - [`DebugMarkerSetObjectNameEXT`]
//! - [`DebugMarkerSetObjectTagEXT`]
//!# New structures
//! - [`DebugMarkerMarkerInfoEXT`]
//! - [`DebugMarkerObjectNameInfoEXT`]
//! - [`DebugMarkerObjectTagInfoEXT`]
//!# New enums
//! - [`DebugReportObjectTypeEXT`]
//!# New constants
//! - [`EXT_DEBUG_MARKER_EXTENSION_NAME`]
//! - [`EXT_DEBUG_MARKER_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT`
//!# Known issues & F.A.Q
//!1) Should the tag or name for an object be specified using the `pNext`
//!parameter in the object’s `Vk*CreateInfo` structure? **RESOLVED** : No.
//!While this fits with other Vulkan patterns and would allow more type safety
//!and future proofing against future objects, it has notable downsides.
//!In particular passing the name at `Vk*CreateInfo` time does not allow
//!renaming, prevents late binding of naming information, and does not allow
//!naming of implicitly created objects such as queues and swapchain images.2) Should the command
//! annotation functions [`CmdDebugMarkerBeginEXT`]
//!and [`CmdDebugMarkerEndEXT`] support the ability to specify a color? **RESOLVED** : Yes.
//!The functions have been expanded to take an optional color which can be used
//!at will by implementations consuming the command buffer annotations in their
//!visualisation.3) Should the functions added in this extension accept an extensible
//!structure as their parameter for a more flexible API, as opposed to direct
//!function parameters? If so, which functions? **RESOLVED** : Yes.
//!All functions have been modified to take a structure type with extensible
//!`pNext` pointer, to allow future extensions to add additional annotation
//!information in the same commands.
//!# Version History
//! - Revision 1, 2016-02-24 (Baldur Karlsson)  - Initial draft, based on LunarG marker spec
//! - Revision 2, 2016-02-26 (Baldur Karlsson)  - Renamed Dbg to DebugMarker in function names  -
//!   Allow markers in secondary command buffers under certain circumstances  - Minor language
//!   tweaks and edits
//! - Revision 3, 2016-04-23 (Baldur Karlsson)  - Reorganise spec layout to closer match desired
//!   organisation  - Added optional color to markers (both regions and inserted labels)  - Changed
//!   functions to take extensible structs instead of direct function parameters
//! - Revision 4, 2017-01-31 (Baldur Karlsson)  - Added explicit dependency on VK_EXT_debug_report
//!   - Moved definition of [`DebugReportObjectTypeEXT`] to debug report chapter.  - Fixed typo in
//!   dates in revision history
//!# Other info
//! * 2017-01-31
//! * No known IP claims.
//! * - Baldur Karlsson  - Dan Ginsburg, Valve  - Jon Ashburn, LunarG  - Kyle Spagnoli, NVIDIA
//!# Related
//! - [`DebugMarkerMarkerInfoEXT`]
//! - [`DebugMarkerObjectNameInfoEXT`]
//! - [`DebugMarkerObjectTagInfoEXT`]
//! - [`DebugReportObjectTypeEXT`]
//! - [`CmdDebugMarkerBeginEXT`]
//! - [`CmdDebugMarkerEndEXT`]
//! - [`CmdDebugMarkerInsertEXT`]
//! - [`DebugMarkerSetObjectNameEXT`]
//! - [`DebugMarkerSetObjectTagEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
#[doc(alias = "VkDebugReportObjectTypeEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
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
        Self::DebugReportObjectTypeUnknownExt
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
/// - [`object_name`] is a null-terminated UTF-8 string specifying the name to apply to [`object`].
///# Description
///Applications **may**  change the name associated with an object simply by
///calling [`DebugMarkerSetObjectNameEXT`] again with a new string.
///To remove a previously set name, [`object_name`] **should**  be set to an
///empty string.
///## Valid Usage
/// - [`object_type`] **must**  not be `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT`
/// - [`object`] **must**  not be [`crate::utils::Handle::null`]
/// - [`object`] **must**  be a Vulkan object of the type associated with [`object_type`] as defined
///   in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types)
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`object_type`] **must**  be a valid [`DebugReportObjectTypeEXT`] value
/// - [`object_name`] **must**  be a null-terminated UTF-8 string
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
#[doc(alias = "VkDebugMarkerObjectNameInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DebugMarkerObjectNameInfoEXT<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`object_type`] is a [`DebugReportObjectTypeEXT`] specifying the
    ///type of the object to be named.
    pub object_type: DebugReportObjectTypeEXT,
    ///[`object`] is the object to be named.
    pub object: u64,
    ///[`object_name`] is a null-terminated UTF-8 string specifying the name
    ///to apply to [`object`].
    pub object_name: &'lt CStr,
}
impl<'lt> Default for DebugMarkerObjectNameInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object: 0,
            object_name: std::ptr::null(),
        }
    }
}
impl<'lt> DebugMarkerObjectNameInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::object_type`]
    pub fn object_type(&self) -> DebugReportObjectTypeEXT {
        self.object_type
    }
    ///Gets the value of [`Self::object`]
    pub fn object(&self) -> u64 {
        self.object
    }
    ///Gets the value of [`Self::object_name`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn object_name(&self) -> &'lt CStr {
        self.object_name
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::object_type`]
    pub fn object_type_mut(&mut self) -> &mut DebugReportObjectTypeEXT {
        &mut self.object_type
    }
    ///Gets a mutable reference to the value of [`Self::object`]
    pub fn object_mut(&mut self) -> &mut u64 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::object_type`]
    pub fn set_object_type(
        &mut self,
        value: crate::extensions::ext_debug_marker::DebugReportObjectTypeEXT,
    ) -> &mut Self {
        self.object_type = value;
        self
    }
    ///Sets the raw value of [`Self::object`]
    pub fn set_object(&mut self, value: u64) -> &mut Self {
        self.object = value;
        self
    }
    ///Sets the raw value of [`Self::object_name`]
    pub fn set_object_name(&mut self, value: &'lt std::ffi::CStr) -> &mut Self {
        self.object_name = value;
        self
    }
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
/// - [`tag`] is a pointer to an array of [`tag_size`] bytes containing the data to be associated
///   with the object.
///# Description
///The [`tag_name`] parameter gives a name or identifier to the type of data
///being tagged.
///This can be used by debugging layers to easily filter for only data that can
///be used by that implementation.
///## Valid Usage
/// - [`object_type`] **must**  not be `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT`
/// - [`object`] **must**  not be [`crate::utils::Handle::null`]
/// - [`object`] **must**  be a Vulkan object of the type associated with [`object_type`] as defined
///   in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types)
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`object_type`] **must**  be a valid [`DebugReportObjectTypeEXT`] value
/// - [`tag`] **must**  be a valid pointer to an array of [`tag_size`] bytes
/// - [`tag_size`] **must**  be greater than `0`
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
#[doc(alias = "VkDebugMarkerObjectTagInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DebugMarkerObjectTagInfoEXT<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`object_type`] is a [`DebugReportObjectTypeEXT`] specifying the
    ///type of the object to be named.
    pub object_type: DebugReportObjectTypeEXT,
    ///[`object`] is the object to be tagged.
    pub object: u64,
    ///[`tag_name`] is a numerical identifier of the tag.
    pub tag_name: u64,
    ///[`tag_size`] is the number of bytes of data to attach to the object.
    pub tag_size: usize,
    ///[`tag`] is a pointer to an array of [`tag_size`] bytes containing
    ///the data to be associated with the object.
    pub tag: *const c_void,
}
impl<'lt> Default for DebugMarkerObjectTagInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object: 0,
            tag_name: 0,
            tag_size: 0,
            tag: std::ptr::null(),
        }
    }
}
impl<'lt> DebugMarkerObjectTagInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::tag`]
    pub fn tag_raw(&self) -> *const c_void {
        self.tag
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::tag`]
    pub fn set_tag_raw(&mut self, value: *const c_void) -> &mut Self {
        self.tag = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::object_type`]
    pub fn object_type(&self) -> DebugReportObjectTypeEXT {
        self.object_type
    }
    ///Gets the value of [`Self::object`]
    pub fn object(&self) -> u64 {
        self.object
    }
    ///Gets the value of [`Self::tag_name`]
    pub fn tag_name(&self) -> u64 {
        self.tag_name
    }
    ///Gets the value of [`Self::tag_size`]
    pub fn tag_size(&self) -> usize {
        self.tag_size
    }
    ///Gets the value of [`Self::tag`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn tag(&self) -> &[c_void] {
        std::slice::from_raw_parts(self.tag, self.tag_size as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::object_type`]
    pub fn object_type_mut(&mut self) -> &mut DebugReportObjectTypeEXT {
        &mut self.object_type
    }
    ///Gets a mutable reference to the value of [`Self::object`]
    pub fn object_mut(&mut self) -> &mut u64 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::tag_name`]
    pub fn tag_name_mut(&mut self) -> &mut u64 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::tag_size`]
    pub fn tag_size_mut(&mut self) -> &mut usize {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::object_type`]
    pub fn set_object_type(
        &mut self,
        value: crate::extensions::ext_debug_marker::DebugReportObjectTypeEXT,
    ) -> &mut Self {
        self.object_type = value;
        self
    }
    ///Sets the raw value of [`Self::object`]
    pub fn set_object(&mut self, value: u64) -> &mut Self {
        self.object = value;
        self
    }
    ///Sets the raw value of [`Self::tag_name`]
    pub fn set_tag_name(&mut self, value: u64) -> &mut Self {
        self.tag_name = value;
        self
    }
    ///Sets the raw value of [`Self::tag_size`]
    pub fn set_tag_size(&mut self, value: usize) -> &mut Self {
        self.tag_size = value;
        self
    }
    ///Sets the raw value of [`Self::tag`]
    pub fn set_tag(&mut self, value: &'lt [std::ffi::c_void]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.tag = value.as_ptr();
        self.tag_size = len_;
        self
    }
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
/// - [`marker_name`] is a pointer to a null-terminated UTF-8 string containing the name of the
///   marker.
/// - [`color`] is an  **optional**  RGBA color value that can be associated with the marker. A
///   particular implementation  **may**  choose to ignore this color value. The values contain RGBA
///   values in order, in the range 0.0 to 1.0. If all elements in [`color`] are set to 0.0 then it
///   is ignored.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`marker_name`] **must**  be a null-terminated UTF-8 string
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
#[doc(alias = "VkDebugMarkerMarkerInfoEXT")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DebugMarkerMarkerInfoEXT<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`marker_name`] is a pointer to a null-terminated UTF-8 string
    ///containing the name of the marker.
    pub marker_name: &'lt CStr,
    ///[`color`] is an  **optional**  RGBA color value that can be associated with
    ///the marker.
    ///A particular implementation  **may**  choose to ignore this color value.
    ///The values contain RGBA values in order, in the range 0.0 to 1.0.
    ///If all elements in [`color`] are set to 0.0 then it is ignored.
    pub color: [f32; 4],
}
impl<'lt> Default for DebugMarkerMarkerInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            marker_name: std::ptr::null(),
            color: [0.0; 4],
        }
    }
}
impl<'lt> DebugMarkerMarkerInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::marker_name`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn marker_name(&self) -> &'lt CStr {
        self.marker_name
    }
    ///Gets the value of [`Self::color`]
    pub fn color(&self) -> &[f32; 4] {
        &getter
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::color`]
    pub fn color_mut(&mut self) -> &mut [f32; 4] {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::marker_name`]
    pub fn set_marker_name(&mut self, value: &'lt std::ffi::CStr) -> &mut Self {
        self.marker_name = value;
        self
    }
    ///Sets the raw value of [`Self::color`]
    pub fn set_color(&mut self, value: [f32; 4]) -> &mut Self {
        self.color = value;
        self
    }
}
