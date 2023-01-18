[vkDestroyIndirectCommandsLayoutNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html) - Destroy an indirect commands layout

# C Specifications
Indirect command layouts are destroyed by:
```c
// Provided by VK_NV_device_generated_commands
void vkDestroyIndirectCommandsLayoutNV(
    VkDevice                                    device,
    VkIndirectCommandsLayoutNV                  indirectCommandsLayout,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the logical device that destroys the layout.
- [`indirect_commands_layout`] is the layout to destroy.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
## Valid Usage
-    All submitted commands that refer to [`indirect_commands_layout`] **must**  have completed execution
-    If [`AllocationCallbacks`] were provided when [`indirect_commands_layout`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`indirect_commands_layout`] was created, [`p_allocator`] **must**  be `NULL`
-    The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`indirect_commands_layout`] is not [`crate::Handle::null`], [`indirect_commands_layout`] **must**  be a valid [`IndirectCommandsLayoutNV`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`indirect_commands_layout`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`indirect_commands_layout`] **must**  be externally synchronized

# Related
- [`VK_NV_device_generated_commands`]
- [`AllocationCallbacks`]
- [`Device`]
- [`IndirectCommandsLayoutNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        