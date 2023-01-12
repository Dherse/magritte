[`conditional_rendering_enable`] specifies whether the command buffer
 **can**  be executed while conditional rendering is active in the primary
command buffer.
If this is `VK_TRUE`, then this command buffer  **can**  be executed
whether the primary command buffer has active conditional rendering or
not.
If this is `VK_FALSE`, then the primary command buffer  **must**  not
have conditional rendering active.