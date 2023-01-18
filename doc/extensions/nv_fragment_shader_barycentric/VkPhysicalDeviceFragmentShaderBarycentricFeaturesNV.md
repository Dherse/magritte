[VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV.html) - Structure describing barycentric support in fragment shaders that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceFragmentShaderBarycentricFeaturesNV`] structure is
defined as:
```c
// Provided by VK_NV_fragment_shader_barycentric
typedef struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           fragmentShaderBarycentric;
} VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV;
```

# Members
This structure describes the following feature:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`fragment_shader_barycentric`] indicates that the implementation supports the `BaryCoordNV` and `BaryCoordNoPerspNV` SPIR-V fragment shader built-ins and supports the `PerVertexNV` SPIR-V decoration on fragment shader input variables.
See [Barycentric Interpolation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-barycentric) for more
information.If the [`PhysicalDeviceFragmentShaderBarycentricFeaturesNV`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceFragmentShaderBarycentricFeaturesNV`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV`

# Related
- [`VK_NV_fragment_shader_barycentric`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        