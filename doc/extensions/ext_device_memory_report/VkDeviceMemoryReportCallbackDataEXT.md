[VkDeviceMemoryReportCallbackDataEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportCallbackDataEXT.html) - Structure specifying parameters returned to the callback

# C Specifications
The definition of [`DeviceMemoryReportCallbackDataEXT`] is:
```c
// Provided by VK_EXT_device_memory_report
typedef struct VkDeviceMemoryReportCallbackDataEXT {
    VkStructureType                     sType;
    void*                               pNext;
    VkDeviceMemoryReportFlagsEXT        flags;
    VkDeviceMemoryReportEventTypeEXT    type;
    uint64_t                            memoryObjectId;
    VkDeviceSize                        size;
    VkObjectType                        objectType;
    uint64_t                            objectHandle;
    uint32_t                            heapIndex;
} VkDeviceMemoryReportCallbackDataEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is 0 and reserved for future use.
- [`type_`] is a [`DeviceMemoryReportEventTypeEXT`] type specifying the type of event reported in this [`DeviceMemoryReportCallbackDataEXT`] structure.
- [`memory_object_id`] is the unique id for the underlying memory object as described below.
- [`size`] is the size of the memory object in bytes. If [`type_`] is `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT`, `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT` or `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT`, [`size`] is a valid [`DeviceSize`] value. Otherwise, [`size`] is undefined.
- [`object_type`] is a [`ObjectType`] value specifying the type of the object associated with this device memory report event. If [`type_`] is `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT`, `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT`, `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT`, `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT` or `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT`, [`object_type`] is a valid [`ObjectType`] enum. Otherwise, [`object_type`] is undefined.
- [`object_handle`] is the object this device memory report event is attributed to. If [`type_`] is `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT`, `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT`, `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT` or `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT`, [`object_handle`] is a valid Vulkan handle of the type associated with [`object_type`] as defined in the [[`ObjectType`] and Vulkan Handle Relationship](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-types) table. Otherwise, [`object_handle`] is undefined.
- [`heap_index`] describes which memory heap this device memory allocation is made from. If [`type_`] is `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT` or `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT`, [`heap_index`] corresponds to one of the valid heaps from the [`PhysicalDeviceMemoryProperties`] structure. Otherwise, [`heap_index`] is undefined.

# Description
[`memory_object_id`] is used to avoid double-counting on the same memory
object.If an internally-allocated device memory object or a [`DeviceMemory`] **cannot**  be exported, [`memory_object_id`] **must**  be unique in the
[`Device`].If an internally-allocated device memory object or a [`DeviceMemory`]
supports being exported, [`memory_object_id`] **must**  be unique system wide.If an internal device memory object or a [`DeviceMemory`] is backed by
an imported external memory object, [`memory_object_id`] **must**  be unique
system wide.
## Implementor’s NoteIf the heap backing an internally-allocated device memory  **cannot**  be used to
back [`DeviceMemory`], implementations  **can**  advertise that heap with no
types.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_EXT_device_memory_report`]
- [`DeviceMemoryReportEventTypeEXT`]
- [`DeviceMemoryReportFlagsEXT`]
- [`DeviceSize`]
- [`ObjectType`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        