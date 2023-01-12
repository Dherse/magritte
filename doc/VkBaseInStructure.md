[VkBaseInStructure](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBaseInStructure.html) - Base structure for a read-only pointer chain

# C Specifications
The [`BaseInStructure`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkBaseInStructure {
    VkStructureType                    sType;
    const struct VkBaseInStructure*    pNext;
} VkBaseInStructure;
```

# Members
- [`s_type`] is the structure type of the structure being iterated through.
- [`next`] is `NULL` or a pointer to the next structure in a structure chain.

# Description
[`BaseInStructure`] can be used to facilitate iterating through a
read-only structure pointer chain.

# Related
- [`crate::vulkan1_0`]
- [`BaseInStructure`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        