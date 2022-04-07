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
//! - [`cmd_debug_marker_begin_ext`]
//! - [`cmd_debug_marker_end_ext`]
//! - [`cmd_debug_marker_insert_ext`]
//! - [`debug_marker_set_object_name_ext`]
//! - [`debug_marker_set_object_tag_ext`]
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
//! annotation functions [`cmd_debug_marker_begin_ext`]
//!and [`cmd_debug_marker_end_ext`] support the ability to specify a color? **RESOLVED** : Yes.
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
//! - [`cmd_debug_marker_begin_ext`]
//! - [`cmd_debug_marker_end_ext`]
//! - [`cmd_debug_marker_insert_ext`]
//! - [`debug_marker_set_object_name_ext`]
//! - [`debug_marker_set_object_tag_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseInStructure, CommandBuffer, Device, StructureType, VulkanResultCodes},
    AsRaw, Unique, VulkanResult,
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
    os::raw::c_char,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_MARKER_SPEC_VERSION")]
pub const EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_MARKER_EXTENSION_NAME")]
pub const EXT_DEBUG_MARKER_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_debug_marker");
///[vkDebugMarkerSetObjectNameEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html) - Give a user-friendly name to an object
///# C Specifications
///An object can be given a user-friendly name by calling:
///```c
///// Provided by VK_EXT_debug_marker
///VkResult vkDebugMarkerSetObjectNameEXT(
///    VkDevice                                    device,
///    const VkDebugMarkerObjectNameInfoEXT*       pNameInfo);
///```
/// # Parameters
/// - [`device`] is the device that created the object.
/// - [`p_name_info`] is a pointer to a [`DebugMarkerObjectNameInfoEXT`] structure specifying the
///   parameters of the name to set on the object.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_name_info`] **must**  be a valid pointer to a valid [`DebugMarkerObjectNameInfoEXT`]
///   structure
///
/// ## Host Synchronization
/// - Host access to `pNameInfo->object` **must**  be externally synchronized
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_EXT_debug_marker`]
/// - [`DebugMarkerObjectNameInfoEXT`]
/// - [`Device`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
pub type FNDebugMarkerSetObjectNameExt = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_name_info: *const DebugMarkerObjectNameInfoEXT<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkDebugMarkerSetObjectTagEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html) - Attach arbitrary data to an object
///# C Specifications
///In addition to setting a name for an object, debugging and validation layers
///may have uses for additional binary data on a per-object basis that has no
///other place in the Vulkan API.
///For example, a [`ShaderModule`] could have additional debugging data
///attached to it to aid in offline shader tracing.
///To attach data to an object, call:
///```c
///// Provided by VK_EXT_debug_marker
///VkResult vkDebugMarkerSetObjectTagEXT(
///    VkDevice                                    device,
///    const VkDebugMarkerObjectTagInfoEXT*        pTagInfo);
///```
/// # Parameters
/// - [`device`] is the device that created the object.
/// - [`p_tag_info`] is a pointer to a [`DebugMarkerObjectTagInfoEXT`] structure specifying the
///   parameters of the tag to attach to the object.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_tag_info`] **must**  be a valid pointer to a valid [`DebugMarkerObjectTagInfoEXT`]
///   structure
///
/// ## Host Synchronization
/// - Host access to `pTagInfo->object` **must**  be externally synchronized
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_EXT_debug_marker`]
/// - [`DebugMarkerObjectTagInfoEXT`]
/// - [`Device`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
pub type FNDebugMarkerSetObjectTagExt = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_tag_info: *const DebugMarkerObjectTagInfoEXT<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkCmdDebugMarkerBeginEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerBeginEXT.html) - Open a command buffer marker region
///# C Specifications
///A marker region can be opened by calling:
///```c
///// Provided by VK_EXT_debug_marker
///void vkCmdDebugMarkerBeginEXT(
///    VkCommandBuffer                             commandBuffer,
///    const VkDebugMarkerMarkerInfoEXT*           pMarkerInfo);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - [`p_marker_info`] is a pointer to a [`DebugMarkerMarkerInfoEXT`] structure specifying the
///   parameters of the marker region to open.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_marker_info`] **must**  be a valid pointer to a valid [`DebugMarkerMarkerInfoEXT`]
///   structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
///
/// ## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
/// ## Command Properties
/// # Related
/// - [`VK_EXT_debug_marker`]
/// - [`CommandBuffer`]
/// - [`DebugMarkerMarkerInfoEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdDebugMarkerBeginEXT")]
pub type FNCmdDebugMarkerBeginExt = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT<'lt>,
    ),
>;
///[vkCmdDebugMarkerEndEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerEndEXT.html) - Close a command buffer marker region
///# C Specifications
///A marker region can be closed by calling:
///```c
///// Provided by VK_EXT_debug_marker
///void vkCmdDebugMarkerEndEXT(
///    VkCommandBuffer                             commandBuffer);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// # Description
/// An application  **may**  open a marker region in one command buffer and close it
/// in another, or otherwise split marker regions across multiple command
/// buffers or multiple queue submissions.
/// When viewed from the linear series of submissions to a single queue, the
/// calls to [`cmd_debug_marker_begin_ext`] and [`cmd_debug_marker_end_ext`] **must**  be matched
/// and balanced.
/// ## Valid Usage
/// - There  **must**  be an outstanding [`cmd_debug_marker_begin_ext`] command prior to the
///   [`cmd_debug_marker_end_ext`] on the queue that [`command_buffer`] is submitted to
/// - If [`command_buffer`] is a secondary command buffer, there  **must**  be an outstanding
///   [`cmd_debug_marker_begin_ext`] command recorded to [`command_buffer`] that has not previously
///   been ended by a call to [`cmd_debug_marker_end_ext`]
///
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
///
/// ## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
/// ## Command Properties
/// # Related
/// - [`VK_EXT_debug_marker`]
/// - [`CommandBuffer`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdDebugMarkerEndEXT")]
pub type FNCmdDebugMarkerEndExt = Option<unsafe extern "system" fn(command_buffer: CommandBuffer)>;
///[vkCmdDebugMarkerInsertEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerInsertEXT.html) - Insert a marker label into a command buffer
///# C Specifications
///A single marker label can be inserted into a command buffer by calling:
///```c
///// Provided by VK_EXT_debug_marker
///void vkCmdDebugMarkerInsertEXT(
///    VkCommandBuffer                             commandBuffer,
///    const VkDebugMarkerMarkerInfoEXT*           pMarkerInfo);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - [`p_marker_info`] is a pointer to a [`DebugMarkerMarkerInfoEXT`] structure specifying the
///   parameters of the marker to insert.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_marker_info`] **must**  be a valid pointer to a valid [`DebugMarkerMarkerInfoEXT`]
///   structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
///
/// ## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
/// ## Command Properties
/// # Related
/// - [`VK_EXT_debug_marker`]
/// - [`CommandBuffer`]
/// - [`DebugMarkerMarkerInfoEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdDebugMarkerInsertEXT")]
pub type FNCmdDebugMarkerInsertExt = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT<'lt>,
    ),
>;
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
/// # Related
/// - [`VK_EXT_debug_marker`]
/// - [`VK_EXT_debug_report`]
/// - [`DebugMarkerObjectNameInfoEXT`]
/// - [`DebugMarkerObjectTagInfoEXT`]
/// - [`debug_report_message_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugReportObjectTypeEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DebugReportObjectTypeEXT(i32);
impl const Default for DebugReportObjectTypeEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl DebugReportObjectTypeEXT {
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const UNKNOWN: Self = Self(0);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const INSTANCE: Self = Self(1);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const PHYSICAL_DEVICE: Self = Self(2);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEVICE: Self = Self(3);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const QUEUE: Self = Self(4);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const SEMAPHORE: Self = Self(5);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const COMMAND_BUFFER: Self = Self(6);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const FENCE: Self = Self(7);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEVICE_MEMORY: Self = Self(8);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const BUFFER: Self = Self(9);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const IMAGE: Self = Self(10);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const EVENT: Self = Self(11);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const QUERY_POOL: Self = Self(12);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const BUFFER_VIEW: Self = Self(13);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const IMAGE_VIEW: Self = Self(14);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const SHADER_MODULE: Self = Self(15);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const PIPELINE_CACHE: Self = Self(16);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const PIPELINE_LAYOUT: Self = Self(17);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const RENDER_PASS: Self = Self(18);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const PIPELINE: Self = Self(19);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const SAMPLER: Self = Self(21);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DESCRIPTOR_POOL: Self = Self(22);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DESCRIPTOR_SET: Self = Self(23);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const FRAMEBUFFER: Self = Self(24);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const COMMAND_POOL: Self = Self(25);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const SURFACE_KHR: Self = Self(26);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const SWAPCHAIN_KHR: Self = Self(27);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DEBUG_REPORT_CALLBACK: Self = Self(28);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DISPLAY_KHR: Self = Self(29);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DISPLAY_MODE_KHR: Self = Self(30);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const VALIDATION_CACHE: Self = Self(33);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_debug_report`]
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nvx_binary_import`]
    #[cfg(feature = "VK_NVX_binary_import")]
    pub const CU_MODULE_NVX: Self = Self(1000029000);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nvx_binary_import`]
    #[cfg(feature = "VK_NVX_binary_import")]
    pub const CU_FUNCTION_NVX: Self = Self(1000029001);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_acceleration_structure`]
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::fuchsia_buffer_collection`]
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366000);
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
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`object_type`] is a [`DebugReportObjectTypeEXT`] specifying the type of the object to be
///   named.
/// - [`object`] is the object to be named.
/// - [`object_name`] is a null-terminated UTF-8 string specifying the name to apply to [`object`].
/// # Description
/// Applications **may**  change the name associated with an object simply by
/// calling [`debug_marker_set_object_name_ext`] again with a new string.
/// To remove a previously set name, [`object_name`] **should**  be set to an
/// empty string.
/// ## Valid Usage
/// - [`object_type`] **must**  not be `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT`
/// - [`object`] **must**  not be [`crate::Handle::null`]
/// - [`object`] **must**  be a Vulkan object of the type associated with [`object_type`] as defined
///   in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types)
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`object_type`] **must**  be a valid [`DebugReportObjectTypeEXT`] value
/// - [`object_name`] **must**  be a null-terminated UTF-8 string
/// # Related
/// - [`VK_EXT_debug_marker`]
/// - [`DebugReportObjectTypeEXT`]
/// - [`StructureType`]
/// - [`debug_marker_set_object_name_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugMarkerObjectNameInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DebugMarkerObjectNameInfoEXT<'lt> {
    ///Lifetime field
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
    pub object_name: *const c_char,
}
impl<'lt> Default for DebugMarkerObjectNameInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
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
    ///Gets the raw value of [`Self::object_name`]
    pub fn object_name_raw(&self) -> *const c_char {
        self.object_name
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::object_name`]
    pub fn set_object_name_raw(mut self, value: *const c_char) -> Self {
        self.object_name = value;
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
        CStr::from_ptr(self.object_name)
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
        &mut self.object
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::object_type`]
    pub fn set_object_type(mut self, value: crate::extensions::ext_debug_marker::DebugReportObjectTypeEXT) -> Self {
        self.object_type = value;
        self
    }
    ///Sets the value of [`Self::object`]
    pub fn set_object(mut self, value: u64) -> Self {
        self.object = value;
        self
    }
    ///Sets the value of [`Self::object_name`]
    pub fn set_object_name(mut self, value: *const std::os::raw::c_char) -> Self {
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`object_type`] is a [`DebugReportObjectTypeEXT`] specifying the type of the object to be
///   named.
/// - [`object`] is the object to be tagged.
/// - [`tag_name`] is a numerical identifier of the tag.
/// - [`tag_size`] is the number of bytes of data to attach to the object.
/// - [`tag`] is a pointer to an array of [`tag_size`] bytes containing the data to be associated
///   with the object.
/// # Description
/// The [`tag_name`] parameter gives a name or identifier to the type of data
/// being tagged.
/// This can be used by debugging layers to easily filter for only data that can
/// be used by that implementation.
/// ## Valid Usage
/// - [`object_type`] **must**  not be `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT`
/// - [`object`] **must**  not be [`crate::Handle::null`]
/// - [`object`] **must**  be a Vulkan object of the type associated with [`object_type`] as defined
///   in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types)
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`object_type`] **must**  be a valid [`DebugReportObjectTypeEXT`] value
/// - [`tag`] **must**  be a valid pointer to an array of [`tag_size`] bytes
/// - [`tag_size`] **must**  be greater than `0`
/// # Related
/// - [`VK_EXT_debug_marker`]
/// - [`DebugReportObjectTypeEXT`]
/// - [`StructureType`]
/// - [`debug_marker_set_object_tag_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugMarkerObjectTagInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DebugMarkerObjectTagInfoEXT<'lt> {
    ///Lifetime field
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
            s_type: StructureType::DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::tag`]
    pub fn set_tag_raw(mut self, value: *const c_void) -> Self {
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
        &mut self.object
    }
    ///Gets a mutable reference to the value of [`Self::tag_name`]
    pub fn tag_name_mut(&mut self) -> &mut u64 {
        &mut self.tag_name
    }
    ///Gets a mutable reference to the value of [`Self::tag_size`]
    pub fn tag_size_mut(&mut self) -> &mut usize {
        &mut self.tag_size
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::object_type`]
    pub fn set_object_type(mut self, value: crate::extensions::ext_debug_marker::DebugReportObjectTypeEXT) -> Self {
        self.object_type = value;
        self
    }
    ///Sets the value of [`Self::object`]
    pub fn set_object(mut self, value: u64) -> Self {
        self.object = value;
        self
    }
    ///Sets the value of [`Self::tag_name`]
    pub fn set_tag_name(mut self, value: u64) -> Self {
        self.tag_name = value;
        self
    }
    ///Sets the value of [`Self::tag_size`]
    pub fn set_tag_size(mut self, value: usize) -> Self {
        self.tag_size = value;
        self
    }
    ///Sets the value of [`Self::tag`]
    pub fn set_tag(mut self, value: &'lt [std::ffi::c_void]) -> Self {
        let len_ = value.len() as usize;
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`marker_name`] is a pointer to a null-terminated UTF-8 string containing the name of the
///   marker.
/// - [`color`] is an  **optional**  RGBA color value that can be associated with the marker. A
///   particular implementation  **may**  choose to ignore this color value. The values contain RGBA
///   values in order, in the range 0.0 to 1.0. If all elements in [`color`] are set to 0.0 then it
///   is ignored.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`marker_name`] **must**  be a null-terminated UTF-8 string
/// # Related
/// - [`VK_EXT_debug_marker`]
/// - [`StructureType`]
/// - [`cmd_debug_marker_begin_ext`]
/// - [`cmd_debug_marker_insert_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugMarkerMarkerInfoEXT")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DebugMarkerMarkerInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`marker_name`] is a pointer to a null-terminated UTF-8 string
    ///containing the name of the marker.
    pub marker_name: *const c_char,
    ///[`color`] is an  **optional**  RGBA color value that can be associated with
    ///the marker.
    ///A particular implementation  **may**  choose to ignore this color value.
    ///The values contain RGBA values in order, in the range 0.0 to 1.0.
    ///If all elements in [`color`] are set to 0.0 then it is ignored.
    pub color: [f32; 4 as usize],
}
impl<'lt> Default for DebugMarkerMarkerInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DEBUG_MARKER_MARKER_INFO_EXT,
            p_next: std::ptr::null(),
            marker_name: std::ptr::null(),
            color: [0.0; 4 as usize],
        }
    }
}
impl<'lt> DebugMarkerMarkerInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::marker_name`]
    pub fn marker_name_raw(&self) -> *const c_char {
        self.marker_name
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::marker_name`]
    pub fn set_marker_name_raw(mut self, value: *const c_char) -> Self {
        self.marker_name = value;
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
        CStr::from_ptr(self.marker_name)
    }
    ///Gets the value of [`Self::color`]
    pub fn color(&self) -> &[f32; 4 as usize] {
        &self.color
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::color`]
    pub fn color_mut(&mut self) -> &mut [f32; 4 as usize] {
        &mut self.color
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::marker_name`]
    pub fn set_marker_name(mut self, value: *const std::os::raw::c_char) -> Self {
        self.marker_name = value;
        self
    }
    ///Sets the value of [`Self::color`]
    pub fn set_color(mut self, value: [f32; 4 as usize]) -> Self {
        self.color = value;
        self
    }
}
impl Device {
    ///[vkDebugMarkerSetObjectNameEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html) - Give a user-friendly name to an object
    ///# C Specifications
    ///An object can be given a user-friendly name by calling:
    ///```c
    ///// Provided by VK_EXT_debug_marker
    ///VkResult vkDebugMarkerSetObjectNameEXT(
    ///    VkDevice                                    device,
    ///    const VkDebugMarkerObjectNameInfoEXT*       pNameInfo);
    ///```
    /// # Parameters
    /// - [`device`] is the device that created the object.
    /// - [`p_name_info`] is a pointer to a [`DebugMarkerObjectNameInfoEXT`] structure specifying
    ///   the parameters of the name to set on the object.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_name_info`] **must**  be a valid pointer to a valid [`DebugMarkerObjectNameInfoEXT`]
    ///   structure
    ///
    /// ## Host Synchronization
    /// - Host access to `pNameInfo->object` **must**  be externally synchronized
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`VK_EXT_debug_marker`]
    /// - [`DebugMarkerObjectNameInfoEXT`]
    /// - [`Device`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn debug_marker_set_object_name_ext<'lt>(
        self: &Unique<Device>,
        p_name_info: &DebugMarkerObjectNameInfoEXT<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_debug_marker()
            .and_then(|vtable| vtable.debug_marker_set_object_name_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_debug_marker()
            .and_then(|vtable| vtable.debug_marker_set_object_name_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_name_info as *const DebugMarkerObjectNameInfoEXT<'lt>);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkDebugMarkerSetObjectTagEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html) - Attach arbitrary data to an object
    ///# C Specifications
    ///In addition to setting a name for an object, debugging and validation layers
    ///may have uses for additional binary data on a per-object basis that has no
    ///other place in the Vulkan API.
    ///For example, a [`ShaderModule`] could have additional debugging data
    ///attached to it to aid in offline shader tracing.
    ///To attach data to an object, call:
    ///```c
    ///// Provided by VK_EXT_debug_marker
    ///VkResult vkDebugMarkerSetObjectTagEXT(
    ///    VkDevice                                    device,
    ///    const VkDebugMarkerObjectTagInfoEXT*        pTagInfo);
    ///```
    /// # Parameters
    /// - [`device`] is the device that created the object.
    /// - [`p_tag_info`] is a pointer to a [`DebugMarkerObjectTagInfoEXT`] structure specifying the
    ///   parameters of the tag to attach to the object.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_tag_info`] **must**  be a valid pointer to a valid [`DebugMarkerObjectTagInfoEXT`]
    ///   structure
    ///
    /// ## Host Synchronization
    /// - Host access to `pTagInfo->object` **must**  be externally synchronized
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`VK_EXT_debug_marker`]
    /// - [`DebugMarkerObjectTagInfoEXT`]
    /// - [`Device`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn debug_marker_set_object_tag_ext<'lt>(
        self: &Unique<Device>,
        p_tag_info: &DebugMarkerObjectTagInfoEXT<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_debug_marker()
            .and_then(|vtable| vtable.debug_marker_set_object_tag_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_debug_marker()
            .and_then(|vtable| vtable.debug_marker_set_object_tag_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_tag_info as *const DebugMarkerObjectTagInfoEXT<'lt>);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl CommandBuffer {
    ///[vkCmdDebugMarkerBeginEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerBeginEXT.html) - Open a command buffer marker region
    ///# C Specifications
    ///A marker region can be opened by calling:
    ///```c
    ///// Provided by VK_EXT_debug_marker
    ///void vkCmdDebugMarkerBeginEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkDebugMarkerMarkerInfoEXT*           pMarkerInfo);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - [`p_marker_info`] is a pointer to a [`DebugMarkerMarkerInfoEXT`] structure specifying the
    ///   parameters of the marker region to open.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_marker_info`] **must**  be a valid pointer to a valid [`DebugMarkerMarkerInfoEXT`]
    ///   structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, or compute operations
    ///
    /// ## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    /// ## Command Properties
    /// # Related
    /// - [`VK_EXT_debug_marker`]
    /// - [`CommandBuffer`]
    /// - [`DebugMarkerMarkerInfoEXT`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdDebugMarkerBeginEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_debug_marker_begin_ext<'lt>(
        self: &Unique<CommandBuffer>,
        p_marker_info: &DebugMarkerMarkerInfoEXT<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_marker()
            .and_then(|vtable| vtable.cmd_debug_marker_begin_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_marker()
            .and_then(|vtable| vtable.cmd_debug_marker_begin_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_marker_info as *const DebugMarkerMarkerInfoEXT<'lt>);
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdDebugMarkerEndEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerEndEXT.html) - Close a command buffer marker region
    ///# C Specifications
    ///A marker region can be closed by calling:
    ///```c
    ///// Provided by VK_EXT_debug_marker
    ///void vkCmdDebugMarkerEndEXT(
    ///    VkCommandBuffer                             commandBuffer);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// # Description
    /// An application  **may**  open a marker region in one command buffer and close it
    /// in another, or otherwise split marker regions across multiple command
    /// buffers or multiple queue submissions.
    /// When viewed from the linear series of submissions to a single queue, the
    /// calls to [`cmd_debug_marker_begin_ext`] and [`cmd_debug_marker_end_ext`] **must**  be
    /// matched and balanced.
    /// ## Valid Usage
    /// - There  **must**  be an outstanding [`cmd_debug_marker_begin_ext`] command prior to the
    ///   [`cmd_debug_marker_end_ext`] on the queue that [`command_buffer`] is submitted to
    /// - If [`command_buffer`] is a secondary command buffer, there  **must**  be an outstanding
    ///   [`cmd_debug_marker_begin_ext`] command recorded to [`command_buffer`] that has not
    ///   previously been ended by a call to [`cmd_debug_marker_end_ext`]
    ///
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, or compute operations
    ///
    /// ## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    /// ## Command Properties
    /// # Related
    /// - [`VK_EXT_debug_marker`]
    /// - [`CommandBuffer`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdDebugMarkerEndEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_debug_marker_end_ext(self: &Unique<CommandBuffer>) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_marker()
            .and_then(|vtable| vtable.cmd_debug_marker_end_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_marker()
            .and_then(|vtable| vtable.cmd_debug_marker_end_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw());
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdDebugMarkerInsertEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerInsertEXT.html) - Insert a marker label into a command buffer
    ///# C Specifications
    ///A single marker label can be inserted into a command buffer by calling:
    ///```c
    ///// Provided by VK_EXT_debug_marker
    ///void vkCmdDebugMarkerInsertEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkDebugMarkerMarkerInfoEXT*           pMarkerInfo);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - [`p_marker_info`] is a pointer to a [`DebugMarkerMarkerInfoEXT`] structure specifying the
    ///   parameters of the marker to insert.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_marker_info`] **must**  be a valid pointer to a valid [`DebugMarkerMarkerInfoEXT`]
    ///   structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, or compute operations
    ///
    /// ## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    /// ## Command Properties
    /// # Related
    /// - [`VK_EXT_debug_marker`]
    /// - [`CommandBuffer`]
    /// - [`DebugMarkerMarkerInfoEXT`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdDebugMarkerInsertEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_debug_marker_insert_ext<'lt>(
        self: &Unique<CommandBuffer>,
        p_marker_info: &DebugMarkerMarkerInfoEXT<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_marker()
            .and_then(|vtable| vtable.cmd_debug_marker_insert_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_marker()
            .and_then(|vtable| vtable.cmd_debug_marker_insert_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_marker_info as *const DebugMarkerMarkerInfoEXT<'lt>);
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_debug_marker`
pub struct DeviceExtDebugMarkerVTable {
    ///See [`FNDebugMarkerSetObjectNameExt`] for more information.
    pub debug_marker_set_object_name_ext: FNDebugMarkerSetObjectNameExt,
    ///See [`FNDebugMarkerSetObjectTagExt`] for more information.
    pub debug_marker_set_object_tag_ext: FNDebugMarkerSetObjectTagExt,
    ///See [`FNCmdDebugMarkerBeginExt`] for more information.
    pub cmd_debug_marker_begin_ext: FNCmdDebugMarkerBeginExt,
    ///See [`FNCmdDebugMarkerEndExt`] for more information.
    pub cmd_debug_marker_end_ext: FNCmdDebugMarkerEndExt,
    ///See [`FNCmdDebugMarkerInsertExt`] for more information.
    pub cmd_debug_marker_insert_ext: FNCmdDebugMarkerInsertExt,
}
impl DeviceExtDebugMarkerVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            debug_marker_set_object_name_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkDebugMarkerSetObjectNameEXT").as_ptr(),
                ))
            },
            debug_marker_set_object_tag_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkDebugMarkerSetObjectTagEXT").as_ptr()))
            },
            cmd_debug_marker_begin_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdDebugMarkerBeginEXT").as_ptr()))
            },
            cmd_debug_marker_end_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdDebugMarkerEndEXT").as_ptr()))
            },
            cmd_debug_marker_insert_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdDebugMarkerInsertEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::debug_marker_set_object_name_ext`]. See [`FNDebugMarkerSetObjectNameExt`] for
    /// more information.
    pub fn debug_marker_set_object_name_ext(&self) -> FNDebugMarkerSetObjectNameExt {
        self.debug_marker_set_object_name_ext
    }
    ///Gets [`Self::debug_marker_set_object_tag_ext`]. See [`FNDebugMarkerSetObjectTagExt`] for
    /// more information.
    pub fn debug_marker_set_object_tag_ext(&self) -> FNDebugMarkerSetObjectTagExt {
        self.debug_marker_set_object_tag_ext
    }
    ///Gets [`Self::cmd_debug_marker_begin_ext`]. See [`FNCmdDebugMarkerBeginExt`] for more
    /// information.
    pub fn cmd_debug_marker_begin_ext(&self) -> FNCmdDebugMarkerBeginExt {
        self.cmd_debug_marker_begin_ext
    }
    ///Gets [`Self::cmd_debug_marker_end_ext`]. See [`FNCmdDebugMarkerEndExt`] for more
    /// information.
    pub fn cmd_debug_marker_end_ext(&self) -> FNCmdDebugMarkerEndExt {
        self.cmd_debug_marker_end_ext
    }
    ///Gets [`Self::cmd_debug_marker_insert_ext`]. See [`FNCmdDebugMarkerInsertExt`] for more
    /// information.
    pub fn cmd_debug_marker_insert_ext(&self) -> FNCmdDebugMarkerInsertExt {
        self.cmd_debug_marker_insert_ext
    }
}
