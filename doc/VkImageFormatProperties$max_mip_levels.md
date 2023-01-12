[`max_mip_levels`] is the maximum number of mipmap levels.
[`max_mip_levels`] **must**  be equal to the number of levels in the
complete mipmap chain based on the `maxExtent.width`,
`maxExtent.height`, and `maxExtent.depth`, except
when one of the following conditions is true, in which case it  **may** 
instead be `1`:
 - [`get_physical_device_image_format_properties`]::`tiling` was `VK_IMAGE_TILING_LINEAR`
 - [`PhysicalDeviceImageFormatInfo2`]::`tiling` was `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`
 - the [`PhysicalDeviceImageFormatInfo2`]::`pNext` chain included a [`PhysicalDeviceExternalImageFormatInfo`] structure with a handle type included in the `handleTypes` member for which mipmap image support is not required
 - image `format` is one of the [formats that require a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
 - `flags` contains `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`