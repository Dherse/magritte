[VkVertexInputBindingDivisorDescriptionEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDivisorDescriptionEXT.html) - Structure specifying a divisor used in instanced rendering

# C Specifications
The individual divisor values per binding are specified using the
[`VertexInputBindingDivisorDescriptionEXT`] structure which is defined
as:
```c
// Provided by VK_EXT_vertex_attribute_divisor
typedef struct VkVertexInputBindingDivisorDescriptionEXT {
    uint32_t    binding;
    uint32_t    divisor;
} VkVertexInputBindingDivisorDescriptionEXT;
```

# Members
- [`binding`] is the binding number for which the divisor is specified.
- [`divisor`] is the number of successive instances that will use the same value of the vertex attribute when instanced rendering is enabled. For example, if the divisor is N, the same vertex attribute will be applied to N successive instances before moving on to the next vertex attribute. The maximum value of [`divisor`] is implementation-dependent and can be queried using [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::max_vertex_attrib_divisor`]. A value of `0` **can**  be used for the divisor if the [`vertexAttributeInstanceRateZeroDivisor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateZeroDivisor) feature is enabled. In this case, the same vertex attribute will be applied to all instances.

# Description
If this structure is not used to define a divisor value for an attribute,
then the divisor has a logical default value of 1.
## Valid Usage
-  [`binding`] **must**  be less than [`PhysicalDeviceLimits::max_vertex_input_bindings`]
-    If the `vertexAttributeInstanceRateZeroDivisor` feature is not enabled, [`divisor`] **must**  not be `0`
-    If the `vertexAttributeInstanceRateDivisor` feature is not enabled, [`divisor`] **must**  be `1`
-  [`divisor`] **must**  be a value between `0` and [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::max_vertex_attrib_divisor`], inclusive
-  [`VertexInputBindingDescription::input_rate`] **must**  be of type `VK_VERTEX_INPUT_RATE_INSTANCE` for this [`binding`]

# Related
- [`VK_EXT_vertex_attribute_divisor`]
- [`PipelineVertexInputDivisorStateCreateInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        