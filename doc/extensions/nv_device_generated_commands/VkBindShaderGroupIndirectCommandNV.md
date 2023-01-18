[VkBindShaderGroupIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindShaderGroupIndirectCommandNV.html) - Structure specifying input data for a single shader group command token

# C Specifications
The [`BindShaderGroupIndirectCommandNV`] structure specifies the input
data for the `VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV` token.
```c
// Provided by VK_NV_device_generated_commands
typedef struct VkBindShaderGroupIndirectCommandNV {
    uint32_t    groupIndex;
} VkBindShaderGroupIndirectCommandNV;
```

# Members
- `index` specifies which shader group of the current bound graphics pipeline is used.

# Description
## Valid Usage
-    The current bound graphics pipeline, as well as the pipelines it may reference,  **must**  have been created with `VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV`
-    The `index` **must**  be within range of the accessible shader groups of the current bound graphics pipeline. See [`cmd_bind_pipeline_shader_group_nv`] for further details

# Related
- [`VK_NV_device_generated_commands`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        