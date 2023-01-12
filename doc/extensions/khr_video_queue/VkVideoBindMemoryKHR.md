[VkVideoBindMemoryKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoBindMemoryKHR.html) - Structure specifying device memory heap entry for video session object

# C Specifications
The [`VideoBindMemoryKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoBindMemoryKHR {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           memoryBindIndex;
    VkDeviceMemory     memory;
    VkDeviceSize       memoryOffset;
    VkDeviceSize       memorySize;
} VkVideoBindMemoryKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory_bind_index`] is the index of the device memory heap returned in [`VideoGetMemoryPropertiesKHR`]::[`memory_bind_index`] from [`get_video_session_memory_requirements_khr`].
- [`memory`] is the allocated device memory to be bound to the video session’s heap with index [`memory_bind_index`].
- [`memory_offset`] is the start offset of the region of [`memory`] which is to be bound.
- [`memory_size`] is the size in bytes of the region of [`memory`], starting from [`memory_offset`] bytes, to be bound.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_BIND_MEMORY_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle

# Related
- [`khr_video_queue`]
- [`DeviceMemory`]
- [`DeviceSize`]
- [`StructureType`]
- [`bind_video_session_memory_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        