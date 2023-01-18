[`SYNC_FD`] specifies a POSIX
file descriptor handle to a Linux Sync File or Android Fence object.
It can be used with any native API accepting a valid sync file or fence
as input.
It owns a reference to the underlying synchronization primitive
associated with the file descriptor.
Implementations which support importing this handle type  **must**  accept
any type of sync or fence FD supported by the native system they are
running on.