[VK_NV_shading_rate_image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_shading_rate_image.html) - device extension

# Description
This extension allows applications to use a variable shading rate when
processing fragments of rasterized primitives.
By default, Vulkan will spawn one fragment shader for each pixel covered by
a primitive.
In this extension, applications can bind a *shading rate image* that can be
used to vary the number of fragment shader invocations across the
framebuffer.
Some portions of the screen may be configured to spawn up to 16 fragment
shaders for each pixel, while other portions may use a single fragment
shader invocation for a 4x4 block of pixels.
This can be useful for use cases like eye tracking, where the portion of the
framebuffer that the user is looking at directly can be processed at high
frequency, while distant corners of the image can be processed at lower
frequency.
Each texel in the shading rate image represents a fixed-size rectangle in
the framebuffer, covering 16x16 pixels in the initial implementation of this
extension.
When rasterizing a primitive covering one of these rectangles, the Vulkan
implementation reads a texel in the bound shading rate image and looks up
the fetched value in a palette to determine a base shading rate.In addition to the API support controlling rasterization, this extension
also adds Vulkan support for the
[`SPV_NV_shading_rate`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_shading_rate.html) extension to
SPIR-V.
That extension provides two fragment shader variable decorations that allow
fragment shaders to determine the shading rate used for processing the
fragment:
- `FragmentSizeNV`, which indicates the width and height of the set of pixels processed by the fragment shader.
- `InvocationsPerPixel`, which indicates the maximum number of fragment shader invocations that could be spawned for the pixel(s) covered by the fragment.
When using SPIR-V in conjunction with the OpenGL Shading Language (GLSL),
the fragment shader capabilities are provided by the
`GL_NV_shading_rate_image` language extension and correspond to the built-in
variables `gl_FragmentSizeNV` and `gl_InvocationsPerPixelNV`,
respectively.

# Registered extension number
165

# Revision
3

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Pat Brown [nvpbrown](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_shading_rate_image] @nvpbrown%0A<<Here describe the issue or question you have about the VK_NV_shading_rate_image extension>>)

# New commands
- [`cmd_bind_shading_rate_image_nv`]
- [`cmd_set_coarse_sample_order_nv`]
- [`cmd_set_viewport_shading_rate_palette_nv`]

# New structures
- [`CoarseSampleLocationNV`]
- [`CoarseSampleOrderCustomNV`]
- [`ShadingRatePaletteNV`]
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceShadingRateImageFeaturesNV`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceShadingRateImagePropertiesNV`] 
- Extending [`PipelineViewportStateCreateInfo`]:  - [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`]  - [`PipelineViewportShadingRateImageStateCreateInfoNV`]

# New enums
- [`CoarseSampleOrderTypeNV`]
- [`ShadingRatePaletteEntryNV`]

# New constants
- [`NV_SHADING_RATE_IMAGE_EXTENSION_NAME`]
- [`NV_SHADING_RATE_IMAGE_SPEC_VERSION`]
- Extending [`AccessFlagBits`]:  - `VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV` 
- Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV`  - `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` 
- Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV` 
- Extending [`ImageUsageFlagBits`]:  - `VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV` 
- Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV`  - `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV`  - `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV`

# Known issues & F.A.Q.
(1) When using shading rates specifying “coarse” fragments covering
    multiple pixels, we will generate a combined coverage mask that combines
    the coverage masks of all pixels covered by the fragment.
    By default, these masks are combined in an implementation-dependent
    order.
    Should we provide a mechanism allowing applications to query or specify
    an exact order? **RESOLVED** : Yes, this feature is useful for cases where most of the fragment
shader can be evaluated once for an entire coarse fragment, but where some
per-pixel computations are also required.
For example, a per-pixel alpha test may want to kill all the samples for
some pixels in a coarse fragment.
This sort of test can be implemented using an output sample mask, but such a
shader would need to know which bit in the mask corresponds to each sample
in the coarse fragment.
We are including a mechanism to allow aplications to specify the orders of
coverage samples for each shading rate and sample count, either as static
pipeline state or dynamically via a command buffer.
This portion of the extension has its own feature bit.We will not be providing a query to determine the implementation-dependent
default ordering.
The thinking here is that if an application cares enough about the coarse
fragment sample ordering to perform such a query, it could instead just set
its own order, also using custom per-pixel sample locations if required.(2) For the pipeline stage
`VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`, should we specify a
precise location in the pipeline the shading rate image is accessed (after
geometry shading, but before the early fragment tests) or leave it
under-specified in case there are other implementations that access the
image in a different pipeline location? **RESOLVED**  We are specifying the pipeline stage to be between the final
[pre-rasterization shader
stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization) (`VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`) and before the first
stage used for fragment processing
(`VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT`), which seems to be the
natural place to access the shading rate image.(3) How do centroid-sampled variables work with fragments larger than one
pixel? **RESOLVED**  For single-pixel fragments, fragment shader inputs decorated with
`Centroid` are sampled at an implementation-dependent location in the
intersection of the area of the primitive being rasterized and the area of
the pixel that corresponds to the fragment.
With multi-pixel fragments, we follow a similar pattern, using the
intersection of the primitive and the  **set**  of pixels corresponding to the
fragment.One important thing to keep in mind when using such “coarse” shading rates
is that fragment attributes are sampled at the center of the fragment by
default, regardless of the set of pixels/samples covered by the fragment.
For fragments with a size of 4x4 pixels, this center location will be more
than two pixels (1.5 * sqrt(2)) away from the center of the pixels at the
corners of the fragment.
When rendering a primitive that covers only a small part of a coarse
fragment, sampling a color outside the primitive can produce overly bright
or dark color values if the color values have a large gradient.
To deal with this, an application can use centroid sampling on attributes
where “extrapolation” artifacts can lead to overly bright or dark pixels.
Note that this same problem also exists for multisampling with single-pixel
fragments, but is less severe because it only affects certain samples of a
pixel and such bright/dark samples may be averaged with other samples that
do not have a similar problem.

# Version history
- Revision 3, 2019-07-18 (Mathias Schott)  - Fully list extension interfaces in this appendix. 
- Revision 2, 2018-09-13 (Pat Brown)  - Miscellaneous edits preparing the specification for publication. 
- Revision 1, 2018-08-08 (Pat Brown)  - Internal revisions

# Other information
* 2019-07-18
*   - This extension requires [`SPV_NV_shading_rate`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_shading_rate.html)  - This extension provides API support for [`GL_NV_shading_rate_image`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_shading_rate_image.txt) 
*   - Pat Brown, NVIDIA  - Carsten Rohde, NVIDIA  - Jeff Bolz, NVIDIA  - Daniel Koch, NVIDIA  - Mathias Schott, NVIDIA  - Matthew Netsch, Qualcomm Technologies, Inc.

# Related
- [`CoarseSampleLocationNV`]
- [`CoarseSampleOrderCustomNV`]
- [`CoarseSampleOrderTypeNV`]
- [`PhysicalDeviceShadingRateImageFeaturesNV`]
- [`PhysicalDeviceShadingRateImagePropertiesNV`]
- [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`]
- [`PipelineViewportShadingRateImageStateCreateInfoNV`]
- [`ShadingRatePaletteEntryNV`]
- [`ShadingRatePaletteNV`]
- [`cmd_bind_shading_rate_image_nv`]
- [`cmd_set_coarse_sample_order_nv`]
- [`cmd_set_viewport_shading_rate_palette_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        