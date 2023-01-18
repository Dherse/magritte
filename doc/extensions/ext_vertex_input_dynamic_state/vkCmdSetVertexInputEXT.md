[vkCmdSetVertexInputEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html) - Set the vertex input state dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the vertex input attribute
and vertex input binding descriptions, call:
```c
// Provided by VK_EXT_vertex_input_dynamic_state
void vkCmdSetVertexInputEXT(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    vertexBindingDescriptionCount,
    const VkVertexInputBindingDescription2EXT*  pVertexBindingDescriptions,
    uint32_t                                    vertexAttributeDescriptionCount,
    const VkVertexInputAttributeDescription2EXT* pVertexAttributeDescriptions);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`vertex_binding_description_count`] is the number of vertex binding descriptions provided in [`p_vertex_binding_descriptions`].
- [`p_vertex_binding_descriptions`] is a pointer to an array of [`VertexInputBindingDescription2EXT`] structures.
- [`vertex_attribute_description_count`] is the number of vertex attribute descriptions provided in [`p_vertex_attribute_descriptions`].
- [`p_vertex_attribute_descriptions`] is a pointer to an array of [`VertexInputAttributeDescription2EXT`] structures.

# Description
This command sets the vertex input attribute and vertex input binding
descriptions state for subsequent drawing commands when the graphics
pipeline is created with `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`GraphicsPipelineCreateInfo::vertex_input_state`] values used to
create the currently active pipeline.If the bound pipeline state object was also created with the
`VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE` dynamic state enabled,
then [`cmd_bind_vertex_buffers2`] can be used instead of
[`cmd_set_vertex_input_ext`] to dynamically set the stride.
## Valid Usage
-    The [vertexInputDynamicState](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexInputDynamicState) feature  **must**  be enabled
-  [`vertex_binding_description_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_vertex_input_bindings`]
-  [`vertex_attribute_description_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_vertex_input_attributes`]
-    For every `binding` specified by each element of [`p_vertex_attribute_descriptions`], a [`VertexInputBindingDescription2EXT`] **must**  exist in [`p_vertex_binding_descriptions`] with the same value of `binding`
-    All elements of [`p_vertex_binding_descriptions`] **must**  describe distinct binding numbers
-    All elements of [`p_vertex_attribute_descriptions`] **must**  describe distinct attribute locations

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-    If [`vertex_binding_description_count`] is not `0`, [`p_vertex_binding_descriptions`] **must**  be a valid pointer to an array of [`vertex_binding_description_count`] valid [`VertexInputBindingDescription2EXT`] structures
-    If [`vertex_attribute_description_count`] is not `0`, [`p_vertex_attribute_descriptions`] **must**  be a valid pointer to an array of [`vertex_attribute_description_count`] valid [`VertexInputAttributeDescription2EXT`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_EXT_vertex_input_dynamic_state`]
- [`CommandBuffer`]
- [`VertexInputAttributeDescription2EXT`]
- [`VertexInputBindingDescription2EXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        