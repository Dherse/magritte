[VkMemoryOverallocationBehaviorAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryOverallocationBehaviorAMD.html) - Specify memory overallocation behavior

# C Specifications
Possible values for
[`DeviceMemoryOverallocationCreateInfoAMD::overallocation_behavior`]
include:
```c
// Provided by VK_AMD_memory_overallocation_behavior
typedef enum VkMemoryOverallocationBehaviorAMD {
    VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD = 0,
    VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD = 1,
    VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD = 2,
} VkMemoryOverallocationBehaviorAMD;
```

# Description
- [`VK_MEMORY_OVERALLOCATION_BEHAVIOR_AMD`] lets the implementation decide if overallocation is allowed.
- [`VK_MEMORY_OVERALLOCATION_BEHAVIOR_AMD`] specifies overallocation is allowed if platform permits.
- [`VK_MEMORY_OVERALLOCATION_BEHAVIOR_AMD`] specifies the application is not allowed to allocate device memory beyond the heap sizes reported by [`PhysicalDeviceMemoryProperties`]. Allocations that are not explicitly made by the application within the scope of the Vulkan instance are not accounted for.

# Related
- [`amd_memory_overallocation_behavior`]
- [`DeviceMemoryOverallocationCreateInfoAMD`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        