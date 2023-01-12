[VkIndirectStateFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectStateFlagBitsNV.html) - Bitmask specifiying state that can be altered on the device

# C Specifications
A subset of the graphics pipeline state  **can**  be altered using indirect state
flags:
```c
// Provided by VK_NV_device_generated_commands
typedef enum VkIndirectStateFlagBitsNV {
    VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV = 0x00000001,
} VkIndirectStateFlagBitsNV;
```

# Description
- [`VK_INDIRECT_STATE_FLAG_BITS_NV`] allows to toggle the [`FrontFace`] rasterization state for subsequent draw operations.

# Related
- [`nv_device_generated_commands`]
- [VkIndirectStateFlagsNV]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        