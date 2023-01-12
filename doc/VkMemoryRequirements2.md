[VkMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements2.html) - Structure specifying memory requirements

# C Specifications
The [`MemoryRequirements2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkMemoryRequirements2 {
    VkStructureType         sType;
    void*                   pNext;
    VkMemoryRequirements    memoryRequirements;
} VkMemoryRequirements2;
```
or the equivalent
```c
// Provided by VK_KHR_get_memory_requirements2, VK_NV_ray_tracing
typedef VkMemoryRequirements2 VkMemoryRequirements2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory_requirements`] is a [`MemoryRequirements`] structure describing the memory requirements of the resource.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`MemoryDedicatedRequirements`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique

# Related
- [`crate::vulkan1_1`]
- [`MemoryRequirements`]
- [`StructureType`]
- [`VideoGetMemoryPropertiesKHR`]
- [`get_buffer_memory_requirements2`]
- [`get_buffer_memory_requirements2_khr`]
- [`get_device_buffer_memory_requirements`]
- [`get_device_buffer_memory_requirements_khr`]
- [`get_device_image_memory_requirements`]
- [`get_device_image_memory_requirements_khr`]
- [`get_generated_commands_memory_requirements_nv`]
- [`get_image_memory_requirements2`]
- [`get_image_memory_requirements2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        