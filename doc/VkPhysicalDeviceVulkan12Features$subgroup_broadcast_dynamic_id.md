If
[`subgroup_broadcast_dynamic_id`] is [`TRUE`], the “Id” operand of
`OpGroupNonUniformBroadcast` **can**  be dynamically uniform within a
subgroup, and the “Index” operand of
`OpGroupNonUniformQuadBroadcast` **can**  be dynamically uniform within
the derivative group.
If it is [`FALSE`], these operands  **must**  be constants.