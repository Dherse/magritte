[VK_EXT_texel_buffer_alignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_texel_buffer_alignment.html) - device extension

# Description
This extension adds more expressive alignment requirements for uniform and
storage texel buffers.
Some implementations have single texel alignment requirements that cannot be
expressed via
[`PhysicalDeviceLimits::min_texel_buffer_offset_alignment`].

# Registered extension number
282

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Deprecation state
- *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)

# Contacts
- Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_texel_buffer_alignment] @jeffbolznv%0A<<Here describe the issue or question you have about the VK_EXT_texel_buffer_alignment extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceTexelBufferAlignmentFeaturesEXT`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceTexelBufferAlignmentPropertiesEXT`]

# New constants
- [`EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME`]
- [`EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT`

# Version history
- Revision 1, 2019-06-06 (Jeff Bolz)  - Initial draft

# Other information
* 2019-06-06
*   - Promoted to Vulkan 1.3 Core 
* No known IP claims.
*   - Jeff Bolz, NVIDIA

# Related
- [`PhysicalDeviceTexelBufferAlignmentFeaturesEXT`]
- [`PhysicalDeviceTexelBufferAlignmentPropertiesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        