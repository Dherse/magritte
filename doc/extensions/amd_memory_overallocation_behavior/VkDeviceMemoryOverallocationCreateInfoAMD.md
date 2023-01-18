[VkDeviceMemoryOverallocationCreateInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryOverallocationCreateInfoAMD.html) - Specify memory overallocation behavior for a Vulkan device

# C Specifications
To specify whether device memory allocation is allowed beyond the size
reported by [`PhysicalDeviceMemoryProperties`], add a
[`DeviceMemoryOverallocationCreateInfoAMD`] structure to the [`p_next`]
chain of the [`DeviceCreateInfo`] structure.
If this structure is not specified, it is as if the
`VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD` value is used.
```c
// Provided by VK_AMD_memory_overallocation_behavior
typedef struct VkDeviceMemoryOverallocationCreateInfoAMD {
    VkStructureType                      sType;
    const void*                          pNext;
    VkMemoryOverallocationBehaviorAMD    overallocationBehavior;
} VkDeviceMemoryOverallocationCreateInfoAMD;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`overallocation_behavior`] is the desired overallocation behavior.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD`
-  [`overallocation_behavior`] **must**  be a valid [`MemoryOverallocationBehaviorAMD`] value

# Related
- [`VK_AMD_memory_overallocation_behavior`]
- [`MemoryOverallocationBehaviorAMD`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        