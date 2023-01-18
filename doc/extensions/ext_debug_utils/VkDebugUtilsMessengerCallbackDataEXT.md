[VkDebugUtilsMessengerCallbackDataEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html) - Structure specifying parameters returned to the callback

# C Specifications
The definition of [`DebugUtilsMessengerCallbackDataEXT`] is:
```c
// Provided by VK_EXT_debug_utils
typedef struct VkDebugUtilsMessengerCallbackDataEXT {
    VkStructureType                              sType;
    const void*                                  pNext;
    VkDebugUtilsMessengerCallbackDataFlagsEXT    flags;
    const char*                                  pMessageIdName;
    int32_t                                      messageIdNumber;
    const char*                                  pMessage;
    uint32_t                                     queueLabelCount;
    const VkDebugUtilsLabelEXT*                  pQueueLabels;
    uint32_t                                     cmdBufLabelCount;
    const VkDebugUtilsLabelEXT*                  pCmdBufLabels;
    uint32_t                                     objectCount;
    const VkDebugUtilsObjectNameInfoEXT*         pObjects;
} VkDebugUtilsMessengerCallbackDataEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is `0` and is reserved for future use.
- [`message_id_name`] is a null-terminated string that identifies the particular message ID that is associated with the provided message. If the message corresponds to a validation layer message, then this string may contain the portion of the Vulkan specification that is believed to have been violated.
- [`message_id_number`] is the ID number of the triggering message. If the message corresponds to a validation layer message, then this number is related to the internal number associated with the message being triggered.
- [`message`] is a null-terminated string detailing the trigger conditions.
- [`queue_label_count`] is a count of items contained in the [`queue_labels`] array.
- [`queue_labels`] is `NULL` or a pointer to an array of [`DebugUtilsLabelEXT`] active in the current [`Queue`] at the time the callback was triggered. Refer to [Queue Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-queue-labels) for more information.
- [`cmd_buf_label_count`] is a count of items contained in the [`cmd_buf_labels`] array.
- [`cmd_buf_labels`] is `NULL` or a pointer to an array of [`DebugUtilsLabelEXT`] active in the current [`CommandBuffer`] at the time the callback was triggered. Refer to [Command Buffer Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-command-buffer-labels) for more information.
- [`object_count`] is a count of items contained in the [`objects`] array.
- [`objects`] is a pointer to an array of [`DebugUtilsObjectNameInfoEXT`] objects related to the detected issue. The array is roughly in order or importance, but the 0th element is always guaranteed to be the most important object for this message.

# Description
Since adding queue and command buffer labels behaves like pushing and
popping onto a stack, the order of both [`queue_labels`] and
[`cmd_buf_labels`] is based on the order the labels were defined.
The result is that the first label in either [`queue_labels`] or
[`cmd_buf_labels`] will be the first defined (and therefore the oldest)
while the last label in each list will be the most recent.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`
-    If [`message_id_name`] is not `NULL`, [`message_id_name`] **must**  be a null-terminated UTF-8 string
-  [`message`] **must**  be a null-terminated UTF-8 string
-    If [`queue_label_count`] is not `0`, [`queue_labels`] **must**  be a valid pointer to an array of [`queue_label_count`] valid [`DebugUtilsLabelEXT`] structures
-    If [`cmd_buf_label_count`] is not `0`, [`cmd_buf_labels`] **must**  be a valid pointer to an array of [`cmd_buf_label_count`] valid [`DebugUtilsLabelEXT`] structures
-    If [`object_count`] is not `0`, [`objects`] **must**  be a valid pointer to an array of [`object_count`] valid [`DebugUtilsObjectNameInfoEXT`] structures

# Related
- [`VK_EXT_debug_utils`]
- [`DebugUtilsLabelEXT`]
- [`DebugUtilsMessengerCallbackDataFlagsEXT`]
- [`DebugUtilsObjectNameInfoEXT`]
- [`StructureType`]
- [`submit_debug_utils_message_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        