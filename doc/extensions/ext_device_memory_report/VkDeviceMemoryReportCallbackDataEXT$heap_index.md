[`heap_index`] describes which memory heap this device memory
allocation is made from.
If [`type_`] is `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT`
or `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT`,
[`heap_index`] corresponds to one of the valid heaps from the
[`PhysicalDeviceMemoryProperties`] structure.
Otherwise, [`heap_index`] is undefined.