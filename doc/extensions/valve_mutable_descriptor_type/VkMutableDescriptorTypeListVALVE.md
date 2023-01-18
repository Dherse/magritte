[VkMutableDescriptorTypeListVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeListVALVE.html) - Structure describing descriptor types that a given descriptor may mutate to

# C Specifications
The list of potential descriptor types a given mutable descriptor  **can** 
mutate to is passed in a [`MutableDescriptorTypeListVALVE`] structure.The [`MutableDescriptorTypeListVALVE`] structure is defined as:
```c
// Provided by VK_VALVE_mutable_descriptor_type
typedef struct VkMutableDescriptorTypeListVALVE {
    uint32_t                   descriptorTypeCount;
    const VkDescriptorType*    pDescriptorTypes;
} VkMutableDescriptorTypeListVALVE;
```

# Members
- [`descriptor_type_count`] is the number of elements in [`descriptor_types`].
- [`descriptor_types`] is `NULL` or a pointer to an array of [`descriptor_type_count`][`DescriptorType`] values defining which descriptor types a given binding may mutate to.

# Description
## Valid Usage
-  [`descriptor_type_count`] **must**  not be `0` if the corresponding binding is of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
-  [`descriptor_types`] **must**  be a valid pointer to an array of [`descriptor_type_count`] valid, unique [`DescriptorType`] values if the given binding is of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` type
-  [`descriptor_type_count`] **must**  be `0` if the corresponding binding is not of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
-  [`descriptor_types`] **must**  not contain `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
-  [`descriptor_types`] **must**  not contain `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
-  [`descriptor_types`] **must**  not contain `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`
-  [`descriptor_types`] **must**  not contain `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`

## Valid Usage (Implicit)
-    If [`descriptor_type_count`] is not `0`, [`descriptor_types`] **must**  be a valid pointer to an array of [`descriptor_type_count`] valid [`DescriptorType`] values

# Related
- [`VK_VALVE_mutable_descriptor_type`]
- [`DescriptorType`]
- [`MutableDescriptorTypeCreateInfoVALVE`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        