[VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT.html) - Structure describing whether the implementation supports additional 2-plane 444 Y′C<sub>B</sub>C<sub>R</sub> formats

# C Specifications
The [`PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_ycbcr_2plane_444_formats
typedef struct VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           ycbcr2plane444Formats;
} VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT;
```

# Members
This structure describes the following feature:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`ycbcr2plane444_formats`] indicates that the implementation supports the following 2-plane 444 Y′C<sub>B</sub>C<sub>R</sub> formats:  - `VK_FORMAT_G8_B8R8_2PLANE_444_UNORM`  - `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16`  - `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16`  - `VK_FORMAT_G16_B16R16_2PLANE_444_UNORM` 
If the [`PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT`

# Related
- [`VK_EXT_ycbcr_2plane_444_formats`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        