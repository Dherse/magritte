[VK_AMD_device_coherent_memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_device_coherent_memory.html) - device extension

# Description
This extension adds the device coherent and device uncached memory types.
Any device accesses to device coherent memory are automatically made visible
to any other device access.
Device uncached memory indicates to applications that caches are disabled
for a particular memory type, which guarantees device coherence.Device coherent and uncached memory are expected to have lower performance
for general access than non-device coherent memory, but can be useful in
certain scenarios; particularly so for debugging.

# Registered extension number
230

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_device_coherent_memory] @tobski%0A<<Here describe the issue or question you have about the VK_AMD_device_coherent_memory extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceCoherentMemoryFeaturesAMD`]

# New constants
- [`AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME`]
- [`AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION`]
- Extending [`MemoryPropertyFlagBits`]:  - `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD`  - `VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD`

# Version history
- Revision 1, 2019-02-04 (Tobias Hector)  - Initial revision

# Other information
* 2019-02-04
*   - Ping Fu, AMD  - Timothy Lottes, AMD  - Tobias Hector, AMD

# Related
- [`PhysicalDeviceCoherentMemoryFeaturesAMD`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        