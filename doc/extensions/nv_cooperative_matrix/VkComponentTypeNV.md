[VkComponentTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComponentTypeNV.html) - Specify SPIR-V cooperative matrix component type

# C Specifications
Possible values for [`ComponentTypeNV`] include:
```c
// Provided by VK_NV_cooperative_matrix
typedef enum VkComponentTypeNV {
    VK_COMPONENT_TYPE_FLOAT16_NV = 0,
    VK_COMPONENT_TYPE_FLOAT32_NV = 1,
    VK_COMPONENT_TYPE_FLOAT64_NV = 2,
    VK_COMPONENT_TYPE_SINT8_NV = 3,
    VK_COMPONENT_TYPE_SINT16_NV = 4,
    VK_COMPONENT_TYPE_SINT32_NV = 5,
    VK_COMPONENT_TYPE_SINT64_NV = 6,
    VK_COMPONENT_TYPE_UINT8_NV = 7,
    VK_COMPONENT_TYPE_UINT16_NV = 8,
    VK_COMPONENT_TYPE_UINT32_NV = 9,
    VK_COMPONENT_TYPE_UINT64_NV = 10,
} VkComponentTypeNV;
```

# Description
- [`FLOAT16`] corresponds to SPIR-V `OpTypeFloat` 16.
- [`FLOAT32`] corresponds to SPIR-V `OpTypeFloat` 32.
- [`FLOAT64`] corresponds to SPIR-V `OpTypeFloat` 64.
- [`SINT8`] corresponds to SPIR-V `OpTypeInt` 8 1.
- [`SINT16`] corresponds to SPIR-V `OpTypeInt` 16 1.
- [`SINT32`] corresponds to SPIR-V `OpTypeInt` 32 1.
- [`SINT64`] corresponds to SPIR-V `OpTypeInt` 64 1.
- [`UINT8`] corresponds to SPIR-V `OpTypeInt` 8 0.
- [`UINT16`] corresponds to SPIR-V `OpTypeInt` 16 0.
- [`UINT32`] corresponds to SPIR-V `OpTypeInt` 32 0.
- [`UINT64`] corresponds to SPIR-V `OpTypeInt` 64 0.

# Related
- [`VK_NV_cooperative_matrix`]
- [`CooperativeMatrixPropertiesNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        