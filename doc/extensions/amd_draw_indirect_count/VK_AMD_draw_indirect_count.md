[VK_AMD_draw_indirect_count](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_draw_indirect_count.html) - device extension

# Description
This extension allows an application to source the number of draws for
indirect drawing commands from a buffer.
This enables applications to generate an arbitrary number of drawing
commands and execute them without host intervention.

# Registered extension number
34

# Revision
2

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Promoted* to `[`VK_KHR_draw_indirect_count`]` extension  - Which in turn was *promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)

# Contacts
- Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_draw_indirect_count] @drakos-amd%0A<<Here describe the issue or question you have about the VK_AMD_draw_indirect_count extension>>)

# New commands
- [`cmd_draw_indexed_indirect_count_amd`]
- [`cmd_draw_indirect_count_amd`]

# New constants
- [`AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME`]
- [`AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION`]

# Version history
- Revision 2, 2016-08-23 (Dominik Witczak)  - Minor fixes 
- Revision 1, 2016-07-21 (Matthaeus Chajdas)  - Initial draft

# Other information
* 2016-08-23
*   - Promoted to `[`VK_KHR_draw_indirect_count`]` 
* No known IP claims.
*   - Matthaeus G. Chajdas, AMD  - Derrick Owens, AMD  - Graham Sellers, AMD  - Daniel Rakos, AMD  - Dominik Witczak, AMD

# Related
- [`cmd_draw_indexed_indirect_count_amd`]
- [`cmd_draw_indirect_count_amd`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        