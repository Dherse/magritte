[VkPhysicalDeviceRobustness2FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html) - Structure describing the out-of-bounds behavior for an implementation

# C Specifications
The [`PhysicalDeviceRobustness2FeaturesEXT`] structure is defined as:
```c
// Provided by VK_EXT_robustness2
typedef struct VkPhysicalDeviceRobustness2FeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           robustBufferAccess2;
    VkBool32           robustImageAccess2;
    VkBool32           nullDescriptor;
} VkPhysicalDeviceRobustness2FeaturesEXT;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`robust_buffer_access2`] indicates whether buffer accesses are tightly bounds-checked against the range of the descriptor. Uniform buffers  **must**  be bounds-checked to the range of the descriptor, where the range is rounded up to a multiple of [robustUniformBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustUniformBufferAccessSizeAlignment). Storage buffers  **must**  be bounds-checked to the range of the descriptor, where the range is rounded up to a multiple of [robustStorageBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustStorageBufferAccessSizeAlignment). Out of bounds buffer loads will return zero values, and formatted loads will have (0,0,1) values inserted for missing G, B, or A components based on the format.
- [`robust_image_access2`] indicates whether image accesses are tightly bounds-checked against the dimensions of the image view. Out of bounds image loads will return zero values, with (0,0,1) values [inserted for missing G, B, or A components]() based on the format.
- [`null_descriptor`] indicates whether descriptors  **can**  be written with a [`crate::Handle::null`] resource or view, which are considered valid to access and act as if the descriptor were bound to nothing.
If the [`PhysicalDeviceRobustness2FeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceRobustness2FeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage
-    If [`robust_buffer_access2`] is enabled then [`robustBufferAccess`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess) **must**  also be enabled

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT`

# Related
- [`ext_robustness2`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        