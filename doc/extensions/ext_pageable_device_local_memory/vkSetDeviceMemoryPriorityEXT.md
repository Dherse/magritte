[vkSetDeviceMemoryPriorityEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html) - Change a memory allocation priority

# C Specifications
To modify the priority of an existing memory allocation, call:
```c
// Provided by VK_EXT_pageable_device_local_memory
void vkSetDeviceMemoryPriorityEXT(
    VkDevice                                    device,
    VkDeviceMemory                              memory,
    float                                       priority);
```

# Parameters
- [`device`] is the logical device that owns the memory.
- [`memory`] is the [`DeviceMemory`] object to which the new priority will be applied.
- [`priority`] is a floating-point value between `0` and `1`, indicating the priority of the allocation relative to other memory allocations. Larger values are higher priority. The granularity of the priorities is implementation-dependent.

# Description
Memory allocations with higher priority  **may**  be more likely to stay in
device-local memory when the system is under memory pressure.
## Valid Usage
-  [`priority`] **must**  be between `0` and `1`, inclusive

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`memory`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`VK_EXT_pageable_device_local_memory`]
- [`Device`]
- [`DeviceMemory`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        