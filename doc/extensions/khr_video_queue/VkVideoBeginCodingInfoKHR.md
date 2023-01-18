[VkVideoBeginCodingInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoBeginCodingInfoKHR.html) - Structure specifying parameters of decode starts

# C Specifications
The [`VideoBeginCodingInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoBeginCodingInfoKHR {
    VkStructureType                       sType;
    const void*                           pNext;
    VkVideoBeginCodingFlagsKHR            flags;
    VkVideoCodingQualityPresetFlagsKHR    codecQualityPreset;
    VkVideoSessionKHR                     videoSession;
    VkVideoSessionParametersKHR           videoSessionParameters;
    uint32_t                              referenceSlotCount;
    const VkVideoReferenceSlotKHR*        pReferenceSlots;
} VkVideoBeginCodingInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`codec_quality_preset`] is a bitmask of [`VideoCodingQualityPresetFlagBitsKHR`] specifying the Video Decode or Encode quality preset.
- [`video_session`] is the video session object to be bound for the processing of the video commands.
- [`video_session_parameters`] is [`crate::Handle::null`] or a handle of a [`VideoSessionParametersKHR`] object to be used for the processing of the video commands. If [`crate::Handle::null`], then no video session parameters apply to this command buffer context.
- [`reference_slot_count`] is the number of reference slot entries provided in [`reference_slots`].
- [`reference_slots`] is a pointer to an array of [`VideoReferenceSlotKHR`] structures specifying reference slots, used within the video command context between this [`cmd_begin_video_coding_khr`] command and the [`cmd_end_video_coding_khr`] commmand that follows. Each reference slot provides a slot index and the [`VideoPictureResourceKHR`] specifying the reference picture resource bound to this slot index. A slot index  **must**  not appear more than once in [`reference_slots`] in a given command.

# Description
## Valid Usage
-  [`VideoBeginCodingInfoKHR`]::[`reference_slot_count`] **must**  not exceed the value specified in [`VideoSessionCreateInfoKHR::max_reference_pictures_slots_count`] when creating the video session object that is being provided in [`video_session`]
-    If [`video_session_parameters`] is not [`crate::Handle::null`], it  **must**  have been created using [`video_session`] as a parent object

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`
-  [`codec_quality_preset`] **must**  be a valid combination of [`VideoCodingQualityPresetFlagBitsKHR`] values
-  [`codec_quality_preset`] **must**  not be `0`
-  [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
-    If [`video_session_parameters`] is not [`crate::Handle::null`], [`video_session_parameters`] **must**  be a valid [`VideoSessionParametersKHR`] handle
-    If [`reference_slot_count`] is not `0`, [`reference_slots`] **must**  be a valid pointer to an array of [`reference_slot_count`] valid [`VideoReferenceSlotKHR`] structures
-    If [`video_session_parameters`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`video_session`]
-    Both of [`video_session`], and [`video_session_parameters`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`VK_KHR_video_queue`]
- [`StructureType`]
- [`VideoBeginCodingFlagsKHR`]
- [`VideoCodingQualityPresetFlagsKHR`]
- [`VideoReferenceSlotKHR`]
- [`VideoSessionKHR`]
- [`VideoSessionParametersKHR`]
- [`cmd_begin_video_coding_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        