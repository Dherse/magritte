[`VK_DESCRIPTOR_POOL_CREATE_FLAG_BITS`] specifies that
descriptor sets  **can**  return their individual allocations to the pool,
i.e. all of [`allocate_descriptor_sets`], [`free_descriptor_sets`],
and [`reset_descriptor_pool`] are allowed.
Otherwise, descriptor sets allocated from the pool  **must**  not be
individually freed back to the pool, i.e. only
[`allocate_descriptor_sets`] and [`reset_descriptor_pool`] are
allowed.