[VK_EXT_multi_draw](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_multi_draw.html) - device extension

# Description
Processing multiple draw commands in sequence incurs measurable overhead
within drivers due to repeated state checks and updates during dispatch.
This extension enables passing the entire sequence of draws directly to the
driver in order to avoid any such overhead, using an array of
[`MultiDrawInfoEXT`] or [`MultiDrawIndexedInfoEXT`] structs with
[`cmd_draw_multi_ext`] or [`cmd_draw_multi_indexed_ext`], respectively.
These functions could be used any time multiple draw commands are being
recorded without any state changes between them in order to maximize
performance.

# Registered extension number
393

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Mike Blumenkrantz [zmike](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_multi_draw] @zmike%0A<<Here describe the issue or question you have about the VK_EXT_multi_draw extension>>)

# New commands
- [`cmd_draw_multi_ext`]
- [`cmd_draw_multi_indexed_ext`]

# New structures
- [`MultiDrawIndexedInfoEXT`]
- [`MultiDrawInfoEXT`]
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceMultiDrawFeaturesEXT`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceMultiDrawPropertiesEXT`]

# New constants
- [`EXT_MULTI_DRAW_EXTENSION_NAME`]
- [`EXT_MULTI_DRAW_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT`

# Version history
- Revision 1, 2021-01-20 (Mike Blumenkrantz)  - Initial version

# Other information
* 2021-05-19
* No known IP claims.
*   - Mike Blumenkrantz, VALVE  - Piers Daniell, NVIDIA  - Jason Ekstrand, INTEL  - Spencer Fricke, SAMSUNG  - Ricardo Garcia, IGALIA  - Jon Leech, KHRONOS  - Stu Smith, AMD

# Related
- [`MultiDrawIndexedInfoEXT`]
- [`MultiDrawInfoEXT`]
- [`PhysicalDeviceMultiDrawFeaturesEXT`]
- [`PhysicalDeviceMultiDrawPropertiesEXT`]
- [`cmd_draw_multi_ext`]
- [`cmd_draw_multi_indexed_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        