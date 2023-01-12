[VkPipelineVertexInputStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html) - Structure specifying parameters of a newly created pipeline vertex input state

# C Specifications
The [`PipelineVertexInputStateCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineVertexInputStateCreateInfo {
    VkStructureType                             sType;
    const void*                                 pNext;
    VkPipelineVertexInputStateCreateFlags       flags;
    uint32_t                                    vertexBindingDescriptionCount;
    const VkVertexInputBindingDescription*      pVertexBindingDescriptions;
    uint32_t                                    vertexAttributeDescriptionCount;
    const VkVertexInputAttributeDescription*    pVertexAttributeDescriptions;
} VkPipelineVertexInputStateCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`vertex_binding_description_count`] is the number of vertex binding descriptions provided in [`vertex_binding_descriptions`].
- [`vertex_binding_descriptions`] is a pointer to an array of [`VertexInputBindingDescription`] structures.
- [`vertex_attribute_description_count`] is the number of vertex attribute descriptions provided in [`vertex_attribute_descriptions`].
- [`vertex_attribute_descriptions`] is a pointer to an array of [`VertexInputAttributeDescription`] structures.

# Description
## Valid Usage
-  [`vertex_binding_description_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_vertex_input_bindings`]
-  [`vertex_attribute_description_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_vertex_input_attributes`]
-    For every `binding` specified by each element of [`vertex_attribute_descriptions`], a [`VertexInputBindingDescription`] **must**  exist in [`vertex_binding_descriptions`] with the same value of `binding`
-    All elements of [`vertex_binding_descriptions`] **must**  describe distinct binding numbers
-    All elements of [`vertex_attribute_descriptions`] **must**  describe distinct attribute locations

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`PipelineVertexInputDivisorStateCreateInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be `0`
-    If [`vertex_binding_description_count`] is not `0`, [`vertex_binding_descriptions`] **must**  be a valid pointer to an array of [`vertex_binding_description_count`] valid [`VertexInputBindingDescription`] structures
-    If [`vertex_attribute_description_count`] is not `0`, [`vertex_attribute_descriptions`] **must**  be a valid pointer to an array of [`vertex_attribute_description_count`] valid [`VertexInputAttributeDescription`] structures

# Related
- [`crate::vulkan1_0`]
- [`GraphicsPipelineCreateInfo`]
- [`GraphicsShaderGroupCreateInfoNV`]
- [`PipelineVertexInputStateCreateFlags`]
- [`StructureType`]
- [`VertexInputAttributeDescription`]
- [`VertexInputBindingDescription`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        