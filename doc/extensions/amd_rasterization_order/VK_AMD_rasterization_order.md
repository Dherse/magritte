[VK_AMD_rasterization_order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_rasterization_order.html) - device extension

# Description
This extension introduces the possibility for the application to control the
order of primitive rasterization.
In unextended Vulkan, the following stages are guaranteed to execute in *API
order*:
- depth bounds test
- stencil test, stencil op, and stencil write
- depth test and depth write
- occlusion queries
- blending, logic op, and color write
This extension enables applications to opt into a relaxed, implementation
defined primitive rasterization order that may allow better parallel
processing of primitives and thus enabling higher primitive throughput.
It is applicable in cases where the primitive rasterization order is known
to not affect the output of the rendering or any differences caused by a
different rasterization order are not a concern from the point of view of
the application’s purpose.A few examples of cases when using the relaxed primitive rasterization order
would not have an effect on the final rendering:
- If the primitives rendered are known to not overlap in framebuffer space.
- If depth testing is used with a comparison operator of `VK_COMPARE_OP_LESS`, `VK_COMPARE_OP_LESS_OR_EQUAL`, `VK_COMPARE_OP_GREATER`, or `VK_COMPARE_OP_GREATER_OR_EQUAL`, and the primitives rendered are known to not overlap in clip space.
- If depth testing is not used and blending is enabled for all attachments with a commutative blend operator.

# Registered extension number
19

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_rasterization_order] @drakos-amd%0A<<Here describe the issue or question you have about the VK_AMD_rasterization_order extension>>)

# New structures
- Extending [`PipelineRasterizationStateCreateInfo`]:  - [`PipelineRasterizationStateRasterizationOrderAMD`]

# New enums
- [`RasterizationOrderAMD`]

# New constants
- [`AMD_RASTERIZATION_ORDER_EXTENSION_NAME`]
- [`AMD_RASTERIZATION_ORDER_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD`

# Known issues & F.A.Q.
1) How is this extension useful to application developers? **RESOLVED** : Allows them to increase primitive throughput for cases when
strict API order rasterization is not important due to the nature of the
content, the configuration used, or the requirements towards the output of
the rendering.2) How does this extension interact with content optimizations aiming to
reduce overdraw by appropriately ordering the input primitives? **RESOLVED** : While the relaxed rasterization order might somewhat limit the
effectiveness of such content optimizations, most of the benefits of it are
expected to be retained even when the relaxed rasterization order is used,
so applications  **should**  still apply these optimizations even if they intend
to use the extension.3) Are there any guarantees about the primitive rasterization order when
using the new relaxed mode? **RESOLVED** : No.
In this case the rasterization order is completely implementation-dependent,
but in practice it is expected to partially still follow the order of
incoming primitives.4) Does the new relaxed rasterization order have any adverse effect on
repeatability and other invariance rules of the API? **RESOLVED** : Yes, in the sense that it extends the list of exceptions when
the repeatability requirement does not apply.

# Version history
- Revision 1, 2016-04-25 (Daniel Rakos)  - Initial draft.

# Other information
* 2016-04-25
* No known IP claims.
*   - Matthaeus G. Chajdas, AMD  - Jaakko Konttinen, AMD  - Daniel Rakos, AMD  - Graham Sellers, AMD  - Dominik Witczak, AMD

# Related
- [`PipelineRasterizationStateRasterizationOrderAMD`]
- [`RasterizationOrderAMD`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        