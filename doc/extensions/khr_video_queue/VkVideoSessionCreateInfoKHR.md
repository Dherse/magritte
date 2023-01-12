[VkVideoSessionCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateInfoKHR.html) - Structure specifying parameters of a newly created video decode session

# C Specifications
The [`VideoSessionCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoSessionCreateInfoKHR {
    VkStructureType                 sType;
    const void*                     pNext;
    uint32_t                        queueFamilyIndex;
    VkVideoSessionCreateFlagsKHR    flags;
    const VkVideoProfileKHR*        pVideoProfile;
    VkFormat                        pictureFormat;
    VkExtent2D                      maxCodedExtent;
    VkFormat                        referencePicturesFormat;
    uint32_t                        maxReferencePicturesSlotsCount;
    uint32_t                        maxReferencePicturesActiveCount;
    const VkExtensionProperties*    pStdHeaderVersion;
} VkVideoSessionCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`queue_family_index`] is the queue family of the created video session.
- [`flags`] is a bitmask of [`VideoSessionCreateFlagBitsKHR`] specifying creation flags.
- [`video_profile`] is a pointer to a [`VideoProfileKHR`] structure.
- [`picture_format`] is the format of the [image views](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views) representing decoded [Output](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture) or encoded [Input](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#input-encode-picture) pictures.
- [`max_coded_extent`] is the maximum width and height of the coded pictures that this instance will be able to support.
- [`reference_pictures_format`] is the format of the [DPB](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb) image views representing the [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture).
- [`max_reference_pictures_slots_count`] is the maximum number of [DPB Slots](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) that can be activated with associated [Video Picture Resources](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for the created video session.
- [`max_reference_pictures_active_count`] is the maximum number of active [DPB Slots](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) that can be used as Dpb or Reconstructed [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture) within a single decode or encode operation for the created video session.
- [`std_header_version`] is a pointer to a [`ExtensionProperties`] structure requesting the Video Std header version to use for `codecOperation` in [`video_profile`].

# Description
## Valid Usage
-  [`video_profile`] **must**  be a pointer to a valid [`VideoProfileKHR`] structure whose [`p_next`] chain  **must**  include a valid codec-specific profile structure
-    If [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture) are required for use with the created video session, the [`max_reference_pictures_slots_count`] **must**  be set to a value bigger than `0`
-  [`max_reference_pictures_slots_count`] **cannot**  exceed the implementation reported [`VideoCapabilitiesKHR`]::[`max_reference_pictures_slots_count`]
-    If [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture) are required for use with the created video session, the [`max_reference_pictures_active_count`] **must**  be set to a value bigger than `0`
-  [`max_reference_pictures_active_count`] **cannot**  exceed the implementation reported [`VideoCapabilitiesKHR`]::[`max_reference_pictures_active_count`]
-  [`max_reference_pictures_active_count`] **cannot**  exceed the [`max_reference_pictures_slots_count`]
-  [`max_coded_extent`] **cannot**  be smaller than [`VideoCapabilitiesKHR::min_extent`] and bigger than [`VideoCapabilitiesKHR::max_extent`]
-  [`reference_pictures_format`] **must**  be one of the supported formats in [`VideoFormatPropertiesKHR`]`format` returned by the [`get_physical_device_video_format_properties_khr`] when the [`PhysicalDeviceVideoFormatInfoKHR`]`imageUsage` contains `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` or `VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR` depending on the session codec operation
-  [`picture_format`] for decode output  **must**  be one of the supported formats in [`VideoFormatPropertiesKHR`]`format` returned by the [`get_physical_device_video_format_properties_khr`] when the [`PhysicalDeviceVideoFormatInfoKHR`]`imageUsage` contains `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`
-  [`picture_format`] targeting encode operations  **must**  be one of the supported formats in [`VideoFormatPropertiesKHR`]`format` returned by the [`get_physical_device_video_format_properties_khr`] when the [`PhysicalDeviceVideoFormatInfoKHR`]`imageUsage` contains `VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be a valid combination of [`VideoSessionCreateFlagBitsKHR`] values
-  [`video_profile`] **must**  be a valid pointer to a valid [`VideoProfileKHR`] structure
-  [`picture_format`] **must**  be a valid [`Format`] value
-  [`reference_pictures_format`] **must**  be a valid [`Format`] value
-  [`std_header_version`] **must**  be a valid pointer to a valid [`ExtensionProperties`] structure

# Related
- [`khr_video_queue`]
- [`ExtensionProperties`]
- [`Extent2D`]
- [`Format`]
- [`StructureType`]
- [`VideoProfileKHR`]
- [VkVideoSessionCreateFlagsKHR]()
- [`create_video_session_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        