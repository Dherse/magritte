If an allocation is associated with a
[`ValidationCacheEXT`] or
[`PipelineCache`] object, the allocator will use the
[`VK_SYSTEM_ALLOCATION_SCOPE`] allocation scope.
The most specific allocator available is used (cache, else device, else
instance).
Else,