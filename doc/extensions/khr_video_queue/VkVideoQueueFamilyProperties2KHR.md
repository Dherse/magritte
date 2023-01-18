[VkVideoQueueFamilyProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoQueueFamilyProperties2KHR.html) - Structure specifying the codec video operations

# C Specifications
The [`VideoQueueFamilyProperties2KHR`] structure is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoQueueFamilyProperties2KHR {
    VkStructureType                  sType;
    void*                            pNext;
    VkVideoCodecOperationFlagsKHR    videoCodecOperations;
} VkVideoQueueFamilyProperties2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`video_codec_operations`] is a bitmask of [`VideoCodecOperationFlagBitsKHR`] specifying supported video codec operation(s).

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_QUEUE_FAMILY_PROPERTIES_2_KHR`
-  [`video_codec_operations`] **must**  be a valid combination of [`VideoCodecOperationFlagBitsKHR`] values
-  [`video_codec_operations`] **must**  not be `0`

# Related
- [`VK_KHR_video_queue`]
- [`StructureType`]
- [`VideoCodecOperationFlagsKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        