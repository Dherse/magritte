[`DRM_FORMAT_MODIFIER_EXT`] indicates that the image’s
tiling is defined by a [Linux DRM format
modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier).
The modifier is specified at image creation with
[`ImageDrmFormatModifierListCreateInfoEXT`] or
[`ImageDrmFormatModifierExplicitCreateInfoEXT`], and  **can**  be queried
with [`get_image_drm_format_modifier_properties_ext`].