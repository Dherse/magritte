//![VK_EXT_debug_report](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_report.html) - instance extension
//!# Description
//!Due to the nature of the Vulkan interface, there is very little error
//!information available to the developer and application.
//!By enabling optional validation layers and using the [`VK_EXT_debug_report`]
//!extension, developers **can** obtain much more detailed feedback on the
//!application’s use of Vulkan.
//!This extension defines a way for layers and the implementation to call back
//!to the application for events of interest to the application.
//!# Revision
//!10
//!# Dependencies
//! - *Deprecated* by
//!`[`VK_EXT_debug_utils`]`
//!extension
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Courtney Goeltzenleuchter [courtney-g](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_debug_report]
//!   @courtney-g%0A<<Here describe the issue or question you have about the VK_EXT_debug_report
//!   extension>>)
//!# New handles
//! - [`DebugReportCallbackEXT`]
//!# New functions & commands
//! - [`CreateDebugReportCallbackEXT`]
//! - [`DebugReportMessageEXT`]
//! - [`DestroyDebugReportCallbackEXT`]
//!# New structures
//! - Extending [`InstanceCreateInfo`]:
//! - [`DebugReportCallbackCreateInfoEXT`]
//!# New enums
//! - [`DebugReportFlagBitsEXT`]
//! - [`DebugReportObjectTypeEXT`]
//!# New bitmasks
//! - [`DebugReportFlagsEXT`]
//!# New constants
//! - [`EXT_DEBUG_REPORT_EXTENSION_NAME`]
//! - [`EXT_DEBUG_REPORT_SPEC_VERSION`]
//! - Extending [`ObjectType`]:
//! - `VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT`
//! - Extending [`VulkanResultCodes`]:
//! - `VK_ERROR_VALIDATION_FAILED_EXT`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT`If [Version 1.1]() is supported:
//! - Extending [`DebugReportObjectTypeEXT`]:
//! - `VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT`
//! - `VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT`
//!# Known issues & F.A.Q
//!1) What is the hierarchy / seriousness of the message flags? E.g.
//!`ERROR` > `WARN` > `PERF_WARN` …​**RESOLVED**: There is no specific hierarchy.
//!Each bit is independent and should be checked via bitwise AND.
//!For example:
//!```c
//!    if (localFlags & VK_DEBUG_REPORT_ERROR_BIT_EXT) {
//!        process error message
//!    }
//!    if (localFlags & VK_DEBUG_REPORT_DEBUG_BIT_EXT) {
//!        process debug message
//!    }
//!```
//!The validation layers do use them in a hierarchical way (`ERROR` >
//!`WARN` > `PERF`, `WARN` > `DEBUG` > `INFO`) and they (at
//!least at the time of this writing) only set one bit at a time.
//!But it is not a requirement of this extension.It is possible that a layer may intercept and
//! change, or augment the flags
//!with extension values the application’s debug report handler may not be
//!familiar with, so it is important to treat each flag independently.2) Should there be a VU
//! requiring
//![`DebugReportCallbackCreateInfoEXT::flags`] to be non-zero?**RESOLVED**: It may not be very
//! useful, but we do not need VU statement
//!requiring the [`DebugReportCallbackCreateInfoEXT`]`::msgFlags` at
//!create-time to be non-zero.
//!One can imagine that apps may prefer it as it allows them to set the mask as
//!desired - including nothing - at runtime without having to check.3) What is the difference
//! between `VK_DEBUG_REPORT_DEBUG_BIT_EXT` and
//!`VK_DEBUG_REPORT_INFORMATION_BIT_EXT`?**RESOLVED**: `VK_DEBUG_REPORT_DEBUG_BIT_EXT` specifies
//! information that
//!could be useful debugging the Vulkan implementation itself.4) How do you compare handles
//! returned by the debug_report callback to the
//!application’s handles?**RESOLVED**: Due to the different nature of dispatchable and
//! nondispatchable
//!handles there is no generic way (that we know of) that works for common
//!compilers with 32bit, 64bit, C and C++.
//!We recommend applications use the same cast that the validation layers use:+
//!```c
//!reinterpret_cast<uint64_t &>(dispatchableHandle)
//!(uint64_t)(nondispatchableHandle)
//!```
//!+
//!This does require that the app treat dispatchable and nondispatchable
//!handles differently.
//!# Version History
//! - Revision 1, 2015-05-20 (Courtney Goetzenleuchter)
//! - Initial draft, based on LunarG KHR spec, other KHR specs
//! - Revision 2, 2016-02-16 (Courtney Goetzenleuchter)
//! - Update usage, documentation
//! - Revision 3, 2016-06-14 (Courtney Goetzenleuchter)
//! - Update VK_EXT_DEBUG_REPORT_SPEC_VERSION to indicate added support for
//!vkCreateInstance and vkDestroyInstance
//! - Revision 4, 2016-12-08 (Mark Lobodzinski)
//! - Added Display_KHR, DisplayModeKHR extension objects
//! - Added ObjectTable_NVX, IndirectCommandsLayout_NVX extension objects
//! - Bumped spec revision
//! - Retroactively added version history
//! - Revision 5, 2017-01-31 (Baldur Karlsson)
//! - Moved definition of [`DebugReportObjectTypeEXT`] from debug marker
//!chapter
//! - Revision 6, 2017-01-31 (Baldur Karlsson)
//! - Added VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT
//! - Revision 7, 2017-04-20 (Courtney Goeltzenleuchter)
//! - Clarify wording and address questions from developers.
//! - Revision 8, 2017-04-21 (Courtney Goeltzenleuchter)
//! - Remove unused enum VkDebugReportErrorEXT
//! - Revision 9, 2017-09-12 (Tobias Hector)
//! - Added interactions with Vulkan 1.1
//! - Revision 10, 2020-12-14 (Courtney Goetzenleuchter)
//! - Add issue 4 discussing matching handles returned by the extension,
//!based on suggestion in public issue 368.
//!# Other info
//! * 2020-12-14
//! * No known IP claims.
//!*
//! - Courtney Goeltzenleuchter, LunarG
//! - Dan Ginsburg, Valve
//! - Jon Ashburn, LunarG
//! - Mark Lobodzinski, LunarG
//!# Related
//! - [`PFNDebugReportCallbackEXT`]
//! - [`DebugReportCallbackCreateInfoEXT`]
//! - [`DebugReportCallbackEXT`]
//! - [`DebugReportFlagBitsEXT`]
//! - [`DebugReportFlagsEXT`]
//! - [`DebugReportObjectTypeEXT`]
//! - [`CreateDebugReportCallbackEXT`]
//! - [`DebugReportMessageEXT`]
//! - [`DestroyDebugReportCallbackEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_REPORT_SPEC_VERSION")]
pub const EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 10;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_REPORT_EXTENSION_NAME")]
pub const EXT_DEBUG_REPORT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_debug_report");
