[VkPhysicalDeviceUniformBufferStandardLayoutFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceUniformBufferStandardLayoutFeatures.html) - Structure indicating support for std430-like packing in uniform buffers

# C Specifications
The [`PhysicalDeviceUniformBufferStandardLayoutFeatures`] structure is
defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           uniformBufferStandardLayout;
} VkPhysicalDeviceUniformBufferStandardLayoutFeatures;
```
or the equivalent
```c
// Provided by VK_KHR_uniform_buffer_standard_layout
typedef VkPhysicalDeviceUniformBufferStandardLayoutFeatures VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR;
```

# Members
This structure describes the following feature:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`uniform_buffer_standard_layout`] indicates that the implementation supports the same layouts for uniform buffers as for storage and other kinds of buffers. See [Standard Buffer Layout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-resources-standard-layout).
If the [`PhysicalDeviceUniformBufferStandardLayoutFeatures`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceUniformBufferStandardLayoutFeatures`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES`

# Related
- [`VK_KHR_uniform_buffer_standard_layout`]
- [`crate::vulkan1_2`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        