[VkVideoGetMemoryPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoGetMemoryPropertiesKHR.html) - Structure specifying video session required memory heap type

# C Specifications
The [`VideoGetMemoryPropertiesKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoGetMemoryPropertiesKHR {
    VkStructureType           sType;
    const void*               pNext;
    uint32_t                  memoryBindIndex;
    VkMemoryRequirements2*    pMemoryRequirements;
} VkVideoGetMemoryPropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory_bind_index`] is the memory bind index of the memory heap type described by the information returned in [`memory_requirements`].
- [`memory_requirements`] is a pointer to a [`MemoryRequirements2`] structure in which the requested memory heap requirements for the heap with index [`memory_bind_index`] are returned.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_GET_MEMORY_PROPERTIES_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements2`] structure

# Related
- [`khr_video_queue`]
- [`MemoryRequirements2`]
- [`StructureType`]
- [`get_video_session_memory_requirements_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        