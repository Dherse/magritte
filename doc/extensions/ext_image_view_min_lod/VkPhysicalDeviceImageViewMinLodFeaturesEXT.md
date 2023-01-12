[VkPhysicalDeviceImageViewMinLodFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewMinLodFeaturesEXT.html) - Structure describing whether clamping the min lod of a image view is supported by the implementation

# C Specifications
The [`PhysicalDeviceImageViewMinLodFeaturesEXT`] structure is defined
as:
```c
// Provided by VK_EXT_image_view_min_lod
typedef struct VkPhysicalDeviceImageViewMinLodFeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           minLod;
} VkPhysicalDeviceImageViewMinLodFeaturesEXT;
```

# Members
This structure describes the following features:

# Description
- [`min_lod`] indicates whether the implementation supports clamping the minimum LOD value during [Image Level(s) Selection](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-image-level-selection) and [Integer Texel Coordinate Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-integer-coordinate-operations) with a given [`ImageView`] by [`ImageViewMinLodCreateInfoEXT`]::[`min_lod`].
If the [`PhysicalDeviceImageViewMinLodFeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceImageViewMinLodFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT`

# Related
- [`ext_image_view_min_lod`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        