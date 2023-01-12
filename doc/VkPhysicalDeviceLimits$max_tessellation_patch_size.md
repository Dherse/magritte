[`max_tessellation_patch_size`] is
the maximum patch size, in vertices, of patches that  **can**  be processed
by the tessellation control shader and tessellation primitive generator.
The `patchControlPoints` member of the
[`PipelineTessellationStateCreateInfo`] structure specified at
pipeline creation time and the value provided in the `OutputVertices`
execution mode of shader modules  **must**  be less than or equal to this
limit.
See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation).