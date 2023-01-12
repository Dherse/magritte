[VkDeviceOrHostAddressKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressKHR.html) - Union specifying a device or host address

# C Specifications
The [`DeviceOrHostAddressKHR`] union is defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef union VkDeviceOrHostAddressKHR {
    VkDeviceAddress    deviceAddress;
    void*              hostAddress;
} VkDeviceOrHostAddressKHR;
```

# Members
- [`device_address`] is a buffer device address as returned by the [`get_buffer_device_address_khr`] command.
- [`host_address`] is a host memory address.

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureBuildGeometryInfoKHR`]
- [`CopyAccelerationStructureToMemoryInfoKHR`]
- [`DeviceAddress`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        