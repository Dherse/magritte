[VkPhysicalDeviceType](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceType.html) - Supported physical device types

# C Specifications
The physical device types which  **may**  be returned in
[`PhysicalDeviceProperties::device_type`] are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkPhysicalDeviceType {
    VK_PHYSICAL_DEVICE_TYPE_OTHER = 0,
    VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1,
    VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2,
    VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3,
    VK_PHYSICAL_DEVICE_TYPE_CPU = 4,
} VkPhysicalDeviceType;
```

# Description
- [`OTHER`] - the device does not match any other available types.
- [`INTEGRATED_GPU`] - the device is typically one embedded in or tightly coupled with the host.
- [`DISCRETE_GPU`] - the device is typically a separate processor connected to the host via an interlink.
- [`VIRTUAL_GPU`] - the device is typically a virtual node in a virtualization environment.
- [`CPU`] - the device is typically running on the same processors as the host.
The physical device type is advertised for informational purposes only, and
does not directly affect the operation of the system.
However, the device type  **may**  correlate with other advertised properties or
capabilities of the system, such as how many memory heaps there are.

# Related
- [`crate::vulkan1_0`]
- [`PhysicalDeviceProperties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        