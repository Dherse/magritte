[`subset_allocation`] specifies whether logical devices created from
the group support allocating device memory on a subset of devices, via
the `deviceMask` member of the [`MemoryAllocateFlagsInfo`].
If this is [`FALSE`], then all device memory allocations are made
across all physical devices in the group.
If [`physical_device_count`] is `1`, then [`subset_allocation`] **must** 
be [`FALSE`].