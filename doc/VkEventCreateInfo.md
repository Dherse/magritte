[VkEventCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkEventCreateInfo.html) - Structure specifying parameters of a newly created event

# C Specifications
The [`EventCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkEventCreateInfo {
    VkStructureType       sType;
    const void*           pNext;
    VkEventCreateFlags    flags;
} VkEventCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`EventCreateFlagBits`] defining additional creation parameters.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EVENT_CREATE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be a valid combination of [`EventCreateFlagBits`] values

# Related
- [`crate::vulkan1_0`]
- [VkEventCreateFlags]()
- [`StructureType`]
- [`create_event`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        