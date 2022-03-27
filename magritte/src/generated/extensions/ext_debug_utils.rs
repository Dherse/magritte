use crate::vulkan1_0::{BaseInStructure, ObjectType, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_UTILS_SPEC_VERSION")]
pub const EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_UTILS_EXTENSION_NAME")]
pub const EXT_DEBUG_UTILS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_debug_utils");
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`object_type`] is a [`ObjectType`] specifying the type of the object to be named.
/// - [`object_handle`] is the object to be named.
/// - [`p_object_name`] is either `NULL` or a null-terminated UTF-8 string specifying the name to
///   apply to [`object_handle`].
///# Description
///Applications **may** change the name associated with an object simply by
///calling [`SetDebugUtilsObjectNameEXT`] again with a new string.
///If [`p_object_name`] is either `NULL` or an empty string, then any
///previously set name is removed.Valid Usage
/// - If [`object_type`] is `VK_OBJECT_TYPE_UNKNOWN`, [`object_handle`]**must** not be
///   [`crate::utils::Handle::null`]
/// -    If [`object_type`] is not `VK_OBJECT_TYPE_UNKNOWN`, [`object_handle`]**must** be [`crate::utils::Handle::null`] or a valid Vulkan handle of the type associated with [`object_type`] as defined in the [[`ObjectType`] and Vulkan Handle Relationship](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-types) table
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`object_type`]**must** be a valid [`ObjectType`] value
/// - If [`p_object_name`] is not `NULL`, [`p_object_name`]**must** be a null-terminated UTF-8
///   string
///# Related
/// - [`VK_EXT_debug_utils`]
/// - [`DebugUtilsMessengerCallbackDataEXT`]
/// - [`ObjectType`]
/// - [`StructureType`]
/// - [`SetDebugUtilsObjectNameEXT`]
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
pub struct DebugUtilsObjectNameInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`object_type`] is a [`ObjectType`] specifying the type of the
    ///object to be named.
    object_type: ObjectType,
    ///[`object_handle`] is the object to be named.
    object_handle: u64,
    ///[`p_object_name`] is either `NULL` or a null-terminated UTF-8 string
    ///specifying the name to apply to [`object_handle`].
    p_object_name: &'lt CStr,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`object_type`] is a [`ObjectType`] specifying the type of the object to be named.
/// - [`object_handle`] is the object to be tagged.
/// - [`tag_name`] is a numerical identifier of the tag.
/// - [`tag_size`] is the number of bytes of data to attach to the object.
/// - [`p_tag`] is a pointer to an array of [`tag_size`] bytes containing the data to be associated
///   with the object.
///# Description
///The [`tag_name`] parameter gives a name or identifier to the type of data
///being tagged.
///This can be used by debugging layers to easily filter for only data that can
///be used by that implementation.Valid Usage
/// - [`object_type`]**must** not be `VK_OBJECT_TYPE_UNKNOWN`
/// - [`object_handle`]**must** be a valid Vulkan handle of the type associated with [`object_type`]
///   as defined in the [[`ObjectType`] and Vulkan Handle Relationship](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-types)
///   table
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`object_type`]**must** be a valid [`ObjectType`] value
/// - [`p_tag`]**must** be a valid pointer to an array of [`tag_size`] bytes
/// - [`tag_size`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_debug_utils`]
/// - [`ObjectType`]
/// - [`StructureType`]
/// - [`SetDebugUtilsObjectTagEXT`]
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
pub struct DebugUtilsObjectTagInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`object_type`] is a [`ObjectType`] specifying the type of the
    ///object to be named.
    object_type: ObjectType,
    ///[`object_handle`] is the object to be tagged.
    object_handle: u64,
    ///[`tag_name`] is a numerical identifier of the tag.
    tag_name: u64,
    ///[`tag_size`] is the number of bytes of data to attach to the object.
    tag_size: usize,
    ///[`p_tag`] is a pointer to an array of [`tag_size`] bytes containing
    ///the data to be associated with the object.
    p_tag: *mut c_void,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`p_label_name`] is a pointer to a null-terminated UTF-8 string containing the name of the
///   label.
/// - [`color`] is an optional RGBA color value that can be associated with the label. A particular
///   implementation **may** choose to ignore this color value. The values contain RGBA values in
///   order, in the range 0.0 to 1.0. If all elements in [`color`] are set to 0.0 then it is
///   ignored.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`p_label_name`]**must** be a null-terminated UTF-8 string
///# Related
/// - [`VK_EXT_debug_utils`]
/// - [`DebugUtilsMessengerCallbackDataEXT`]
/// - [`StructureType`]
/// - [`CmdBeginDebugUtilsLabelEXT`]
/// - [`CmdInsertDebugUtilsLabelEXT`]
/// - [`QueueBeginDebugUtilsLabelEXT`]
/// - [`QueueInsertDebugUtilsLabelEXT`]
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
pub struct DebugUtilsLabelEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`p_label_name`] is a pointer to a null-terminated UTF-8 string
    ///containing the name of the label.
    p_label_name: &'lt CStr,
    ///[`color`] is an optional RGBA color value that can be associated with
    ///the label.
    ///A particular implementation **may** choose to ignore this color value.
    ///The values contain RGBA values in order, in the range 0.0 to 1.0.
    ///If all elements in [`color`] are set to 0.0 then it is ignored.
    color: [f32; 4],
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is `0` and is reserved for future use.
/// - [`message_severity`] is a bitmask of [`DebugUtilsMessageSeverityFlagBitsEXT`] specifying which
///   severity of event(s) will cause this callback to be called.
/// - [`message_type`] is a bitmask of [`DebugUtilsMessageTypeFlagBitsEXT`] specifying which type of
///   event(s) will cause this callback to be called.
/// - [`pfn_user_callback`] is the application callback function to call.
/// - [`p_user_data`] is user data to be passed to the callback.
///# Description
///For each [`DebugUtilsMessengerEXT`] that is created the
///[`DebugUtilsMessengerCreateInfoEXT`]::[`message_severity`] and
///[`DebugUtilsMessengerCreateInfoEXT`]::[`message_type`] determine when
///that [`DebugUtilsMessengerCreateInfoEXT`]::[`pfn_user_callback`] is
///called.
///The process to determine if the user’s [`pfn_user_callback`] is triggered
///when an event occurs is as follows:
///0. The implementation will perform a bitwise AND of the event’s
/// [`DebugUtilsMessageSeverityFlagBitsEXT`] with the [`message_severity`] provided during creation
/// of the [`DebugUtilsMessengerEXT`] object.  0. If the value is 0, the message is skipped.
///2. The implementation will perform bitwise AND of the event’s
/// [`DebugUtilsMessageTypeFlagBitsEXT`] with the [`message_type`] provided during the creation of
/// the [`DebugUtilsMessengerEXT`] object.  0. If the value is 0, the message is skipped.
///4. The callback will trigger a debug message for the current event
///The callback will come directly from the component that detected the event,
///unless some other layer intercepts the calls for its own purposes (filter
///them in a different way, log to a system error log, etc.).An application **can** receive
/// multiple callbacks if multiple
///[`DebugUtilsMessengerEXT`] objects are created.
///A callback will always be executed in the same thread as the originating
///Vulkan call.A callback **can** be called from multiple threads simultaneously (if the
///application is making Vulkan calls from multiple threads).Valid Usage
/// - [`pfn_user_callback`]**must** be a valid [`PFNDebugUtilsMessengerCallbackEXT`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT`
/// - [`flags`]**must** be `0`
/// - [`message_severity`]**must** be a valid combination of
///   [`DebugUtilsMessageSeverityFlagBitsEXT`] values
/// - [`message_severity`]**must** not be `0`
/// - [`message_type`]**must** be a valid combination of [`DebugUtilsMessageTypeFlagBitsEXT`] values
/// - [`message_type`]**must** not be `0`
/// - [`pfn_user_callback`]**must** be a valid [`PFNDebugUtilsMessengerCallbackEXT`] value
///# Related
/// - [`PFNDebugUtilsMessengerCallbackEXT`]
/// - [`VK_EXT_debug_utils`]
/// - [`DebugUtilsMessageSeverityFlagsEXT`]
/// - [`DebugUtilsMessageTypeFlagsEXT`]
/// - [`DebugUtilsMessengerCreateFlagsEXT`]
/// - [`StructureType`]
/// - [`CreateDebugUtilsMessengerEXT`]
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
pub struct DebugUtilsMessengerCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is `0` and is reserved for future use.
    flags: DebugUtilsMessengerCreateFlagsEXT,
    ///[`message_severity`] is a bitmask of
    ///[`DebugUtilsMessageSeverityFlagBitsEXT`] specifying which severity
    ///of event(s) will cause this callback to be called.
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    ///[`message_type`] is a bitmask of
    ///[`DebugUtilsMessageTypeFlagBitsEXT`] specifying which type of
    ///event(s) will cause this callback to be called.
    message_type: DebugUtilsMessageTypeFlagsEXT,
    ///[`pfn_user_callback`] is the application callback function to call.
    pfn_user_callback: PFNDebugUtilsMessengerCallbackEXT<'lt>,
    ///[`p_user_data`] is user data to be passed to the callback.
    p_user_data: *const c_void,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is `0` and is reserved for future use.
/// - [`p_message_id_name`] is a null-terminated string that identifies the particular message ID
///   that is associated with the provided message. If the message corresponds to a validation layer
///   message, then this string may contain the portion of the Vulkan specification that is believed
///   to have been violated.
/// - [`message_id_number`] is the ID number of the triggering message. If the message corresponds
///   to a validation layer message, then this number is related to the internal number associated
///   with the message being triggered.
/// - [`p_message`] is a null-terminated string detailing the trigger conditions.
/// - [`queue_label_count`] is a count of items contained in the [`p_queue_labels`] array.
/// - [`p_queue_labels`] is `NULL` or a pointer to an array of [`DebugUtilsLabelEXT`] active in the current [`Queue`] at the time the callback was triggered. Refer to [Queue Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-queue-labels) for more information.
/// - [`cmd_buf_label_count`] is a count of items contained in the [`p_cmd_buf_labels`] array.
/// - [`p_cmd_buf_labels`] is `NULL` or a pointer to an array of [`DebugUtilsLabelEXT`] active in the current [`CommandBuffer`] at the time the callback was triggered. Refer to [Command Buffer Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-command-buffer-labels) for more information.
/// - [`object_count`] is a count of items contained in the [`p_objects`] array.
/// - [`p_objects`] is a pointer to an array of [`DebugUtilsObjectNameInfoEXT`] objects related to
///   the detected issue. The array is roughly in order or importance, but the 0th element is always
///   guaranteed to be the most important object for this message.
///# Description
///Since adding queue and command buffer labels behaves like pushing and
///popping onto a stack, the order of both [`p_queue_labels`] and
///[`p_cmd_buf_labels`] is based on the order the labels were defined.
///The result is that the first label in either [`p_queue_labels`] or
///[`p_cmd_buf_labels`] will be the first defined (and therefore the oldest)
///while the last label in each list will be the most recent.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
/// - If [`p_message_id_name`] is not `NULL`, [`p_message_id_name`]**must** be a null-terminated
///   UTF-8 string
/// - [`p_message`]**must** be a null-terminated UTF-8 string
/// - If [`queue_label_count`] is not `0`, [`p_queue_labels`]**must** be a valid pointer to an array
///   of [`queue_label_count`] valid [`DebugUtilsLabelEXT`] structures
/// - If [`cmd_buf_label_count`] is not `0`, [`p_cmd_buf_labels`]**must** be a valid pointer to an
///   array of [`cmd_buf_label_count`] valid [`DebugUtilsLabelEXT`] structures
/// - If [`object_count`] is not `0`, [`p_objects`]**must** be a valid pointer to an array of
///   [`object_count`] valid [`DebugUtilsObjectNameInfoEXT`] structures
///# Related
/// - [`VK_EXT_debug_utils`]
/// - [`DebugUtilsLabelEXT`]
/// - [`DebugUtilsMessengerCallbackDataFlagsEXT`]
/// - [`DebugUtilsObjectNameInfoEXT`]
/// - [`StructureType`]
/// - [`SubmitDebugUtilsMessageEXT`]
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
pub struct DebugUtilsMessengerCallbackDataEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is `0` and is reserved for future use.
    flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    ///[`p_message_id_name`] is a null-terminated string that identifies the
    ///particular message ID that is associated with the provided message.
    ///If the message corresponds to a validation layer message, then this
    ///string may contain the portion of the Vulkan specification that is
    ///believed to have been violated.
    p_message_id_name: &'lt CStr,
    ///[`message_id_number`] is the ID number of the triggering message.
    ///If the message corresponds to a validation layer message, then this
    ///number is related to the internal number associated with the message
    ///being triggered.
    message_id_number: i32,
    ///[`p_message`] is a null-terminated string detailing the trigger
    ///conditions.
    p_message: &'lt CStr,
    ///[`queue_label_count`] is a count of items contained in the
    ///[`p_queue_labels`] array.
    queue_label_count: u32,
    ///[`p_queue_labels`] is `NULL` or a pointer to an array of
    ///[`DebugUtilsLabelEXT`] active in the current [`Queue`] at the
    ///time the callback was triggered.
    ///Refer to [Queue Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-queue-labels) for more information.
    p_queue_labels: *mut DebugUtilsLabelEXT<'lt>,
    ///[`cmd_buf_label_count`] is a count of items contained in the
    ///[`p_cmd_buf_labels`] array.
    cmd_buf_label_count: u32,
    ///[`p_cmd_buf_labels`] is `NULL` or a pointer to an array of
    ///[`DebugUtilsLabelEXT`] active in the current [`CommandBuffer`]
    ///at the time the callback was triggered.
    ///Refer to [Command Buffer Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-command-buffer-labels) for
    ///more information.
    p_cmd_buf_labels: *mut DebugUtilsLabelEXT<'lt>,
    ///[`object_count`] is a count of items contained in the [`p_objects`]
    ///array.
    object_count: u32,
    ///[`p_objects`] is a pointer to an array of
    ///[`DebugUtilsObjectNameInfoEXT`] objects related to the detected
    ///issue.
    ///The array is roughly in order or importance, but the 0th element is
    ///always guaranteed to be the most important object for this message.
    p_objects: *mut DebugUtilsObjectNameInfoEXT<'lt>,
}
