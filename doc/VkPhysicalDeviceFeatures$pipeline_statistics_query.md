[`pipeline_statistics_query`]
specifies whether the pipeline statistics queries are supported.
If this feature is not enabled, queries of type
`VK_QUERY_TYPE_PIPELINE_STATISTICS` **cannot**  be created, and none of
the [`QueryPipelineStatisticFlagBits`] bits  **can**  be set in the
`pipelineStatistics` member of the [`QueryPoolCreateInfo`]
structure.