[VK_KHR_present_id](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_present_id.html) - device extension

# Description
This device extension allows an application that uses the
`[`VK_KHR_swapchain`]` extension to provide an identifier for present
operations on a swapchain.
An application  **can**  use this to reference specific present operations in
other extensions.

# Registered extension number
295

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_swapchain`]`

# Contacts
- Keith Packard [keithp](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_present_id] @keithp%0A<<Here describe the issue or question you have about the VK_KHR_present_id extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDevicePresentIdFeaturesKHR`] 
- Extending [`PresentInfoKHR`]:  - [`PresentIdKHR`]

# New constants
- [`KHR_PRESENT_ID_EXTENSION_NAME`]
- [`KHR_PRESENT_ID_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR`  - `VK_STRUCTURE_TYPE_PRESENT_ID_KHR`

# Known issues & F.A.Q.
None.

# Version history
- Revision 1, 2019-05-15 (Keith Packard)  - Initial version

# Other information
* 2019-05-15
* No known IP claims.
*   - Keith Packard, Valve  - Ian Elliott, Google  - Alon Or-bach, Samsung

# Related
- [`PhysicalDevicePresentIdFeaturesKHR`]
- [`PresentIdKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        