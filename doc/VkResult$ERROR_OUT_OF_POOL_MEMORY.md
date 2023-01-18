[`ERROR_OUT_OF_POOL_MEMORY`] A pool memory allocation has failed.
This  **must**  only be returned if no attempt to allocate host or device
memory was made to accommodate the new allocation.
If the failure was definitely due to fragmentation of the pool,
[`ERROR_FRAGMENTED_POOL`] **should**  be returned instead.