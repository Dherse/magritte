[VkPhysicalDeviceSubgroupSizeControlProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlProperties.html) - Structure describing the control subgroup size properties of an implementation

# C Specifications
The [`PhysicalDeviceSubgroupSizeControlProperties`] structure is defined
as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPhysicalDeviceSubgroupSizeControlProperties {
    VkStructureType       sType;
    void*                 pNext;
    uint32_t              minSubgroupSize;
    uint32_t              maxSubgroupSize;
    uint32_t              maxComputeWorkgroupSubgroups;
    VkShaderStageFlags    requiredSubgroupSizeStages;
} VkPhysicalDeviceSubgroupSizeControlProperties;
```
or the equivalent
```c
// Provided by VK_EXT_subgroup_size_control
typedef VkPhysicalDeviceSubgroupSizeControlProperties VkPhysicalDeviceSubgroupSizeControlPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

# Description
- [`min_subgroup_size`] is the minimum subgroup size supported by this device. [`min_subgroup_size`] is at least one if any of the physical device’s queues support `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`. [`min_subgroup_size`] is a power-of-two. [`min_subgroup_size`] is less than or equal to [`max_subgroup_size`]. [`min_subgroup_size`] is less than or equal to [subgroupSize](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-subgroup-size).
- [`max_subgroup_size`] is the maximum subgroup size supported by this device. [`max_subgroup_size`] is at least one if any of the physical device’s queues support `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`. [`max_subgroup_size`] is a power-of-two. [`max_subgroup_size`] is greater than or equal to [`min_subgroup_size`]. [`max_subgroup_size`] is greater than or equal to [subgroupSize](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-subgroup-size).
- [`max_compute_workgroup_subgroups`] is the maximum number of subgroups supported by the implementation within a workgroup.
- [`required_subgroup_size_stages`] is a bitfield of what shader stages support having a required subgroup size specified.
If the [`PhysicalDeviceSubgroupSizeControlProperties`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.If [`PhysicalDeviceSubgroupProperties::supported_operations`]
includes [`VK_SUBGROUP_FEATURE_QUAD_BIT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-subgroup-quad),
[`min_subgroup_size`] **must**  be greater than or equal to 4.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES`

# Related
- [`ext_subgroup_size_control`]
- [`crate::vulkan1_3`]
- [VkShaderStageFlags]()
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        