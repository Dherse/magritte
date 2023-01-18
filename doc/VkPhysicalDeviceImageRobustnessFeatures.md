[VkPhysicalDeviceImageRobustnessFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageRobustnessFeatures.html) - Structure describing the out-of-bounds behavior for an implementation

# C Specifications
The [`PhysicalDeviceImageRobustnessFeatures`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPhysicalDeviceImageRobustnessFeatures {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           robustImageAccess;
} VkPhysicalDeviceImageRobustnessFeatures;
```
or the equivalent
```c
// Provided by VK_EXT_image_robustness
typedef VkPhysicalDeviceImageRobustnessFeatures VkPhysicalDeviceImageRobustnessFeaturesEXT;
```

# Members
This structure describes the following feature:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`robust_image_access`] indicates whether image accesses are tightly bounds-checked against the dimensions of the image view. [Invalid texels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-input-validation) resulting from out of bounds image loads will be replaced as described in [Texel Replacement](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-texel-replacement), with either (0,0,1) or (0,0,0) values inserted for missing G, B, or A components based on the format.
If the [`PhysicalDeviceImageRobustnessFeatures`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceImageRobustnessFeatures`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES`

# Related
- [`VK_EXT_image_robustness`]
- [`crate::vulkan1_3`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        