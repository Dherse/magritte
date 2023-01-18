[VkDisplayEventInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayEventInfoEXT.html) - Describe a display event to create

# C Specifications
The [`DisplayEventInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_display_control
typedef struct VkDisplayEventInfoEXT {
    VkStructureType          sType;
    const void*              pNext;
    VkDisplayEventTypeEXT    displayEvent;
} VkDisplayEventInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`display_event`] is a [`DisplayEventTypeEXT`] specifying when the fence will be signaled.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`display_event`] **must**  be a valid [`DisplayEventTypeEXT`] value

# Related
- [`VK_EXT_display_control`]
- [`DisplayEventTypeEXT`]
- [`StructureType`]
- [`register_display_event_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        