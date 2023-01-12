[VkPhysicalDeviceCustomBorderColorFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html) - Structure describing whether custom border colors can be supported by an implementation

# C Specifications
The [`PhysicalDeviceCustomBorderColorFeaturesEXT`] structure is defined
as:
```c
// Provided by VK_EXT_custom_border_color
typedef struct VkPhysicalDeviceCustomBorderColorFeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           customBorderColors;
    VkBool32           customBorderColorWithoutFormat;
} VkPhysicalDeviceCustomBorderColorFeaturesEXT;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`custom_border_colors`] indicates that the implementation supports providing a `borderColor` value with one of the following values at sampler creation time:  - `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`  - `VK_BORDER_COLOR_INT_CUSTOM_EXT` 
- [`custom_border_color_without_format`] indicates that explicit formats are not required for custom border colors and the value of the `format` member of the [`SamplerCustomBorderColorCreateInfoEXT`] structure  **may**  be `VK_FORMAT_UNDEFINED`. If this feature bit is not set, applications  **must**  provide the [`Format`] of the image view(s) being sampled by this sampler in the `format` member of the [`SamplerCustomBorderColorCreateInfoEXT`] structure.
If the [`PhysicalDeviceCustomBorderColorFeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceCustomBorderColorFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT`

# Related
- [`ext_custom_border_color`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        