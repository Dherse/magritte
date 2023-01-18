[VK_EXT_separate_stencil_usage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_separate_stencil_usage.html) - device extension

# Description
This extension allows specifying separate usage flags for the stencil aspect
of images with a depth-stencil format at image creation time.

# Registered extension number
247

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)

# Contacts
- Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_separate_stencil_usage] @drakos-amd%0A<<Here describe the issue or question you have about the VK_EXT_separate_stencil_usage extension>>)

# New structures
- Extending [`ImageCreateInfo`], [`PhysicalDeviceImageFormatInfo2`]:  - [`ImageStencilUsageCreateInfoEXT`]

# New constants
- [`EXT_SEPARATE_STENCIL_USAGE_EXTENSION_NAME`]
- [`EXT_SEPARATE_STENCIL_USAGE_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO_EXT`

# Version history
- Revision 1, 2018-11-08 (Daniel Rakos)  - Internal revisions.

# Other information
* 2018-11-08
*   - Promoted to Vulkan 1.2 Core 
* No known IP claims.
*   - Daniel Rakos, AMD  - Jordan Logan, AMD

# Related
- [`ImageStencilUsageCreateInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        