[vkCmdBindVertexBuffers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html) - Bind vertex buffers to a command buffer

# C Specifications
To bind vertex buffers to a command buffer for use in subsequent drawing
commands, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdBindVertexBuffers(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    firstBinding,
    uint32_t                                    bindingCount,
    const VkBuffer*                             pBuffers,
    const VkDeviceSize*                         pOffsets);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.
- [`first_binding`] is the index of the first vertex input binding whose state is updated by the command.
- [`binding_count`] is the number of vertex input bindings whose state is updated by the command.
- [`p_buffers`] is a pointer to an array of buffer handles.
- [`p_offsets`] is a pointer to an array of buffer offsets.

# Description
The values taken from elements i of [`p_buffers`] and [`p_offsets`]
replace the current state for the vertex input binding
[`first_binding`] +  i, for i in [0,
[`binding_count`]).
The vertex input binding is updated to start at the offset indicated by
[`p_offsets`][i] from the start of the buffer [`p_buffers`][i].
All vertex input attributes that use each of these bindings will use these
updated addresses in their address calculations for subsequent drawing
commands.
If the [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor) feature is enabled,
elements of [`p_buffers`] **can**  be [`crate::Handle::null`], and  **can**  be used by
the vertex shader.
If a vertex input attribute is bound to a vertex input binding that is
[`crate::Handle::null`], the values taken from memory are considered to be
zero, and missing G, B, or A components are
[filled with (0,0,1)](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#fxvertex-input-extraction).
## Valid Usage
-  [`first_binding`] **must**  be less than [`PhysicalDeviceLimits::max_vertex_input_bindings`]
-    The sum of [`first_binding`] and [`binding_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_vertex_input_bindings`]
-    All elements of [`p_offsets`] **must**  be less than the size of the corresponding element in [`p_buffers`]
-    All elements of [`p_buffers`] **must**  have been created with the `VK_BUFFER_USAGE_VERTEX_BUFFER_BIT` flag
-    Each element of [`p_buffers`] that is non-sparse  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    If the [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor) feature is not enabled, all elements of [`p_buffers`] **must**  not be [`crate::Handle::null`]
-    If an element of [`p_buffers`] is [`crate::Handle::null`], then the corresponding element of [`p_offsets`] **must**  be zero

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_buffers`] **must**  be a valid pointer to an array of [`binding_count`] valid or [`crate::Handle::null`][`Buffer`] handles
-  [`p_offsets`] **must**  be a valid pointer to an array of [`binding_count`][`DeviceSize`] values
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-  [`binding_count`] **must**  be greater than `0`
-    Both of [`command_buffer`], and the elements of [`p_buffers`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`Buffer`]
- [`CommandBuffer`]
- [`DeviceSize`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        