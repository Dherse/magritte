[VkValidationCheckEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCheckEXT.html) - Specify validation checks to disable

# C Specifications
Possible values of elements of the
[`ValidationFlagsEXT::disabled_validation_checks`] array,
specifying validation checks to be disabled, are:
```c
// Provided by VK_EXT_validation_flags
typedef enum VkValidationCheckEXT {
    VK_VALIDATION_CHECK_ALL_EXT = 0,
    VK_VALIDATION_CHECK_SHADERS_EXT = 1,
} VkValidationCheckEXT;
```

# Description
- [`VK_VALIDATION_CHECK_EXT`] specifies that all validation checks are disabled.
- [`VK_VALIDATION_CHECK_EXT`] specifies that shader validation is disabled.

# Related
- [`ext_validation_flags`]
- [`ValidationFlagsEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        