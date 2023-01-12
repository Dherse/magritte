[VkPhysicalDeviceFragmentDensityMapFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html) - Structure describing fragment density map features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceFragmentDensityMapFeaturesEXT`] structure is defined
as:
```c
// Provided by VK_EXT_fragment_density_map
typedef struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           fragmentDensityMap;
    VkBool32           fragmentDensityMapDynamic;
    VkBool32           fragmentDensityMapNonSubsampledImages;
} VkPhysicalDeviceFragmentDensityMapFeaturesEXT;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`fragment_density_map`] specifies whether the implementation supports render passes with a fragment density map attachment. If this feature is not enabled and the [`p_next`] chain of [`RenderPassCreateInfo`] includes a [`RenderPassFragmentDensityMapCreateInfoEXT`] structure, `fragmentDensityMapAttachment` **must**  be `VK_ATTACHMENT_UNUSED`.
- [`fragment_density_map_dynamic`] specifies whether the implementation supports dynamic fragment density map image views. If this feature is not enabled, `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT` **must**  not be included in [`ImageViewCreateInfo::flags`].
- [`fragment_density_map_non_subsampled_images`] specifies whether the implementation supports regular non-subsampled image attachments with fragment density map render passes. If this feature is not enabled, render passes with a [fragment density map attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapattachment) **must**  only have [subsampled attachments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-subsamplesampler) bound.
If the [`PhysicalDeviceFragmentDensityMapFeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceFragmentDensityMapFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT`

# Related
- [`ext_fragment_density_map`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        