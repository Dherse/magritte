[vkGetShaderInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html) - Get information about a shader in a pipeline

# C Specifications
Information about a particular shader that has been compiled as part of a
pipeline object can be extracted by calling:
```c
// Provided by VK_AMD_shader_info
VkResult vkGetShaderInfoAMD(
    VkDevice                                    device,
    VkPipeline                                  pipeline,
    VkShaderStageFlagBits                       shaderStage,
    VkShaderInfoTypeAMD                         infoType,
    size_t*                                     pInfoSize,
    void*                                       pInfo);
```

# Parameters
- [`device`] is the device that created [`pipeline`].
- [`pipeline`] is the target of the query.
- [`shader_stage`] is a [`ShaderStageFlagBits`] specifying the particular shader within the pipeline about which information is being queried.
- [`info_type`] describes what kind of information is being queried.
- [`p_info_size`] is a pointer to a value related to the amount of data the query returns, as described below.
- [`p_info`] is either `NULL` or a pointer to a buffer.

# Description
If [`p_info`] is `NULL`, then the maximum size of the information that  **can** 
be retrieved about the shader, in bytes, is returned in [`p_info_size`].
Otherwise, [`p_info_size`] **must**  point to a variable set by the user to the
size of the buffer, in bytes, pointed to by [`p_info`], and on return the
variable is overwritten with the amount of data actually written to
[`p_info`].
If [`p_info_size`] is less than the maximum size that  **can**  be retrieved by
the pipeline cache, then at most [`p_info_size`] bytes will be written to
[`p_info`], and `VK_INCOMPLETE` will be returned, instead of
`VK_SUCCESS`, to indicate that not all required of the pipeline cache
was returned.Not all information is available for every shader and implementations may
not support all kinds of information for any shader.
When a certain type of information is unavailable, the function returns
`VK_ERROR_FEATURE_NOT_PRESENT`.If information is successfully and fully queried, the function will return
`VK_SUCCESS`.For [`info_type`]`VK_SHADER_INFO_TYPE_STATISTICS_AMD`, a
[`ShaderStatisticsInfoAMD`] structure will be written to the buffer
pointed to by [`p_info`].
This structure will be populated with statistics regarding the physical
device resources used by that shader along with other miscellaneous
information and is described in further detail below.For [`info_type`]`VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD`, [`p_info`] is
a pointer to a UTF-8 null-terminated string containing human-readable
disassembly.
The exact formatting and contents of the disassembly string are
vendor-specific.The formatting and contents of all other types of information, including
[`info_type`]`VK_SHADER_INFO_TYPE_BINARY_AMD`, are left to the vendor
and are not further specified by this extension.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`pipeline`] **must**  be a valid [`Pipeline`] handle
-  [`shader_stage`] **must**  be a valid [`ShaderStageFlagBits`] value
-  [`info_type`] **must**  be a valid [`ShaderInfoTypeAMD`] value
-  [`p_info_size`] **must**  be a valid pointer to a `size_t` value
-    If the value referenced by [`p_info_size`] is not `0`, and [`p_info`] is not `NULL`, [`p_info`] **must**  be a valid pointer to an array of [`p_info_size`] bytes
-  [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_FEATURE_NOT_PRESENT`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`VK_AMD_shader_info`]
- [`Device`]
- [`Pipeline`]
- [`ShaderInfoTypeAMD`]
- [`ShaderStageFlagBits`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        