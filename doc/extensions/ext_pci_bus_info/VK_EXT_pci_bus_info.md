[VK_EXT_pci_bus_info](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_pci_bus_info.html) - device extension

# Description
This extension adds a new query to obtain PCI bus information about a
physical device.Not all physical devices have PCI bus information, either due to the device
not being connected to the system through a PCI interface or due to platform
specific restrictions and policies.
Thus this extension is only expected to be supported by physical devices
which can provide the information.As a consequence, applications should always check for the presence of the
extension string for each individual physical device for which they intend
to issue the new query for and should not have any assumptions about the
availability of the extension on any given platform.

# Registered extension number
213

# Revision
2

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Contacts
- Matthaeus G. Chajdas [anteru](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_pci_bus_info] @anteru%0A<<Here describe the issue or question you have about the VK_EXT_pci_bus_info extension>>)

# New structures
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDevicePciBusInfoPropertiesEXT`]

# New constants
- `VK_EXT_PCI_BUS_INFO_EXTENSION_NAME`
- `VK_EXT_PCI_BUS_INFO_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT`

# Version history
- Revision 2, 2018-12-10 (Daniel Rakos)  - Changed all members of the new structure to have the uint32_t type 
- Revision 1, 2018-10-11 (Daniel Rakos)  - Initial revision

# Other information
* 2018-12-10
* No known IP claims.
*   - Matthaeus G. Chajdas, AMD  - Daniel Rakos, AMD

# Related
- [`PhysicalDevicePciBusInfoPropertiesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        