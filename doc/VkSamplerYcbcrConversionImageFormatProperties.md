[VkSamplerYcbcrConversionImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionImageFormatProperties.html) - Structure specifying combined image sampler descriptor count for multi-planar images

# C Specifications
To determine the number of combined image samplers required to support a
multi-planar format, add [`SamplerYcbcrConversionImageFormatProperties`]
to the [`p_next`] chain of the [`ImageFormatProperties2`] structure in
a call to [`get_physical_device_image_format_properties2`].The [`SamplerYcbcrConversionImageFormatProperties`] structure is defined
as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkSamplerYcbcrConversionImageFormatProperties {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           combinedImageSamplerDescriptorCount;
} VkSamplerYcbcrConversionImageFormatProperties;
```
or the equivalent
```c
// Provided by VK_KHR_sampler_ycbcr_conversion
typedef VkSamplerYcbcrConversionImageFormatProperties VkSamplerYcbcrConversionImageFormatPropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`combined_image_sampler_descriptor_count`] is the number of combined image sampler descriptors that the implementation uses to access the format.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES`

# Related
- [`crate::vulkan1_1`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        