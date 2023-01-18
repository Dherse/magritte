[VK_EXT_robustness2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_robustness2.html) - device extension

# Description
This extension adds stricter requirements for how out of bounds reads and
writes are handled.
Most accesses  **must**  be tightly bounds-checked, out of bounds writes  **must**  be
discarded, out of bound reads  **must**  return zero.
Rather than allowing multiple possible (0,0,0,x) vectors, the out of
bounds values are treated as zero, and then missing components are inserted
based on the format as described in [Conversion to RGBA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-conversion-to-rgba) and [vertex input attribute
extraction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fxvertex-input-extraction).These additional requirements  **may**  be expensive on some implementations, and
should only be enabled when truly necessary.This extension also adds support for “null descriptors”, where
[`crate::Handle::null`] **can**  be used instead of a valid handle.
Accesses to null descriptors have well-defined behavior, and do not rely on
robustness.

# Registered extension number
287

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Liam Middlebrook [liam-middlebrook](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_robustness2] @liam-middlebrook%0A<<Here describe the issue or question you have about the VK_EXT_robustness2 extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceRobustness2FeaturesEXT`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceRobustness2PropertiesEXT`]

# New constants
- [`EXT_ROBUSTNESS_2_EXTENSION_NAME`]
- [`EXT_ROBUSTNESS_2_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT`

# Known issues & F.A.Q.
0. Why do [`PhysicalDeviceRobustness2PropertiesEXT::robust_uniform_buffer_access_size_alignment`] and [`PhysicalDeviceRobustness2PropertiesEXT::robust_storage_buffer_access_size_alignment`] exist?
 **RESOLVED** : Some implementations cannot efficiently tightly bounds-check all
buffer accesses.
Rather, the size of the bound range is padded to some power of two multiple,
up to 256 bytes for uniform buffers and up to 4 bytes for storage buffers,
and that padded size is bounds-checked.
This is sufficient to implement D3D-like behavior, because D3D only allows
binding whole uniform buffers or ranges that are a multiple of 256 bytes,
and D3D raw and structured buffers only support 32-bit accesses.

# Version history
- Revision 1, 2019-11-01 (Jeff Bolz, Liam Middlebrook)  - Initial draft

# Other information
* 2020-01-29
* No known IP claims.
*   - Liam Middlebrook, NVIDIA  - Jeff Bolz, NVIDIA

# Related
- [`PhysicalDeviceRobustness2FeaturesEXT`]
- [`PhysicalDeviceRobustness2PropertiesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        