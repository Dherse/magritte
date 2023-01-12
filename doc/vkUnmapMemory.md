[vkUnmapMemory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html) - Unmap a previously mapped memory object

# C Specifications
To unmap a memory object once host access to it is no longer needed by the
application, call:
```c
// Provided by VK_VERSION_1_0
void vkUnmapMemory(
    VkDevice                                    device,
    VkDeviceMemory                              memory);
```

# Parameters
- [`device`] is the logical device that owns the memory.
- [`memory`] is the memory object to be unmapped.

# Description
## Valid Usage
-  [`memory`] **must**  be currently host mapped

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`memory`] **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`memory`] **must**  be externally synchronized

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`DeviceMemory`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        