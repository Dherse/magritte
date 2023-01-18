[VkPrivateDataSlotCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateInfo.html) - Structure specifying the parameters of private data slot construction

# C Specifications
The [`PrivateDataSlotCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPrivateDataSlotCreateInfo {
    VkStructureType                 sType;
    const void*                     pNext;
    VkPrivateDataSlotCreateFlags    flags;
} VkPrivateDataSlotCreateInfo;
```
or the equivalent
```c
// Provided by VK_EXT_private_data
typedef VkPrivateDataSlotCreateInfo VkPrivateDataSlotCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`

# Related
- [`VK_EXT_private_data`]
- [`crate::vulkan1_3`]
- [`PrivateDataSlotCreateFlags`]
- [`StructureType`]
- [`create_private_data_slot`]
- [`create_private_data_slot_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        