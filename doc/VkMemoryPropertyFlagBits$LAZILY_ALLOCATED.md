[`LAZILY_ALLOCATED`] bit specifies that the
memory type only allows device access to the memory.
Memory types  **must**  not have both
[`LAZILY_ALLOCATED`] and
[`HOST_VISIBLE`] set.
Additionally, the object’s backing memory  **may**  be provided by the
implementation lazily as specified in [Lazily Allocated Memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-device-lazy_allocation).