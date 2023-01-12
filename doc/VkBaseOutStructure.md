[VkBaseOutStructure](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBaseOutStructure.html) - Base structure for a read-only pointer chain

# C Specifications
The [`BaseOutStructure`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkBaseOutStructure {
    VkStructureType               sType;
    struct VkBaseOutStructure*    pNext;
} VkBaseOutStructure;
```

# Members
- [`s_type`] is the structure type of the structure being iterated through.
- [`next`] is `NULL` or a pointer to the next structure in a structure chain.

# Description
[`BaseOutStructure`] can be used to facilitate iterating through a
structure pointer chain that returns data back to the application.

# Related
- [`crate::vulkan1_0`]
- [`BaseOutStructure`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        