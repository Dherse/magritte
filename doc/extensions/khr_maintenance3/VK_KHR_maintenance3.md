[VK_KHR_maintenance3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance3.html) - device extension

# Description
[`VK_KHR_maintenance3`] adds a collection of minor features that were
intentionally left out or overlooked from the original Vulkan 1.0 release.The new features are as follows:
- A limit on the maximum number of descriptors that are supported in a single descriptor set layout. Some implementations have a limit on the total size of descriptors in a set, which cannot be expressed in terms of the limits in Vulkan 1.0.
- A limit on the maximum size of a single memory allocation. Some platforms have kernel interfaces that limit the maximum size of an allocation.

# Registered extension number
169

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Deprecation state
- *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)

# Contacts
- Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_maintenance3] @jeffbolznv%0A<<Here describe the issue or question you have about the VK_KHR_maintenance3 extension>>)

# New commands
- [`get_descriptor_set_layout_support_khr`]

# New structures
- [`DescriptorSetLayoutSupportKHR`]
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceMaintenance3PropertiesKHR`]

# New constants
- [`KHR_MAINTENANCE3_EXTENSION_NAME`]
- [`KHR_MAINTENANCE3_SPEC_VERSION`]
- [`KHR_MAINTENANCE_3_EXTENSION_NAME`]
- [`KHR_MAINTENANCE_3_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR`

# Version history
- Revision 1, 2017-08-22

# Other information
* 2017-09-05
*   - Promoted to Vulkan 1.1 Core 
*   - Jeff Bolz, NVIDIA

# Related
- [`DescriptorSetLayoutSupportKHR`]
- [`PhysicalDeviceMaintenance3PropertiesKHR`]
- [`get_descriptor_set_layout_support_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        