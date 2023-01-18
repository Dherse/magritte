[VkPhysicalDeviceBorderColorSwizzleFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBorderColorSwizzleFeaturesEXT.html) - Structure describing whether samplers with custom border colors require the component swizzle specified in order to have defined behavior

# C Specifications
The [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`] structure is defined
as:
```c
// Provided by VK_EXT_border_color_swizzle
typedef struct VkPhysicalDeviceBorderColorSwizzleFeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           borderColorSwizzle;
    VkBool32           borderColorSwizzleFromImage;
} VkPhysicalDeviceBorderColorSwizzleFeaturesEXT;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`border_color_swizzle`] indicates that defined values are returned by sampled image operations when used with a sampler that uses a `VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK`, `VK_BORDER_COLOR_INT_OPAQUE_BLACK`, `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`, or `VK_BORDER_COLOR_INT_CUSTOM_EXT``borderColor` and an image view that uses a non-[identity component mapping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings), when either [`border_color_swizzle_from_image`] is enabled or the [`SamplerBorderColorComponentMappingCreateInfoEXT`] is specified.
- [`border_color_swizzle_from_image`] indicates that the implementation will return the correct border color values from sampled image operations under the conditions expressed above, without the application having to specify the border color component mapping when creating the sampler object. If this feature bit is not set, applications  **can**  chain a [`SamplerBorderColorComponentMappingCreateInfoEXT`] structure when creating samplers for use with image views that do not have an [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings) and, when those samplers are combined with image views using the same component mapping, sampled image operations that use opaque black or custom border colors will return the correct border color values.
If the [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceBorderColorSwizzleFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT`

# Related
- [`VK_EXT_border_color_swizzle`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        