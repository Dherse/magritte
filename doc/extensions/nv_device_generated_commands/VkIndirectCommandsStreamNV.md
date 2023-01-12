[VkIndirectCommandsStreamNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsStreamNV.html) - Structure specifying input streams for generated command tokens

# C Specifications
The [`IndirectCommandsStreamNV`] structure specifies the input data for
one or more tokens at processing time.
```c
// Provided by VK_NV_device_generated_commands
typedef struct VkIndirectCommandsStreamNV {
    VkBuffer        buffer;
    VkDeviceSize    offset;
} VkIndirectCommandsStreamNV;
```

# Members
- [`buffer`] specifies the [`Buffer`] storing the functional arguments for each sequence. These arguments  **can**  be written by the device.
- [`offset`] specified an offset into [`buffer`] where the arguments start.

# Description
## Valid Usage
-    The [`buffer`]’s usage flag  **must**  have the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
-    The [`offset`] **must**  be aligned to [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::min_indirect_commands_buffer_offset_alignment`]
-    If [`buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object

## Valid Usage (Implicit)
-  [`buffer`] **must**  be a valid [`Buffer`] handle

# Related
- [`nv_device_generated_commands`]
- [`Buffer`]
- [`DeviceSize`]
- [`GeneratedCommandsInfoNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        