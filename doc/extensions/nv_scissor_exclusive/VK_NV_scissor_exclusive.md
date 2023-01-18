[VK_NV_scissor_exclusive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_scissor_exclusive.html) - device extension

# Description
This extension adds support for an exclusive scissor test to Vulkan.
The exclusive scissor test behaves like the scissor test, except that the
exclusive scissor test fails for pixels inside the corresponding rectangle
and passes for pixels outside the rectangle.
If the same rectangle is used for both the scissor and exclusive scissor
tests, the exclusive scissor test will pass if and only if the scissor test
fails.

# Registered extension number
206

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Pat Brown [nvpbrown](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_scissor_exclusive] @nvpbrown%0A<<Here describe the issue or question you have about the VK_NV_scissor_exclusive extension>>)

# New commands
- [`cmd_set_exclusive_scissor_nv`]

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceExclusiveScissorFeaturesNV`] 
- Extending [`PipelineViewportStateCreateInfo`]:  - [`PipelineViewportExclusiveScissorStateCreateInfoNV`]

# New constants
- [`NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME`]
- [`NV_SCISSOR_EXCLUSIVE_SPEC_VERSION`]
- Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV`  - `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV`

# Known issues & F.A.Q.
1) For the scissor test, the viewport state must be created with a matching
   number of scissor and viewport rectangles.
   Should we have the same requirement for exclusive scissors? **RESOLVED** : For exclusive scissors, we relax this requirement and allow an
exclusive scissor rectangle count that is either zero or equal to the number
of viewport rectangles.
If you pass in an exclusive scissor count of zero, the exclusive scissor
test is treated as disabled.

# Version history
- Revision 1, 2018-07-31 (Pat Brown)  - Internal revisions

# Other information
* 2018-07-31
* No known IP claims.
* None
*   - Pat Brown, NVIDIA  - Jeff Bolz, NVIDIA  - Piers Daniell, NVIDIA  - Daniel Koch, NVIDIA

# Related
- [`PhysicalDeviceExclusiveScissorFeaturesNV`]
- [`PipelineViewportExclusiveScissorStateCreateInfoNV`]
- [`cmd_set_exclusive_scissor_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        