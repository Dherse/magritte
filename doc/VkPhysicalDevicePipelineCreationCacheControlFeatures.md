[VkPhysicalDevicePipelineCreationCacheControlFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineCreationCacheControlFeatures.html) - Structure describing whether pipeline cache control can be supported by an implementation

# C Specifications
The [`PhysicalDevicePipelineCreationCacheControlFeatures`] structure is
defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPhysicalDevicePipelineCreationCacheControlFeatures {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           pipelineCreationCacheControl;
} VkPhysicalDevicePipelineCreationCacheControlFeatures;
```
or the equivalent
```c
// Provided by VK_EXT_pipeline_creation_cache_control
typedef VkPhysicalDevicePipelineCreationCacheControlFeatures VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT;
```

# Members
This structure describes the following feature:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`pipeline_creation_cache_control`] indicates that the implementation supports:  - The following  **can**  be used in `Vk*PipelineCreateInfo`::`flags`:   - `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT`   - `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`   - The following  **can**  be used in [`PipelineCacheCreateInfo::flags`]:   - `VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`  
If the [`PhysicalDevicePipelineCreationCacheControlFeatures`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDevicePipelineCreationCacheControlFeatures`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES`

# Related
- [`VK_EXT_pipeline_creation_cache_control`]
- [`crate::vulkan1_3`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        