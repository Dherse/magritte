[VK_AMD_memory_overallocation_behavior](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_memory_overallocation_behavior.html) - device extension

# Description
This extension allows controlling whether explicit overallocation beyond the
device memory heap sizes (reported by
[`PhysicalDeviceMemoryProperties`]) is allowed or not.
Overallocation may lead to performance loss and is not supported for all
platforms.

# Registered extension number
190

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Martin Dinkov [mdinkov](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_memory_overallocation_behavior] @mdinkov%0A<<Here describe the issue or question you have about the VK_AMD_memory_overallocation_behavior extension>>)

# New structures
- Extending [`DeviceCreateInfo`]:  - [`DeviceMemoryOverallocationCreateInfoAMD`]

# New enums
- [`MemoryOverallocationBehaviorAMD`]

# New constants
- [`AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME`]
- [`AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD`

# Version history
- Revision 1, 2018-09-19 (Martin Dinkov)  - Initial draft.

# Other information
* 2018-09-19
* No known IP claims.
*   - Martin Dinkov, AMD  - Matthaeus Chajdas, AMD  - Daniel Rakos, AMD  - Jon Campbell, AMD

# Related
- [`DeviceMemoryOverallocationCreateInfoAMD`]
- [`MemoryOverallocationBehaviorAMD`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        