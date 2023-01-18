[`INPUT_ASSEMBLY_PRIMITIVES`]
specifies that queries managed by the pool will count the number of
primitives processed by the [input assembly](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing) stage.
If primitive restart is enabled, restarting the primitive topology has
no effect on the count.
Incomplete primitives  **may**  be counted.