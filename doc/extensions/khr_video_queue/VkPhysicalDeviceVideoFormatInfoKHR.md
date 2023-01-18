[VkPhysicalDeviceVideoFormatInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVideoFormatInfoKHR.html) - Structure specifying the codec video format

# C Specifications
The [`PhysicalDeviceVideoFormatInfoKHR`] input structure is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkPhysicalDeviceVideoFormatInfoKHR {
    VkStructureType              sType;
    void*                        pNext;
    VkImageUsageFlags            imageUsage;
    const VkVideoProfilesKHR*    pVideoProfiles;
} VkPhysicalDeviceVideoFormatInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`image_usage`] is a bitmask of [`ImageUsageFlagBits`] specifying intended video image usages.
- [`video_profiles`] is a pointer to a [`VideoProfilesKHR`] structure providing the video profile(s) of video session(s) that will use the image. For most use cases, the image is used by a single video session and a single video profile is provided. For a use case such as transcode, where a decode session output image  **may**  be used as encode input for one or more encode sessions, multiple video profiles representing the video sessions that will share the image  **may**  be provided.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_KHR_video_queue`]
- [`ImageUsageFlags`]
- [`StructureType`]
- [`VideoProfilesKHR`]
- [`get_physical_device_video_format_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        