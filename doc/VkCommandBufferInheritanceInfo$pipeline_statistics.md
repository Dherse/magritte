[`pipeline_statistics`] is a bitmask of
[`QueryPipelineStatisticFlagBits`] specifying the set of pipeline
statistics that  **can**  be counted by an active query in the primary
command buffer when this secondary command buffer is executed.
If this value includes a given bit, then this command buffer  **can**  be
executed whether the primary command buffer has a pipeline statistics
query active that includes this bit or not.
If this value excludes a given bit, then the active pipeline statistics
query  **must**  not be from a query pool that counts that statistic.