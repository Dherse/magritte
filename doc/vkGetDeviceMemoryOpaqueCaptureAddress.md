[vkGetDeviceMemoryOpaqueCaptureAddress](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html) - Query an opaque capture address of a memory object

# C Specifications
To query a 64-bit opaque capture address value from a memory object, call:
```c
// Provided by VK_VERSION_1_2
uint64_t vkGetDeviceMemoryOpaqueCaptureAddress(
    VkDevice                                    device,
    const VkDeviceMemoryOpaqueCaptureAddressInfo* pInfo);
```
or the equivalent command
```c
// Provided by VK_KHR_buffer_device_address
uint64_t vkGetDeviceMemoryOpaqueCaptureAddressKHR(
    VkDevice                                    device,
    const VkDeviceMemoryOpaqueCaptureAddressInfo* pInfo);
```

# Parameters
- [`device`] is the logical device that the memory object was allocated on.
- [`p_info`] is a pointer to a [`DeviceMemoryOpaqueCaptureAddressInfo`] structure specifying the memory object to retrieve an address for.

# Description
The 64-bit return value is an opaque address representing the start of
`pInfo->memory`.If the memory object was allocated with a non-zero value of
[`MemoryOpaqueCaptureAddressAllocateInfo::opaque_capture_address`],
the return value  **must**  be the same address.
## Valid Usage
-    The [bufferDeviceAddress](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddress) feature  **must**  be enabled
-    If [`device`] was created with multiple physical devices, then the [bufferDeviceAddressMultiDevice](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddressMultiDevice) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`DeviceMemoryOpaqueCaptureAddressInfo`] structure

# Related
- [`khr_buffer_device_address`]
- [`crate::vulkan1_2`]
- [`Device`]
- [`DeviceMemoryOpaqueCaptureAddressInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        