[VkValidationFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFlagsEXT.html) - Specify validation checks to disable for a Vulkan instance

# C Specifications
When creating a Vulkan instance for which you wish to disable validation
checks, add a [`ValidationFlagsEXT`] structure to the [`p_next`] chain
of the [`InstanceCreateInfo`] structure, specifying the checks to be
disabled.
```c
// Provided by VK_EXT_validation_flags
typedef struct VkValidationFlagsEXT {
    VkStructureType                sType;
    const void*                    pNext;
    uint32_t                       disabledValidationCheckCount;
    const VkValidationCheckEXT*    pDisabledValidationChecks;
} VkValidationFlagsEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`disabled_validation_check_count`] is the number of checks to disable.
- [`disabled_validation_checks`] is a pointer to an array of [`ValidationCheckEXT`] values specifying the validation checks to be disabled.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT`
-  [`disabled_validation_checks`] **must**  be a valid pointer to an array of [`disabled_validation_check_count`] valid [`ValidationCheckEXT`] values
-  [`disabled_validation_check_count`] **must**  be greater than `0`

# Related
- [`ext_validation_flags`]
- [`StructureType`]
- [`ValidationCheckEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        