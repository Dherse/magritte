[`max_lod`] is used to clamp the [maximum of the computed LOD value](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-level-of-detail-operation).
To avoid clamping the maximum value, set [`max_lod`] to the constant
`VK_LOD_CLAMP_NONE`.