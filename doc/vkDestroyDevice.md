[vkDestroyDevice](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html) - Destroy a logical device

# C Specifications
To destroy a device, call:
```c
// Provided by VK_VERSION_1_0
void vkDestroyDevice(
    VkDevice                                    device,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the logical device to destroy.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
To ensure that no work is active on the device, [`device_wait_idle`] **can** 
be used to gate the destruction of the device.
Prior to destroying a device, an application is responsible for
destroying/freeing any Vulkan objects that were created using that device as
the first parameter of the corresponding `vkCreate*` or
`vkAllocate*` command.
## Valid Usage
-    All child objects created on [`device`] **must**  have been destroyed prior to destroying [`device`]
-    If [`AllocationCallbacks`] were provided when [`device`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`device`] was created, [`p_allocator`] **must**  be `NULL`

## Valid Usage (Implicit)
-    If [`device`] is not `NULL`, [`device`] **must**  be a valid [`Device`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure

## Host Synchronization
- Host access to [`device`] **must**  be externally synchronized
- Host access to all [`Queue`] objects created from [`device`] **must**  be externally synchronized

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        