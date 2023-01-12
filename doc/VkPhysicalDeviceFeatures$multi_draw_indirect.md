[`multi_draw_indirect`] specifies whether
multiple draw indirect is supported.
If this feature is not enabled, the `drawCount` parameter to the
[`cmd_draw_indirect`] and [`cmd_draw_indexed_indirect`] commands
 **must**  be 0 or 1.
The `maxDrawIndirectCount` member of the
[`PhysicalDeviceLimits`] structure  **must**  also be 1 if this feature
is not supported.
See [maxDrawIndirectCount](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxDrawIndirectCount).