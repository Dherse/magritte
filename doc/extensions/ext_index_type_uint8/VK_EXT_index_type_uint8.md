[VK_EXT_index_type_uint8](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_index_type_uint8.html) - device extension

# Description
This extension allows `uint8_t` indices to be used with
[`cmd_bind_index_buffer`].

# Registered extension number
266

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_index_type_uint8] @pdaniell-nv%0A<<Here describe the issue or question you have about the VK_EXT_index_type_uint8 extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceIndexTypeUint8FeaturesEXT`]

# New constants
- [`EXT_INDEX_TYPE_UINT8_EXTENSION_NAME`]
- [`EXT_INDEX_TYPE_UINT8_SPEC_VERSION`]
- Extending [`IndexType`]:  - `VK_INDEX_TYPE_UINT8_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT`

# Version history
- Revision 1, 2019-05-02 (Piers Daniell)  - Internal revisions

# Other information
* 2019-05-02
* No known IP claims.
*   - Jeff Bolz, NVIDIA

# Related
- [`PhysicalDeviceIndexTypeUint8FeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        