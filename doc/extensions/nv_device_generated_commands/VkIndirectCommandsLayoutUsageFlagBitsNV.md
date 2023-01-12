[VkIndirectCommandsLayoutUsageFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html) - Bitmask specifying allowed usage of an indirect commands layout

# C Specifications
Bits which  **can**  be set in
[`IndirectCommandsLayoutCreateInfoNV::flags`], specifying usage
hints of an indirect command layout, are:
```c
// Provided by VK_NV_device_generated_commands
typedef enum VkIndirectCommandsLayoutUsageFlagBitsNV {
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV = 0x00000001,
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV = 0x00000002,
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV = 0x00000004,
} VkIndirectCommandsLayoutUsageFlagBitsNV;
```

# Description
- [`VK_INDIRECT_COMMANDS_LAYOUT_USAGE_FLAG_BITS_NV`] specifies that the layout is always used with the manual preprocessing step through calling [`cmd_preprocess_generated_commands_nv`] and executed by [`cmd_execute_generated_commands_nv`] with `isPreprocessed` set to `VK_TRUE`.
- [`VK_INDIRECT_COMMANDS_LAYOUT_USAGE_FLAG_BITS_NV`] specifies that the input data for the sequences is not implicitly indexed from 0..sequencesUsed but a user provided [`Buffer`] encoding the index is provided.
- [`VK_INDIRECT_COMMANDS_LAYOUT_USAGE_FLAG_BITS_NV`] specifies that the processing of sequences  **can**  happen at an implementation-dependent order, which is not: guaranteed to be coherent using the same input data.

# Related
- [`nv_device_generated_commands`]
- [VkIndirectCommandsLayoutUsageFlagsNV]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        