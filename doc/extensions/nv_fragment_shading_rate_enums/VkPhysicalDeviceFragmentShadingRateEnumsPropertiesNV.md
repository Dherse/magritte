[VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.html) - Structure describing fragment shading rate limits that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`] structure is
defined as:
```c
// Provided by VK_NV_fragment_shading_rate_enums
typedef struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    VkStructureType          sType;
    void*                    pNext;
    VkSampleCountFlagBits    maxFragmentShadingRateInvocationCount;
} VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`max_fragment_shading_rate_invocation_count`] is a [`SampleCountFlagBits`] value indicating the maximum number of fragment shader invocations per fragment supported in pipeline, primitive, and attachment fragment shading rates.

# Description
If the [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.These properties are related to [fragment
shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate).
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV`
-  [`max_fragment_shading_rate_invocation_count`] **must**  be a valid [`SampleCountFlagBits`] value

# Related
- [`nv_fragment_shading_rate_enums`]
- [`SampleCountFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        