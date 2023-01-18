[VK_KHR_get_physical_device_properties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_physical_device_properties2.html) - instance extension

# Description
This extension provides new entry points to query device features, device
properties, and format properties in a way that can be easily extended by
other extensions, without introducing any further entry points.
The Vulkan 1.0 feature/limit/formatproperty structures do not include
`sType`/`pNext` members.
This extension wraps them in new structures with `sType`/`pNext`
members, so an application can query a chain of feature/limit/formatproperty
structures by constructing the chain and letting the implementation fill
them in.
A new command is added for each `vkGetPhysicalDevice*` command in core
Vulkan 1.0.
The new feature structure (and a `pNext` chain of extending structures)
can also be passed in to device creation to enable features.This extension also allows applications to use the physical-device
components of device extensions before [`create_device`] is called.

# Registered extension number
60

# Revision
2

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)

# Contacts
- Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_get_physical_device_properties2] @jeffbolznv%0A<<Here describe the issue or question you have about the VK_KHR_get_physical_device_properties2 extension>>)

# New commands
- [`get_physical_device_features2_khr`]
- [`get_physical_device_format_properties2_khr`]
- [`get_physical_device_image_format_properties2_khr`]
- [`get_physical_device_memory_properties2_khr`]
- [`get_physical_device_properties2_khr`]
- [`get_physical_device_queue_family_properties2_khr`]
- [`get_physical_device_sparse_image_format_properties2_khr`]

# New structures
- [`FormatProperties2KHR`]
- [`ImageFormatProperties2KHR`]
- [`PhysicalDeviceImageFormatInfo2KHR`]
- [`PhysicalDeviceMemoryProperties2KHR`]
- [`PhysicalDeviceProperties2KHR`]
- [`PhysicalDeviceSparseImageFormatInfo2KHR`]
- [`QueueFamilyProperties2KHR`]
- [`SparseImageFormatProperties2KHR`]
- Extending [`DeviceCreateInfo`]:  - [`PhysicalDeviceFeatures2KHR`]

# New constants
- [`KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME`]
- [`KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR`  - `VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR`  - `VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR`  - `VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR`

# Version history
- Revision 1, 2016-09-12 (Jeff Bolz)  - Internal revisions 
- Revision 2, 2016-11-02 (Ian Elliott)  - Added ability for applications to use the physical-device components of device extensions before vkCreateDevice is called.

# Other information
* 2017-09-05
* No known IP claims.
*   - Promoted to Vulkan 1.1 Core 
*   - Jeff Bolz, NVIDIA  - Ian Elliott, Google

# Related
- [`FormatProperties2KHR`]
- [`ImageFormatProperties2KHR`]
- [`PhysicalDeviceFeatures2KHR`]
- [`PhysicalDeviceImageFormatInfo2KHR`]
- [`PhysicalDeviceMemoryProperties2KHR`]
- [`PhysicalDeviceProperties2KHR`]
- [`PhysicalDeviceSparseImageFormatInfo2KHR`]
- [`QueueFamilyProperties2KHR`]
- [`SparseImageFormatProperties2KHR`]
- [`get_physical_device_features2_khr`]
- [`get_physical_device_format_properties2_khr`]
- [`get_physical_device_image_format_properties2_khr`]
- [`get_physical_device_memory_properties2_khr`]
- [`get_physical_device_properties2_khr`]
- [`get_physical_device_queue_family_properties2_khr`]
- [`get_physical_device_sparse_image_format_properties2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        