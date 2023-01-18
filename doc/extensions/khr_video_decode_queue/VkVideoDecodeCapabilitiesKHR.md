[VkVideoDecodeCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilitiesKHR.html) - Structure specifying decode capabilities

# C Specifications
When calling [`get_physical_device_video_capabilities_khr`] with
`pVideoProfile->videoCodecOperation` specified as one of the decode
operation bits, the [`VideoDecodeCapabilitiesKHR`] structure  **must**  be
included in the [`p_next`] chain of the [`VideoCapabilitiesKHR`]
structure to retrieve capabilities specific to video decoding.The [`VideoDecodeCapabilitiesKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_decode_queue
typedef struct VkVideoDecodeCapabilitiesKHR {
    VkStructureType                    sType;
    void*                              pNext;
    VkVideoDecodeCapabilityFlagsKHR    flags;
} VkVideoDecodeCapabilitiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`VideoDecodeCapabilityFlagBitsKHR`] describing supported decoding features.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR`

# Related
- [`VK_KHR_video_decode_queue`]
- [`StructureType`]
- [`VideoDecodeCapabilityFlagsKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        