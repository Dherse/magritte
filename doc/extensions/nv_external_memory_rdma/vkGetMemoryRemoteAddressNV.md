[vkGetMemoryRemoteAddressNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryRemoteAddressNV.html) - Get an address for a memory object accessible by remote devices

# C Specifications
To export an address representing the payload of a Vulkan device memory
object accessible by remote devices, call:
```c
// Provided by VK_NV_external_memory_rdma
VkResult vkGetMemoryRemoteAddressNV(
    VkDevice                                    device,
    const VkMemoryGetRemoteAddressInfoNV*       pMemoryGetRemoteAddressInfo,
    VkRemoteAddressNV*                          pAddress);
```

# Parameters
- [`device`] is the logical device that created the device memory being exported.
- [`p_memory_get_remote_address_info`] is a pointer to a [`MemoryGetRemoteAddressInfoNV`] structure containing parameters of the export operation.
- [`p_address`] will return the address representing the payload of the device memory object.

# Description
More communication may be required between the kernel-mode drivers of the
devices involved.
This information is out of scope of this documentation and should be
requested from the vendors of the devices.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_memory_get_remote_address_info`] **must**  be a valid pointer to a valid [`MemoryGetRemoteAddressInfoNV`] structure
-  [`p_address`] **must**  be a valid pointer to a [`RemoteAddressNV`] value

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`

# Related
- [`nv_external_memory_rdma`]
- [`Device`]
- [`MemoryGetRemoteAddressInfoNV`]
- [`RemoteAddressNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        