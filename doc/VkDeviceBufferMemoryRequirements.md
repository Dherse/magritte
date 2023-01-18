[VkDeviceBufferMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceBufferMemoryRequirements.html) - (None)

# C Specifications
The [`DeviceBufferMemoryRequirements`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkDeviceBufferMemoryRequirements {
    VkStructureType              sType;
    const void*                  pNext;
    const VkBufferCreateInfo*    pCreateInfo;
} VkDeviceBufferMemoryRequirements;
```
or the equivalent
```c
// Provided by VK_KHR_maintenance4
typedef VkDeviceBufferMemoryRequirements VkDeviceBufferMemoryRequirementsKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`create_info`] is a pointer to a [`BufferCreateInfo`] structure containing parameters affecting creation of the buffer to query.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS`
-  [`p_next`] **must**  be `NULL`
-  [`create_info`] **must**  be a valid pointer to a valid [`BufferCreateInfo`] structure

# Related
- [`VK_KHR_maintenance4`]
- [`crate::vulkan1_3`]
- [`BufferCreateInfo`]
- [`StructureType`]
- [`get_device_buffer_memory_requirements`]
- [`get_device_buffer_memory_requirements_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        