[VkPhysicalDeviceSparseImageFormatInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2.html) - Structure specifying sparse image format inputs

# C Specifications
The [`PhysicalDeviceSparseImageFormatInfo2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDeviceSparseImageFormatInfo2 {
    VkStructureType          sType;
    const void*              pNext;
    VkFormat                 format;
    VkImageType              type;
    VkSampleCountFlagBits    samples;
    VkImageUsageFlags        usage;
    VkImageTiling            tiling;
} VkPhysicalDeviceSparseImageFormatInfo2;
```
or the equivalent
```c
// Provided by VK_KHR_get_physical_device_properties2
typedef VkPhysicalDeviceSparseImageFormatInfo2 VkPhysicalDeviceSparseImageFormatInfo2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`format`] is the image format.
- [`type_`] is the dimensionality of image.
- [`samples`] is a [`SampleCountFlagBits`] value specifying the number of samples per texel.
- [`usage`] is a bitmask describing the intended usage of the image.
- [`tiling`] is the tiling arrangement of the texel blocks in memory.

# Description
## Valid Usage
-  [`samples`] **must**  be a bit value that is set in [`ImageFormatProperties::sample_counts`] returned by [`get_physical_device_image_format_properties`] with [`format`], [`type_`], [`tiling`], and [`usage`] equal to those in this command and `flags` equal to the value that is set in [`ImageCreateInfo::flags`] when the image is created

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2`
-  [`p_next`] **must**  be `NULL`
-  [`format`] **must**  be a valid [`Format`] value
-  [`type_`] **must**  be a valid [`ImageType`] value
-  [`samples`] **must**  be a valid [`SampleCountFlagBits`] value
-  [`usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
-  [`usage`] **must**  not be `0`
-  [`tiling`] **must**  be a valid [`ImageTiling`] value

# Related
- [`crate::vulkan1_1`]
- [`Format`]
- [`ImageTiling`]
- [`ImageType`]
- [`ImageUsageFlags`]
- [`SampleCountFlagBits`]
- [`StructureType`]
- [`get_physical_device_sparse_image_format_properties2`]
- [`get_physical_device_sparse_image_format_properties2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        