[`PRIMITIVE_TOPOLOGY`] specifies that the
`topology` state in [`PipelineInputAssemblyStateCreateInfo`]
only specifies the [topology class](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-topology-class),
and the specific topology order and adjacency  **must**  be set dynamically
with [`cmd_set_primitive_topology`] before any drawing commands.