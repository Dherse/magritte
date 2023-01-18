[vkGetBufferDeviceAddress](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html) - Query an address of a buffer

# C Specifications
To query a 64-bit buffer device address value through which buffer memory
 **can**  be accessed in a shader, call:
```c
// Provided by VK_VERSION_1_2
VkDeviceAddress vkGetBufferDeviceAddress(
    VkDevice                                    device,
    const VkBufferDeviceAddressInfo*            pInfo);
```
or the equivalent command
```c
// Provided by VK_KHR_buffer_device_address
VkDeviceAddress vkGetBufferDeviceAddressKHR(
    VkDevice                                    device,
    const VkBufferDeviceAddressInfo*            pInfo);
```
or the equivalent command
```c
// Provided by VK_EXT_buffer_device_address
VkDeviceAddress vkGetBufferDeviceAddressEXT(
    VkDevice                                    device,
    const VkBufferDeviceAddressInfo*            pInfo);
```

# Parameters
- [`device`] is the logical device that the buffer was created on.
- [`p_info`] is a pointer to a [`BufferDeviceAddressInfo`] structure specifying the buffer to retrieve an address for.

# Description
The 64-bit return value is an address of the start of `pInfo->buffer`.
The address range starting at this value and whose size is the size of the
buffer  **can**  be used in a shader to access the memory bound to that buffer,
using the
`SPV_KHR_physical_storage_buffer` extension
or the equivalent
`SPV_EXT_physical_storage_buffer` extension
and the `PhysicalStorageBuffer` storage class.
For example, this value  **can**  be stored in a uniform buffer, and the shader
 **can**  read the value from the uniform buffer and use it to do a dependent
read/write to this buffer.
A value of zero is reserved as a “null” pointer and  **must**  not be returned
as a valid buffer device address.
All loads, stores, and atomics in a shader through
`PhysicalStorageBuffer` pointers  **must**  access addresses in the address
range of some buffer.If the buffer was created with a non-zero value of
[`BufferOpaqueCaptureAddressCreateInfo::opaque_capture_address`] or
[`BufferDeviceAddressCreateInfoEXT::device_address`],
the return value will be the same address that was returned at capture time.
## Valid Usage
-    The [bufferDeviceAddress](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddress) or [[`PhysicalDeviceBufferDeviceAddressFeaturesEXT::buffer_device_address`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddressEXT) feature  **must**  be enabled
-    If [`device`] was created with multiple physical devices, then the [bufferDeviceAddressMultiDevice](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddressMultiDevice) or [[`PhysicalDeviceBufferDeviceAddressFeaturesEXT::buffer_device_address_multi_device`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddressMultiDeviceEXT) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`BufferDeviceAddressInfo`] structure

# Related
- [`VK_KHR_buffer_device_address`]
- [`crate::vulkan1_2`]
- [`BufferDeviceAddressInfo`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        