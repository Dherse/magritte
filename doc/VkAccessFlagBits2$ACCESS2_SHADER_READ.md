[`ACCESS2_SHADER_READ`]
specifies read access to a [shader binding
table](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shader-binding-table) in any shader pipeline.
In addition, it
is equivalent to the logical OR of:
 - [`ACCESS2_UNIFORM_READ`]
 - [`ACCESS2_SHADER_SAMPLED_READ`]
 - [`ACCESS2_SHADER_STORAGE_READ`]