[VkVideoFormatPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoFormatPropertiesKHR.html) - Structure enumerating the video image formats

# C Specifications
The [`VideoFormatPropertiesKHR`] output structure for
[`get_physical_device_video_format_properties_khr`] is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoFormatPropertiesKHR {
    VkStructureType    sType;
    void*              pNext;
    VkFormat           format;
} VkVideoFormatPropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`format`] is one of the supported formats reported by the implementation.

# Description
If the `pVideoProfiles` or `imageUsage` provided in input structure
`pVideoFormatInfo` are not supported,
`VK_ERROR_FORMAT_NOT_SUPPORTED` is returned.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_KHR_video_queue`]
- [`Format`]
- [`StructureType`]
- [`get_physical_device_video_format_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        