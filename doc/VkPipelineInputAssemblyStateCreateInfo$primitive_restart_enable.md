[`primitive_restart_enable`] controls whether a special vertex index
value is treated as restarting the assembly of primitives.
This enable only applies to indexed draws ([`cmd_draw_indexed`],
[`cmd_draw_multi_indexed_ext`],
and [`cmd_draw_indexed_indirect`]), and the special index value is
either 0xFFFFFFFF when the `indexType` parameter of
[`cmd_bind_index_buffer`] is equal to `VK_INDEX_TYPE_UINT32`,
0xFF when `indexType` is equal to `VK_INDEX_TYPE_UINT8_EXT`,
or 0xFFFF when `indexType` is equal to `VK_INDEX_TYPE_UINT16`.
Primitive restart is not allowed for “list” topologies, unless one of
the features
[`primitiveTopologyPatchListRestart`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveTopologyPatchListRestart)
(for `VK_PRIMITIVE_TOPOLOGY_PATCH_LIST`) or
[`primitiveTopologyListRestart`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveTopologyListRestart)
(for all other list topologies) is enabled.