[VkBindVertexBufferIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindVertexBufferIndirectCommandNV.html) - Structure specifying input data for a single vertex buffer command token

# C Specifications
The [`BindVertexBufferIndirectCommandNV`] structure specifies the input
data for the `VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV` token.
```c
// Provided by VK_NV_device_generated_commands
typedef struct VkBindVertexBufferIndirectCommandNV {
    VkDeviceAddress    bufferAddress;
    uint32_t           size;
    uint32_t           stride;
} VkBindVertexBufferIndirectCommandNV;
```

# Members
- [`buffer_address`] specifies a physical address of the [`Buffer`] used as vertex input binding.
- [`size`] is the byte size range which is available for this operation from the provided address.
- [`stride`] is the byte size stride for this vertex input binding as in [`VertexInputBindingDescription`]::[`stride`]. It is only used if [`IndirectCommandsLayoutTokenNV::vertex_dynamic_stride`] was set, otherwise the stride is inherited from the current bound graphics pipeline.

# Description
## Valid Usage
-    The buffer’s usage flag from which the address was acquired  **must**  have the `VK_BUFFER_USAGE_VERTEX_BUFFER_BIT` bit set
-    Each element of the buffer from which the address was acquired and that is non-sparse  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object

# Related
- [`VK_NV_device_generated_commands`]
- [`DeviceAddress`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        