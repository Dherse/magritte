[`occlusion_query_enable`] specifies whether the command buffer  **can**  be
executed while an occlusion query is active in the primary command
buffer.
If this is `VK_TRUE`, then this command buffer  **can**  be executed
whether the primary command buffer has an occlusion query active or not.
If this is `VK_FALSE`, then the primary command buffer  **must**  not
have an occlusion query active.