[VkShaderInfoTypeAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderInfoTypeAMD.html) - Enum specifying which type of shader information to query

# C Specifications
Possible values of [`get_shader_info_amd`]`::infoType`, specifying the
information being queried from a shader, are:
```c
// Provided by VK_AMD_shader_info
typedef enum VkShaderInfoTypeAMD {
    VK_SHADER_INFO_TYPE_STATISTICS_AMD = 0,
    VK_SHADER_INFO_TYPE_BINARY_AMD = 1,
    VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD = 2,
} VkShaderInfoTypeAMD;
```

# Description
- [`VK_SHADER_INFO_TYPE_AMD`] specifies that device resources used by a shader will be queried.
- [`VK_SHADER_INFO_TYPE_AMD`] specifies that implementation-specific information will be queried.
- [`VK_SHADER_INFO_TYPE_AMD`] specifies that human-readable dissassembly of a shader.

# Related
- [`amd_shader_info`]
- [`get_shader_info_amd`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        