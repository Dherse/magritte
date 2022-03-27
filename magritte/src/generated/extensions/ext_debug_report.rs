use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_REPORT_SPEC_VERSION")]
pub const EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 10;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_REPORT_EXTENSION_NAME")]
pub const EXT_DEBUG_REPORT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_debug_report");
///[VkDebugReportCallbackCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackCreateInfoEXT.html) - Structure specifying parameters of a newly created debug report callback
///# C Specifications
///The definition of [`DebugReportCallbackCreateInfoEXT`] is:
///```c
///// Provided by VK_EXT_debug_report
///typedef struct VkDebugReportCallbackCreateInfoEXT {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkDebugReportFlagsEXT           flags;
///    PFN_vkDebugReportCallbackEXT    pfnCallback;
///    void*                           pUserData;
///} VkDebugReportCallbackCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`DebugReportFlagBitsEXT`] specifying which event(s) will cause this
///   callback to be called.
/// - [`pfn_callback`] is the application callback function to call.
/// - [`p_user_data`] is user data to be passed to the callback.
///# Description
///For each [`DebugReportCallbackEXT`] that is created the
///[`DebugReportCallbackCreateInfoEXT`]::[`flags`] determine when that
///[`DebugReportCallbackCreateInfoEXT`]::[`pfn_callback`] is called.
///When an event happens, the implementation will do a bitwise AND of the
///event’s [`DebugReportFlagBitsEXT`] flags to each
///[`DebugReportCallbackEXT`] object’s flags.
///For each non-zero result the corresponding callback will be called.
///The callback will come directly from the component that detected the event,
///unless some other layer intercepts the calls for its own purposes (filter
///them in a different way, log to a system error log, etc.).An application **may** receive
/// multiple callbacks if multiple
///[`DebugReportCallbackEXT`] objects were created.
///A callback will always be executed in the same thread as the originating
///Vulkan call.A callback may be called from multiple threads simultaneously (if the
///application is making Vulkan calls from multiple threads).Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT`
/// - [`flags`]**must** be a valid combination of [`DebugReportFlagBitsEXT`] values
/// - [`pfn_callback`]**must** be a valid [`PFNDebugReportCallbackEXT`] value
///# Related
/// - [`PFNDebugReportCallbackEXT`]
/// - [`VK_EXT_debug_report`]
/// - [`DebugReportFlagsEXT`]
/// - [`StructureType`]
/// - [`CreateDebugReportCallbackEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DebugReportCallbackCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`DebugReportFlagBitsEXT`] specifying
    ///which event(s) will cause this callback to be called.
    flags: DebugReportFlagsEXT,
    ///[`pfn_callback`] is the application callback function to call.
    pfn_callback: PFNDebugReportCallbackEXT<'lt>,
    ///[`p_user_data`] is user data to be passed to the callback.
    p_user_data: *const c_void,
}
