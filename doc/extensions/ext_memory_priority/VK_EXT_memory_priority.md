[VK_EXT_memory_priority](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_memory_priority.html) - device extension

# Description
This extension adds a `priority` value specified at memory allocation
time.
On some systems with both device-local and non-device-local memory heaps,
the implementation may transparently move memory from one heap to another
when a heap becomes full (for example, when the total memory used across all
processes exceeds the size of the heap).
In such a case, this priority value may be used to determine which
allocations are more likely to remain in device-local memory.

# Registered extension number
239

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Contacts
- Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_memory_priority] @jeffbolznv%0A<<Here describe the issue or question you have about the VK_EXT_memory_priority extension>>)

# New structures
- Extending [`MemoryAllocateInfo`]:  - [`MemoryPriorityAllocateInfoEXT`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceMemoryPriorityFeaturesEXT`]

# New constants
- `VK_EXT_MEMORY_PRIORITY_EXTENSION_NAME`
- `VK_EXT_MEMORY_PRIORITY_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT`

# Version history
- Revision 1, 2018-10-08 (Jeff Bolz)  - Initial revision

# Other information
* 2018-10-08
*   - Jeff Bolz, NVIDIA  - Jeff Juliano, NVIDIA

# Related
- [`MemoryPriorityAllocateInfoEXT`]
- [`PhysicalDeviceMemoryPriorityFeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        