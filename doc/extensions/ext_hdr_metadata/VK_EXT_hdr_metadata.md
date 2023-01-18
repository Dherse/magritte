[VK_EXT_hdr_metadata](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_hdr_metadata.html) - device extension

# Description
This extension defines two new structures and a function to assign SMPTE
(the Society of Motion Picture and Television Engineers) 2086 metadata and
CTA (Consumer Technology Association) 861.3 metadata to a swapchain.
The metadata includes the color primaries, white point, and luminance range
of the reference monitor, which all together define the color volume
containing all the possible colors the reference monitor can produce.
The reference monitor is the display where creative work is done and
creative intent is established.
To preserve such creative intent as much as possible and achieve consistent
color reproduction on different viewing displays, it is useful for the
display pipeline to know the color volume of the original reference monitor
where content was created or tuned.
This avoids performing unnecessary mapping of colors that are not
displayable on the original reference monitor.
The metadata also includes the `maxContentLightLevel` and
`maxFrameAverageLightLevel` as defined by CTA 861.3.While the general purpose of the metadata is to assist in the transformation
between different color volumes of different displays and help achieve
better color reproduction, it is not in the scope of this extension to
define how exactly the metadata should be used in such a process.
It is up to the implementation to determine how to make use of the metadata.

# Registered extension number
106

# Revision
2

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_swapchain`]`

# Contacts
- Courtney Goeltzenleuchter [courtney-g](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_hdr_metadata] @courtney-g%0A<<Here describe the issue or question you have about the VK_EXT_hdr_metadata extension>>)

# New commands
- [`set_hdr_metadata_ext`]

# New structures
- [`HdrMetadataEXT`]
- [`XyColorEXT`]

# New constants
- [`EXT_HDR_METADATA_EXTENSION_NAME`]
- [`EXT_HDR_METADATA_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_HDR_METADATA_EXT`

# Known issues & F.A.Q.
1) Do we need a query function? **PROPOSED** : No, Vulkan does not provide queries for state that the
application can track on its own.2) Should we specify default if not specified by the application? **PROPOSED** : No, that leaves the default up to the display.

# Version history
- Revision 1, 2016-12-27 (Courtney Goeltzenleuchter)  - Initial version 
- Revision 2, 2018-12-19 (Courtney Goeltzenleuchter)  - Correct implicit validity for VkHdrMetadataEXT structure

# Other information
* 2018-12-19
* No known IP claims.
*   - Courtney Goeltzenleuchter, Google

# Related
- [`HdrMetadataEXT`]
- [`XyColorEXT`]
- [`set_hdr_metadata_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        