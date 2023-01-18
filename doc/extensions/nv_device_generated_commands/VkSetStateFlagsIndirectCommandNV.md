[VkSetStateFlagsIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSetStateFlagsIndirectCommandNV.html) - Structure specifying input data for a single state flag command token

# C Specifications
The [`SetStateFlagsIndirectCommandNV`] structure specifies the input
data for the `VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV` token.
Which state is changed depends on the [`IndirectStateFlagBitsNV`]
specified at [`IndirectCommandsLayoutNV`] creation time.
```c
// Provided by VK_NV_device_generated_commands
typedef struct VkSetStateFlagsIndirectCommandNV {
    uint32_t    data;
} VkSetStateFlagsIndirectCommandNV;
```

# Members
- [`data`] encodes packed state that this command alters.  - Bit `0`: If set represents `VK_FRONT_FACE_CLOCKWISE`, otherwise `VK_FRONT_FACE_COUNTER_CLOCKWISE`

# Related
- [`VK_NV_device_generated_commands`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        