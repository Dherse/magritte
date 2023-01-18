[vkResetCommandBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html) - Reset a command buffer to the initial state

# C Specifications
To reset a command buffer, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkResetCommandBuffer(
    VkCommandBuffer                             commandBuffer,
    VkCommandBufferResetFlags                   flags);
```

# Parameters
- [`command_buffer`] is the command buffer to reset. The command buffer  **can**  be in any state other than [pending](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle), and is moved into the [initial state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).
- [`flags`] is a bitmask of [`CommandBufferResetFlagBits`] controlling the reset operation.

# Description
Any primary command buffer that is in the [recording or executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle) and has [`command_buffer`] recorded into
it, becomes [invalid](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).
## Valid Usage
-  [`command_buffer`] **must**  not be in the [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-  [`command_buffer`] **must**  have been allocated from a pool that was created with the `VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT`

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`flags`] **must**  be a valid combination of [`CommandBufferResetFlagBits`] values

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`CommandBufferResetFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        