[vkGetDeviceMemoryCommitment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html) - Query the current commitment for a VkDeviceMemory

# C Specifications
To determine the amount of lazily-allocated memory that is currently
committed for a memory object, call:
```c
// Provided by VK_VERSION_1_0
void vkGetDeviceMemoryCommitment(
    VkDevice                                    device,
    VkDeviceMemory                              memory,
    VkDeviceSize*                               pCommittedMemoryInBytes);
```

# Parameters
- [`device`] is the logical device that owns the memory.
- [`memory`] is the memory object being queried.
- [`p_committed_memory_in_bytes`] is a pointer to a [`DeviceSize`] value in which the number of bytes currently committed is returned, on success.

# Description
The implementation  **may**  update the commitment at any time, and the value
returned by this query  **may**  be out of date.The implementation guarantees to allocate any committed memory from the
`heapIndex` indicated by the memory type that the memory object was
created with.
## Valid Usage
-  [`memory`] **must**  have been created with a memory type that reports `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`p_committed_memory_in_bytes`] **must**  be a valid pointer to a [`DeviceSize`] value
-  [`memory`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`DeviceMemory`]
- [`DeviceSize`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        