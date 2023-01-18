[vkGetAccelerationStructureDeviceAddressKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html) - Query an address of a acceleration structure

# C Specifications
To query the 64-bit device address for an acceleration structure, call:
```c
// Provided by VK_KHR_acceleration_structure
VkDeviceAddress vkGetAccelerationStructureDeviceAddressKHR(
    VkDevice                                    device,
    const VkAccelerationStructureDeviceAddressInfoKHR* pInfo);
```

# Parameters
- [`device`] is the logical device that the acceleration structure was created on.
- [`p_info`] is a pointer to a [`AccelerationStructureDeviceAddressInfoKHR`] structure specifying the acceleration structure to retrieve an address for.

# Description
The 64-bit return value is an address of the acceleration structure, which
can be used for device and shader operations that involve acceleration
structures, such as
ray traversal and
acceleration structure building.If the acceleration structure was created with a non-zero value of
[`AccelerationStructureCreateInfoKHR::device_address`], the return
value will be the same address.If the acceleration structure was created with a `type` of
`VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`, the returned address  **must** 
be consistent with the relative offset to other acceleration structures with
`type``VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR` allocated with
the same [`Buffer`].
That is, the difference in returned addresses between the two  **must**  be the
same as the difference in offsets provided at acceleration structure
creation.
## Valid Usage
-    If [`device`] was created with multiple physical devices, then the [bufferDeviceAddressMultiDevice](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddressMultiDevice) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`AccelerationStructureDeviceAddressInfoKHR`] structure

# Related
- [`VK_KHR_acceleration_structure`]
- [`AccelerationStructureDeviceAddressInfoKHR`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        