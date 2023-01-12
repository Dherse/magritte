[VkPipelineVertexInputDivisorStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputDivisorStateCreateInfoEXT.html) - Structure specifying vertex attributes assignment during instanced rendering

# C Specifications
If
[`vertexAttributeInstanceRateDivisor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateDivisor)
feature is enabled and the [`p_next`] chain of
[`PipelineVertexInputStateCreateInfo`] includes a
[`PipelineVertexInputDivisorStateCreateInfoEXT`] structure, then that
structure controls how vertex attributes are assigned to an instance when
instanced rendering is enabled.The [`PipelineVertexInputDivisorStateCreateInfoEXT`] structure is
defined as:
```c
// Provided by VK_EXT_vertex_attribute_divisor
typedef struct VkPipelineVertexInputDivisorStateCreateInfoEXT {
    VkStructureType                                     sType;
    const void*                                         pNext;
    uint32_t                                            vertexBindingDivisorCount;
    const VkVertexInputBindingDivisorDescriptionEXT*    pVertexBindingDivisors;
} VkPipelineVertexInputDivisorStateCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`vertex_binding_divisor_count`] is the number of elements in the [`vertex_binding_divisors`] array.
- [`vertex_binding_divisors`] is a pointer to an array of [`VertexInputBindingDivisorDescriptionEXT`] structures specifying the divisor value for each binding.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT`
-  [`vertex_binding_divisors`] **must**  be a valid pointer to an array of [`vertex_binding_divisor_count`][`VertexInputBindingDivisorDescriptionEXT`] structures
-  [`vertex_binding_divisor_count`] **must**  be greater than `0`

# Related
- [`ext_vertex_attribute_divisor`]
- [`StructureType`]
- [`VertexInputBindingDivisorDescriptionEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        