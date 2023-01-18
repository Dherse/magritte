[VK_KHR_device_group_creation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_device_group_creation.html) - instance extension

# Description
This extension provides instance-level commands to enumerate groups of
physical devices, and to create a logical device from a subset of one of
those groups.
Such a logical device can then be used with new features in the
`[`VK_KHR_device_group`]` extension.

# Registered extension number
71

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)

# Contacts
- Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_device_group_creation] @jeffbolznv%0A<<Here describe the issue or question you have about the VK_KHR_device_group_creation extension>>)

# New commands
- [`enumerate_physical_device_groups_khr`]

# New structures
- [`PhysicalDeviceGroupPropertiesKHR`]
- Extending [`DeviceCreateInfo`]:  - [`DeviceGroupDeviceCreateInfoKHR`]

# New constants
- [`KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME`]
- [`KHR_DEVICE_GROUP_CREATION_SPEC_VERSION`]
- [`MAX_DEVICE_GROUP_SIZE_KHR`]
- Extending [`MemoryHeapFlagBits`]:  - `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR`

# Version history
- Revision 1, 2016-10-19 (Jeff Bolz)  - Internal revisions

# Other information
* 2016-10-19
* No known IP claims.
*   - Promoted to Vulkan 1.1 Core 
*   - Jeff Bolz, NVIDIA

# Related
- [`MAX_DEVICE_GROUP_SIZE_KHR`]
- [`DeviceGroupDeviceCreateInfoKHR`]
- [`PhysicalDeviceGroupPropertiesKHR`]
- [`enumerate_physical_device_groups_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        