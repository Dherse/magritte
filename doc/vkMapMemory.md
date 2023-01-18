[vkMapMemory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html) - Map a memory object into application address space

# C Specifications
To retrieve a host virtual address pointer to a region of a mappable memory
object, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkMapMemory(
    VkDevice                                    device,
    VkDeviceMemory                              memory,
    VkDeviceSize                                offset,
    VkDeviceSize                                size,
    VkMemoryMapFlags                            flags,
    void**                                      ppData);
```

# Parameters
- [`device`] is the logical device that owns the memory.
- [`memory`] is the [`DeviceMemory`] object to be mapped.
- [`offset`] is a zero-based byte offset from the beginning of the memory object.
- [`size`] is the size of the memory range to map, or [`WHOLE_SIZE`] to map from [`offset`] to the end of the allocation.
- [`flags`] is reserved for future use.
- [`pp_data`] is a pointer to a `void *` variable in which is returned a host-accessible pointer to the beginning of the mapped range. This pointer minus [`offset`] **must**  be aligned to at least [`PhysicalDeviceLimits::min_memory_map_alignment`].

# Description
After a successful call to [`map_memory`] the memory object [`memory`]
is considered to be currently *host mapped*.[`map_memory`] does not check whether the device memory is currently in
use before returning the host-accessible pointer.
The application  **must**  guarantee that any previously submitted command that
writes to this range has completed before the host reads from or writes to
that range, and that any previously submitted command that reads from that
range has completed before the host writes to that region (see
[here](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-host-writes) for details on fulfilling
such a guarantee).
If the device memory was allocated without the
`VK_MEMORY_PROPERTY_HOST_COHERENT_BIT` set, these guarantees  **must**  be
made for an extended range: the application  **must**  round down the start of
the range to the nearest multiple of
[`PhysicalDeviceLimits::non_coherent_atom_size`], and round the end
of the range up to the nearest multiple of
[`PhysicalDeviceLimits::non_coherent_atom_size`].While a range of device memory is host mapped, the application is
responsible for synchronizing both device and host access to that memory
range.
## Valid Usage
-  [`memory`] **must**  not be currently host mapped
-  [`offset`] **must**  be less than the size of [`memory`]
-    If [`size`] is not equal to [`WHOLE_SIZE`], [`size`] **must**  be greater than `0`
-    If [`size`] is not equal to [`WHOLE_SIZE`], [`size`] **must**  be less than or equal to the size of the [`memory`] minus [`offset`]
-  [`memory`] **must**  have been created with a memory type that reports `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT`
-  [`memory`] **must**  not have been allocated with multiple instances

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`flags`] **must**  be `0`
-  [`pp_data`] **must**  be a valid pointer to a pointer value
-  [`memory`] **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`memory`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_MEMORY_MAP_FAILED`

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`DeviceMemory`]
- [`DeviceSize`]
- [`MemoryMapFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        