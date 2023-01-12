[VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.html) - Structure indicating support for fragment shading rate enums

# C Specifications
The [`PhysicalDeviceFragmentShadingRateEnumsFeaturesNV`] structure is
defined as:
```c
// Provided by VK_NV_fragment_shading_rate_enums
typedef struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           fragmentShadingRateEnums;
    VkBool32           supersampleFragmentShadingRates;
    VkBool32           noInvocationFragmentShadingRates;
} VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`fragment_shading_rate_enums`] indicates that the implementation supports specifying fragment shading rates using the [`FragmentShadingRateNV`] enumerated type.
- [`supersample_fragment_shading_rates`] indicates that the implementation supports fragment shading rate enum values indicating more than one invocation per fragment.
- [`no_invocation_fragment_shading_rates`] indicates that the implementation supports a fragment shading rate enum value indicating that no fragment shaders should be invoked when that shading rate is used.
If the [`PhysicalDeviceFragmentShadingRateEnumsFeaturesNV`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceFragmentShadingRateEnumsFeaturesNV`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV`

# Related
- [`nv_fragment_shading_rate_enums`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        