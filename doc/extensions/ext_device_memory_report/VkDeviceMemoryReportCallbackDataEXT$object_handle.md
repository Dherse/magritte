[`object_handle`] is the object this device memory report event is
attributed to.
If [`type_`] is `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT`,
`VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT`,
`VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT` or
`VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT`,
[`object_handle`] is a valid Vulkan handle of the type associated with
[`object_type`] as defined in the [[`ObjectType`] and Vulkan Handle Relationship](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-types) table.
Otherwise, [`object_handle`] is undefined.