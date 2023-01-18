[VkDisplayModeCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeCreateInfoKHR.html) - Structure specifying parameters of a newly created display mode object

# C Specifications
The [`DisplayModeCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_display
typedef struct VkDisplayModeCreateInfoKHR {
    VkStructureType                sType;
    const void*                    pNext;
    VkDisplayModeCreateFlagsKHR    flags;
    VkDisplayModeParametersKHR     parameters;
} VkDisplayModeCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use, and  **must**  be zero.
- [`parameters`] is a [`DisplayModeParametersKHR`] structure describing the display parameters to use in creating the new mode. If the parameters are not compatible with the specified display, the implementation  **must**  return `VK_ERROR_INITIALIZATION_FAILED`.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`
-  [`parameters`] **must**  be a valid [`DisplayModeParametersKHR`] structure

# Related
- [`VK_KHR_display`]
- [`DisplayModeCreateFlagsKHR`]
- [`DisplayModeParametersKHR`]
- [`StructureType`]
- [`create_display_mode_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        