[VkPhysicalDeviceFragmentShadingRateKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateKHR.html) - Structure returning information about sample count specific additional multisampling capabilities

# C Specifications
The [`PhysicalDeviceFragmentShadingRateKHR`] structure is defined as
```c
// Provided by VK_KHR_fragment_shading_rate
typedef struct VkPhysicalDeviceFragmentShadingRateKHR {
    VkStructureType       sType;
    void*                 pNext;
    VkSampleCountFlags    sampleCounts;
    VkExtent2D            fragmentSize;
} VkPhysicalDeviceFragmentShadingRateKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`sample_counts`] is a bitmask of sample counts for which the shading rate described by [`fragment_size`] is supported.
- [`fragment_size`] is a [`Extent2D`] describing the width and height of a supported shading rate.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_KHR_fragment_shading_rate`]
- [`Extent2D`]
- [`SampleCountFlags`]
- [`StructureType`]
- [`get_physical_device_fragment_shading_rates_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        