[`RELEASE_RESOURCES`] specifies that most
or all memory resources currently owned by the command buffer  **should**  be
returned to the parent command pool.
If this flag is not set, then the command buffer  **may**  hold onto memory
resources and reuse them when recording commands.
`commandBuffer` is moved to the [initial
state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).