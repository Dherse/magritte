[VK_AMD_shader_info](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_info.html) - device extension

# Description
This extension adds a way to query certain information about a compiled
shader which is part of a pipeline.
This information may include shader disassembly, shader binary and various
statistics about a shader’s resource usage.While this extension provides a mechanism for extracting this information,
the details regarding the contents or format of this information are not
specified by this extension and may be provided by the vendor externally.Furthermore, all information types are optionally supported, and users
should not assume every implementation supports querying every type of
information.

# Registered extension number
43

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Jaakko Konttinen [jaakkoamd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_shader_info] @jaakkoamd%0A<<Here describe the issue or question you have about the VK_AMD_shader_info extension>>)

# New commands
- [`get_shader_info_amd`]

# New structures
- [`ShaderResourceUsageAMD`]
- [`ShaderStatisticsInfoAMD`]

# New enums
- [`ShaderInfoTypeAMD`]

# New constants
- [`AMD_SHADER_INFO_EXTENSION_NAME`]
- [`AMD_SHADER_INFO_SPEC_VERSION`]

# Version history
- Revision 1, 2017-10-09 (Jaakko Konttinen)  - Initial revision

# Other information
* 2017-10-09
* No known IP claims.
*   - Jaakko Konttinen, AMD

# Related
- [`ShaderInfoTypeAMD`]
- [`ShaderResourceUsageAMD`]
- [`ShaderStatisticsInfoAMD`]
- [`get_shader_info_amd`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        