[VkPhysicalDeviceVulkanMemoryModelFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkanMemoryModelFeatures.html) - Structure describing features supported by the memory model

# C Specifications
The [`PhysicalDeviceVulkanMemoryModelFeatures`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkPhysicalDeviceVulkanMemoryModelFeatures {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           vulkanMemoryModel;
    VkBool32           vulkanMemoryModelDeviceScope;
    VkBool32           vulkanMemoryModelAvailabilityVisibilityChains;
} VkPhysicalDeviceVulkanMemoryModelFeatures;
```
or the equivalent
```c
// Provided by VK_KHR_vulkan_memory_model
typedef VkPhysicalDeviceVulkanMemoryModelFeatures VkPhysicalDeviceVulkanMemoryModelFeaturesKHR;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`vulkan_memory_model`] indicates whether the Vulkan Memory Model is supported, as defined in [Vulkan Memory Model](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-model). This also indicates whether shader modules  **can**  declare the `VulkanMemoryModel` capability.
- [`vulkan_memory_model_device_scope`] indicates whether the Vulkan Memory Model can use [`Device`] scope synchronization. This also indicates whether shader modules  **can**  declare the `VulkanMemoryModelDeviceScope` capability.
- [`vulkan_memory_model_availability_visibility_chains`] indicates whether the Vulkan Memory Model can use [availability and visibility chains](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-model-availability-visibility) with more than one element.
If the [`PhysicalDeviceVulkanMemoryModelFeaturesKHR`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceVulkanMemoryModelFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES`

# Related
- [`crate::vulkan1_2`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        