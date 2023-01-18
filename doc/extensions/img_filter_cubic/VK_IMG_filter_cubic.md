[VK_IMG_filter_cubic](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_IMG_filter_cubic.html) - device extension

# Description
[`VK_IMG_filter_cubic`] adds an additional, high quality cubic filtering mode
to Vulkan, using a Catmull-Rom bicubic filter.
Performing this kind of filtering can be done in a shader by using 16
samples and a number of instructions, but this can be inefficient.
The cubic filter mode exposes an optimized high quality texture sampling
using fixed texture sampling functionality.

# Registered extension number
16

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_IMG_filter_cubic] @tobski%0A<<Here describe the issue or question you have about the VK_IMG_filter_cubic extension>>)

# New constants
- [`IMG_FILTER_CUBIC_EXTENSION_NAME`]
- [`IMG_FILTER_CUBIC_SPEC_VERSION`]
- Extending [`Filter`]:  - `VK_FILTER_CUBIC_IMG` 
- Extending [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG`

# Version history
- Revision 1, 2016-02-23 (Tobias Hector)  - Initial version

# Other information
* 2016-02-23
*   - Tobias Hector, Imagination Technologies
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        