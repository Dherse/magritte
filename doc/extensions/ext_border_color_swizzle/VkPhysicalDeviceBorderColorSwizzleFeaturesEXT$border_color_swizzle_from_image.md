[`border_color_swizzle_from_image`] indicates that the implementation will
return the correct border color values from sampled image operations
under the conditions expressed above, without the application having to
specify the border color component mapping when creating the sampler
object.
If this feature bit is not set, applications  **can**  chain a
[`SamplerBorderColorComponentMappingCreateInfoEXT`] structure when
creating samplers for use with image views that do not have an
[identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings) and, when
those samplers are combined with image views using the same component
mapping, sampled image operations that use opaque black or custom border
colors will return the correct border color values.