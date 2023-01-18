[VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT.html) - Structure describing max value of vertex attribute divisor that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_vertex_attribute_divisor
typedef struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           maxVertexAttribDivisor;
} VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`max_vertex_attrib_divisor`] is the maximum value of the number of instances that will repeat the value of vertex attribute data when instanced rendering is enabled.

# Description
If the [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT`

# Related
- [`VK_EXT_vertex_attribute_divisor`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        