[VkVideoCodingControlInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlInfoKHR.html) - Structure specifying parameters of coding control

# C Specifications
The [`VideoCodingControlInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoCodingControlInfoKHR {
    VkStructureType                 sType;
    const void*                     pNext;
    VkVideoCodingControlFlagsKHR    flags;
} VkVideoCodingControlInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`VideoCodingControlFlagsKHR`] specifying control flags.

# Description
## Valid Usage
-    The first command buffer submitted for a newly created video session  **must**  set the `VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR` bit in [`VideoCodingControlInfoKHR`]::[`flags`] to reset the session device context before any video decode or encode operations are performed on the session.

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`VideoEncodeRateControlInfoKHR`] or [`VideoEncodeRateControlLayerInfoKHR`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`VideoCodingControlFlagBitsKHR`] values

# Related
- [`VK_KHR_video_queue`]
- [`StructureType`]
- [`VideoCodingControlFlagsKHR`]
- [`cmd_control_video_coding_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        