[VkQueueFamilyQueryResultStatusProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyQueryResultStatusProperties2KHR.html) - Structure specifying support for result status query

# C Specifications
The [`QueueFamilyQueryResultStatusProperties2KHR`] structure is defined
as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkQueueFamilyQueryResultStatusProperties2KHR {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           supported;
} VkQueueFamilyQueryResultStatusProperties2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`supported`] reports `VK_TRUE` if query type `VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR` and use of `VK_QUERY_RESULT_WITH_STATUS_BIT_KHR` are supported.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_2_KHR`

# Related
- [`khr_video_queue`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        