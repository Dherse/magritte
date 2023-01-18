[VK_NV_corner_sampled_image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_corner_sampled_image.html) - device extension

# Description
This extension adds support for a new image organization, which this
extension refers to as “corner-sampled” images.
A corner-sampled image differs from a conventional image in the following
ways:
- Texels are centered on integer coordinates. See [Unnormalized Texel Coordinate Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-unnormalized-to-integer)
- Normalized coordinates are scaled using coord × (dim - 1) rather than coord × dim, where dim is the size of one dimension of the image. See [normalized texel coordinate transform](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-normalized-to-unnormalized).
- Partial derivatives are scaled using coord × (dim - 1) rather than coord × dim. See [Scale Factor Operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-scale-factor).
- Calculation of the next higher lod size goes according to ⌈dim / 2⌉ rather than ⌊dim / 2⌋. See [Image Miplevel Sizing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-miplevel-sizing).
- The minimum level size is 2x2 for 2D images and 2x2x2 for 3D images. See [Image Miplevel Sizing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-miplevel-sizing).
This image organization is designed to facilitate a system like Ptex with
separate textures for each face of a subdivision or polygon mesh.
Placing sample locations at pixel corners allows applications to maintain
continuity between adjacent patches by duplicating values along shared
edges.
Additionally, using the modified mipmapping logic along with texture
dimensions of the form 2<sup>n</sup>+1 allows continuity across shared edges
even if the adjacent patches use different level-of-detail values.

# Registered extension number
51

# Revision
2

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_corner_sampled_image] @dgkoch%0A<<Here describe the issue or question you have about the VK_NV_corner_sampled_image extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceCornerSampledImageFeaturesNV`]

# New constants
- [`NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME`]
- [`NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION`]
- Extending [`ImageCreateFlagBits`]:  - `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV`

# Known issues & F.A.Q.
0. What should this extension be named? **DISCUSSION** : While naming this extension, we chose the most distinctive aspect of the image organization and referred to such images as “corner-sampled images”. As a result, we decided to name the extension NV_corner_sampled_image.
1. Do we need a format feature flag so formats can advertise if they support corner-sampling? **DISCUSSION** : Currently NVIDIA supports this for all 2D and 3D formats, but not for cube maps or depth-stencil formats. A format feature might be useful if other vendors would only support this on some formats.
2. Do integer texel coordinates have a different range for corner-sampled images? **RESOLVED** : No, these are unchanged.
3. Do unnormalized sampler coordinates work with corner-sampled images? Are there any functional differences? **RESOLVED** : Yes. Unnormalized coordinates are treated as already scaled for corner-sample usage.
4. Should we have a diagram in the “Image Operations” chapter demonstrating different texel sampling locations? **UNRESOLVED** : Probaby, but later.

# Version history
- Revision 1, 2018-08-14 (Daniel Koch)  - Internal revisions 
- Revision 2, 2018-08-14 (Daniel Koch)  - ???

# Other information
* 2018-08-13
*   - Jeff Bolz, NVIDIA  - Pat Brown, NVIDIA  - Chris Lentini, NVIDIA

# Related
- [`PhysicalDeviceCornerSampledImageFeaturesNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        