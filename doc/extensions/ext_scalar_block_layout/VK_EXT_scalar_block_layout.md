[VK_EXT_scalar_block_layout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_scalar_block_layout.html) - device extension

# Description
This extension enables C-like structure layout for SPIR-V blocks.
It modifies the alignment rules for uniform buffers, storage buffers and
push constants, allowing non-scalar types to be aligned solely based on the
size of their components, without additional requirements.

# Registered extension number
222

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Deprecation state
- *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)

# Contacts
- Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_scalar_block_layout] @tobski%0A<<Here describe the issue or question you have about the VK_EXT_scalar_block_layout extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceScalarBlockLayoutFeaturesEXT`]

# New constants
- `VK_EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME`
- `VK_EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT`

# Version history
- Revision 1, 2018-11-14 (Tobias Hector)  - Initial draft

# Other information
* 2018-11-14
*   - Promoted to Vulkan 1.2 Core 
*   - Jeff Bolz  - Jan-Harald Fredriksen  - Graeme Leese  - Jason Ekstrand  - John Kessenich

# Related
- [`PhysicalDeviceScalarBlockLayoutFeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        