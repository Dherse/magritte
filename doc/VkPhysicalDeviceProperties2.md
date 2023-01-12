[VkPhysicalDeviceProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties2.html) - Structure specifying physical device properties

# C Specifications
The [`PhysicalDeviceProperties2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDeviceProperties2 {
    VkStructureType               sType;
    void*                         pNext;
    VkPhysicalDeviceProperties    properties;
} VkPhysicalDeviceProperties2;
```
or the equivalent
```c
// Provided by VK_KHR_get_physical_device_properties2
typedef VkPhysicalDeviceProperties2 VkPhysicalDeviceProperties2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`properties`] is a [`PhysicalDeviceProperties`] structure describing properties of the physical device. This structure is written with the same values as if it were written by [`get_physical_device_properties`].

# Description
The [`p_next`] chain of this structure is used to extend the structure with
properties defined by extensions.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`PhysicalDeviceAccelerationStructurePropertiesKHR`], [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`], [`PhysicalDeviceConservativeRasterizationPropertiesEXT`], [`PhysicalDeviceCooperativeMatrixPropertiesNV`], [`PhysicalDeviceCustomBorderColorPropertiesEXT`], [`PhysicalDeviceDepthStencilResolveProperties`], [`PhysicalDeviceDescriptorIndexingProperties`], [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`], [`PhysicalDeviceDiscardRectanglePropertiesEXT`], [`PhysicalDeviceDriverProperties`], [`PhysicalDeviceDrmPropertiesEXT`], [`PhysicalDeviceExternalMemoryHostPropertiesEXT`], [`PhysicalDeviceFloatControlsProperties`], [`PhysicalDeviceFragmentDensityMap2PropertiesEXT`], [`PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM`], [`PhysicalDeviceFragmentDensityMapPropertiesEXT`], [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`], [`PhysicalDeviceFragmentShadingRatePropertiesKHR`], [`PhysicalDeviceIdProperties`], [`PhysicalDeviceInlineUniformBlockProperties`], [`PhysicalDeviceLineRasterizationPropertiesEXT`], [`PhysicalDeviceMaintenance3Properties`], [`PhysicalDeviceMaintenance4Properties`], [`PhysicalDeviceMeshShaderPropertiesNV`], [`PhysicalDeviceMultiDrawPropertiesEXT`], [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`], [`PhysicalDeviceMultiviewProperties`], [`PhysicalDevicePciBusInfoPropertiesEXT`], [`PhysicalDevicePerformanceQueryPropertiesKHR`], [`PhysicalDevicePointClippingProperties`], [`PhysicalDevicePortabilitySubsetPropertiesKHR`], [`PhysicalDeviceProtectedMemoryProperties`], [`PhysicalDeviceProvokingVertexPropertiesEXT`], [`PhysicalDevicePushDescriptorPropertiesKHR`], [`PhysicalDeviceRayTracingPipelinePropertiesKHR`], [`PhysicalDeviceRayTracingPropertiesNV`], [`PhysicalDeviceRobustness2PropertiesEXT`], [`PhysicalDeviceSampleLocationsPropertiesEXT`], [`PhysicalDeviceSamplerFilterMinmaxProperties`], [`PhysicalDeviceShaderCoreProperties2AMD`], [`PhysicalDeviceShaderCorePropertiesAMD`], [`PhysicalDeviceShaderIntegerDotProductProperties`], [`PhysicalDeviceShaderSmBuiltinsPropertiesNV`], [`PhysicalDeviceShadingRateImagePropertiesNV`], [`PhysicalDeviceSubgroupProperties`], [`PhysicalDeviceSubgroupSizeControlProperties`], [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`], [`PhysicalDeviceTexelBufferAlignmentProperties`], [`PhysicalDeviceTimelineSemaphoreProperties`], [`PhysicalDeviceTransformFeedbackPropertiesEXT`], [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`], [`PhysicalDeviceVulkan11Properties`], [`PhysicalDeviceVulkan12Properties`], or [`PhysicalDeviceVulkan13Properties`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique

# Related
- [`crate::vulkan1_1`]
- [`PhysicalDeviceProperties`]
- [`StructureType`]
- [`get_physical_device_properties2`]
- [`get_physical_device_properties2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        