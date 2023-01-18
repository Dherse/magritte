[`UPDATE_UNUSED_WHILE_PENDING`] indicates
that descriptors in this binding  **can**  be updated after a command buffer
has bound this descriptor set, or while a command buffer that uses this
descriptor set is pending execution, as long as the descriptors that are
updated are not used by those command buffers.
If [`PARTIALLY_BOUND`] is also set, then
descriptors  **can**  be updated as long as they are not dynamically used by
any shader invocations.
If [`PARTIALLY_BOUND`] is not set, then
descriptors  **can**  be updated as long as they are not statically used by
any shader invocations.