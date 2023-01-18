[vkGetDeviceImageMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirements.html) - Returns the memory requirements for specified Vulkan object

# C Specifications
To determine the memory requirements for an image resource without creating
an object, call:
```c
// Provided by VK_VERSION_1_3
void vkGetDeviceImageMemoryRequirements(
    VkDevice                                    device,
    const VkDeviceImageMemoryRequirements*      pInfo,
    VkMemoryRequirements2*                      pMemoryRequirements);
```
or the equivalent command
```c
// Provided by VK_KHR_maintenance4
void vkGetDeviceImageMemoryRequirementsKHR(
    VkDevice                                    device,
    const VkDeviceImageMemoryRequirements*      pInfo,
    VkMemoryRequirements2*                      pMemoryRequirements);
```

# Parameters
- [`device`] is the logical device intended to own the image.
- [`p_info`] is a pointer to a [`DeviceImageMemoryRequirements`] structure containing parameters required for the memory requirements query.
- [`p_memory_requirements`] is a pointer to a [`MemoryRequirements2`] structure in which the memory requirements of the image object are returned.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`DeviceImageMemoryRequirements`] structure
-  [`p_memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements2`] structure

# Related
- [`VK_KHR_maintenance4`]
- [`crate::vulkan1_3`]
- [`Device`]
- [`DeviceImageMemoryRequirements`]
- [`MemoryRequirements2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        