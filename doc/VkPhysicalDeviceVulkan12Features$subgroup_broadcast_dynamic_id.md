If
[`subgroup_broadcast_dynamic_id`] is `VK_TRUE`, the “Id” operand of
`OpGroupNonUniformBroadcast` **can**  be dynamically uniform within a
subgroup, and the “Index” operand of
`OpGroupNonUniformQuadBroadcast` **can**  be dynamically uniform within
the derivative group.
If it is `VK_FALSE`, these operands  **must**  be constants.