[`ERROR_FRAGMENTED_POOL`] A pool allocation has failed due to
fragmentation of the pool’s memory.
This  **must**  only be returned if no attempt to allocate host or device
memory was made to accommodate the new allocation.
This  **should**  be returned in preference to
[`ERROR_OUT_OF_POOL_MEMORY`], but only if the implementation is
certain that the pool allocation failure was due to fragmentation.