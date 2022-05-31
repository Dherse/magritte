//![VK_EXT_debug_utils](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html) - instance extension
//!# Description
//!Due to the nature of the Vulkan interface, there is very little error
//!information available to the developer and application.
//!By using the [`VK_EXT_debug_utils`] extension, developers  **can**  obtain more
//!information.
//!When combined with validation layers, even more detailed feedback on the
//!application’s use of Vulkan will be provided.This extension provides the following capabilities:
//! - The ability to create a debug messenger which will pass along debug messages to an application
//!   supplied callback.
//! - The ability to identify specific Vulkan objects using a name or tag to improve tracking.
//! - The ability to identify specific sections within a [`Queue`] or [`CommandBuffer`] using labels
//!   to aid organization and offline analysis in external tools.
//!The main difference between this extension and `[`ext_debug_report`]`
//!and `[`ext_debug_marker`]` is that those extensions use
//![`DebugReportObjectTypeEXT`] to identify objects.
//!This extension uses the core [`ObjectType`] in place of
//![`DebugReportObjectTypeEXT`].
//!The primary reason for this move is that no future object type handle
//!enumeration values will be added to [`DebugReportObjectTypeEXT`] since
//!the creation of [`ObjectType`].In addition, this extension combines the functionality of both
//!`[`ext_debug_report`]` and `[`ext_debug_marker`]` by allowing
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
//! - [`cmd_begin_debug_utils_label_ext`]
//! - [`cmd_end_debug_utils_label_ext`]
//! - [`cmd_insert_debug_utils_label_ext`]
//! - [`create_debug_utils_messenger_ext`]
//! - [`destroy_debug_utils_messenger_ext`]
//! - [`queue_begin_debug_utils_label_ext`]
//! - [`queue_end_debug_utils_label_ext`]
//! - [`queue_insert_debug_utils_label_ext`]
//! - [`set_debug_utils_object_name_ext`]
//! - [`set_debug_utils_object_tag_ext`]
//! - [`submit_debug_utils_message_ext`]
//!# New structures
//! - [`DebugUtilsLabelEXT`]
//! - [`DebugUtilsMessengerCallbackDataEXT`]
//! - [`DebugUtilsObjectNameInfoEXT`]
//! - [`DebugUtilsObjectTagInfoEXT`]
//! - Extending [`InstanceCreateInfo`]:  - [`DebugUtilsMessengerCreateInfoEXT`]
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
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT`  -
//!   `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT`  -
//!   `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT`
//!# Known issues & F.A.Q
//!1) Should we just name this extension `VK_EXT_debug_report2` **RESOLVED** : No.
//!There is enough additional changes to the structures to break backwards
//!compatibility.
//!So, a new name was decided that would not indicate any interaction with the
//!previous extension.2) Will validation layers immediately support all the new features.
//! **RESOLVED** : Not immediately.
//!As one can imagine, there is a lot of work involved with converting the
//!validation layer logging over to the new functionality.
//!Basic logging, as seen in the origin `[`ext_debug_report`]` extension
//!will be made available immediately.
//!However, adding the labels and object names will take time.
//!Since the priority for Khronos at this time is to continue focusing on Valid
//!Usage statements, it may take a while before the new functionality is fully
//!exposed.3) If the validation layers will not expose the new functionality
//!immediately, then what is the point of this extension? **RESOLVED** : We needed a replacement
//! for `[`ext_debug_report`]` because
//!the [`DebugReportObjectTypeEXT`] enumeration will no longer be updated
//!and any new objects will need to be debugged using the new functionality
//!provided by this extension.4) Should this extension be split into two separate parts (1
//! extension that
//!is an instance extension providing the callback functionality, and another
//!device extension providing the general debug marker and annotation
//!functionality)? **RESOLVED** : No, the functionality for this extension is too closely related.
//!If we did split up the extension, where would the structures and enums live,
//!and how would you define that the device behavior in the instance extension
//!is really only valid if the device extension is enabled, and the
//!functionality is passed in.
//!It is cleaner to just define this all as an instance extension, plus it
//!allows the application to enable all debug functionality provided with one
//!enable string during [`create_instance`].
//!# Version History
//! - Revision 1, 2017-09-14 (Mark Young and all listed Contributors)  - Initial draft, based on
//!   `[`ext_debug_report`]` and `[`ext_debug_marker`]` in addition to previous feedback supplied
//!   from various companies including Valve, Epic, and Oxide games.
//! - Revision 2, 2020-04-03 (Mark Young and Piers Daniell)  - Updated to allow either `NULL` or an
//!   empty string to be passed in for `pObjectName` in [`DebugUtilsObjectNameInfoEXT`], because the
//!   loader and various drivers support `NULL` already.
//!# Other info
//! * 2020-04-03
//! * 2
//! * No known IP claims.
//! * - This extension is written against version 1.0 of the Vulkan API.  - Requires [`ObjectType`]
//! * - Mark Young, LunarG  - Baldur Karlsson  - Ian Elliott, Google  - Courtney Goeltzenleuchter,
//!   Google  - Karl Schultz, LunarG  - Mark Lobodzinski, LunarG  - Mike Schuchardt, LunarG  -
//!   Jaakko Konttinen, AMD  - Dan Ginsburg, Valve Software  - Rolando Olivares, Epic Games  - Dan
//!   Baker, Oxide Games  - Kyle Spagnoli, NVIDIA  - Jon Ashburn, LunarG  - Piers Daniell, NVIDIA
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
//! - [`cmd_begin_debug_utils_label_ext`]
//! - [`cmd_end_debug_utils_label_ext`]
//! - [`cmd_insert_debug_utils_label_ext`]
//! - [`create_debug_utils_messenger_ext`]
//! - [`destroy_debug_utils_messenger_ext`]
//! - [`queue_begin_debug_utils_label_ext`]
//! - [`queue_end_debug_utils_label_ext`]
//! - [`queue_insert_debug_utils_label_ext`]
//! - [`set_debug_utils_object_name_ext`]
//! - [`set_debug_utils_object_tag_ext`]
//! - [`submit_debug_utils_message_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    entry::Entry,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, CommandBuffer, Device, Instance, ObjectType, Queue,
        StructureType, VulkanResultCodes,
    },
    AsRaw, Handle, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
    mem::MaybeUninit,
    os::raw::c_char,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_UTILS_SPEC_VERSION")]
pub const EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_UTILS_EXTENSION_NAME")]
pub const EXT_DEBUG_UTILS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_debug_utils");
///[PFN_vkDebugUtilsMessengerCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html) - Application-defined debug messenger callback function
///# C Specifications
///The prototype for the
///[`DebugUtilsMessengerCreateInfoEXT::pfn_user_callback`] function
///implemented by the application is:
///```c
///// Provided by VK_EXT_debug_utils
///typedef VkBool32 (VKAPI_PTR *PFN_vkDebugUtilsMessengerCallbackEXT)(
///    VkDebugUtilsMessageSeverityFlagBitsEXT           messageSeverity,
///    VkDebugUtilsMessageTypeFlagsEXT                  messageTypes,
///    const VkDebugUtilsMessengerCallbackDataEXT*      pCallbackData,
///    void*                                            pUserData);
///```
/// # Parameters
/// - [`message_severity`] specifies the [`DebugUtilsMessageSeverityFlagBitsEXT`] that triggered
///   this callback.
/// - [`message_types`] is a bitmask of [`DebugUtilsMessageTypeFlagBitsEXT`] specifying which type
///   of event(s) triggered this callback.
/// - [`p_callback_data`] contains all the callback related data in the
///   [`DebugUtilsMessengerCallbackDataEXT`] structure.
/// - [`p_user_data`] is the user data provided when the [`DebugUtilsMessengerEXT`] was created.
/// # Description
/// The callback returns a [`Bool32`], which is interpreted in a
/// layer-specified manner.
/// The application  **should**  always return [`FALSE`].
/// The [`TRUE`] value is reserved for use in layer development.
/// ## Valid Usage
/// - The callback  **must**  not make calls to any Vulkan commands
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsMessengerCreateInfoEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "PFN_vkDebugUtilsMessengerCallbackEXT")]
pub type PFNDebugUtilsMessengerCallbackEXT = Option<
    for<'lt> unsafe extern "system" fn(
        message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'lt>,
        p_user_data: *mut c_void,
    ) -> Bool32,
>;
///[vkSetDebugUtilsObjectNameEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html) - Give a user-friendly name to an object
///# C Specifications
///```c
///// Provided by VK_EXT_debug_utils
///VkResult vkSetDebugUtilsObjectNameEXT(
///    VkDevice                                    device,
///    const VkDebugUtilsObjectNameInfoEXT*        pNameInfo);
///```
/// # Parameters
/// - [`device`] is the device that created the object.
/// - [`p_name_info`] is a pointer to a [`DebugUtilsObjectNameInfoEXT`] structure specifying
///   parameters of the name to set on the object.
/// # Description
/// ## Valid Usage
/// - `pNameInfo->objectType` **must**  not be `VK_OBJECT_TYPE_UNKNOWN`
/// - `pNameInfo->objectHandle` **must**  not be [`crate::Handle::null`]
///
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_name_info`] **must**  be a valid pointer to a valid [`DebugUtilsObjectNameInfoEXT`]
///   structure
///
/// ## Host Synchronization
/// - Host access to `pNameInfo->objectHandle` **must**  be externally synchronized
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsObjectNameInfoEXT`]
/// - [`Device`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
pub type FNSetDebugUtilsObjectNameExt = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_name_info: *const DebugUtilsObjectNameInfoEXT<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkSetDebugUtilsObjectTagEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html) - Attach arbitrary data to an object
///# C Specifications
///```c
///// Provided by VK_EXT_debug_utils
///VkResult vkSetDebugUtilsObjectTagEXT(
///    VkDevice                                    device,
///    const VkDebugUtilsObjectTagInfoEXT*         pTagInfo);
///```
/// # Parameters
/// - [`device`] is the device that created the object.
/// - [`p_tag_info`] is a pointer to a [`DebugUtilsObjectTagInfoEXT`] structure specifying
///   parameters of the tag to attach to the object.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_tag_info`] **must**  be a valid pointer to a valid [`DebugUtilsObjectTagInfoEXT`]
///   structure
///
/// ## Host Synchronization
/// - Host access to `pTagInfo->objectHandle` **must**  be externally synchronized
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsObjectTagInfoEXT`]
/// - [`Device`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
pub type FNSetDebugUtilsObjectTagExt = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_tag_info: *const DebugUtilsObjectTagInfoEXT<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkQueueBeginDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html) - Open a queue debug label region
///# C Specifications
///A queue debug label region is opened by calling:
///```c
///// Provided by VK_EXT_debug_utils
///void vkQueueBeginDebugUtilsLabelEXT(
///    VkQueue                                     queue,
///    const VkDebugUtilsLabelEXT*                 pLabelInfo);
///```
/// # Parameters
/// - [`queue`] is the queue in which to start a debug label region.
/// - [`p_label_info`] is a pointer to a [`DebugUtilsLabelEXT`] structure specifying parameters of
///   the label region to open.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`queue`] **must**  be a valid [`Queue`] handle
/// - [`p_label_info`] **must**  be a valid pointer to a valid [`DebugUtilsLabelEXT`] structure
///
/// ## Command Properties
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsLabelEXT`]
/// - [`Queue`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
pub type FNQueueBeginDebugUtilsLabelExt =
    Option<for<'lt> unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT<'lt>)>;
///[vkQueueEndDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html) - Close a queue debug label region
///# C Specifications
///A queue debug label region is closed by calling:
///```c
///// Provided by VK_EXT_debug_utils
///void vkQueueEndDebugUtilsLabelEXT(
///    VkQueue                                     queue);
///```
/// # Parameters
/// - [`queue`] is the queue in which a debug label region should be closed.
/// # Description
/// The calls to [`queue_begin_debug_utils_label_ext`] and
/// [`queue_end_debug_utils_label_ext`] **must**  be matched and balanced.
/// ## Valid Usage
/// - There  **must**  be an outstanding [`queue_begin_debug_utils_label_ext`] command prior to the
///   [`queue_end_debug_utils_label_ext`] on the queue
///
/// ## Valid Usage (Implicit)
/// - [`queue`] **must**  be a valid [`Queue`] handle
///
/// ## Command Properties
/// # Related
/// - [`ext_debug_utils`]
/// - [`Queue`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
pub type FNQueueEndDebugUtilsLabelExt = Option<unsafe extern "system" fn(queue: Queue)>;
///[vkQueueInsertDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html) - Insert a label into a queue
///# C Specifications
///A single label can be inserted into a queue by calling:
///```c
///// Provided by VK_EXT_debug_utils
///void vkQueueInsertDebugUtilsLabelEXT(
///    VkQueue                                     queue,
///    const VkDebugUtilsLabelEXT*                 pLabelInfo);
///```
/// # Parameters
/// - [`queue`] is the queue into which a debug label will be inserted.
/// - [`p_label_info`] is a pointer to a [`DebugUtilsLabelEXT`] structure specifying parameters of
///   the label to insert.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`queue`] **must**  be a valid [`Queue`] handle
/// - [`p_label_info`] **must**  be a valid pointer to a valid [`DebugUtilsLabelEXT`] structure
///
/// ## Command Properties
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsLabelEXT`]
/// - [`Queue`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
pub type FNQueueInsertDebugUtilsLabelExt =
    Option<for<'lt> unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT<'lt>)>;
///[vkCreateDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html) - Create a debug messenger object
///# C Specifications
///A debug messenger triggers a debug callback with a debug message when an
///event of interest occurs.
///To create a debug messenger which will trigger a debug callback, call:
///```c
///// Provided by VK_EXT_debug_utils
///VkResult vkCreateDebugUtilsMessengerEXT(
///    VkInstance                                  instance,
///    const VkDebugUtilsMessengerCreateInfoEXT*   pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkDebugUtilsMessengerEXT*                   pMessenger);
///```
/// # Parameters
/// - [`instance`] is the instance the messenger will be used with.
/// - [`p_create_info`] is a pointer to a [`DebugUtilsMessengerCreateInfoEXT`] structure containing
///   the callback pointer, as well as defining conditions under which this messenger will trigger
///   the callback.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// - [`p_messenger`] is a pointer to a [`DebugUtilsMessengerEXT`] handle in which the created
///   object is returned.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`DebugUtilsMessengerCreateInfoEXT`]
///   structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_messenger`] **must**  be a valid pointer to a [`DebugUtilsMessengerEXT`] handle
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// The application  **must**  ensure that [`create_debug_utils_messenger_ext`] is
/// not executed in parallel with any Vulkan command that is also called with
/// [`instance`] or child of [`instance`] as the dispatchable argument.
/// # Related
/// - [`ext_debug_utils`]
/// - [`AllocationCallbacks`]
/// - [`DebugUtilsMessengerCreateInfoEXT`]
/// - [`DebugUtilsMessengerEXT`]
/// - [`Instance`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
pub type FNCreateDebugUtilsMessengerExt = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const DebugUtilsMessengerCreateInfoEXT<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_messenger: *mut DebugUtilsMessengerEXT,
    ) -> VulkanResultCodes,
>;
///[vkDestroyDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html) - Destroy a debug messenger object
///# C Specifications
///To destroy a [`DebugUtilsMessengerEXT`] object, call:
///```c
///// Provided by VK_EXT_debug_utils
///void vkDestroyDebugUtilsMessengerEXT(
///    VkInstance                                  instance,
///    VkDebugUtilsMessengerEXT                    messenger,
///    const VkAllocationCallbacks*                pAllocator);
///```
/// # Parameters
/// - [`instance`] is the instance where the callback was created.
/// - [`messenger`] is the [`DebugUtilsMessengerEXT`] object to destroy. [`messenger`] is an
///   externally synchronized object and  **must**  not be used on more than one thread at a time.
///   This means that [`destroy_debug_utils_messenger_ext`] **must**  not be called when a callback
///   is active.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// # Description
/// ## Valid Usage
/// - If [`AllocationCallbacks`] were provided when [`messenger`] was created, a compatible set of
///   callbacks  **must**  be provided here
/// - If no [`AllocationCallbacks`] were provided when [`messenger`] was created, [`p_allocator`]
///   **must**  be `NULL`
///
/// ## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - If [`messenger`] is not [`crate::Handle::null`], [`messenger`] **must**  be a valid
///   [`DebugUtilsMessengerEXT`] handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - If [`messenger`] is a valid handle, it  **must**  have been created, allocated, or retrieved
///   from [`instance`]
///
/// ## Host Synchronization
/// - Host access to [`messenger`] **must**  be externally synchronized
/// The application  **must**  ensure that [`destroy_debug_utils_messenger_ext`] is
/// not executed in parallel with any Vulkan command that is also called with
/// [`instance`] or child of [`instance`] as the dispatchable argument.
/// # Related
/// - [`ext_debug_utils`]
/// - [`AllocationCallbacks`]
/// - [`DebugUtilsMessengerEXT`]
/// - [`Instance`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
pub type FNDestroyDebugUtilsMessengerExt = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        messenger: DebugUtilsMessengerEXT,
        p_allocator: *const AllocationCallbacks<'lt>,
    ),
>;
///[vkSubmitDebugUtilsMessageEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html) - Inject a message into a debug stream
///# C Specifications
///There may be times that a user wishes to intentionally submit a debug
///message.
///To do this, call:
///```c
///// Provided by VK_EXT_debug_utils
///void vkSubmitDebugUtilsMessageEXT(
///    VkInstance                                  instance,
///    VkDebugUtilsMessageSeverityFlagBitsEXT      messageSeverity,
///    VkDebugUtilsMessageTypeFlagsEXT             messageTypes,
///    const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData);
///```
/// # Parameters
/// - [`instance`] is the debug stream’s [`Instance`].
/// - [`message_severity`] is a [`DebugUtilsMessageSeverityFlagBitsEXT`] value specifying the
///   severity of this event/message.
/// - [`message_types`] is a bitmask of [`DebugUtilsMessageTypeFlagBitsEXT`] specifying which type
///   of event(s) to identify with this message.
/// - [`p_callback_data`] contains all the callback related data in the
///   [`DebugUtilsMessengerCallbackDataEXT`] structure.
/// # Description
/// The call will propagate through the layers and generate callback(s) as
/// indicated by the message’s flags.
/// The parameters are passed on to the callback in addition to the
/// `pUserData` value that was defined at the time the messenger was
/// registered.
/// ## Valid Usage
/// - The `objectType` member of each element of `pCallbackData->pObjects` **must**  not be
///   `VK_OBJECT_TYPE_UNKNOWN`
///
/// ## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - [`message_severity`] **must**  be a valid [`DebugUtilsMessageSeverityFlagBitsEXT`] value
/// - [`message_types`] **must**  be a valid combination of [`DebugUtilsMessageTypeFlagBitsEXT`]
///   values
/// - [`message_types`] **must**  not be `0`
/// - [`p_callback_data`] **must**  be a valid pointer to a valid
///   [`DebugUtilsMessengerCallbackDataEXT`] structure
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsMessageSeverityFlagBitsEXT`]
/// - [`DebugUtilsMessageTypeFlagsEXT`]
/// - [`DebugUtilsMessengerCallbackDataEXT`]
/// - [`Instance`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
pub type FNSubmitDebugUtilsMessageExt = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'lt>,
    ),
>;
///[vkCmdBeginDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html) - Open a command buffer debug label region
///# C Specifications
///A command buffer debug label region can be opened by calling:
///```c
///// Provided by VK_EXT_debug_utils
///void vkCmdBeginDebugUtilsLabelEXT(
///    VkCommandBuffer                             commandBuffer,
///    const VkDebugUtilsLabelEXT*                 pLabelInfo);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - [`p_label_info`] is a pointer to a [`DebugUtilsLabelEXT`] structure specifying parameters of
///   the label region to open.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_label_info`] **must**  be a valid pointer to a valid [`DebugUtilsLabelEXT`] structure
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
/// - [`ext_debug_utils`]
/// - [`CommandBuffer`]
/// - [`DebugUtilsLabelEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
pub type FNCmdBeginDebugUtilsLabelExt = Option<
    for<'lt> unsafe extern "system" fn(command_buffer: CommandBuffer, p_label_info: *const DebugUtilsLabelEXT<'lt>),
>;
///[vkCmdEndDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html) - Close a command buffer label region
///# C Specifications
///A command buffer label region can be closed by calling:
///```c
///// Provided by VK_EXT_debug_utils
///void vkCmdEndDebugUtilsLabelEXT(
///    VkCommandBuffer                             commandBuffer);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// # Description
/// An application  **may**  open a debug label region in one command buffer and
/// close it in another, or otherwise split debug label regions across multiple
/// command buffers or multiple queue submissions.
/// When viewed from the linear series of submissions to a single queue, the
/// calls to [`cmd_begin_debug_utils_label_ext`] and
/// [`cmd_end_debug_utils_label_ext`] **must**  be matched and balanced.There  **can**  be problems
/// reporting command buffer debug labels during the
/// recording process because command buffers  **may**  be recorded out of sequence
/// with the resulting execution order.
/// Since the recording order  **may**  be different, a solitary command buffer  **may**
/// have an inconsistent view of the debug label regions by itself.
/// Therefore, if an issue occurs during the recording of a command buffer, and
/// the environment requires returning debug labels, the implementation  **may**
/// return only those labels it is aware of.
/// This is true even if the implementation is aware of only the debug labels
/// within the command buffer being actively recorded.
/// ## Valid Usage
/// - There  **must**  be an outstanding [`cmd_begin_debug_utils_label_ext`] command prior to the
///   [`cmd_end_debug_utils_label_ext`] on the queue that [`command_buffer`] is submitted to
/// - If [`command_buffer`] is a secondary command buffer, there  **must**  be an outstanding
///   [`cmd_begin_debug_utils_label_ext`] command recorded to [`command_buffer`] that has not
///   previously been ended by a call to [`cmd_end_debug_utils_label_ext`]
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
/// - [`ext_debug_utils`]
/// - [`CommandBuffer`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
pub type FNCmdEndDebugUtilsLabelExt = Option<unsafe extern "system" fn(command_buffer: CommandBuffer)>;
///[vkCmdInsertDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html) - Insert a label into a command buffer
///# C Specifications
///A single debug label can be inserted into a command buffer by calling:
///```c
///// Provided by VK_EXT_debug_utils
///void vkCmdInsertDebugUtilsLabelEXT(
///    VkCommandBuffer                             commandBuffer,
///    const VkDebugUtilsLabelEXT*                 pLabelInfo);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - `pInfo` is a pointer to a [`DebugUtilsLabelEXT`] structure specifying parameters of the label
///   to insert.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_label_info`] **must**  be a valid pointer to a valid [`DebugUtilsLabelEXT`] structure
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
/// - [`ext_debug_utils`]
/// - [`CommandBuffer`]
/// - [`DebugUtilsLabelEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
pub type FNCmdInsertDebugUtilsLabelExt = Option<
    for<'lt> unsafe extern "system" fn(command_buffer: CommandBuffer, p_label_info: *const DebugUtilsLabelEXT<'lt>),
>;
///[VkDebugUtilsMessageSeverityFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html) - Bitmask specifying which severities of events cause a debug messenger callback
///# C Specifications
///Bits which  **can**  be set in
///[`DebugUtilsMessengerCreateInfoEXT::message_severity`], specifying
///event severities which cause a debug messenger to call the callback, are:
///```c
///// Provided by VK_EXT_debug_utils
///typedef enum VkDebugUtilsMessageSeverityFlagBitsEXT {
///    VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT = 0x00000001,
///    VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT = 0x00000010,
///    VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT = 0x00000100,
///    VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT = 0x00001000,
///} VkDebugUtilsMessageSeverityFlagBitsEXT;
///```
/// # Description
/// - [`VERBOSE`] specifies the most verbose output indicating all diagnostic messages from the
///   Vulkan loader, layers, and drivers should be captured.
/// - [`INFO`] specifies an informational message such as resource details that may be handy when
///   debugging an application.
/// - [`WARNING`] specifies use of Vulkan that  **may**  expose an app bug. Such cases may not be
///   immediately harmful, such as a fragment shader outputting to a location with no attachment.
///   Other cases  **may**  point to behavior that is almost certainly bad when unintended such as
///   using an image whose memory has not been filled. In general if you see a warning but you know
///   that the behavior is intended/desired, then simply ignore the warning.
/// - [`ERROR`] specifies that the application has violated a valid usage condition of the
///   specification.
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsMessageSeverityFlagsEXT`]
/// - [`submit_debug_utils_message_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugUtilsMessageSeverityFlagBitsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DebugUtilsMessageSeverityFlagBitsEXT(u32);
impl const Default for DebugUtilsMessageSeverityFlagBitsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl DebugUtilsMessageSeverityFlagBitsEXT {
    ///[`VERBOSE`] specifies the most
    ///verbose output indicating all diagnostic messages from the Vulkan
    ///loader, layers, and drivers should be captured.
    pub const VERBOSE: Self = Self(1);
    ///[`INFO`] specifies an
    ///informational message such as resource details that may be handy when
    ///debugging an application.
    pub const INFO: Self = Self(16);
    ///[`WARNING`] specifies use of
    ///Vulkan that  **may**  expose an app bug.
    ///Such cases may not be immediately harmful, such as a fragment shader
    ///outputting to a location with no attachment.
    ///Other cases  **may**  point to behavior that is almost certainly bad when
    ///unintended such as using an image whose memory has not been filled.
    ///In general if you see a warning but you know that the behavior is
    ///intended/desired, then simply ignore the warning.
    pub const WARNING: Self = Self(256);
    ///[`ERROR`] specifies that the
    ///application has violated a valid usage condition of the specification.
    pub const ERROR: Self = Self(4096);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for DebugUtilsMessageSeverityFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DebugUtilsMessageSeverityFlagBitsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DebugUtilsMessageSeverityFlagBitsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        DebugUtilsMessageSeverityFlagBitsEXT::VERBOSE => f.write_str("VERBOSE")?,
                        DebugUtilsMessageSeverityFlagBitsEXT::INFO => f.write_str("INFO")?,
                        DebugUtilsMessageSeverityFlagBitsEXT::WARNING => f.write_str("WARNING")?,
                        DebugUtilsMessageSeverityFlagBitsEXT::ERROR => f.write_str("ERROR")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DebugUtilsMessageSeverityFlagBitsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkDebugUtilsMessageTypeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html) - Bitmask specifying which types of events cause a debug messenger callback
///# C Specifications
///Bits which  **can**  be set in
///[`DebugUtilsMessengerCreateInfoEXT::message_type`], specifying
///event types which cause a debug messenger to call the callback, are:
///```c
///// Provided by VK_EXT_debug_utils
///typedef enum VkDebugUtilsMessageTypeFlagBitsEXT {
///    VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT = 0x00000001,
///    VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT = 0x00000002,
///    VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT = 0x00000004,
///} VkDebugUtilsMessageTypeFlagBitsEXT;
///```
/// # Description
/// - [`GENERAL`] specifies that some general event has occurred. This is typically a
///   non-specification, non-performance event.
/// - [`VALIDATION`] specifies that something has occurred during validation against the Vulkan
///   specification that may indicate invalid behavior.
/// - [`PERFORMANCE`] specifies a potentially non-optimal use of Vulkan, e.g. using
///   [`cmd_clear_color_image`] when setting [`AttachmentDescription::load_op`] to
///   `VK_ATTACHMENT_LOAD_OP_CLEAR` would have worked.
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsMessageTypeFlagsEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugUtilsMessageTypeFlagBitsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DebugUtilsMessageTypeFlagBitsEXT(u32);
impl const Default for DebugUtilsMessageTypeFlagBitsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl DebugUtilsMessageTypeFlagBitsEXT {
    ///[`GENERAL`] specifies that some
    ///general event has occurred.
    ///This is typically a non-specification, non-performance event.
    pub const GENERAL: Self = Self(1);
    ///[`VALIDATION`] specifies that
    ///something has occurred during validation against the Vulkan
    ///specification that may indicate invalid behavior.
    pub const VALIDATION: Self = Self(2);
    ///[`PERFORMANCE`] specifies a
    ///potentially non-optimal use of Vulkan, e.g. using
    ///[`cmd_clear_color_image`] when setting
    ///[`AttachmentDescription`]::`loadOp` to
    ///`VK_ATTACHMENT_LOAD_OP_CLEAR` would have worked.
    pub const PERFORMANCE: Self = Self(4);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for DebugUtilsMessageTypeFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DebugUtilsMessageTypeFlagBitsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DebugUtilsMessageTypeFlagBitsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        DebugUtilsMessageTypeFlagBitsEXT::GENERAL => f.write_str("GENERAL")?,
                        DebugUtilsMessageTypeFlagBitsEXT::VALIDATION => f.write_str("VALIDATION")?,
                        DebugUtilsMessageTypeFlagBitsEXT::PERFORMANCE => f.write_str("PERFORMANCE")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DebugUtilsMessageTypeFlagBitsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkDebugUtilsMessageSeverityFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html) - Bitmask specifying which severities of events cause a debug messenger callback
///# C Specifications
///Bits which  **can**  be set in
///[`DebugUtilsMessengerCreateInfoEXT::message_severity`], specifying
///event severities which cause a debug messenger to call the callback, are:
///```c
///// Provided by VK_EXT_debug_utils
///typedef enum VkDebugUtilsMessageSeverityFlagBitsEXT {
///    VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT = 0x00000001,
///    VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT = 0x00000010,
///    VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT = 0x00000100,
///    VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT = 0x00001000,
///} VkDebugUtilsMessageSeverityFlagBitsEXT;
///```
/// # Description
/// - [`VERBOSE`] specifies the most verbose output indicating all diagnostic messages from the
///   Vulkan loader, layers, and drivers should be captured.
/// - [`INFO`] specifies an informational message such as resource details that may be handy when
///   debugging an application.
/// - [`WARNING`] specifies use of Vulkan that  **may**  expose an app bug. Such cases may not be
///   immediately harmful, such as a fragment shader outputting to a location with no attachment.
///   Other cases  **may**  point to behavior that is almost certainly bad when unintended such as
///   using an image whose memory has not been filled. In general if you see a warning but you know
///   that the behavior is intended/desired, then simply ignore the warning.
/// - [`ERROR`] specifies that the application has violated a valid usage condition of the
///   specification.
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsMessageSeverityFlagsEXT`]
/// - [`submit_debug_utils_message_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugUtilsMessageSeverityFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DebugUtilsMessageSeverityFlagsEXT(u32);
impl const Default for DebugUtilsMessageSeverityFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<DebugUtilsMessageSeverityFlagBitsEXT> for DebugUtilsMessageSeverityFlagsEXT {
    fn from(from: DebugUtilsMessageSeverityFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl DebugUtilsMessageSeverityFlagsEXT {
    ///[`VERBOSE`] specifies the most
    ///verbose output indicating all diagnostic messages from the Vulkan
    ///loader, layers, and drivers should be captured.
    pub const VERBOSE: Self = Self(1);
    ///[`INFO`] specifies an
    ///informational message such as resource details that may be handy when
    ///debugging an application.
    pub const INFO: Self = Self(16);
    ///[`WARNING`] specifies use of
    ///Vulkan that  **may**  expose an app bug.
    ///Such cases may not be immediately harmful, such as a fragment shader
    ///outputting to a location with no attachment.
    ///Other cases  **may**  point to behavior that is almost certainly bad when
    ///unintended such as using an image whose memory has not been filled.
    ///In general if you see a warning but you know that the behavior is
    ///intended/desired, then simply ignore the warning.
    pub const WARNING: Self = Self(256);
    ///[`ERROR`] specifies that the
    ///application has violated a valid usage condition of the specification.
    pub const ERROR: Self = Self(4096);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::VERBOSE;
        }
        {
            all |= Self::INFO;
        }
        {
            all |= Self::WARNING;
        }
        {
            all |= Self::ERROR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DebugUtilsMessageSeverityFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DebugUtilsMessageSeverityFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DebugUtilsMessageSeverityFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DebugUtilsMessageSeverityFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DebugUtilsMessageSeverityFlagsEXT> for DebugUtilsMessageSeverityFlagsEXT {
    fn extend<T: IntoIterator<Item = DebugUtilsMessageSeverityFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<DebugUtilsMessageSeverityFlagBitsEXT> for DebugUtilsMessageSeverityFlagsEXT {
    fn extend<T: IntoIterator<Item = DebugUtilsMessageSeverityFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<DebugUtilsMessageSeverityFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<DebugUtilsMessageSeverityFlagsEXT> for DebugUtilsMessageSeverityFlagsEXT {
    fn from_iter<T: IntoIterator<Item = DebugUtilsMessageSeverityFlagsEXT>>(
        iterator: T,
    ) -> DebugUtilsMessageSeverityFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<DebugUtilsMessageSeverityFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<DebugUtilsMessageSeverityFlagBitsEXT> for DebugUtilsMessageSeverityFlagsEXT {
    fn from_iter<T: IntoIterator<Item = DebugUtilsMessageSeverityFlagBitsEXT>>(
        iterator: T,
    ) -> DebugUtilsMessageSeverityFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<DebugUtilsMessageSeverityFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DebugUtilsMessageSeverityFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DebugUtilsMessageSeverityFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DebugUtilsMessageSeverityFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(DebugUtilsMessageSeverityFlagsEXT::VERBOSE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VERBOSE))?;
                    }
                    if self.0.contains(DebugUtilsMessageSeverityFlagsEXT::INFO) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(INFO))?;
                    }
                    if self.0.contains(DebugUtilsMessageSeverityFlagsEXT::WARNING) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(WARNING))?;
                    }
                    if self.0.contains(DebugUtilsMessageSeverityFlagsEXT::ERROR) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(ERROR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DebugUtilsMessageSeverityFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkDebugUtilsMessageTypeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html) - Bitmask specifying which types of events cause a debug messenger callback
///# C Specifications
///Bits which  **can**  be set in
///[`DebugUtilsMessengerCreateInfoEXT::message_type`], specifying
///event types which cause a debug messenger to call the callback, are:
///```c
///// Provided by VK_EXT_debug_utils
///typedef enum VkDebugUtilsMessageTypeFlagBitsEXT {
///    VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT = 0x00000001,
///    VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT = 0x00000002,
///    VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT = 0x00000004,
///} VkDebugUtilsMessageTypeFlagBitsEXT;
///```
/// # Description
/// - [`GENERAL`] specifies that some general event has occurred. This is typically a
///   non-specification, non-performance event.
/// - [`VALIDATION`] specifies that something has occurred during validation against the Vulkan
///   specification that may indicate invalid behavior.
/// - [`PERFORMANCE`] specifies a potentially non-optimal use of Vulkan, e.g. using
///   [`cmd_clear_color_image`] when setting [`AttachmentDescription::load_op`] to
///   `VK_ATTACHMENT_LOAD_OP_CLEAR` would have worked.
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsMessageTypeFlagsEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugUtilsMessageTypeFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DebugUtilsMessageTypeFlagsEXT(u32);
impl const Default for DebugUtilsMessageTypeFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<DebugUtilsMessageTypeFlagBitsEXT> for DebugUtilsMessageTypeFlagsEXT {
    fn from(from: DebugUtilsMessageTypeFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl DebugUtilsMessageTypeFlagsEXT {
    ///[`GENERAL`] specifies that some
    ///general event has occurred.
    ///This is typically a non-specification, non-performance event.
    pub const GENERAL: Self = Self(1);
    ///[`VALIDATION`] specifies that
    ///something has occurred during validation against the Vulkan
    ///specification that may indicate invalid behavior.
    pub const VALIDATION: Self = Self(2);
    ///[`PERFORMANCE`] specifies a
    ///potentially non-optimal use of Vulkan, e.g. using
    ///[`cmd_clear_color_image`] when setting
    ///[`AttachmentDescription`]::`loadOp` to
    ///`VK_ATTACHMENT_LOAD_OP_CLEAR` would have worked.
    pub const PERFORMANCE: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::GENERAL;
        }
        {
            all |= Self::VALIDATION;
        }
        {
            all |= Self::PERFORMANCE;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DebugUtilsMessageTypeFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DebugUtilsMessageTypeFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DebugUtilsMessageTypeFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DebugUtilsMessageTypeFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DebugUtilsMessageTypeFlagsEXT> for DebugUtilsMessageTypeFlagsEXT {
    fn extend<T: IntoIterator<Item = DebugUtilsMessageTypeFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<DebugUtilsMessageTypeFlagBitsEXT> for DebugUtilsMessageTypeFlagsEXT {
    fn extend<T: IntoIterator<Item = DebugUtilsMessageTypeFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<DebugUtilsMessageTypeFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<DebugUtilsMessageTypeFlagsEXT> for DebugUtilsMessageTypeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = DebugUtilsMessageTypeFlagsEXT>>(iterator: T) -> DebugUtilsMessageTypeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<DebugUtilsMessageTypeFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<DebugUtilsMessageTypeFlagBitsEXT> for DebugUtilsMessageTypeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = DebugUtilsMessageTypeFlagBitsEXT>>(
        iterator: T,
    ) -> DebugUtilsMessageTypeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<DebugUtilsMessageTypeFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DebugUtilsMessageTypeFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DebugUtilsMessageTypeFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DebugUtilsMessageTypeFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(DebugUtilsMessageTypeFlagsEXT::GENERAL) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(GENERAL))?;
                    }
                    if self.0.contains(DebugUtilsMessageTypeFlagsEXT::VALIDATION) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(VALIDATION))?;
                    }
                    if self.0.contains(DebugUtilsMessageTypeFlagsEXT::PERFORMANCE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(PERFORMANCE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DebugUtilsMessageTypeFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkDebugUtilsMessengerCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateFlagsEXT.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_EXT_debug_utils
///typedef VkFlags VkDebugUtilsMessengerCreateFlagsEXT;
///```
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsMessengerCreateInfoEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DebugUtilsMessengerCreateFlagsEXT(u32);
impl const Default for DebugUtilsMessengerCreateFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DebugUtilsMessengerCreateFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(DebugUtilsMessengerCreateFlagsEXT))
            .field(&self.0)
            .finish()
    }
}
///[VkDebugUtilsMessengerCallbackDataFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataFlagsEXT.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_EXT_debug_utils
///typedef VkFlags VkDebugUtilsMessengerCallbackDataFlagsEXT;
///```
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsMessengerCallbackDataEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DebugUtilsMessengerCallbackDataFlagsEXT(u32);
impl const Default for DebugUtilsMessengerCallbackDataFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DebugUtilsMessengerCallbackDataFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(DebugUtilsMessengerCallbackDataFlagsEXT))
            .field(&self.0)
            .finish()
    }
}
///[VkDebugUtilsObjectNameInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html) - Specify parameters of a name to give to an object
///# C Specifications
///The [`DebugUtilsObjectNameInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_debug_utils
///typedef struct VkDebugUtilsObjectNameInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkObjectType       objectType;
///    uint64_t           objectHandle;
///    const char*        pObjectName;
///} VkDebugUtilsObjectNameInfoEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`object_type`] is a [`ObjectType`] specifying the type of the object to be named.
/// - [`object_handle`] is the object to be named.
/// - [`object_name`] is either `NULL` or a null-terminated UTF-8 string specifying the name to
///   apply to [`object_handle`].
/// # Description
/// Applications **may**  change the name associated with an object simply by
/// calling [`set_debug_utils_object_name_ext`] again with a new string.
/// If [`object_name`] is either `NULL` or an empty string, then any
/// previously set name is removed.
/// ## Valid Usage
/// - If [`object_type`] is `VK_OBJECT_TYPE_UNKNOWN`, [`object_handle`] **must**  not be
///   [`crate::Handle::null`]
/// -    If [`object_type`] is not `VK_OBJECT_TYPE_UNKNOWN`, [`object_handle`] **must**  be [`crate::Handle::null`] or a valid Vulkan handle of the type associated with [`object_type`] as defined in the [[`ObjectType`] and Vulkan Handle Relationship](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-types) table
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`object_type`] **must**  be a valid [`ObjectType`] value
/// - If [`object_name`] is not `NULL`, [`object_name`] **must**  be a null-terminated UTF-8 string
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsMessengerCallbackDataEXT`]
/// - [`ObjectType`]
/// - [`StructureType`]
/// - [`set_debug_utils_object_name_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugUtilsObjectNameInfoEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct DebugUtilsObjectNameInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`object_type`] is a [`ObjectType`] specifying the type of the
    ///object to be named.
    pub object_type: ObjectType,
    ///[`object_handle`] is the object to be named.
    pub object_handle: u64,
    ///[`object_name`] is either `NULL` or a null-terminated UTF-8 string
    ///specifying the name to apply to [`object_handle`].
    pub object_name: *const c_char,
}
impl<'lt> Default for DebugUtilsObjectNameInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object_handle: 0,
            object_name: std::ptr::null(),
        }
    }
}
impl<'lt> DebugUtilsObjectNameInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::object_name`]
    pub fn object_name_raw(&self) -> *const c_char {
        self.object_name
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::object_name`]
    pub fn set_object_name_raw(&mut self, value: *const c_char) -> &mut Self {
        self.object_name = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::object_name`]
    pub fn with_object_name_raw(mut self, value: *const c_char) -> Self {
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
    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }
    ///Gets the value of [`Self::object_handle`]
    pub fn object_handle(&self) -> u64 {
        self.object_handle
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
    pub fn object_type_mut(&mut self) -> &mut ObjectType {
        &mut self.object_type
    }
    ///Gets a mutable reference to the value of [`Self::object_handle`]
    pub fn object_handle_mut(&mut self) -> &mut u64 {
        &mut self.object_handle
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::object_type`]
    pub fn set_object_type(&mut self, value: crate::vulkan1_0::ObjectType) -> &mut Self {
        self.object_type = value;
        self
    }
    ///Sets the value of [`Self::object_handle`]
    pub fn set_object_handle(&mut self, value: u64) -> &mut Self {
        self.object_handle = value;
        self
    }
    ///Sets the value of [`Self::object_name`]
    pub fn set_object_name(&mut self, value: *const std::os::raw::c_char) -> &mut Self {
        self.object_name = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::object_type`]
    pub fn with_object_type(mut self, value: crate::vulkan1_0::ObjectType) -> Self {
        self.object_type = value;
        self
    }
    ///Sets the value of [`Self::object_handle`]
    pub fn with_object_handle(mut self, value: u64) -> Self {
        self.object_handle = value;
        self
    }
    ///Sets the value of [`Self::object_name`]
    pub fn with_object_name(mut self, value: *const std::os::raw::c_char) -> Self {
        self.object_name = value;
        self
    }
}
///[VkDebugUtilsObjectTagInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectTagInfoEXT.html) - Specify parameters of a tag to attach to an object
///# C Specifications
///The [`DebugUtilsObjectTagInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_debug_utils
///typedef struct VkDebugUtilsObjectTagInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkObjectType       objectType;
///    uint64_t           objectHandle;
///    uint64_t           tagName;
///    size_t             tagSize;
///    const void*        pTag;
///} VkDebugUtilsObjectTagInfoEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`object_type`] is a [`ObjectType`] specifying the type of the object to be named.
/// - [`object_handle`] is the object to be tagged.
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
/// - [`object_type`] **must**  not be `VK_OBJECT_TYPE_UNKNOWN`
/// -  [`object_handle`] **must**  be a valid Vulkan handle of the type associated with [`object_type`] as defined in the [[`ObjectType`] and Vulkan Handle Relationship](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-types) table
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`object_type`] **must**  be a valid [`ObjectType`] value
/// - [`tag`] **must**  be a valid pointer to an array of [`tag_size`] bytes
/// - [`tag_size`] **must**  be greater than `0`
/// # Related
/// - [`ext_debug_utils`]
/// - [`ObjectType`]
/// - [`StructureType`]
/// - [`set_debug_utils_object_tag_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugUtilsObjectTagInfoEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct DebugUtilsObjectTagInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`object_type`] is a [`ObjectType`] specifying the type of the
    ///object to be named.
    pub object_type: ObjectType,
    ///[`object_handle`] is the object to be tagged.
    pub object_handle: u64,
    ///[`tag_name`] is a numerical identifier of the tag.
    pub tag_name: u64,
    ///[`tag_size`] is the number of bytes of data to attach to the object.
    pub tag_size: usize,
    ///[`tag`] is a pointer to an array of [`tag_size`] bytes containing
    ///the data to be associated with the object.
    pub tag: *const c_void,
}
impl<'lt> Default for DebugUtilsObjectTagInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DEBUG_UTILS_OBJECT_TAG_INFO_EXT,
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object_handle: 0,
            tag_name: 0,
            tag_size: 0,
            tag: std::ptr::null(),
        }
    }
}
impl<'lt> DebugUtilsObjectTagInfoEXT<'lt> {
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
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::tag`]
    pub fn with_tag_raw(mut self, value: *const c_void) -> Self {
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
    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }
    ///Gets the value of [`Self::object_handle`]
    pub fn object_handle(&self) -> u64 {
        self.object_handle
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
    pub fn object_type_mut(&mut self) -> &mut ObjectType {
        &mut self.object_type
    }
    ///Gets a mutable reference to the value of [`Self::object_handle`]
    pub fn object_handle_mut(&mut self) -> &mut u64 {
        &mut self.object_handle
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
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::object_type`]
    pub fn set_object_type(&mut self, value: crate::vulkan1_0::ObjectType) -> &mut Self {
        self.object_type = value;
        self
    }
    ///Sets the value of [`Self::object_handle`]
    pub fn set_object_handle(&mut self, value: u64) -> &mut Self {
        self.object_handle = value;
        self
    }
    ///Sets the value of [`Self::tag_name`]
    pub fn set_tag_name(&mut self, value: u64) -> &mut Self {
        self.tag_name = value;
        self
    }
    ///Sets the value of [`Self::tag_size`]
    pub fn set_tag_size(&mut self, value: usize) -> &mut Self {
        self.tag_size = value;
        self
    }
    ///Sets the value of [`Self::tag`]
    pub fn set_tag(&mut self, value: &'lt [std::ffi::c_void]) -> &mut Self {
        let len_ = value.len() as usize;
        let len_ = len_;
        self.tag = value.as_ptr();
        self.tag_size = len_;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::object_type`]
    pub fn with_object_type(mut self, value: crate::vulkan1_0::ObjectType) -> Self {
        self.object_type = value;
        self
    }
    ///Sets the value of [`Self::object_handle`]
    pub fn with_object_handle(mut self, value: u64) -> Self {
        self.object_handle = value;
        self
    }
    ///Sets the value of [`Self::tag_name`]
    pub fn with_tag_name(mut self, value: u64) -> Self {
        self.tag_name = value;
        self
    }
    ///Sets the value of [`Self::tag_size`]
    pub fn with_tag_size(mut self, value: usize) -> Self {
        self.tag_size = value;
        self
    }
    ///Sets the value of [`Self::tag`]
    pub fn with_tag(mut self, value: &'lt [std::ffi::c_void]) -> Self {
        let len_ = value.len() as usize;
        let len_ = len_;
        self.tag = value.as_ptr();
        self.tag_size = len_;
        self
    }
}
///[VkDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsLabelEXT.html) - Specify parameters of a label region
///# C Specifications
///The [`DebugUtilsLabelEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_debug_utils
///typedef struct VkDebugUtilsLabelEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    const char*        pLabelName;
///    float              color[4];
///} VkDebugUtilsLabelEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`label_name`] is a pointer to a null-terminated UTF-8 string containing the name of the
///   label.
/// - [`color`] is an optional RGBA color value that can be associated with the label. A particular
///   implementation  **may**  choose to ignore this color value. The values contain RGBA values in
///   order, in the range 0.0 to 1.0. If all elements in [`color`] are set to 0.0 then it is
///   ignored.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`label_name`] **must**  be a null-terminated UTF-8 string
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsMessengerCallbackDataEXT`]
/// - [`StructureType`]
/// - [`cmd_begin_debug_utils_label_ext`]
/// - [`cmd_insert_debug_utils_label_ext`]
/// - [`queue_begin_debug_utils_label_ext`]
/// - [`queue_insert_debug_utils_label_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugUtilsLabelEXT")]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct DebugUtilsLabelEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`label_name`] is a pointer to a null-terminated UTF-8 string
    ///containing the name of the label.
    pub label_name: *const c_char,
    ///[`color`] is an optional RGBA color value that can be associated with
    ///the label.
    ///A particular implementation  **may**  choose to ignore this color value.
    ///The values contain RGBA values in order, in the range 0.0 to 1.0.
    ///If all elements in [`color`] are set to 0.0 then it is ignored.
    pub color: [f32; 4 as usize],
}
impl<'lt> Default for DebugUtilsLabelEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DEBUG_UTILS_LABEL_EXT,
            p_next: std::ptr::null(),
            label_name: std::ptr::null(),
            color: [0.0; 4 as usize],
        }
    }
}
impl<'lt> DebugUtilsLabelEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::label_name`]
    pub fn label_name_raw(&self) -> *const c_char {
        self.label_name
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::label_name`]
    pub fn set_label_name_raw(&mut self, value: *const c_char) -> &mut Self {
        self.label_name = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::label_name`]
    pub fn with_label_name_raw(mut self, value: *const c_char) -> Self {
        self.label_name = value;
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
    ///Gets the value of [`Self::label_name`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn label_name(&self) -> &'lt CStr {
        CStr::from_ptr(self.label_name)
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
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::label_name`]
    pub fn set_label_name(&mut self, value: *const std::os::raw::c_char) -> &mut Self {
        self.label_name = value;
        self
    }
    ///Sets the value of [`Self::color`]
    pub fn set_color(&mut self, value: [f32; 4 as usize]) -> &mut Self {
        self.color = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::label_name`]
    pub fn with_label_name(mut self, value: *const std::os::raw::c_char) -> Self {
        self.label_name = value;
        self
    }
    ///Sets the value of [`Self::color`]
    pub fn with_color(mut self, value: [f32; 4 as usize]) -> Self {
        self.color = value;
        self
    }
}
///[VkDebugUtilsMessengerCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html) - Structure specifying parameters of a newly created debug messenger
///# C Specifications
///The definition of [`DebugUtilsMessengerCreateInfoEXT`] is:
///```c
///// Provided by VK_EXT_debug_utils
///typedef struct VkDebugUtilsMessengerCreateInfoEXT {
///    VkStructureType                         sType;
///    const void*                             pNext;
///    VkDebugUtilsMessengerCreateFlagsEXT     flags;
///    VkDebugUtilsMessageSeverityFlagsEXT     messageSeverity;
///    VkDebugUtilsMessageTypeFlagsEXT         messageType;
///    PFN_vkDebugUtilsMessengerCallbackEXT    pfnUserCallback;
///    void*                                   pUserData;
///} VkDebugUtilsMessengerCreateInfoEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is `0` and is reserved for future use.
/// - [`message_severity`] is a bitmask of [`DebugUtilsMessageSeverityFlagBitsEXT`] specifying which
///   severity of event(s) will cause this callback to be called.
/// - [`message_type`] is a bitmask of [`DebugUtilsMessageTypeFlagBitsEXT`] specifying which type of
///   event(s) will cause this callback to be called.
/// - [`pfn_user_callback`] is the application callback function to call.
/// - [`user_data`] is user data to be passed to the callback.
/// # Description
/// For each [`DebugUtilsMessengerEXT`] that is created the
/// [`DebugUtilsMessengerCreateInfoEXT`]::[`message_severity`] and
/// [`DebugUtilsMessengerCreateInfoEXT`]::[`message_type`] determine when
/// that [`DebugUtilsMessengerCreateInfoEXT`]::[`pfn_user_callback`] is
/// called.
/// The process to determine if the user’s [`pfn_user_callback`] is triggered
/// when an event occurs is as follows:
/// 0. The implementation will perform a bitwise AND of the event’s
/// [`DebugUtilsMessageSeverityFlagBitsEXT`] with the [`message_severity`] provided during creation
/// of the [`DebugUtilsMessengerEXT`] object.  0. If the value is 0, the message is skipped.
/// 2. The implementation will perform bitwise AND of the event’s
/// [`DebugUtilsMessageTypeFlagBitsEXT`] with the [`message_type`] provided during the creation of
/// the [`DebugUtilsMessengerEXT`] object.  0. If the value is 0, the message is skipped.
/// 4. The callback will trigger a debug message for the current event
/// The callback will come directly from the component that detected the event,
/// unless some other layer intercepts the calls for its own purposes (filter
/// them in a different way, log to a system error log, etc.).An application  **can**  receive
/// multiple callbacks if multiple
/// [`DebugUtilsMessengerEXT`] objects are created.
/// A callback will always be executed in the same thread as the originating
/// Vulkan call.A callback  **can**  be called from multiple threads simultaneously (if the
/// application is making Vulkan calls from multiple threads).
/// ## Valid Usage
/// - [`pfn_user_callback`] **must**  be a valid [`PFNDebugUtilsMessengerCallbackEXT`]
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT`
/// - [`flags`] **must**  be `0`
/// - [`message_severity`] **must**  be a valid combination of
///   [`DebugUtilsMessageSeverityFlagBitsEXT`] values
/// - [`message_severity`] **must**  not be `0`
/// - [`message_type`] **must**  be a valid combination of [`DebugUtilsMessageTypeFlagBitsEXT`]
///   values
/// - [`message_type`] **must**  not be `0`
/// - [`pfn_user_callback`] **must**  be a valid [`PFNDebugUtilsMessengerCallbackEXT`] value
/// # Related
/// - [`PFNDebugUtilsMessengerCallbackEXT`]
/// - [`ext_debug_utils`]
/// - [`DebugUtilsMessageSeverityFlagsEXT`]
/// - [`DebugUtilsMessageTypeFlagsEXT`]
/// - [`DebugUtilsMessengerCreateFlagsEXT`]
/// - [`StructureType`]
/// - [`create_debug_utils_messenger_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugUtilsMessengerCreateInfoEXT")]
#[derive(Clone)]
#[repr(C)]
pub struct DebugUtilsMessengerCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is `0` and is reserved for future use.
    pub flags: DebugUtilsMessengerCreateFlagsEXT,
    ///[`message_severity`] is a bitmask of
    ///[`DebugUtilsMessageSeverityFlagBitsEXT`] specifying which severity
    ///of event(s) will cause this callback to be called.
    pub message_severity: DebugUtilsMessageSeverityFlagsEXT,
    ///[`message_type`] is a bitmask of
    ///[`DebugUtilsMessageTypeFlagBitsEXT`] specifying which type of
    ///event(s) will cause this callback to be called.
    pub message_type: DebugUtilsMessageTypeFlagsEXT,
    ///[`pfn_user_callback`] is the application callback function to call.
    pub pfn_user_callback: PFNDebugUtilsMessengerCallbackEXT,
    ///[`user_data`] is user data to be passed to the callback.
    pub user_data: *mut c_void,
}
impl<'lt> Default for DebugUtilsMessengerCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            message_severity: Default::default(),
            message_type: Default::default(),
            pfn_user_callback: None,
            user_data: std::ptr::null_mut(),
        }
    }
}
impl<'lt> DebugUtilsMessengerCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::user_data`]
    pub fn user_data_raw(&self) -> *mut c_void {
        self.user_data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::user_data`]
    pub fn set_user_data_raw(&mut self, value: *mut c_void) -> &mut Self {
        self.user_data = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::user_data`]
    pub fn with_user_data_raw(mut self, value: *mut c_void) -> Self {
        self.user_data = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> DebugUtilsMessengerCreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::message_severity`]
    pub fn message_severity(&self) -> DebugUtilsMessageSeverityFlagsEXT {
        self.message_severity
    }
    ///Gets the value of [`Self::message_type`]
    pub fn message_type(&self) -> DebugUtilsMessageTypeFlagsEXT {
        self.message_type
    }
    ///Gets the value of [`Self::pfn_user_callback`]
    pub fn pfn_user_callback(&self) -> PFNDebugUtilsMessengerCallbackEXT {
        self.pfn_user_callback
    }
    ///Gets the value of [`Self::user_data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn user_data(&self) -> &c_void {
        &*self.user_data
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DebugUtilsMessengerCreateFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::message_severity`]
    pub fn message_severity_mut(&mut self) -> &mut DebugUtilsMessageSeverityFlagsEXT {
        &mut self.message_severity
    }
    ///Gets a mutable reference to the value of [`Self::message_type`]
    pub fn message_type_mut(&mut self) -> &mut DebugUtilsMessageTypeFlagsEXT {
        &mut self.message_type
    }
    ///Gets a mutable reference to the value of [`Self::pfn_user_callback`]
    pub fn pfn_user_callback_mut(&mut self) -> &mut PFNDebugUtilsMessengerCallbackEXT {
        &mut self.pfn_user_callback
    }
    ///Gets a mutable reference to the value of [`Self::user_data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn user_data_mut(&mut self) -> &mut c_void {
        &mut *self.user_data
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::message_severity`]
    pub fn set_message_severity(
        &mut self,
        value: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT,
    ) -> &mut Self {
        self.message_severity = value;
        self
    }
    ///Sets the value of [`Self::message_type`]
    pub fn set_message_type(
        &mut self,
        value: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
    ) -> &mut Self {
        self.message_type = value;
        self
    }
    ///Sets the value of [`Self::pfn_user_callback`]
    pub fn set_pfn_user_callback(
        &mut self,
        value: crate::extensions::ext_debug_utils::PFNDebugUtilsMessengerCallbackEXT,
    ) -> &mut Self {
        self.pfn_user_callback = value;
        self
    }
    ///Sets the value of [`Self::user_data`]
    pub fn set_user_data(&mut self, value: &'lt mut std::ffi::c_void) -> &mut Self {
        self.user_data = value as *mut _;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(mut self, value: crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateFlagsEXT) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::message_severity`]
    pub fn with_message_severity(
        mut self,
        value: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT,
    ) -> Self {
        self.message_severity = value;
        self
    }
    ///Sets the value of [`Self::message_type`]
    pub fn with_message_type(
        mut self,
        value: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
    ) -> Self {
        self.message_type = value;
        self
    }
    ///Sets the value of [`Self::pfn_user_callback`]
    pub fn with_pfn_user_callback(
        mut self,
        value: crate::extensions::ext_debug_utils::PFNDebugUtilsMessengerCallbackEXT,
    ) -> Self {
        self.pfn_user_callback = value;
        self
    }
    ///Sets the value of [`Self::user_data`]
    pub fn with_user_data(mut self, value: &'lt mut std::ffi::c_void) -> Self {
        self.user_data = value as *mut _;
        self
    }
}
///[VkDebugUtilsMessengerCallbackDataEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html) - Structure specifying parameters returned to the callback
///# C Specifications
///The definition of [`DebugUtilsMessengerCallbackDataEXT`] is:
///```c
///// Provided by VK_EXT_debug_utils
///typedef struct VkDebugUtilsMessengerCallbackDataEXT {
///    VkStructureType                              sType;
///    const void*                                  pNext;
///    VkDebugUtilsMessengerCallbackDataFlagsEXT    flags;
///    const char*                                  pMessageIdName;
///    int32_t                                      messageIdNumber;
///    const char*                                  pMessage;
///    uint32_t                                     queueLabelCount;
///    const VkDebugUtilsLabelEXT*                  pQueueLabels;
///    uint32_t                                     cmdBufLabelCount;
///    const VkDebugUtilsLabelEXT*                  pCmdBufLabels;
///    uint32_t                                     objectCount;
///    const VkDebugUtilsObjectNameInfoEXT*         pObjects;
///} VkDebugUtilsMessengerCallbackDataEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is `0` and is reserved for future use.
/// - [`message_id_name`] is a null-terminated string that identifies the particular message ID that
///   is associated with the provided message. If the message corresponds to a validation layer
///   message, then this string may contain the portion of the Vulkan specification that is believed
///   to have been violated.
/// - [`message_id_number`] is the ID number of the triggering message. If the message corresponds
///   to a validation layer message, then this number is related to the internal number associated
///   with the message being triggered.
/// - [`message`] is a null-terminated string detailing the trigger conditions.
/// - [`queue_label_count`] is a count of items contained in the [`queue_labels`] array.
/// - [`queue_labels`] is `NULL` or a pointer to an array of [`DebugUtilsLabelEXT`] active in the current [`Queue`] at the time the callback was triggered. Refer to [Queue Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-queue-labels) for more information.
/// - [`cmd_buf_label_count`] is a count of items contained in the [`cmd_buf_labels`] array.
/// - [`cmd_buf_labels`] is `NULL` or a pointer to an array of [`DebugUtilsLabelEXT`] active in the current [`CommandBuffer`] at the time the callback was triggered. Refer to [Command Buffer Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-command-buffer-labels) for more information.
/// - [`object_count`] is a count of items contained in the [`objects`] array.
/// - [`objects`] is a pointer to an array of [`DebugUtilsObjectNameInfoEXT`] objects related to the
///   detected issue. The array is roughly in order or importance, but the 0th element is always
///   guaranteed to be the most important object for this message.
/// # Description
/// Since adding queue and command buffer labels behaves like pushing and
/// popping onto a stack, the order of both [`queue_labels`] and
/// [`cmd_buf_labels`] is based on the order the labels were defined.
/// The result is that the first label in either [`queue_labels`] or
/// [`cmd_buf_labels`] will be the first defined (and therefore the oldest)
/// while the last label in each list will be the most recent.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// - If [`message_id_name`] is not `NULL`, [`message_id_name`] **must**  be a null-terminated UTF-8
///   string
/// - [`message`] **must**  be a null-terminated UTF-8 string
/// - If [`queue_label_count`] is not `0`, [`queue_labels`] **must**  be a valid pointer to an array
///   of [`queue_label_count`] valid [`DebugUtilsLabelEXT`] structures
/// - If [`cmd_buf_label_count`] is not `0`, [`cmd_buf_labels`] **must**  be a valid pointer to an
///   array of [`cmd_buf_label_count`] valid [`DebugUtilsLabelEXT`] structures
/// - If [`object_count`] is not `0`, [`objects`] **must**  be a valid pointer to an array of
///   [`object_count`] valid [`DebugUtilsObjectNameInfoEXT`] structures
/// # Related
/// - [`ext_debug_utils`]
/// - [`DebugUtilsLabelEXT`]
/// - [`DebugUtilsMessengerCallbackDataFlagsEXT`]
/// - [`DebugUtilsObjectNameInfoEXT`]
/// - [`StructureType`]
/// - [`submit_debug_utils_message_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugUtilsMessengerCallbackDataEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct DebugUtilsMessengerCallbackDataEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is `0` and is reserved for future use.
    pub flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    ///[`message_id_name`] is a null-terminated string that identifies the
    ///particular message ID that is associated with the provided message.
    ///If the message corresponds to a validation layer message, then this
    ///string may contain the portion of the Vulkan specification that is
    ///believed to have been violated.
    pub message_id_name: *const c_char,
    ///[`message_id_number`] is the ID number of the triggering message.
    ///If the message corresponds to a validation layer message, then this
    ///number is related to the internal number associated with the message
    ///being triggered.
    pub message_id_number: i32,
    ///[`message`] is a null-terminated string detailing the trigger
    ///conditions.
    pub message: *const c_char,
    ///[`queue_label_count`] is a count of items contained in the
    ///[`queue_labels`] array.
    pub queue_label_count: u32,
    ///[`queue_labels`] is `NULL` or a pointer to an array of
    ///[`DebugUtilsLabelEXT`] active in the current [`Queue`] at the
    ///time the callback was triggered.
    ///Refer to [Queue Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-queue-labels) for more information.
    pub queue_labels: *const DebugUtilsLabelEXT<'lt>,
    ///[`cmd_buf_label_count`] is a count of items contained in the
    ///[`cmd_buf_labels`] array.
    pub cmd_buf_label_count: u32,
    ///[`cmd_buf_labels`] is `NULL` or a pointer to an array of
    ///[`DebugUtilsLabelEXT`] active in the current [`CommandBuffer`]
    ///at the time the callback was triggered.
    ///Refer to [Command Buffer Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-command-buffer-labels) for
    ///more information.
    pub cmd_buf_labels: *const DebugUtilsLabelEXT<'lt>,
    ///[`object_count`] is a count of items contained in the [`objects`]
    ///array.
    pub object_count: u32,
    ///[`objects`] is a pointer to an array of
    ///[`DebugUtilsObjectNameInfoEXT`] objects related to the detected
    ///issue.
    ///The array is roughly in order or importance, but the 0th element is
    ///always guaranteed to be the most important object for this message.
    pub objects: *const DebugUtilsObjectNameInfoEXT<'lt>,
}
impl<'lt> Default for DebugUtilsMessengerCallbackDataEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            message_id_name: std::ptr::null(),
            message_id_number: 0,
            message: std::ptr::null(),
            queue_label_count: 0,
            queue_labels: std::ptr::null(),
            cmd_buf_label_count: 0,
            cmd_buf_labels: std::ptr::null(),
            object_count: 0,
            objects: std::ptr::null(),
        }
    }
}
impl<'lt> DebugUtilsMessengerCallbackDataEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::message_id_name`]
    pub fn message_id_name_raw(&self) -> *const c_char {
        self.message_id_name
    }
    ///Gets the raw value of [`Self::message`]
    pub fn message_raw(&self) -> *const c_char {
        self.message
    }
    ///Gets the raw value of [`Self::queue_labels`]
    pub fn queue_labels_raw(&self) -> *const DebugUtilsLabelEXT<'lt> {
        self.queue_labels
    }
    ///Gets the raw value of [`Self::cmd_buf_labels`]
    pub fn cmd_buf_labels_raw(&self) -> *const DebugUtilsLabelEXT<'lt> {
        self.cmd_buf_labels
    }
    ///Gets the raw value of [`Self::objects`]
    pub fn objects_raw(&self) -> *const DebugUtilsObjectNameInfoEXT<'lt> {
        self.objects
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::message_id_name`]
    pub fn set_message_id_name_raw(&mut self, value: *const c_char) -> &mut Self {
        self.message_id_name = value;
        self
    }
    ///Sets the raw value of [`Self::message`]
    pub fn set_message_raw(&mut self, value: *const c_char) -> &mut Self {
        self.message = value;
        self
    }
    ///Sets the raw value of [`Self::queue_labels`]
    pub fn set_queue_labels_raw(&mut self, value: *const DebugUtilsLabelEXT<'lt>) -> &mut Self {
        self.queue_labels = value;
        self
    }
    ///Sets the raw value of [`Self::cmd_buf_labels`]
    pub fn set_cmd_buf_labels_raw(&mut self, value: *const DebugUtilsLabelEXT<'lt>) -> &mut Self {
        self.cmd_buf_labels = value;
        self
    }
    ///Sets the raw value of [`Self::objects`]
    pub fn set_objects_raw(&mut self, value: *const DebugUtilsObjectNameInfoEXT<'lt>) -> &mut Self {
        self.objects = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::message_id_name`]
    pub fn with_message_id_name_raw(mut self, value: *const c_char) -> Self {
        self.message_id_name = value;
        self
    }
    ///Sets the raw value of [`Self::message`]
    pub fn with_message_raw(mut self, value: *const c_char) -> Self {
        self.message = value;
        self
    }
    ///Sets the raw value of [`Self::queue_labels`]
    pub fn with_queue_labels_raw(mut self, value: *const DebugUtilsLabelEXT<'lt>) -> Self {
        self.queue_labels = value;
        self
    }
    ///Sets the raw value of [`Self::cmd_buf_labels`]
    pub fn with_cmd_buf_labels_raw(mut self, value: *const DebugUtilsLabelEXT<'lt>) -> Self {
        self.cmd_buf_labels = value;
        self
    }
    ///Sets the raw value of [`Self::objects`]
    pub fn with_objects_raw(mut self, value: *const DebugUtilsObjectNameInfoEXT<'lt>) -> Self {
        self.objects = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> DebugUtilsMessengerCallbackDataFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::message_id_name`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn message_id_name(&self) -> &'lt CStr {
        CStr::from_ptr(self.message_id_name)
    }
    ///Gets the value of [`Self::message_id_number`]
    pub fn message_id_number(&self) -> i32 {
        self.message_id_number
    }
    ///Gets the value of [`Self::message`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn message(&self) -> &'lt CStr {
        CStr::from_ptr(self.message)
    }
    ///Gets the value of [`Self::queue_label_count`]
    pub fn queue_label_count(&self) -> u32 {
        self.queue_label_count
    }
    ///Gets the value of [`Self::queue_labels`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn queue_labels(&self) -> &[DebugUtilsLabelEXT<'lt>] {
        std::slice::from_raw_parts(self.queue_labels, self.queue_label_count as usize)
    }
    ///Gets the value of [`Self::cmd_buf_label_count`]
    pub fn cmd_buf_label_count(&self) -> u32 {
        self.cmd_buf_label_count
    }
    ///Gets the value of [`Self::cmd_buf_labels`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn cmd_buf_labels(&self) -> &[DebugUtilsLabelEXT<'lt>] {
        std::slice::from_raw_parts(self.cmd_buf_labels, self.cmd_buf_label_count as usize)
    }
    ///Gets the value of [`Self::object_count`]
    pub fn object_count(&self) -> u32 {
        self.object_count
    }
    ///Gets the value of [`Self::objects`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn objects(&self) -> &[DebugUtilsObjectNameInfoEXT<'lt>] {
        std::slice::from_raw_parts(self.objects, self.object_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DebugUtilsMessengerCallbackDataFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::message_id_number`]
    pub fn message_id_number_mut(&mut self) -> &mut i32 {
        &mut self.message_id_number
    }
    ///Gets a mutable reference to the value of [`Self::queue_label_count`]
    pub fn queue_label_count_mut(&mut self) -> &mut u32 {
        &mut self.queue_label_count
    }
    ///Gets a mutable reference to the value of [`Self::cmd_buf_label_count`]
    pub fn cmd_buf_label_count_mut(&mut self) -> &mut u32 {
        &mut self.cmd_buf_label_count
    }
    ///Gets a mutable reference to the value of [`Self::object_count`]
    pub fn object_count_mut(&mut self) -> &mut u32 {
        &mut self.object_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::message_id_name`]
    pub fn set_message_id_name(&mut self, value: *const std::os::raw::c_char) -> &mut Self {
        self.message_id_name = value;
        self
    }
    ///Sets the value of [`Self::message_id_number`]
    pub fn set_message_id_number(&mut self, value: i32) -> &mut Self {
        self.message_id_number = value;
        self
    }
    ///Sets the value of [`Self::message`]
    pub fn set_message(&mut self, value: *const std::os::raw::c_char) -> &mut Self {
        self.message = value;
        self
    }
    ///Sets the value of [`Self::queue_label_count`]
    pub fn set_queue_label_count(&mut self, value: u32) -> &mut Self {
        self.queue_label_count = value;
        self
    }
    ///Sets the value of [`Self::queue_labels`]
    pub fn set_queue_labels(
        &mut self,
        value: &'lt [crate::extensions::ext_debug_utils::DebugUtilsLabelEXT<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.queue_labels = value.as_ptr();
        self.queue_label_count = len_;
        self
    }
    ///Sets the value of [`Self::cmd_buf_label_count`]
    pub fn set_cmd_buf_label_count(&mut self, value: u32) -> &mut Self {
        self.cmd_buf_label_count = value;
        self
    }
    ///Sets the value of [`Self::cmd_buf_labels`]
    pub fn set_cmd_buf_labels(
        &mut self,
        value: &'lt [crate::extensions::ext_debug_utils::DebugUtilsLabelEXT<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.cmd_buf_labels = value.as_ptr();
        self.cmd_buf_label_count = len_;
        self
    }
    ///Sets the value of [`Self::object_count`]
    pub fn set_object_count(&mut self, value: u32) -> &mut Self {
        self.object_count = value;
        self
    }
    ///Sets the value of [`Self::objects`]
    pub fn set_objects(
        &mut self,
        value: &'lt [crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.objects = value.as_ptr();
        self.object_count = len_;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(
        mut self,
        value: crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataFlagsEXT,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::message_id_name`]
    pub fn with_message_id_name(mut self, value: *const std::os::raw::c_char) -> Self {
        self.message_id_name = value;
        self
    }
    ///Sets the value of [`Self::message_id_number`]
    pub fn with_message_id_number(mut self, value: i32) -> Self {
        self.message_id_number = value;
        self
    }
    ///Sets the value of [`Self::message`]
    pub fn with_message(mut self, value: *const std::os::raw::c_char) -> Self {
        self.message = value;
        self
    }
    ///Sets the value of [`Self::queue_label_count`]
    pub fn with_queue_label_count(mut self, value: u32) -> Self {
        self.queue_label_count = value;
        self
    }
    ///Sets the value of [`Self::queue_labels`]
    pub fn with_queue_labels(
        mut self,
        value: &'lt [crate::extensions::ext_debug_utils::DebugUtilsLabelEXT<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.queue_labels = value.as_ptr();
        self.queue_label_count = len_;
        self
    }
    ///Sets the value of [`Self::cmd_buf_label_count`]
    pub fn with_cmd_buf_label_count(mut self, value: u32) -> Self {
        self.cmd_buf_label_count = value;
        self
    }
    ///Sets the value of [`Self::cmd_buf_labels`]
    pub fn with_cmd_buf_labels(
        mut self,
        value: &'lt [crate::extensions::ext_debug_utils::DebugUtilsLabelEXT<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.cmd_buf_labels = value.as_ptr();
        self.cmd_buf_label_count = len_;
        self
    }
    ///Sets the value of [`Self::object_count`]
    pub fn with_object_count(mut self, value: u32) -> Self {
        self.object_count = value;
        self
    }
    ///Sets the value of [`Self::objects`]
    pub fn with_objects(
        mut self,
        value: &'lt [crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.objects = value.as_ptr();
        self.object_count = len_;
        self
    }
}
impl Instance {
    ///[vkCreateDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html) - Create a debug messenger object
    ///# C Specifications
    ///A debug messenger triggers a debug callback with a debug message when an
    ///event of interest occurs.
    ///To create a debug messenger which will trigger a debug callback, call:
    ///```c
    ///// Provided by VK_EXT_debug_utils
    ///VkResult vkCreateDebugUtilsMessengerEXT(
    ///    VkInstance                                  instance,
    ///    const VkDebugUtilsMessengerCreateInfoEXT*   pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkDebugUtilsMessengerEXT*                   pMessenger);
    ///```
    /// # Parameters
    /// - [`instance`] is the instance the messenger will be used with.
    /// - [`p_create_info`] is a pointer to a [`DebugUtilsMessengerCreateInfoEXT`] structure
    ///   containing the callback pointer, as well as defining conditions under which this messenger
    ///   will trigger the callback.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// - [`p_messenger`] is a pointer to a [`DebugUtilsMessengerEXT`] handle in which the created
    ///   object is returned.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid
    ///   [`DebugUtilsMessengerCreateInfoEXT`] structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_messenger`] **must**  be a valid pointer to a [`DebugUtilsMessengerEXT`] handle
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// The application  **must**  ensure that [`create_debug_utils_messenger_ext`] is
    /// not executed in parallel with any Vulkan command that is also called with
    /// [`instance`] or child of [`instance`] as the dispatchable argument.
    /// # Related
    /// - [`ext_debug_utils`]
    /// - [`AllocationCallbacks`]
    /// - [`DebugUtilsMessengerCreateInfoEXT`]
    /// - [`DebugUtilsMessengerEXT`]
    /// - [`Instance`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_debug_utils_messenger_ext<'lt>(
        self: &Unique<Instance>,
        p_create_info: &DebugUtilsMessengerCreateInfoEXT<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<DebugUtilsMessengerEXT>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.create_debug_utils_messenger_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.create_debug_utils_messenger_ext())
            .unwrap_unchecked();
        let mut p_messenger = MaybeUninit::<DebugUtilsMessengerEXT>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const DebugUtilsMessengerCreateInfoEXT<'lt>,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_messenger.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(
                _return,
                Unique::new(self, p_messenger.assume_init(), AtomicBool::default()),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl Instance {
    ///[vkDestroyDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html) - Destroy a debug messenger object
    ///# C Specifications
    ///To destroy a [`DebugUtilsMessengerEXT`] object, call:
    ///```c
    ///// Provided by VK_EXT_debug_utils
    ///void vkDestroyDebugUtilsMessengerEXT(
    ///    VkInstance                                  instance,
    ///    VkDebugUtilsMessengerEXT                    messenger,
    ///    const VkAllocationCallbacks*                pAllocator);
    ///```
    /// # Parameters
    /// - [`instance`] is the instance where the callback was created.
    /// - [`messenger`] is the [`DebugUtilsMessengerEXT`] object to destroy. [`messenger`] is an
    ///   externally synchronized object and  **must**  not be used on more than one thread at a
    ///   time. This means that [`destroy_debug_utils_messenger_ext`] **must**  not be called when a
    ///   callback is active.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// # Description
    /// ## Valid Usage
    /// - If [`AllocationCallbacks`] were provided when [`messenger`] was created, a compatible set
    ///   of callbacks  **must**  be provided here
    /// - If no [`AllocationCallbacks`] were provided when [`messenger`] was created,
    ///   [`p_allocator`] **must**  be `NULL`
    ///
    /// ## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - If [`messenger`] is not [`crate::Handle::null`], [`messenger`] **must**  be a valid
    ///   [`DebugUtilsMessengerEXT`] handle
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - If [`messenger`] is a valid handle, it  **must**  have been created, allocated, or
    ///   retrieved from [`instance`]
    ///
    /// ## Host Synchronization
    /// - Host access to [`messenger`] **must**  be externally synchronized
    /// The application  **must**  ensure that [`destroy_debug_utils_messenger_ext`] is
    /// not executed in parallel with any Vulkan command that is also called with
    /// [`instance`] or child of [`instance`] as the dispatchable argument.
    /// # Related
    /// - [`ext_debug_utils`]
    /// - [`AllocationCallbacks`]
    /// - [`DebugUtilsMessengerEXT`]
    /// - [`Instance`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn destroy_debug_utils_messenger_ext<'lt>(
        self: &Unique<Instance>,
        messenger: Option<DebugUtilsMessengerEXT>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.destroy_debug_utils_messenger_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.destroy_debug_utils_messenger_ext())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            messenger.unwrap_or_default(),
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl Instance {
    ///[vkSubmitDebugUtilsMessageEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html) - Inject a message into a debug stream
    ///# C Specifications
    ///There may be times that a user wishes to intentionally submit a debug
    ///message.
    ///To do this, call:
    ///```c
    ///// Provided by VK_EXT_debug_utils
    ///void vkSubmitDebugUtilsMessageEXT(
    ///    VkInstance                                  instance,
    ///    VkDebugUtilsMessageSeverityFlagBitsEXT      messageSeverity,
    ///    VkDebugUtilsMessageTypeFlagsEXT             messageTypes,
    ///    const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData);
    ///```
    /// # Parameters
    /// - [`instance`] is the debug stream’s [`Instance`].
    /// - [`message_severity`] is a [`DebugUtilsMessageSeverityFlagBitsEXT`] value specifying the
    ///   severity of this event/message.
    /// - [`message_types`] is a bitmask of [`DebugUtilsMessageTypeFlagBitsEXT`] specifying which
    ///   type of event(s) to identify with this message.
    /// - [`p_callback_data`] contains all the callback related data in the
    ///   [`DebugUtilsMessengerCallbackDataEXT`] structure.
    /// # Description
    /// The call will propagate through the layers and generate callback(s) as
    /// indicated by the message’s flags.
    /// The parameters are passed on to the callback in addition to the
    /// `pUserData` value that was defined at the time the messenger was
    /// registered.
    /// ## Valid Usage
    /// - The `objectType` member of each element of `pCallbackData->pObjects` **must**  not be
    ///   `VK_OBJECT_TYPE_UNKNOWN`
    ///
    /// ## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - [`message_severity`] **must**  be a valid [`DebugUtilsMessageSeverityFlagBitsEXT`] value
    /// - [`message_types`] **must**  be a valid combination of [`DebugUtilsMessageTypeFlagBitsEXT`]
    ///   values
    /// - [`message_types`] **must**  not be `0`
    /// - [`p_callback_data`] **must**  be a valid pointer to a valid
    ///   [`DebugUtilsMessengerCallbackDataEXT`] structure
    /// # Related
    /// - [`ext_debug_utils`]
    /// - [`DebugUtilsMessageSeverityFlagBitsEXT`]
    /// - [`DebugUtilsMessageTypeFlagsEXT`]
    /// - [`DebugUtilsMessengerCallbackDataEXT`]
    /// - [`Instance`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn submit_debug_utils_message_ext<'lt>(
        self: &Unique<Instance>,
        message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: &DebugUtilsMessengerCallbackDataEXT<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.submit_debug_utils_message_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.submit_debug_utils_message_ext())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            message_severity,
            message_types,
            p_callback_data as *const DebugUtilsMessengerCallbackDataEXT<'lt>,
        );
        ()
    }
}
impl Device {
    ///[vkSetDebugUtilsObjectNameEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html) - Give a user-friendly name to an object
    ///# C Specifications
    ///```c
    ///// Provided by VK_EXT_debug_utils
    ///VkResult vkSetDebugUtilsObjectNameEXT(
    ///    VkDevice                                    device,
    ///    const VkDebugUtilsObjectNameInfoEXT*        pNameInfo);
    ///```
    /// # Parameters
    /// - [`device`] is the device that created the object.
    /// - [`p_name_info`] is a pointer to a [`DebugUtilsObjectNameInfoEXT`] structure specifying
    ///   parameters of the name to set on the object.
    /// # Description
    /// ## Valid Usage
    /// - `pNameInfo->objectType` **must**  not be `VK_OBJECT_TYPE_UNKNOWN`
    /// - `pNameInfo->objectHandle` **must**  not be [`crate::Handle::null`]
    ///
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_name_info`] **must**  be a valid pointer to a valid [`DebugUtilsObjectNameInfoEXT`]
    ///   structure
    ///
    /// ## Host Synchronization
    /// - Host access to `pNameInfo->objectHandle` **must**  be externally synchronized
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`ext_debug_utils`]
    /// - [`DebugUtilsObjectNameInfoEXT`]
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
    #[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn set_debug_utils_object_name_ext<'lt>(
        self: &Unique<Device>,
        p_name_info: &DebugUtilsObjectNameInfoEXT<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.set_debug_utils_object_name_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.set_debug_utils_object_name_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_name_info as *const DebugUtilsObjectNameInfoEXT<'lt>);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkSetDebugUtilsObjectTagEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html) - Attach arbitrary data to an object
    ///# C Specifications
    ///```c
    ///// Provided by VK_EXT_debug_utils
    ///VkResult vkSetDebugUtilsObjectTagEXT(
    ///    VkDevice                                    device,
    ///    const VkDebugUtilsObjectTagInfoEXT*         pTagInfo);
    ///```
    /// # Parameters
    /// - [`device`] is the device that created the object.
    /// - [`p_tag_info`] is a pointer to a [`DebugUtilsObjectTagInfoEXT`] structure specifying
    ///   parameters of the tag to attach to the object.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_tag_info`] **must**  be a valid pointer to a valid [`DebugUtilsObjectTagInfoEXT`]
    ///   structure
    ///
    /// ## Host Synchronization
    /// - Host access to `pTagInfo->objectHandle` **must**  be externally synchronized
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`ext_debug_utils`]
    /// - [`DebugUtilsObjectTagInfoEXT`]
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
    #[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn set_debug_utils_object_tag_ext<'lt>(
        self: &Unique<Device>,
        p_tag_info: &DebugUtilsObjectTagInfoEXT<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.set_debug_utils_object_tag_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.set_debug_utils_object_tag_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_tag_info as *const DebugUtilsObjectTagInfoEXT<'lt>);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Queue {
    ///[vkQueueBeginDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html) - Open a queue debug label region
    ///# C Specifications
    ///A queue debug label region is opened by calling:
    ///```c
    ///// Provided by VK_EXT_debug_utils
    ///void vkQueueBeginDebugUtilsLabelEXT(
    ///    VkQueue                                     queue,
    ///    const VkDebugUtilsLabelEXT*                 pLabelInfo);
    ///```
    /// # Parameters
    /// - [`queue`] is the queue in which to start a debug label region.
    /// - [`p_label_info`] is a pointer to a [`DebugUtilsLabelEXT`] structure specifying parameters
    ///   of the label region to open.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`queue`] **must**  be a valid [`Queue`] handle
    /// - [`p_label_info`] **must**  be a valid pointer to a valid [`DebugUtilsLabelEXT`] structure
    ///
    /// ## Command Properties
    /// # Related
    /// - [`ext_debug_utils`]
    /// - [`DebugUtilsLabelEXT`]
    /// - [`Queue`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn queue_begin_debug_utils_label_ext<'lt>(
        self: &Unique<Queue>,
        p_label_info: &DebugUtilsLabelEXT<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.queue_begin_debug_utils_label_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.queue_begin_debug_utils_label_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_label_info as *const DebugUtilsLabelEXT<'lt>);
        ()
    }
}
impl Queue {
    ///[vkQueueEndDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html) - Close a queue debug label region
    ///# C Specifications
    ///A queue debug label region is closed by calling:
    ///```c
    ///// Provided by VK_EXT_debug_utils
    ///void vkQueueEndDebugUtilsLabelEXT(
    ///    VkQueue                                     queue);
    ///```
    /// # Parameters
    /// - [`queue`] is the queue in which a debug label region should be closed.
    /// # Description
    /// The calls to [`queue_begin_debug_utils_label_ext`] and
    /// [`queue_end_debug_utils_label_ext`] **must**  be matched and balanced.
    /// ## Valid Usage
    /// - There  **must**  be an outstanding [`queue_begin_debug_utils_label_ext`] command prior to
    ///   the [`queue_end_debug_utils_label_ext`] on the queue
    ///
    /// ## Valid Usage (Implicit)
    /// - [`queue`] **must**  be a valid [`Queue`] handle
    ///
    /// ## Command Properties
    /// # Related
    /// - [`ext_debug_utils`]
    /// - [`Queue`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn queue_end_debug_utils_label_ext(self: &Unique<Queue>) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.queue_end_debug_utils_label_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.queue_end_debug_utils_label_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw());
        ()
    }
}
impl Queue {
    ///[vkQueueInsertDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html) - Insert a label into a queue
    ///# C Specifications
    ///A single label can be inserted into a queue by calling:
    ///```c
    ///// Provided by VK_EXT_debug_utils
    ///void vkQueueInsertDebugUtilsLabelEXT(
    ///    VkQueue                                     queue,
    ///    const VkDebugUtilsLabelEXT*                 pLabelInfo);
    ///```
    /// # Parameters
    /// - [`queue`] is the queue into which a debug label will be inserted.
    /// - [`p_label_info`] is a pointer to a [`DebugUtilsLabelEXT`] structure specifying parameters
    ///   of the label to insert.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`queue`] **must**  be a valid [`Queue`] handle
    /// - [`p_label_info`] **must**  be a valid pointer to a valid [`DebugUtilsLabelEXT`] structure
    ///
    /// ## Command Properties
    /// # Related
    /// - [`ext_debug_utils`]
    /// - [`DebugUtilsLabelEXT`]
    /// - [`Queue`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn queue_insert_debug_utils_label_ext<'lt>(
        self: &Unique<Queue>,
        p_label_info: &DebugUtilsLabelEXT<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.queue_insert_debug_utils_label_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.queue_insert_debug_utils_label_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_label_info as *const DebugUtilsLabelEXT<'lt>);
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdBeginDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html) - Open a command buffer debug label region
    ///# C Specifications
    ///A command buffer debug label region can be opened by calling:
    ///```c
    ///// Provided by VK_EXT_debug_utils
    ///void vkCmdBeginDebugUtilsLabelEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkDebugUtilsLabelEXT*                 pLabelInfo);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - [`p_label_info`] is a pointer to a [`DebugUtilsLabelEXT`] structure specifying parameters
    ///   of the label region to open.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_label_info`] **must**  be a valid pointer to a valid [`DebugUtilsLabelEXT`] structure
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
    /// - [`ext_debug_utils`]
    /// - [`CommandBuffer`]
    /// - [`DebugUtilsLabelEXT`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_begin_debug_utils_label_ext<'lt>(
        self: &Unique<CommandBuffer>,
        p_label_info: &DebugUtilsLabelEXT<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.cmd_begin_debug_utils_label_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.cmd_begin_debug_utils_label_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_label_info as *const DebugUtilsLabelEXT<'lt>);
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdEndDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html) - Close a command buffer label region
    ///# C Specifications
    ///A command buffer label region can be closed by calling:
    ///```c
    ///// Provided by VK_EXT_debug_utils
    ///void vkCmdEndDebugUtilsLabelEXT(
    ///    VkCommandBuffer                             commandBuffer);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// # Description
    /// An application  **may**  open a debug label region in one command buffer and
    /// close it in another, or otherwise split debug label regions across multiple
    /// command buffers or multiple queue submissions.
    /// When viewed from the linear series of submissions to a single queue, the
    /// calls to [`cmd_begin_debug_utils_label_ext`] and
    /// [`cmd_end_debug_utils_label_ext`] **must**  be matched and balanced.There  **can**  be
    /// problems reporting command buffer debug labels during the
    /// recording process because command buffers  **may**  be recorded out of sequence
    /// with the resulting execution order.
    /// Since the recording order  **may**  be different, a solitary command buffer  **may**
    /// have an inconsistent view of the debug label regions by itself.
    /// Therefore, if an issue occurs during the recording of a command buffer, and
    /// the environment requires returning debug labels, the implementation  **may**
    /// return only those labels it is aware of.
    /// This is true even if the implementation is aware of only the debug labels
    /// within the command buffer being actively recorded.
    /// ## Valid Usage
    /// - There  **must**  be an outstanding [`cmd_begin_debug_utils_label_ext`] command prior to
    ///   the [`cmd_end_debug_utils_label_ext`] on the queue that [`command_buffer`] is submitted to
    /// - If [`command_buffer`] is a secondary command buffer, there  **must**  be an outstanding
    ///   [`cmd_begin_debug_utils_label_ext`] command recorded to [`command_buffer`] that has not
    ///   previously been ended by a call to [`cmd_end_debug_utils_label_ext`]
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
    /// - [`ext_debug_utils`]
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
    #[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_end_debug_utils_label_ext(self: &Unique<CommandBuffer>) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.cmd_end_debug_utils_label_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.cmd_end_debug_utils_label_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw());
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdInsertDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html) - Insert a label into a command buffer
    ///# C Specifications
    ///A single debug label can be inserted into a command buffer by calling:
    ///```c
    ///// Provided by VK_EXT_debug_utils
    ///void vkCmdInsertDebugUtilsLabelEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkDebugUtilsLabelEXT*                 pLabelInfo);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - `pInfo` is a pointer to a [`DebugUtilsLabelEXT`] structure specifying parameters of the
    ///   label to insert.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_label_info`] **must**  be a valid pointer to a valid [`DebugUtilsLabelEXT`] structure
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
    /// - [`ext_debug_utils`]
    /// - [`CommandBuffer`]
    /// - [`DebugUtilsLabelEXT`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_insert_debug_utils_label_ext<'lt>(
        self: &Unique<CommandBuffer>,
        p_label_info: &DebugUtilsLabelEXT<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.cmd_insert_debug_utils_label_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_debug_utils()
            .and_then(|vtable| vtable.cmd_insert_debug_utils_label_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_label_info as *const DebugUtilsLabelEXT<'lt>);
        ()
    }
}
///[VkDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerEXT.html) - Opaque handle to a debug messenger object
///# C Specifications
///A [`DebugUtilsMessengerEXT`] is a messenger object which handles passing
///along debug messages to a provided debug callback.
///```c
///// Provided by VK_EXT_debug_utils
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDebugUtilsMessengerEXT)
///```
/// # Related
/// - [`ext_debug_utils`]
/// - [`create_debug_utils_messenger_ext`]
/// - [`destroy_debug_utils_messenger_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDebugUtilsMessengerEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct DebugUtilsMessengerEXT(pub u64);
impl DebugUtilsMessengerEXT {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for DebugUtilsMessengerEXT {}
impl Default for DebugUtilsMessengerEXT {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for DebugUtilsMessengerEXT {
    type Parent = Unique<Instance>;
    type VTable = ();
    type Metadata = AtomicBool;
    type Storage = u64;
    #[inline]
    fn as_stored(self) -> Self::Storage {
        self.0
    }
    #[inline]
    unsafe fn from_stored(this: Self::Storage) -> Self {
        Self(this)
    }
    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {
        if !self.metadata().load(Ordering::Acquire) {
            self.instance()
                .destroy_debug_utils_messenger_ext(Some(self.as_raw().coerce()), None);
        }
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<DebugUtilsMessengerEXT> {
    ///Gets the reference to the [`Entry`]
    #[inline]
    pub fn entry(&self) -> &Arc<Entry> {
        self.parent().parent()
    }
    ///Gets the reference to the [`Instance`]
    #[inline]
    pub fn instance(&self) -> &Unique<Instance> {
        self.parent()
    }
    ///Disables the base dropping behaviour of this handle
    #[inline]
    pub fn disable_drop(&self) {
        self.metadata().store(true, Ordering::Relaxed);
    }
}
///The V-table of [`Instance`] for functions from `VK_EXT_debug_utils`
pub struct InstanceExtDebugUtilsVTable {
    ///See [`FNCreateDebugUtilsMessengerExt`] for more information.
    pub create_debug_utils_messenger_ext: FNCreateDebugUtilsMessengerExt,
    ///See [`FNDestroyDebugUtilsMessengerExt`] for more information.
    pub destroy_debug_utils_messenger_ext: FNDestroyDebugUtilsMessengerExt,
    ///See [`FNSubmitDebugUtilsMessageExt`] for more information.
    pub submit_debug_utils_message_ext: FNSubmitDebugUtilsMessageExt,
}
impl InstanceExtDebugUtilsVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Instance,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Instance,
    ) -> Self {
        Self {
            create_debug_utils_messenger_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCreateDebugUtilsMessengerEXT").as_ptr(),
                ))
            },
            destroy_debug_utils_messenger_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkDestroyDebugUtilsMessengerEXT").as_ptr(),
                ))
            },
            submit_debug_utils_message_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkSubmitDebugUtilsMessageEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::create_debug_utils_messenger_ext`]. See [`FNCreateDebugUtilsMessengerExt`] for
    /// more information.
    pub fn create_debug_utils_messenger_ext(&self) -> FNCreateDebugUtilsMessengerExt {
        self.create_debug_utils_messenger_ext
    }
    ///Gets [`Self::destroy_debug_utils_messenger_ext`]. See [`FNDestroyDebugUtilsMessengerExt`]
    /// for more information.
    pub fn destroy_debug_utils_messenger_ext(&self) -> FNDestroyDebugUtilsMessengerExt {
        self.destroy_debug_utils_messenger_ext
    }
    ///Gets [`Self::submit_debug_utils_message_ext`]. See [`FNSubmitDebugUtilsMessageExt`] for more
    /// information.
    pub fn submit_debug_utils_message_ext(&self) -> FNSubmitDebugUtilsMessageExt {
        self.submit_debug_utils_message_ext
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_debug_utils`
pub struct DeviceExtDebugUtilsVTable {
    ///See [`FNSetDebugUtilsObjectNameExt`] for more information.
    pub set_debug_utils_object_name_ext: FNSetDebugUtilsObjectNameExt,
    ///See [`FNSetDebugUtilsObjectTagExt`] for more information.
    pub set_debug_utils_object_tag_ext: FNSetDebugUtilsObjectTagExt,
    ///See [`FNQueueBeginDebugUtilsLabelExt`] for more information.
    pub queue_begin_debug_utils_label_ext: FNQueueBeginDebugUtilsLabelExt,
    ///See [`FNQueueEndDebugUtilsLabelExt`] for more information.
    pub queue_end_debug_utils_label_ext: FNQueueEndDebugUtilsLabelExt,
    ///See [`FNQueueInsertDebugUtilsLabelExt`] for more information.
    pub queue_insert_debug_utils_label_ext: FNQueueInsertDebugUtilsLabelExt,
    ///See [`FNCmdBeginDebugUtilsLabelExt`] for more information.
    pub cmd_begin_debug_utils_label_ext: FNCmdBeginDebugUtilsLabelExt,
    ///See [`FNCmdEndDebugUtilsLabelExt`] for more information.
    pub cmd_end_debug_utils_label_ext: FNCmdEndDebugUtilsLabelExt,
    ///See [`FNCmdInsertDebugUtilsLabelExt`] for more information.
    pub cmd_insert_debug_utils_label_ext: FNCmdInsertDebugUtilsLabelExt,
}
impl DeviceExtDebugUtilsVTable {
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
            set_debug_utils_object_name_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkSetDebugUtilsObjectNameEXT").as_ptr()))
            },
            set_debug_utils_object_tag_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkSetDebugUtilsObjectTagEXT").as_ptr()))
            },
            queue_begin_debug_utils_label_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkQueueBeginDebugUtilsLabelEXT").as_ptr(),
                ))
            },
            queue_end_debug_utils_label_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkQueueEndDebugUtilsLabelEXT").as_ptr()))
            },
            queue_insert_debug_utils_label_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkQueueInsertDebugUtilsLabelEXT").as_ptr(),
                ))
            },
            cmd_begin_debug_utils_label_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdBeginDebugUtilsLabelEXT").as_ptr()))
            },
            cmd_end_debug_utils_label_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdEndDebugUtilsLabelEXT").as_ptr()))
            },
            cmd_insert_debug_utils_label_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdInsertDebugUtilsLabelEXT").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::set_debug_utils_object_name_ext`]. See [`FNSetDebugUtilsObjectNameExt`] for
    /// more information.
    pub fn set_debug_utils_object_name_ext(&self) -> FNSetDebugUtilsObjectNameExt {
        self.set_debug_utils_object_name_ext
    }
    ///Gets [`Self::set_debug_utils_object_tag_ext`]. See [`FNSetDebugUtilsObjectTagExt`] for more
    /// information.
    pub fn set_debug_utils_object_tag_ext(&self) -> FNSetDebugUtilsObjectTagExt {
        self.set_debug_utils_object_tag_ext
    }
    ///Gets [`Self::queue_begin_debug_utils_label_ext`]. See [`FNQueueBeginDebugUtilsLabelExt`] for
    /// more information.
    pub fn queue_begin_debug_utils_label_ext(&self) -> FNQueueBeginDebugUtilsLabelExt {
        self.queue_begin_debug_utils_label_ext
    }
    ///Gets [`Self::queue_end_debug_utils_label_ext`]. See [`FNQueueEndDebugUtilsLabelExt`] for
    /// more information.
    pub fn queue_end_debug_utils_label_ext(&self) -> FNQueueEndDebugUtilsLabelExt {
        self.queue_end_debug_utils_label_ext
    }
    ///Gets [`Self::queue_insert_debug_utils_label_ext`]. See [`FNQueueInsertDebugUtilsLabelExt`]
    /// for more information.
    pub fn queue_insert_debug_utils_label_ext(&self) -> FNQueueInsertDebugUtilsLabelExt {
        self.queue_insert_debug_utils_label_ext
    }
    ///Gets [`Self::cmd_begin_debug_utils_label_ext`]. See [`FNCmdBeginDebugUtilsLabelExt`] for
    /// more information.
    pub fn cmd_begin_debug_utils_label_ext(&self) -> FNCmdBeginDebugUtilsLabelExt {
        self.cmd_begin_debug_utils_label_ext
    }
    ///Gets [`Self::cmd_end_debug_utils_label_ext`]. See [`FNCmdEndDebugUtilsLabelExt`] for more
    /// information.
    pub fn cmd_end_debug_utils_label_ext(&self) -> FNCmdEndDebugUtilsLabelExt {
        self.cmd_end_debug_utils_label_ext
    }
    ///Gets [`Self::cmd_insert_debug_utils_label_ext`]. See [`FNCmdInsertDebugUtilsLabelExt`] for
    /// more information.
    pub fn cmd_insert_debug_utils_label_ext(&self) -> FNCmdInsertDebugUtilsLabelExt {
        self.cmd_insert_debug_utils_label_ext
    }
}
