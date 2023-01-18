[vkGetDeviceBufferMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html) - Returns the memory requirements for specified Vulkan object

# C Specifications
To determine the memory requirements for a buffer resource without creating
an object, call:
```c
// Provided by VK_VERSION_1_3
void vkGetDeviceBufferMemoryRequirements(
    VkDevice                                    device,
    const VkDeviceBufferMemoryRequirements*     pInfo,
    VkMemoryRequirements2*                      pMemoryRequirements);
```
or the equivalent command
```c
// Provided by VK_KHR_maintenance4
void vkGetDeviceBufferMemoryRequirementsKHR(
    VkDevice                                    device,
    const VkDeviceBufferMemoryRequirements*     pInfo,
    VkMemoryRequirements2*                      pMemoryRequirements);
```

# Parameters
- [`device`] is the logical device intended to own the buffer.
- [`p_info`] is a pointer to a [`DeviceBufferMemoryRequirements`] structure containing parameters required for the memory requirements query.
- [`p_memory_requirements`] is a pointer to a [`MemoryRequirements2`] structure in which the memory requirements of the buffer object are returned.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`DeviceBufferMemoryRequirements`] structure
-  [`p_memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements2`] structure

# Related
- [`VK_KHR_maintenance4`]
- [`crate::vulkan1_3`]
- [`Device`]
- [`DeviceBufferMemoryRequirements`]
- [`MemoryRequirements2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        