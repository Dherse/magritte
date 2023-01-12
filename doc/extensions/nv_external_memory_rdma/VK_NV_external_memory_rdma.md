[VK_NV_external_memory_rdma](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory_rdma.html) - device extension

# Description
This extension adds support for allocating memory which can be used for
remote direct memory access (RDMA) from other devices.

# Registered extension number
372

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_external_memory`]`

# Contacts
- Carsten Rohde [crohde](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_external_memory_rdma] @crohde%0A<<Here describe the issue or question you have about the VK_NV_external_memory_rdma extension>>)

# New base types
- [`RemoteAddressNV`]

# New commands
- [`get_memory_remote_address_nv`]

# New structures
- [`MemoryGetRemoteAddressInfoNV`]
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceExternalMemoryRdmaFeaturesNV`]

# New constants
- `VK_NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME`
- `VK_NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION`
- Extending [`ExternalMemoryHandleTypeFlagBits`]:  - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV` 
- Extending [`MemoryPropertyFlagBits`]:  - `VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV`

# Version history
- Revision 1, 2020-12-15 (Carsten Rohde)  - Internal revisions

# Other information
* 2021-04-19
* No known IP claims.
*   - Carsten Rohde, NVIDIA

# Related
- [`MemoryGetRemoteAddressInfoNV`]
- [`PhysicalDeviceExternalMemoryRdmaFeaturesNV`]
- [`RemoteAddressNV`]
- [`get_memory_remote_address_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        