[VK_KHR_zero_initialize_workgroup_memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_zero_initialize_workgroup_memory.html) - device extension

# Description
This extension allows the use of a null constant initializer on shader
Workgroup memory variables, allowing implementations to expose any special
hardware or instructions they may have.
Zero initialization is commonly used by applications running untrusted
content (e.g. web browsers) as way of defeating memory-scraping attacks.

# Registered extension number
326

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Deprecation state
- *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)

# Contacts
- Alan Baker [alan-baker](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_zero_initialize_workgroup_memory] @alan-baker%0A<<Here describe the issue or question you have about the VK_KHR_zero_initialize_workgroup_memory extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR`]

# New constants
- `VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_EXTENSION_NAME`
- `VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR`

# Version history
- Revision 1, 2020-11-18 (Alan Baker)  - Internal draft version

# Other information
* 2020-11-18
*   - Promoted to Vulkan 1.3 Core 
* No known IP claims.
*   - Alan Baker, Google  - Jeff Bolz, Nvidia  - Jason Ekstrand, Intel

# Related
- [`PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        