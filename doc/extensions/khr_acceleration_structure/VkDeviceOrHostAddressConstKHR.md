[VkDeviceOrHostAddressConstKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressConstKHR.html) - Union specifying a const device or host address

# C Specifications
The [`DeviceOrHostAddressConstKHR`] union is defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef union VkDeviceOrHostAddressConstKHR {
    VkDeviceAddress    deviceAddress;
    const void*        hostAddress;
} VkDeviceOrHostAddressConstKHR;
```

# Members
- [`device_address`] is a buffer device address as returned by the [`get_buffer_device_address_khr`] command.
- [`host_address`] is a const host memory address.

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureGeometryAabbsDataKHR`]
- [`AccelerationStructureGeometryInstancesDataKHR`]
- [`AccelerationStructureGeometryMotionTrianglesDataNV`]
- [`AccelerationStructureGeometryTrianglesDataKHR`]
- [`CopyMemoryToAccelerationStructureInfoKHR`]
- [`DeviceAddress`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        