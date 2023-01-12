[VK_EXT_buffer_device_address](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_buffer_device_address.html) - device extension

# Description
This extension allows the application to query a 64-bit buffer device
address value for a buffer, which can be used to access the buffer memory
via the `PhysicalStorageBufferEXT` storage class in the
[`GL_EXT_buffer_reference`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference.txt)
GLSL extension and
[`SPV_EXT_physical_storage_buffer`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_physical_storage_buffer.html)
SPIR-V extension.It also allows buffer device addresses to be provided by a trace replay
tool, so that it matches the address used when the trace was captured.

# Registered extension number
245

# Revision
2

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Deprecation state
- *Deprecated* by `[`khr_buffer_device_address`]` extension  - Which in turn was *promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)

# Contacts
- Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_buffer_device_address] @jeffbolznv%0A<<Here describe the issue or question you have about the VK_EXT_buffer_device_address extension>>)

# New commands
- [`get_buffer_device_address_ext`]

# New structures
- [`BufferDeviceAddressInfoEXT`]
- Extending [`BufferCreateInfo`]:  - [`BufferDeviceAddressCreateInfoEXT`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceBufferAddressFeaturesEXT`]  - [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`]

# New constants
- `VK_EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME`
- `VK_EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION`
- Extending [`BufferCreateFlagBits`]:  - `VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT` 
- Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT` 
- Extending [`VulkanResultCodes`]:  - `VK_ERROR_INVALID_DEVICE_ADDRESS_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT`  - `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT`

# Known issues & F.A.Q.
1) Where is VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT
and VkPhysicalDeviceBufferAddressFeaturesEXT? **RESOLVED** : They were renamed as
`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT`
and [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`] accordingly for
consistency.
Even though, the old names can still be found in the generated header files
for compatibility.

# Version history
- Revision 1, 2018-11-01 (Jeff Bolz)  - Internal revisions 
- Revision 2, 2019-01-06 (Jon Leech)  - Minor updates to appendix for publication

# Other information
* 2019-01-06
* No known IP claims.
*   - This extension requires [`SPV_EXT_physical_storage_buffer`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_physical_storage_buffer.html)  - This extension provides API support for [`GLSL_EXT_buffer_reference`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference.txt) and [`GLSL_EXT_buffer_reference_uvec2`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference_uvec2.txt) 
*   - Jeff Bolz, NVIDIA  - Neil Henning, AMD  - Tobias Hector, AMD  - Jason Ekstrand, Intel  - Baldur Karlsson, Valve

# Related
- [`BufferDeviceAddressCreateInfoEXT`]
- [`BufferDeviceAddressInfoEXT`]
- [`PhysicalDeviceBufferAddressFeaturesEXT`]
- [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`]
- [`get_buffer_device_address_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        