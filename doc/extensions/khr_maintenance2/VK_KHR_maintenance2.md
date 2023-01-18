[VK_KHR_maintenance2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance2.html) - device extension

# Description
[`VK_KHR_maintenance2`] adds a collection of minor features that were
intentionally left out or overlooked from the original Vulkan 1.0 release.The new features are as follows:
- Allow the application to specify which aspect of an input attachment might be read for a given subpass.
- Allow implementations to express the clipping behavior of points.
- Allow creating images with usage flags that may not be supported for the base image’s format, but are supported for image views of the image that have a different but compatible format.
- Allow creating uncompressed image views of compressed images.
- Allow the application to select between an upper-left and lower-left origin for the tessellation domain space.
- Adds two new image layouts for depth stencil images to allow either the depth or stencil aspect to be read-only while the other aspect is writable.

# Registered extension number
118

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)

# Contacts
- Michael Worcester [michaelworcester](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_maintenance2] @michaelworcester%0A<<Here describe the issue or question you have about the VK_KHR_maintenance2 extension>>)

# New structures
- [`InputAttachmentAspectReferenceKHR`]
- Extending [`ImageViewCreateInfo`]:  - [`ImageViewUsageCreateInfoKHR`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDevicePointClippingPropertiesKHR`] 
- Extending [`PipelineTessellationStateCreateInfo`]:  - [`PipelineTessellationDomainOriginStateCreateInfoKHR`] 
- Extending [`RenderPassCreateInfo`]:  - [`RenderPassInputAttachmentAspectCreateInfoKHR`]

# New enums
- [`PointClippingBehaviorKHR`]
- [`TessellationDomainOriginKHR`]

# New constants
- [`KHR_MAINTENANCE2_EXTENSION_NAME`]
- [`KHR_MAINTENANCE2_SPEC_VERSION`]
- [`KHR_MAINTENANCE_2_EXTENSION_NAME`]
- [`KHR_MAINTENANCE_2_SPEC_VERSION`]
- Extending [`ImageCreateFlagBits`]:  - `VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR`  - `VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR` 
- Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR`  - `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR` 
- Extending [`PointClippingBehavior`]:  - `VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR`  - `VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR`  - `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR`  - `VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR` 
- Extending [`TessellationDomainOrigin`]:  - `VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR`  - `VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR`

# Known issues & F.A.Q.
1) What is the default tessellation domain origin? **RESOLVED** : Vulkan 1.0 originally inadvertently documented a lower-left
origin, but the conformance tests and all implementations implemented an
upper-left origin.
This extension adds a control to select between lower-left (for
compatibility with OpenGL) and upper-left, and we retroactively fix
unextended Vulkan to have a default of an upper-left origin.

# Version history
- Revision 1, 2017-04-28

# Other information
* 2017-09-05
*   - Promoted to Vulkan 1.1 Core 
*   - Michael Worcester, Imagination Technologies  - Stuart Smith, Imagination Technologies  - Jeff Bolz, NVIDIA  - Daniel Koch, NVIDIA  - Jan-Harald Fredriksen, ARM  - Daniel Rakos, AMD  - Neil Henning, Codeplay  - Piers Daniell, NVIDIA

# Related
- [`ImageViewUsageCreateInfoKHR`]
- [`InputAttachmentAspectReferenceKHR`]
- [`PhysicalDevicePointClippingPropertiesKHR`]
- [`PipelineTessellationDomainOriginStateCreateInfoKHR`]
- [`PointClippingBehaviorKHR`]
- [`RenderPassInputAttachmentAspectCreateInfoKHR`]
- [`TessellationDomainOriginKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        