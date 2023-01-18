[VK_KHR_fragment_shading_rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_fragment_shading_rate.html) - device extension

# Description
This extension adds the ability to change the rate at which fragments are
shaded.
Rather than the usual single fragment invocation for each pixel covered by a
primitive, multiple pixels can be shaded by a single fragment shader
invocation.Up to three methods are available to the application to change the fragment
shading rate:
- [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline), which allows the specification of a rate per-draw.
- [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive), which allows the specification of a rate per primitive, specified during shading.
- [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment), which allows the specification of a rate per-region of the framebuffer, specified in a specialized image attachment.
Additionally, these rates can all be specified and combined in order to
adjust the overall detail in the image at each point.This functionality can be used to focus shading efforts where higher levels
of detail are needed in some parts of a scene compared to others.
This can be particularly useful in high resolution rendering, or for XR
contexts.This extension also adds support for the `SPV_KHR_fragment_shading_rate`
extension which enables setting the
[primitive fragment shading
rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive), and allows querying the final shading rate from a fragment shader.

# Registered extension number
227

# Revision
2

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_create_renderpass2`]`
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_fragment_shading_rate] @tobski%0A<<Here describe the issue or question you have about the VK_KHR_fragment_shading_rate extension>>)

# New commands
- [`cmd_set_fragment_shading_rate_khr`]
- [`get_physical_device_fragment_shading_rates_khr`]

# New structures
- [`PhysicalDeviceFragmentShadingRateKHR`]
- Extending [`GraphicsPipelineCreateInfo`]:  - [`PipelineFragmentShadingRateStateCreateInfoKHR`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceFragmentShadingRateFeaturesKHR`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceFragmentShadingRatePropertiesKHR`] 
- Extending [`SubpassDescription2`]:  - [`FragmentShadingRateAttachmentInfoKHR`]

# New enums
- [`FragmentShadingRateCombinerOpKHR`]

# New constants
- [`KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME`]
- [`KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION`]
- Extending [`AccessFlagBits`]:  - `VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR` 
- Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR` 
- Extending [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR` 
- Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR` 
- Extending [`ImageUsageFlagBits`]:  - `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR` 
- Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR`  - `VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR` 
If [`VK_KHR_format_feature_flags2`] is supported:
- Extending [`FormatFeatureFlagBits2`]:  - `VK_FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`

# Version history
- Revision 1, 2020-05-06 (Tobias Hector)  - Initial revision 
- Revision 2, 2021-09-30 (Jon Leech)  - Add interaction with `[`VK_KHR_format_feature_flags2`]` to `vk.xml`

# Other information
* 2021-09-30
*   - This extension requires [`SPV_KHR_fragment_shading_rate`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_fragment_shading_rate.html).  - This extension provides API support for [`GL_EXT_fragment_shading_rate`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_fragment_shading_rate.txt) 
*   - Tobias Hector, AMD  - Guennadi Riguer, AMD  - Matthaeus Chajdas, AMD  - Pat Brown, Nvidia  - Matthew Netsch, Qualcomm  - Slawomir Grajewski, Intel  - Jan-Harald Fredriksen, Arm  - Jeff Bolz, Nvidia  - Arseny Kapoulkine, Roblox  - Contributors to the VK_NV_shading_rate_image specification  - Contributors to the VK_EXT_fragment_density_map specification

# Related
- [`FragmentShadingRateAttachmentInfoKHR`]
- [`FragmentShadingRateCombinerOpKHR`]
- [`PhysicalDeviceFragmentShadingRateFeaturesKHR`]
- [`PhysicalDeviceFragmentShadingRateKHR`]
- [`PhysicalDeviceFragmentShadingRatePropertiesKHR`]
- [`PipelineFragmentShadingRateStateCreateInfoKHR`]
- [`cmd_set_fragment_shading_rate_khr`]
- [`get_physical_device_fragment_shading_rates_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        