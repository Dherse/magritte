[VK_EXT_vertex_input_dynamic_state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_vertex_input_dynamic_state.html) - device extension

# Description
One of the states that contributes to the combinatorial explosion of
pipeline state objects that need to be created, is the vertex input binding
and attribute descriptions.
By allowing them to be dynamic applications may reduce the number of
pipeline objects they need to create.This extension adds dynamic state support for what is normally static state
in [`PipelineVertexInputStateCreateInfo`].

# Registered extension number
353

# Revision
2

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_vertex_input_dynamic_state] @pdaniell-nv%0A<<Here describe the issue or question you have about the VK_EXT_vertex_input_dynamic_state extension>>)

# New commands
- [`cmd_set_vertex_input_ext`]

# New structures
- [`VertexInputAttributeDescription2EXT`]
- [`VertexInputBindingDescription2EXT`]
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceVertexInputDynamicStateFeaturesEXT`]

# New constants
- [`EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME`]
- [`EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION`]
- Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT`  - `VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT`

# Version history
- Revision 2, 2020-11-05 (Piers Daniell)  - Make [`VertexInputBindingDescription2EXT`] extensible  - Add new [`VertexInputAttributeDescription2EXT`] struct for the `pVertexAttributeDescriptions` parameter to [`cmd_set_vertex_input_ext`] so it is also extensible 
- Revision 1, 2020-08-21 (Piers Daniell)  - Internal revisions

# Other information
* 2020-08-21
* No known IP claims.
*   - Jeff Bolz, NVIDIA  - Spencer Fricke, Samsung  - Stu Smith, AMD

# Related
- [`PhysicalDeviceVertexInputDynamicStateFeaturesEXT`]
- [`VertexInputAttributeDescription2EXT`]
- [`VertexInputBindingDescription2EXT`]
- [`cmd_set_vertex_input_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        