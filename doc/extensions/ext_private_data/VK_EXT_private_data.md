[VK_EXT_private_data](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_private_data.html) - device extension

# Description
This extension is a device extension which enables attaching arbitrary
payloads to Vulkan objects.
It introduces the idea of private data slots as a means of storing a 64-bit
unsigned integer of application defined data.
Private data slots can be created or destroyed any time an associated device
is available.
Private data slots can be reserved at device creation time, and limiting use
to the amount reserved will allow the extension to exhibit better
performance characteristics.

# Registered extension number
296

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)

# Contacts
- Matthew Rusch [mattruschnv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_private_data] @mattruschnv%0A<<Here describe the issue or question you have about the VK_EXT_private_data extension>>)

# New object types
- [`PrivateDataSlotEXT`]

# New commands
- [`create_private_data_slot_ext`]
- [`destroy_private_data_slot_ext`]
- [`get_private_data_ext`]
- [`set_private_data_ext`]

# New structures
- [`PrivateDataSlotCreateInfoEXT`]
- Extending [`DeviceCreateInfo`]:  - [`DevicePrivateDataCreateInfoEXT`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDevicePrivateDataFeaturesEXT`]

# New bitmasks
- [`PrivateDataSlotCreateFlagsEXT`]

# New constants
- [`EXT_PRIVATE_DATA_EXTENSION_NAME`]
- [`EXT_PRIVATE_DATA_SPEC_VERSION`]
- Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_PRIVATE_DATA_SLOT_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO_EXT`

# Version history
- Revision 1, 2020-01-15 (Matthew Rusch)  - Initial draft

# Other information
* 2020-03-25
*   - Promoted to Vulkan 1.3 Core 
* No known IP claims.
*   - Matthew Rusch, NVIDIA  - Nuno Subtil, NVIDIA  - Piers Daniell, NVIDIA  - Jeff Bolz, NVIDIA

# Related
- [`DevicePrivateDataCreateInfoEXT`]
- [`PhysicalDevicePrivateDataFeaturesEXT`]
- [`PrivateDataSlotCreateFlagsEXT`]
- [`PrivateDataSlotCreateInfoEXT`]
- [`PrivateDataSlotEXT`]
- [`create_private_data_slot_ext`]
- [`destroy_private_data_slot_ext`]
- [`get_private_data_ext`]
- [`set_private_data_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        