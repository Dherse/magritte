[VkVertexInputBindingDescription2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDescription2EXT.html) - Structure specifying the extended vertex input binding description

# C Specifications
The [`VertexInputBindingDescription2EXT`] structure is defined as:
```c
// Provided by VK_EXT_vertex_input_dynamic_state
typedef struct VkVertexInputBindingDescription2EXT {
    VkStructureType      sType;
    void*                pNext;
    uint32_t             binding;
    uint32_t             stride;
    VkVertexInputRate    inputRate;
    uint32_t             divisor;
} VkVertexInputBindingDescription2EXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`binding`] is the binding number that this structure describes.
- [`stride`] is the byte stride between consecutive elements within the buffer.
- [`input_rate`] is a [`VertexInputRate`] value specifying whether vertex attribute addressing is a function of the vertex index or of the instance index.
- [`divisor`] is the number of successive instances that will use the same value of the vertex attribute when instanced rendering is enabled. This member  **can**  be set to a value other than `1` if the [vertexAttributeInstanceRateDivisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateDivisor) feature is enabled. For example, if the divisor is N, the same vertex attribute will be applied to N successive instances before moving on to the next vertex attribute. The maximum value of [`divisor`] is implementation-dependent and can be queried using [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::max_vertex_attrib_divisor`]. A value of `0` **can**  be used for the divisor if the [`vertexAttributeInstanceRateZeroDivisor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateZeroDivisor) feature is enabled. In this case, the same vertex attribute will be applied to all instances.

# Description
## Valid Usage
-  [`binding`] **must**  be less than [`PhysicalDeviceLimits::max_vertex_input_bindings`]
-  [`stride`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_vertex_input_binding_stride`]
-    If the [vertexAttributeInstanceRateZeroDivisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateZeroDivisor) feature is not enabled, [`divisor`] **must**  not be `0`
-    If the [vertexAttributeInstanceRateDivisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateDivisor) feature is not enabled, [`divisor`] **must**  be `1`
-  [`divisor`] **must**  be a value between `0` and [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::max_vertex_attrib_divisor`], inclusive
-    If [`divisor`] is not `1` then [`input_rate`] **must**  be of type `VK_VERTEX_INPUT_RATE_INSTANCE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT`
-  [`input_rate`] **must**  be a valid [`VertexInputRate`] value

# Related
- [`ext_vertex_input_dynamic_state`]
- [`StructureType`]
- [`VertexInputRate`]
- [`cmd_set_vertex_input_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        