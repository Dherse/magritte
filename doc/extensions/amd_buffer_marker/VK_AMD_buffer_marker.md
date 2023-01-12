[VK_AMD_buffer_marker](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_buffer_marker.html) - device extension

# Description
This extension adds a new operation to execute pipelined writes of small
marker values into a [`Buffer`] object.The primary purpose of these markers is to facilitate the development of
debugging tools for tracking which pipelined command contributed to device
loss.

# Registered extension number
180

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_buffer_marker] @drakos-amd%0A<<Here describe the issue or question you have about the VK_AMD_buffer_marker extension>>)

# New commands
- [`cmd_write_buffer_marker_amd`]

# New constants
- `VK_AMD_BUFFER_MARKER_EXTENSION_NAME`
- `VK_AMD_BUFFER_MARKER_SPEC_VERSION`

# Version history
- Revision 1, 2018-01-26 (Jaakko Konttinen)  - Initial revision

# Other information
* 2018-01-26
* No known IP claims.
*   - Matthaeus G. Chajdas, AMD  - Jaakko Konttinen, AMD  - Daniel Rakos, AMD

# Related
- [`cmd_write_buffer_marker_amd`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        