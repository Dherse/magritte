[VkPhysicalDeviceSubpassShadingPropertiesHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassShadingPropertiesHUAWEI.html) - Structure describing subpass shading properties supported by an implementation

# C Specifications
The [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`] structure is
defined as:
```c
// Provided by VK_HUAWEI_subpass_shading
typedef struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           maxSubpassShadingWorkgroupSizeAspectRatio;
} VkPhysicalDeviceSubpassShadingPropertiesHUAWEI;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`max_subpass_shading_workgroup_size_aspect_ratio`] indicates the maximum ratio between the width and height of the portion of the subpass shading shader workgroup size. [`max_subpass_shading_workgroup_size_aspect_ratio`] **must**  be a power-of-two value, and  **must**  be less than or equal to max(`WorkgroupSize.x` / `WorkgroupSize.y`, `WorkgroupSize.y` / `WorkgroupSize.x`).

# Description
If the [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI`

# Related
- [`VK_HUAWEI_subpass_shading`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        