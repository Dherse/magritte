[VK_UUID_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_UUID_SIZE.html) - Length of a universally unique device or driver build identifier

# C Specifications
[`UUID_SIZE`] is the length in `uint8_t` values of an array
containing a universally unique device or driver build identifier, as
returned in [`PhysicalDeviceIdProperties`]::deviceUUID and
[`PhysicalDeviceIdProperties`]::driverUUID.
```c
#define VK_UUID_SIZE                      16U
```
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        