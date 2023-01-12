[VkDisplayPowerInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPowerInfoEXT.html) - Describe the power state of a display

# C Specifications
The [`DisplayPowerInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_display_control
typedef struct VkDisplayPowerInfoEXT {
    VkStructureType           sType;
    const void*               pNext;
    VkDisplayPowerStateEXT    powerState;
} VkDisplayPowerInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`power_state`] is a [`DisplayPowerStateEXT`] value specifying the new power state of the display.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`power_state`] **must**  be a valid [`DisplayPowerStateEXT`] value

# Related
- [`ext_display_control`]
- [`DisplayPowerStateEXT`]
- [`StructureType`]
- [`display_power_control_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        