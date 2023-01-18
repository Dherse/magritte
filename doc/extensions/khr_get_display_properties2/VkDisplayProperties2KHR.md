[VkDisplayProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayProperties2KHR.html) - Structure describing an available display device

# C Specifications
The [`DisplayProperties2KHR`] structure is defined as:
```c
// Provided by VK_KHR_get_display_properties2
typedef struct VkDisplayProperties2KHR {
    VkStructureType           sType;
    void*                     pNext;
    VkDisplayPropertiesKHR    displayProperties;
} VkDisplayProperties2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`display_properties`] is a [`DisplayPropertiesKHR`] structure.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_KHR_get_display_properties2`]
- [`DisplayPropertiesKHR`]
- [`StructureType`]
- [`get_physical_device_display_properties2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        