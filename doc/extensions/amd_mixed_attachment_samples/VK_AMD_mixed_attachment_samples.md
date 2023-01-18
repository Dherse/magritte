[VK_AMD_mixed_attachment_samples](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_mixed_attachment_samples.html) - device extension

# Description
This extension enables applications to use multisampled rendering with a
depth/stencil sample count that is larger than the color sample count.
Having a depth/stencil sample count larger than the color sample count
allows maintaining geometry and coverage information at a higher sample rate
than color information.
All samples are depth/stencil tested, but only the first color sample count
number of samples get a corresponding color output.

# Registered extension number
137

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Matthaeus G. Chajdas [anteru](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_mixed_attachment_samples] @anteru%0A<<Here describe the issue or question you have about the VK_AMD_mixed_attachment_samples extension>>)

# New constants
- [`AMD_MIXED_ATTACHMENT_SAMPLES_EXTENSION_NAME`]
- [`AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION`]

# Known issues & F.A.Q.
None.

# Version history
- Revision 1, 2017-07-24 (Daniel Rakos)  - Internal revisions

# Other information
* 2017-07-24
*   - Mais Alnasser, AMD  - Matthaeus G. Chajdas, AMD  - Maciej Jesionowski, AMD  - Daniel Rakos, AMD
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        