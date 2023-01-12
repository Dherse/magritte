[VkSpecializationMapEntry](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSpecializationMapEntry.html) - Structure specifying a specialization map entry

# C Specifications
The [`SpecializationMapEntry`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSpecializationMapEntry {
    uint32_t    constantID;
    uint32_t    offset;
    size_t      size;
} VkSpecializationMapEntry;
```

# Members
- [`constant_id`] is the ID of the specialization constant in SPIR-V.
- [`offset`] is the byte offset of the specialization constant value within the supplied data buffer.
- [`size`] is the byte size of the specialization constant value within the supplied data buffer.

# Description
If a [`constant_id`] value is not a specialization constant ID used in the
shader, that map entry does not affect the behavior of the pipeline.
## Valid Usage
-    For a [`constant_id`] specialization constant declared in a shader, [`size`] **must**  match the byte size of the [`constant_id`]. If the specialization constant is of type `boolean`, [`size`] **must**  be the byte size of [`Bool32`]

# Related
- [`crate::vulkan1_0`]
- [`SpecializationInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        