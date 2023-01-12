[`query_flags`] specifies the query flags that  **can**  be used by an
active occlusion query in the primary command buffer when this secondary
command buffer is executed.
If this value includes the `VK_QUERY_CONTROL_PRECISE_BIT` bit, then
the active query  **can**  return boolean results or actual sample counts.
If this bit is not set, then the active query  **must**  not use the
`VK_QUERY_CONTROL_PRECISE_BIT` bit.