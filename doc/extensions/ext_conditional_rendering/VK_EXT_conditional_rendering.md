[VK_EXT_conditional_rendering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_conditional_rendering.html) - device extension

# Description
This extension allows the execution of one or more rendering commands to be
conditional on a value in buffer memory.
This may help an application reduce the latency by conditionally discarding
rendering commands without application intervention.
The conditional rendering commands are limited to draws, compute dispatches
and clearing attachments within a conditional rendering block.

# Registered extension number
82

# Revision
2

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Vikram Kushwaha [vkushwaha](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_conditional_rendering] @vkushwaha%0A<<Here describe the issue or question you have about the VK_EXT_conditional_rendering extension>>)

# New commands
- [`cmd_begin_conditional_rendering_ext`]
- [`cmd_end_conditional_rendering_ext`]

# New structures
- [`ConditionalRenderingBeginInfoEXT`]
- Extending [`CommandBufferInheritanceInfo`]:  - [`CommandBufferInheritanceConditionalRenderingInfoEXT`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceConditionalRenderingFeaturesEXT`]

# New enums
- [`ConditionalRenderingFlagBitsEXT`]

# New bitmasks
- [VkConditionalRenderingFlagsEXT]()

# New constants
- `VK_EXT_CONDITIONAL_RENDERING_EXTENSION_NAME`
- `VK_EXT_CONDITIONAL_RENDERING_SPEC_VERSION`
- Extending [`AccessFlagBits`]:  - `VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT` 
- Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT` 
- Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT`  - `VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT`

# Known issues & F.A.Q.
1) Should conditional rendering affect copy and blit commands? **RESOLVED** : Conditional rendering should not affect copies and blits.2) Should secondary command buffers be allowed to execute while conditional
rendering is active in the primary command buffer? **RESOLVED** : The rendering commands in secondary command buffer will be
affected by an active conditional rendering in primary command buffer if the
`conditionalRenderingEnable` is set to `VK_TRUE`.
Conditional rendering  **must**  not be active in the primary command buffer if
`conditionalRenderingEnable` is `VK_FALSE`.

# Version history
- Revision 1, 2018-04-19 (Vikram Kushwaha)  - First Version 
- Revision 2, 2018-05-21 (Vikram Kushwaha)  - Add new pipeline stage, access flags and limit conditional rendering to a subpass or entire render pass.

# Other information
* 2018-05-21
* No known IP claims.
*   - Vikram Kushwaha, NVIDIA  - Daniel Rakos, AMD  - Jesse Hall, Google  - Jeff Bolz, NVIDIA  - Piers Daniell, NVIDIA  - Stuart Smith, Imagination Technologies

# Related
- [`CommandBufferInheritanceConditionalRenderingInfoEXT`]
- [`ConditionalRenderingBeginInfoEXT`]
- [`ConditionalRenderingFlagBitsEXT`]
- [VkConditionalRenderingFlagsEXT]()
- [`PhysicalDeviceConditionalRenderingFeaturesEXT`]
- [`cmd_begin_conditional_rendering_ext`]
- [`cmd_end_conditional_rendering_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        