[VkConformanceVersion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConformanceVersion.html) - Structure containing the conformance test suite version the implementation is compliant with

# C Specifications
The conformance test suite version an implementation is compliant with is
described with the [`ConformanceVersion`] structure:
```c
// Provided by VK_VERSION_1_2
typedef struct VkConformanceVersion {
    uint8_t    major;
    uint8_t    minor;
    uint8_t    subminor;
    uint8_t    patch;
} VkConformanceVersion;
```
or the equivalent
```c
// Provided by VK_KHR_driver_properties
typedef VkConformanceVersion VkConformanceVersionKHR;
```

# Members
- [`major`] is the major version number of the conformance test suite.
- [`minor`] is the minor version number of the conformance test suite.
- [`subminor`] is the subminor version number of the conformance test suite.
- [`patch`] is the patch version number of the conformance test suite.

# Related
- [`khr_driver_properties`]
- [`crate::vulkan1_2`]
- [`PhysicalDeviceDriverProperties`]
- [`PhysicalDeviceVulkan12Properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        