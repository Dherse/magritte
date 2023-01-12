[VkExtent2D](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExtent2D.html) - Structure specifying a two-dimensional extent

# C Specifications
A two-dimensional extent is defined by the structure:
```c
// Provided by VK_VERSION_1_0
typedef struct VkExtent2D {
    uint32_t    width;
    uint32_t    height;
} VkExtent2D;
```

# Members
- [`width`] is the width of the extent.
- [`height`] is the height of the extent.

# Related
- [`crate::vulkan1_0`]
- [`DisplayModeParametersKHR`]
- [`DisplayPlaneCapabilitiesKHR`]
- [`DisplayPropertiesKHR`]
- [`DisplaySurfaceCreateInfoKHR`]
- [`FragmentShadingRateAttachmentInfoKHR`]
- [`MultisamplePropertiesEXT`]
- [`PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM`]
- [`PhysicalDeviceFragmentDensityMapPropertiesEXT`]
- [`PhysicalDeviceFragmentShadingRateKHR`]
- [`PhysicalDeviceFragmentShadingRatePropertiesKHR`]
- [`PhysicalDeviceSampleLocationsPropertiesEXT`]
- [`PhysicalDeviceShadingRateImagePropertiesNV`]
- [`PipelineFragmentShadingRateStateCreateInfoKHR`]
- [`Rect2D`]
- [`RectLayerKHR`]
- [`RenderingFragmentShadingRateAttachmentInfoKHR`]
- [`SampleLocationsInfoEXT`]
- [`SurfaceCapabilities2EXT`]
- [`SurfaceCapabilitiesKHR`]
- [`SwapchainCreateInfoKHR`]
- [`VideoCapabilitiesKHR`]
- [`VideoDecodeInfoKHR`]
- [`VideoEncodeCapabilitiesKHR`]
- [`VideoEncodeInfoKHR`]
- [`VideoPictureResourceKHR`]
- [`VideoSessionCreateInfoKHR`]
- [`cmd_set_fragment_shading_rate_khr`]
- [`get_device_subpass_shading_max_workgroup_size_huawei`]
- [`get_render_area_granularity`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        