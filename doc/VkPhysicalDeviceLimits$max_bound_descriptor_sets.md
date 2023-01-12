[`max_bound_descriptor_sets`] is the
maximum number of descriptor sets that  **can**  be simultaneously used by a
pipeline.
All [`DescriptorSet`] decorations in shader modules  **must**  have a value
less than [`max_bound_descriptor_sets`].
See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sets).