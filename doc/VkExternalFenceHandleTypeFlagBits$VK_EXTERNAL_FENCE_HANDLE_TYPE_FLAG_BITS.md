[`VK_EXTERNAL_FENCE_HANDLE_TYPE_FLAG_BITS`] specifies a POSIX file
descriptor handle that has only limited valid usage outside of Vulkan
and other compatible APIs.
It  **must**  be compatible with the POSIX system calls `dup`, `dup2`,
`close`, and the non-standard system call `dup3`.
Additionally, it  **must**  be transportable over a socket using an
`SCM_RIGHTS` control message.
It owns a reference to the underlying synchronization primitive
represented by its Vulkan fence object.