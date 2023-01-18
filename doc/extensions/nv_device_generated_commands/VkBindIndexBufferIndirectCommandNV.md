[VkBindIndexBufferIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindIndexBufferIndirectCommandNV.html) - Structure specifying input data for a single index buffer command token

# C Specifications
The [`BindIndexBufferIndirectCommandNV`] structure specifies the input
data for the `VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV` token.
```c
// Provided by VK_NV_device_generated_commands
typedef struct VkBindIndexBufferIndirectCommandNV {
    VkDeviceAddress    bufferAddress;
    uint32_t           size;
    VkIndexType        indexType;
} VkBindIndexBufferIndirectCommandNV;
```

# Members
- [`buffer_address`] specifies a physical address of the [`Buffer`] used as index buffer.
- [`size`] is the byte size range which is available for this operation from the provided address.
- [`index_type`] is a [`IndexType`] value specifying how indices are treated. Instead of the Vulkan enum values, a custom `uint32_t` value  **can**  be mapped to an [`IndexType`] by specifying the [`IndirectCommandsLayoutTokenNV::index_types`] and [`IndirectCommandsLayoutTokenNV::index_type_values`] arrays.

# Description
## Valid Usage
-    The buffer’s usage flag from which the address was acquired  **must**  have the `VK_BUFFER_USAGE_INDEX_BUFFER_BIT` bit set
-    The [`buffer_address`] **must**  be aligned to the [`index_type`] used
-    Each element of the buffer from which the address was acquired and that is non-sparse  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object

## Valid Usage (Implicit)
-  [`index_type`] **must**  be a valid [`IndexType`] value

# Related
- [`VK_NV_device_generated_commands`]
- [`DeviceAddress`]
- [`IndexType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        