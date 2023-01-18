[VkVideoProfilesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoProfilesKHR.html) - Structure enumerating the video profiles

# C Specifications
The [`VideoProfilesKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoProfilesKHR {
    VkStructureType             sType;
    void*                       pNext;
    uint32_t                    profileCount;
    const VkVideoProfileKHR*    pProfiles;
} VkVideoProfilesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`profile_count`] is an integer which holds the number of video profiles included in [`profiles`].
- [`profiles`] is a pointer to an array of [`VideoProfileKHR`] structures. Each [`VideoProfileKHR`] structure  **must**  chain the corresponding codec-operation specific extension video profile structure.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_PROFILES_KHR`
-  [`profiles`] **must**  be a valid pointer to an array of [`profile_count`] valid [`VideoProfileKHR`] structures
-  [`profile_count`] **must**  be greater than `0`

# Related
- [`VK_KHR_video_queue`]
- [`PhysicalDeviceVideoFormatInfoKHR`]
- [`StructureType`]
- [`VideoProfileKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        