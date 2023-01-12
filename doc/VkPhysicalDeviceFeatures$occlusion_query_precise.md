[`occlusion_query_precise`] specifies
whether occlusion queries returning actual sample counts are supported.
Occlusion queries are created in a [`QueryPool`] by specifying the
`queryType` of `VK_QUERY_TYPE_OCCLUSION` in the
[`QueryPoolCreateInfo`] structure which is passed to
[`create_query_pool`].
If this feature is enabled, queries of this type  **can**  enable
`VK_QUERY_CONTROL_PRECISE_BIT` in the `flags` parameter to
[`cmd_begin_query`].
If this feature is not supported, the implementation supports only
boolean occlusion queries.
When any samples are passed, boolean queries will return a non-zero
result value, otherwise a result value of zero is returned.
When this feature is enabled and `VK_QUERY_CONTROL_PRECISE_BIT` is
set, occlusion queries will report the actual number of samples passed.