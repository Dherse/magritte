[VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT.html) - Structure describing fragment shader interlock features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_fragment_shader_interlock
typedef struct VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           fragmentShaderSampleInterlock;
    VkBool32           fragmentShaderPixelInterlock;
    VkBool32           fragmentShaderShadingRateInterlock;
} VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`fragment_shader_sample_interlock`] indicates that the implementation supports the `FragmentShaderSampleInterlockEXT` SPIR-V capability.
- [`fragment_shader_pixel_interlock`] indicates that the implementation supports the `FragmentShaderPixelInterlockEXT` SPIR-V capability.
- [`fragment_shader_shading_rate_interlock`] indicates that the implementation supports the `FragmentShaderShadingRateInterlockEXT` SPIR-V capability.
If the [`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT`

# Related
- [`VK_EXT_fragment_shader_interlock`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        