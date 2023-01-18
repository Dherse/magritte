[VK_MAX_DRIVER_INFO_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_INFO_SIZE.html) - Length of a physical device driver information string

# C Specifications
[`MAX_DRIVER_INFO_SIZE`] is the length in `char` values of an array
containing a driver information string, as returned in
[`PhysicalDeviceDriverProperties`]::driverInfo.
```c
#define VK_MAX_DRIVER_INFO_SIZE           256U
```
or the equivalent
```c
#define VK_MAX_DRIVER_INFO_SIZE_KHR       VK_MAX_DRIVER_INFO_SIZE
```

# Related
- [`VK_KHR_driver_properties`]
- [`crate::vulkan1_2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        