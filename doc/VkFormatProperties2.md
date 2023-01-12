[VkFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatProperties2.html) - Structure specifying image format properties

# C Specifications
The [`FormatProperties2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkFormatProperties2 {
    VkStructureType       sType;
    void*                 pNext;
    VkFormatProperties    formatProperties;
} VkFormatProperties2;
```
or the equivalent
```c
// Provided by VK_KHR_get_physical_device_properties2
typedef VkFormatProperties2 VkFormatProperties2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`format_properties`] is a [`FormatProperties`] structure describing features supported by the requested format.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`DrmFormatModifierPropertiesList2EXT`], [`DrmFormatModifierPropertiesListEXT`], [`FormatProperties3`], [`VideoDecodeH264ProfileEXT`], [`VideoDecodeH265ProfileEXT`], [`VideoEncodeH264ProfileEXT`], [`VideoEncodeH265ProfileEXT`], [`VideoProfileKHR`], or [`VideoProfilesKHR`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique

# Related
- [`crate::vulkan1_1`]
- [`FormatProperties`]
- [`StructureType`]
- [`get_physical_device_format_properties2`]
- [`get_physical_device_format_properties2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        