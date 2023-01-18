[VkPhysicalDeviceFragmentDensityMapPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html) - Structure describing fragment density map properties that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceFragmentDensityMapPropertiesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_fragment_density_map
typedef struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkExtent2D         minFragmentDensityTexelSize;
    VkExtent2D         maxFragmentDensityTexelSize;
    VkBool32           fragmentDensityInvocations;
} VkPhysicalDeviceFragmentDensityMapPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`min_fragment_density_texel_size`] is the minimum [fragment density texel size](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-fragment-density-texel-size).
- [`max_fragment_density_texel_size`] is the maximum fragment density texel size.
- [`fragment_density_invocations`] specifies whether the implementation  **may**  invoke additional fragment shader invocations for each covered sample.

# Description
If the [`PhysicalDeviceFragmentDensityMapPropertiesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT`

# Related
- [`VK_EXT_fragment_density_map`]
- [`Bool32`]
- [`Extent2D`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        