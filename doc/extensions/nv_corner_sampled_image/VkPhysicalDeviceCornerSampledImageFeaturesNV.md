[VkPhysicalDeviceCornerSampledImageFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCornerSampledImageFeaturesNV.html) - Structure describing corner sampled image features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceCornerSampledImageFeaturesNV`] structure is defined
as:
```c
// Provided by VK_NV_corner_sampled_image
typedef struct VkPhysicalDeviceCornerSampledImageFeaturesNV {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           cornerSampledImage;
} VkPhysicalDeviceCornerSampledImageFeaturesNV;
```

# Members
This structure describes the following feature:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`corner_sampled_image`] specifies whether images can be created with a [`ImageCreateInfo::flags`] containing `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV`. See [Corner-Sampled Images](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-images-corner-sampled).
If the [`PhysicalDeviceCornerSampledImageFeaturesNV`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceCornerSampledImageFeaturesNV`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV`

# Related
- [`VK_NV_corner_sampled_image`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        