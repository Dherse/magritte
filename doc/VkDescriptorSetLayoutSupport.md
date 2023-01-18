[VkDescriptorSetLayoutSupport](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutSupport.html) - Structure returning information about whether a descriptor set layout can be supported

# C Specifications
Information about support for the descriptor set layout is returned in a
[`DescriptorSetLayoutSupport`] structure:
```c
// Provided by VK_VERSION_1_1
typedef struct VkDescriptorSetLayoutSupport {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           supported;
} VkDescriptorSetLayoutSupport;
```
or the equivalent
```c
// Provided by VK_KHR_maintenance3
typedef VkDescriptorSetLayoutSupport VkDescriptorSetLayoutSupportKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`supported`] specifies whether the descriptor set layout  **can**  be created.

# Description
[`supported`] is set to [`TRUE`] if the descriptor set  **can**  be
created, or else is set to [`FALSE`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`DescriptorSetVariableDescriptorCountLayoutSupport`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique

# Related
- [`crate::vulkan1_1`]
- [`Bool32`]
- [`StructureType`]
- [`get_descriptor_set_layout_support`]
- [`get_descriptor_set_layout_support_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        