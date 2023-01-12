[VK_EXT_image_drm_format_modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_drm_format_modifier.html) - device extension

# Description
This extension provides the ability to use *DRM format modifiers* with
images, enabling Vulkan to better integrate with the Linux ecosystem of
graphics, video, and display APIs.Its functionality closely overlaps with
`EGL_EXT_image_dma_buf_import_modifiers`<sup>[2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn2)</sup>^
and
`EGL_MESA_image_dma_buf_export`<sup>[3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn3)</sup>^.
Unlike the EGL extensions, this extension does not require the use of a
specific handle type (such as a dma_buf) for external memory and provides
more explicit control of image creation.

# Registered extension number
159

# Revision
2

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_bind_memory2`]`
- Requires `[`khr_get_physical_device_properties2`]`
- Requires `[`khr_image_format_list`]`
- Requires `[`khr_sampler_ycbcr_conversion`]`

# Contacts
- Chad Versace [chadversary](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_image_drm_format_modifier] @chadversary%0A<<Here describe the issue or question you have about the VK_EXT_image_drm_format_modifier extension>>)

# New commands
- [`get_image_drm_format_modifier_properties_ext`]

# New structures
- [`DrmFormatModifierPropertiesEXT`]
- [`ImageDrmFormatModifierPropertiesEXT`]
- Extending [`FormatProperties2`]:  - [`DrmFormatModifierPropertiesListEXT`] 
- Extending [`ImageCreateInfo`]:  - [`ImageDrmFormatModifierExplicitCreateInfoEXT`]  - [`ImageDrmFormatModifierListCreateInfoEXT`] 
- Extending [`PhysicalDeviceImageFormatInfo2`]:  - [`PhysicalDeviceImageDrmFormatModifierInfoEXT`] 
If [`khr_format_feature_flags2`] is supported:
- [`DrmFormatModifierProperties2EXT`]
- Extending [`FormatProperties2`]:  - [`DrmFormatModifierPropertiesList2EXT`]

# New constants
- `VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME`
- `VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION`
- Extending [`ImageAspectFlagBits`]:  - `VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT`  - `VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT`  - `VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT`  - `VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT` 
- Extending [`ImageTiling`]:  - `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT` 
- Extending [`VulkanResultCodes`]:  - `VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT`  - `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT`  - `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT`  - `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT` 
If [`khr_format_feature_flags2`] is supported:
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT`

# Known issues & F.A.Q.
1) Should this extension define a single DRM format modifier per
[`Image`]? Or define one per plane?+ **RESOLVED** : There exists a single DRM format modifier per [`Image`]. **DISCUSSION** : Prior art, such as
`EGL_EXT_image_dma_buf_import_modifiers`<sup>[2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn2)</sup>^,
`struct drm_mode_fb_cmd2`<sup>[4](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn4)</sup>^, and
`struct
gbm_import_fd_modifier_data`<sup>[5](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn5)</sup>^,
allows defining one *modifier* per plane.
However, developers of the GBM and kernel APIs concede it was a mistake.
Beginning in Linux 4.10, the kernel requires that the application provide
the same DRM format *modifier* for each plane.
(See Linux commit
[bae781b259269590109e8a4a8227331362b88212](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=bae781b259269590109e8a4a8227331362b88212)).
And GBM provides an entry point, `gbm_bo_get_modifier`, for querying the
*modifier* of the image but does not provide one to query the modifier of
individual planes.2) When creating an image with
[`ImageDrmFormatModifierExplicitCreateInfoEXT`], which is typically used
when *importing* an image, should the application explicitly provide the
size of each plane?+ **RESOLVED** : No.
The application  **must**  not provide the size.
To enforce this, the API requires that
[`ImageDrmFormatModifierExplicitCreateInfoEXT`]::`pPlaneLayouts->size` **must**  be 0. **DISCUSSION** : Prior art, such as
`EGL_EXT_image_dma_buf_import_modifiers`<sup>[2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn2)</sup>^,
`struct drm_mode_fb_cmd2`<sup>[4](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn4)</sup>^, and
`struct
gbm_import_fd_modifier_data`<sup>[5](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn5)</sup>^,
omits from the API the size of each plane.
Instead, the APIs infer each plane’s size from the import parameters, which
include the image’s pixel format and a dma_buf, offset, and row pitch for
each plane.However, Vulkan differs from EGL and GBM with regards to image creation in
the following ways:
-  **Undedicated allocation by default.**  When importing or exporting a set of dma_bufs as an `EGLImage` or `gbm_bo`, common practice mandates that each dma_buf’s memory be dedicated (in the sense of [`khr_dedicated_allocation`]) to the image (though not necessarily dedicated to a single plane). In particular, neither the GBM documentation nor the EGL extension specifications explicitly state this requirement, but in light of common practice this is likely due to under-specification rather than intentional omission. In contrast, [`ext_image_drm_format_modifier`] permits, but does not require, the implementation to require dedicated allocations for images created with `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`.
-  **Separation of image creation and memory allocation.**  When importing a set of dma_bufs as an `EGLImage` or `gbm_bo`, EGL and GBM create the image resource and bind it to memory (the dma_bufs) simultaneously. This allows EGL and GBM to query each dma_buf’s size during image creation. In Vulkan, image creation and memory allocation are independent unless a dedicated allocation is used (as in [`khr_dedicated_allocation`]). Therefore, without requiring dedicated allocation, Vulkan cannot query the size of each dma_buf (or other external handle) when calculating the image’s memory layout. Even if dedication allocation were required, Vulkan cannot calculate the image’s memory layout until after the image is bound to its dma_ufs.
The above differences complicate the potential inference of plane size in
Vulkan.
Consider the following problematic cases:
-  **Padding.**  Some plane of the image may require implementation-dependent padding.
-  **Metadata.**  For some *modifiers*, the image may have a metadata plane which requires a non-trivial calculation to determine its size.
-  **Mipmapped, array, and 3D images.**  The implementation may support `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT` for images whose `mipLevels`, `arrayLayers`, or `depth` is greater than 1. For such images with certain *modifiers*, the calculation of each plane’s size may be non-trivial.
However, an application-provided plane size solves none of the above
problems.For simplicity, consider an external image with a single memory plane.
The implementation is obviously capable calculating the image’s size when
its tiling is `VK_IMAGE_TILING_OPTIMAL`.
Likewise, any reasonable implementation is capable of calculating the
image’s size when its tiling uses a supported *modifier*.Suppose that the external image’s size is smaller than the
implementation-calculated size.
If the application provided the external image’s size to
[`create_image`], the implementation would observe the mismatched size
and recognize its inability to comprehend the external image’s layout
(unless the implementation used the application-provided size to select a
refinement of the tiling layout indicated by the *modifier*, which is
strongly discouraged).
The implementation would observe the conflict, and reject image creation
with `VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT`.
On the other hand, if the application did not provide the external image’s
size to [`create_image`], then the application would observe after
calling [`get_image_memory_requirements`] that the external image’s size is
less than the size required by the implementation.
The application would observe the conflict and refuse to bind the
[`Image`] to the external memory.
In both cases, the result is explicit failure.Suppose that the external image’s size is larger than the
implementation-calculated size.
If the application provided the external image’s size to
[`create_image`], for reasons similar to above the implementation would
observe the mismatched size and recognize its inability to comprehend the
image data residing in the extra size.
The implementation, however, must assume that image data resides in the
entire size provided by the application.
The implementation would observe the conflict and reject image creation with
`VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT`.
On the other hand, if the application did not provide the external image’s
size to [`create_image`], then the application would observe after
calling [`get_image_memory_requirements`] that the external image’s size is
larger than the implementation-usable size.
The application would observe the conflict and refuse to bind the
[`Image`] to the external memory.
In both cases, the result is explicit failure.Therefore, an application-provided size provides no benefit, and this
extension should not require it.
This decision renders [`SubresourceLayout::size`] an unused field
during image creation, and thus introduces a risk that implementations may
require applications to submit sideband creation parameters in the unused
field.
To prevent implementations from relying on sideband data, this extension
*requires* the application to set `size` to 0.
### References

0. [`EGL_EXT_image_dma_buf_import`](https://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import.txt)
1. [`EGL_EXT_image_dma_buf_import_modifiers`](https://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import_modifiers.txt)
2. [`EGL_MESA_image_dma_buf_export`](https://www.khronos.org/registry/EGL/extensions/MESA/EGL_MESA_image_dma_buf_export.txt)
3. [`struct drm_mode_fb_cmd2`](https://git.kernel.org/cgit/linux/kernel/git/torvalds/linux.git/tree/include/uapi/drm/drm_mode.h?id=refs/tags/v4.10#n392)
4. [`gbm.h`](https://cgit.freedesktop.org/mesa/mesa/tree/src/gbm/main/gbm.h?id=refs/tags/mesa-18.0.0-rc1)

### Version History

- Revision 1, 2018-08-29 (Chad Versace)  - First stable revision 
- Revision 2, 2021-09-30 (Jon Leech)  - Add interaction with `[`khr_format_feature_flags2`]` to `vk.xml`

# Other information
* 2021-09-30
* No known IP claims.
*   - Antoine Labour, Google  - Bas Nieuwenhuizen, Google  - Chad Versace, Google  - James Jones, NVIDIA  - Jason Ekstrand, Intel  - Jőrg Wagner, ARM  - Kristian Høgsberg Kristensen, Google  - Ray Smith, ARM

# Related
- [`DrmFormatModifierPropertiesEXT`]
- [`DrmFormatModifierPropertiesListEXT`]
- [`ImageDrmFormatModifierExplicitCreateInfoEXT`]
- [`ImageDrmFormatModifierListCreateInfoEXT`]
- [`ImageDrmFormatModifierPropertiesEXT`]
- [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]
- [`get_image_drm_format_modifier_properties_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        