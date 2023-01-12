[VkPhysicalDeviceCooperativeMatrixPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCooperativeMatrixPropertiesNV.html) - Structure describing cooperative matrix properties supported by an implementation

# C Specifications
The [`PhysicalDeviceCooperativeMatrixPropertiesNV`] structure is defined
as:
```c
// Provided by VK_NV_cooperative_matrix
typedef struct VkPhysicalDeviceCooperativeMatrixPropertiesNV {
    VkStructureType       sType;
    void*                 pNext;
    VkShaderStageFlags    cooperativeMatrixSupportedStages;
} VkPhysicalDeviceCooperativeMatrixPropertiesNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`cooperative_matrix_supported_stages`] is a bitfield of [`ShaderStageFlagBits`] describing the shader stages that cooperative matrix instructions are supported in. [`cooperative_matrix_supported_stages`] will have the `VK_SHADER_STAGE_COMPUTE_BIT` bit set if any of the physical device’s queues support `VK_QUEUE_COMPUTE_BIT`.

# Description
If the [`PhysicalDeviceCooperativeMatrixPropertiesNV`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV`

# Related
- [`nv_cooperative_matrix`]
- [VkShaderStageFlags]()
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        