[`fragment_density_map_non_subsampled_images`] specifies whether the
implementation supports regular non-subsampled image attachments with
fragment density map render passes.
If this feature is not enabled, render passes with a
[fragment density map
attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapattachment) **must**  only have [subsampled
attachments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-subsamplesampler) bound.