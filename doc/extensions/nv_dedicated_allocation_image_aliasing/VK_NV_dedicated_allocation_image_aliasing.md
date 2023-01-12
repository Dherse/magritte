[VK_NV_dedicated_allocation_image_aliasing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_dedicated_allocation_image_aliasing.html) - device extension

# Description
This extension allows applications to alias images on dedicated allocations,
subject to specific restrictions: the extent and the number of layers in the
image being aliased must be smaller than or equal to those of the original
image for which the allocation was created, and every other image parameter
must match.

# Registered extension number
241

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_dedicated_allocation`]`

# Contacts
- Nuno Subtil [nsubtil](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_dedicated_allocation_image_aliasing] @nsubtil%0A<<Here describe the issue or question you have about the VK_NV_dedicated_allocation_image_aliasing extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`]

# New constants
- `VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME`
- `VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV`

# Version history
- Revision 1, 2019-01-04 (Nuno Subtil)  - Internal revisions

# Other information
* 2019-01-04
*   - Nuno Subtil, NVIDIA  - Jeff Bolz, NVIDIA  - Eric Werness, NVIDIA  - Axel Gneiting, id Software

# Related
- [`PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        