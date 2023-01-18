[VkCommandPoolCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateInfo.html) - Structure specifying parameters of a newly created command pool

# C Specifications
The [`CommandPoolCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkCommandPoolCreateInfo {
    VkStructureType             sType;
    const void*                 pNext;
    VkCommandPoolCreateFlags    flags;
    uint32_t                    queueFamilyIndex;
} VkCommandPoolCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`CommandPoolCreateFlagBits`] indicating usage behavior for the pool and command buffers allocated from it.
- [`queue_family_index`] designates a queue family as described in section [Queue Family Properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-queueprops). All command buffers allocated from this command pool  **must**  be submitted on queues from the same queue family.

# Description
## Valid Usage
-    If the protected memory feature is not enabled, the `VK_COMMAND_POOL_CREATE_PROTECTED_BIT` bit of [`flags`] **must**  not be set

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be a valid combination of [`CommandPoolCreateFlagBits`] values

# Related
- [`crate::vulkan1_0`]
- [`CommandPoolCreateFlags`]
- [`StructureType`]
- [`create_command_pool`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        