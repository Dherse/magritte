//![VK_EXT_debug_utils](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html) - instance extension
//!# Description
//!Due to the nature of the Vulkan interface, there is very little error
//!information available to the developer and application.
//!By using the [`VK_EXT_debug_utils`] extension, developers **can** obtain more
//!information.
//!When combined with validation layers, even more detailed feedback on the
//!application’s use of Vulkan will be provided.This extension provides the following capabilities:
//! - The ability to create a debug messenger which will pass along debug
//!messages to an application supplied callback.
//! - The ability to identify specific Vulkan objects using a name or tag to
//!improve tracking.
//! - The ability to identify specific sections within a [`Queue`] or
//![`CommandBuffer`] using labels to aid organization and offline
//!analysis in external tools.
//!The main difference between this extension and `[`VK_EXT_debug_report`]`
//!and `[`VK_EXT_debug_marker`]` is that those extensions use
//![`DebugReportObjectTypeEXT`] to identify objects.
//!This extension uses the core [`ObjectType`] in place of
//![`DebugReportObjectTypeEXT`].
//!The primary reason for this move is that no future object type handle
//!enumeration values will be added to [`DebugReportObjectTypeEXT`] since
//!the creation of [`ObjectType`].In addition, this extension combines the functionality of both
//!`[`VK_EXT_debug_report`]` and `[`VK_EXT_debug_marker`]` by allowing
//!object name and debug markers (now called labels) to be returned to the
//!application’s callback function.
//!This should assist in clarifying the details of a debug message including:
//!what objects are involved and potentially which location within a
//![`Queue`] or [`CommandBuffer`] the message occurred.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Mark Young [marky-lunarg](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_debug_utils]
//!   @marky-lunarg%0A<<Here describe the issue or question you have about the VK_EXT_debug_utils
//!   extension>>)
//!# New handles
//! - [`DebugUtilsMessengerEXT`]
//!# New functions & commands
//! - [`CmdBeginDebugUtilsLabelEXT`]
//! - [`CmdEndDebugUtilsLabelEXT`]
//! - [`CmdInsertDebugUtilsLabelEXT`]
//! - [`CreateDebugUtilsMessengerEXT`]
//! - [`DestroyDebugUtilsMessengerEXT`]
//! - [`QueueBeginDebugUtilsLabelEXT`]
//! - [`QueueEndDebugUtilsLabelEXT`]
//! - [`QueueInsertDebugUtilsLabelEXT`]
//! - [`SetDebugUtilsObjectNameEXT`]
//! - [`SetDebugUtilsObjectTagEXT`]
//! - [`SubmitDebugUtilsMessageEXT`]
//!# New structures
//! - [`DebugUtilsLabelEXT`]
//! - [`DebugUtilsMessengerCallbackDataEXT`]
//! - [`DebugUtilsObjectNameInfoEXT`]
//! - [`DebugUtilsObjectTagInfoEXT`]
//! - Extending [`InstanceCreateInfo`]:
//! - [`DebugUtilsMessengerCreateInfoEXT`]
//!# New enums
//! - [`DebugUtilsMessageSeverityFlagBitsEXT`]
//! - [`DebugUtilsMessageTypeFlagBitsEXT`]
//!# New bitmasks
//! - [`DebugUtilsMessageSeverityFlagsEXT`]
//! - [`DebugUtilsMessageTypeFlagsEXT`]
//! - [`DebugUtilsMessengerCallbackDataFlagsEXT`]
//! - [`DebugUtilsMessengerCreateFlagsEXT`]
//!# New constants
//! - [`EXT_DEBUG_UTILS_EXTENSION_NAME`]
//! - [`EXT_DEBUG_UTILS_SPEC_VERSION`]
//! - Extending [`ObjectType`]:
//! - `VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT`
//! - `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT`
//! - `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT`
//!# Known issues & F.A.Q
//!1) Should we just name this extension `VK_EXT_debug_report2`**RESOLVED**: No.
//!There is enough additional changes to the structures to break backwards
//!compatibility.
//!So, a new name was decided that would not indicate any interaction with the
//!previous extension.2) Will validation layers immediately support all the new
//! features.**RESOLVED**: Not immediately.
//!As one can imagine, there is a lot of work involved with converting the
//!validation layer logging over to the new functionality.
//!Basic logging, as seen in the origin `[`VK_EXT_debug_report`]` extension
//!will be made available immediately.
//!However, adding the labels and object names will take time.
//!Since the priority for Khronos at this time is to continue focusing on Valid
//!Usage statements, it may take a while before the new functionality is fully
//!exposed.3) If the validation layers will not expose the new functionality
//!immediately, then what is the point of this extension?**RESOLVED**: We needed a replacement for
//! `[`VK_EXT_debug_report`]` because
//!the [`DebugReportObjectTypeEXT`] enumeration will no longer be updated
//!and any new objects will need to be debugged using the new functionality
//!provided by this extension.4) Should this extension be split into two separate parts (1
//! extension that
//!is an instance extension providing the callback functionality, and another
//!device extension providing the general debug marker and annotation
//!functionality)?**RESOLVED**: No, the functionality for this extension is too closely related.
//!If we did split up the extension, where would the structures and enums live,
//!and how would you define that the device behavior in the instance extension
//!is really only valid if the device extension is enabled, and the
//!functionality is passed in.
//!It is cleaner to just define this all as an instance extension, plus it
//!allows the application to enable all debug functionality provided with one
//!enable string during [`CreateInstance`].
//!# Version History
//! - Revision 1, 2017-09-14 (Mark Young and all listed Contributors)
//! - Initial draft, based on `[`VK_EXT_debug_report`]` and
//!`[`VK_EXT_debug_marker`]` in addition to previous feedback supplied
//!from various companies including Valve, Epic, and Oxide games.
//!
//! - Revision 2, 2020-04-03 (Mark Young and Piers Daniell)
//! - Updated to allow either `NULL` or an empty string to be passed in for
//!`pObjectName` in [`DebugUtilsObjectNameInfoEXT`], because the
//!loader and various drivers support `NULL` already.
//!# Other info
//! * 2020-04-03
//! * 2
//! * No known IP claims.
//!*
//! - This extension is written against version 1.0 of the Vulkan API.
//! - Requires [`ObjectType`]
//!
//!*
//! - Mark Young, LunarG
//! - Baldur Karlsson
//! - Ian Elliott, Google
//! - Courtney Goeltzenleuchter, Google
//! - Karl Schultz, LunarG
//! - Mark Lobodzinski, LunarG
//! - Mike Schuchardt, LunarG
//! - Jaakko Konttinen, AMD
//! - Dan Ginsburg, Valve Software
//! - Rolando Olivares, Epic Games
//! - Dan Baker, Oxide Games
//! - Kyle Spagnoli, NVIDIA
//! - Jon Ashburn, LunarG
//! - Piers Daniell, NVIDIA
//!# Related
//! - [`PFNDebugUtilsMessengerCallbackEXT`]
//! - [`DebugUtilsLabelEXT`]
//! - [`DebugUtilsMessageSeverityFlagBitsEXT`]
//! - [`DebugUtilsMessageSeverityFlagsEXT`]
//! - [`DebugUtilsMessageTypeFlagBitsEXT`]
//! - [`DebugUtilsMessageTypeFlagsEXT`]
//! - [`DebugUtilsMessengerCallbackDataEXT`]
//! - [`DebugUtilsMessengerCallbackDataFlagsEXT`]
//! - [`DebugUtilsMessengerCreateFlagsEXT`]
//! - [`DebugUtilsMessengerCreateInfoEXT`]
//! - [`DebugUtilsMessengerEXT`]
//! - [`DebugUtilsObjectNameInfoEXT`]
//! - [`DebugUtilsObjectTagInfoEXT`]
//! - [`CmdBeginDebugUtilsLabelEXT`]
//! - [`CmdEndDebugUtilsLabelEXT`]
//! - [`CmdInsertDebugUtilsLabelEXT`]
//! - [`CreateDebugUtilsMessengerEXT`]
//! - [`DestroyDebugUtilsMessengerEXT`]
//! - [`QueueBeginDebugUtilsLabelEXT`]
//! - [`QueueEndDebugUtilsLabelEXT`]
//! - [`QueueInsertDebugUtilsLabelEXT`]
//! - [`SetDebugUtilsObjectNameEXT`]
//! - [`SetDebugUtilsObjectTagEXT`]
//! - [`SubmitDebugUtilsMessageEXT`]
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
#[doc(alias = "VK_EXT_DEBUG_UTILS_SPEC_VERSION")]
pub const EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_UTILS_EXTENSION_NAME")]
pub const EXT_DEBUG_UTILS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_debug_utils");
