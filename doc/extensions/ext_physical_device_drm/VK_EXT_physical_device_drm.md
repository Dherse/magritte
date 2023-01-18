[VK_EXT_physical_device_drm](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_physical_device_drm.html) - device extension

# Description
This extension provides new facilities to query DRM properties for physical
devices, enabling users to match Vulkan physical devices with DRM nodes on
Linux.Its functionality closely overlaps with
`EGL_EXT_device_drm`<sup>[1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_physical_device_drm-fn1)</sup>^.
Unlike the EGL extension, this extension does not expose a string containing
the name of the device file and instead exposes device minor numbers.DRM defines multiple device node types.
Each physical device may have one primary node and one render node
associated.
Physical devices may have no primary node (e.g. if the device does not have
a display subsystem), may have no render node (e.g. if it is a software
rendering engine), or may have neither (e.g. if it is a software rendering
engine without a display subsystem).To query DRM properties for a physical device, chain
[`PhysicalDeviceDrmPropertiesEXT`] to [`PhysicalDeviceProperties2`].

# Registered extension number
354

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Simon Ser [emersion](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_physical_device_drm] @emersion%0A<<Here describe the issue or question you have about the VK_EXT_physical_device_drm extension>>)

# New structures
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceDrmPropertiesEXT`]

# New constants
- [`EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME`]
- [`EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT`

# Version history
- Revision 1, 2021-06-09  - First stable revision

# Other information
* 2021-06-09
* No known IP claims.
*   - Simon Ser

# Related
- [`PhysicalDeviceDrmPropertiesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        