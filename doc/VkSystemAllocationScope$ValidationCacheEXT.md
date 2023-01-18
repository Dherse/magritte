If an allocation is associated with a
[`ValidationCacheEXT`] or
[`PipelineCache`] object, the allocator will use the
[`CACHE`] allocation scope.
The most specific allocator available is used (cache, else device, else
instance).
Else,