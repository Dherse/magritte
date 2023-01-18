[VK_NV_fragment_shading_rate_enums](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_fragment_shading_rate_enums.html) - device extension

# Description
This extension builds on the fragment shading rate functionality provided by
the VK_KHR_fragment_shading_rate extension, adding support for
“supersample” fragment shading rates that trigger multiple fragment shader
invocations per pixel as well as a “no invocations” shading rate that
discards any portions of a primitive that would use that shading rate.

# Registered extension number
327

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_fragment_shading_rate`]`

# Contacts
- Pat Brown [nvpbrown](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_fragment_shading_rate_enums] @nvpbrown%0A<<Here describe the issue or question you have about the VK_NV_fragment_shading_rate_enums extension>>)

# New commands
- [`cmd_set_fragment_shading_rate_enum_nv`]

# New structures
- Extending [`GraphicsPipelineCreateInfo`]:  - [`PipelineFragmentShadingRateEnumStateCreateInfoNV`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceFragmentShadingRateEnumsFeaturesNV`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`]

# New enums
- [`FragmentShadingRateNV`]
- [`FragmentShadingRateTypeNV`]

# New constants
- [`NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME`]
- [`NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV`  - `VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV`

# Known issues & F.A.Q.
0. Why was this extension created?   How should it be named? **RESOLVED** : The primary goal of this extension was to expose support for supersample and “no invocations” shading rates, which are supported by the VK_NV_shading_rate_image extension but not by VK_KHR_fragment_shading_rate. Because VK_KHR_fragment_shading_rate specifies the primitive shading rate using a fragment size in pixels, it lacks a good way to specify supersample rates. To deal with this, we defined enums covering shading rates supported by the KHR extension as well as the new shading rates and added structures and APIs accepting shading rate enums instead of fragment sizes.Since this extension adds two different types of shading rates, both expressed using enums, we chose the extension name VK_NV_fragment_shading_rate_enums.
1. Is this a standalone extension? **RESOLVED** : No, this extension requires VK_KHR_fragment_shading_rate. In order to use the features of this extension, applications must enable the relevant features of KHR extension.
2. How are the shading rate enums used, and how were the enum values assigned? **RESOLVED** : The shading rates supported by the enums in this extension are accepted as pipeline, primitive, and attachment shading rates and behave identically. For the shading rates also supported by the KHR extension, the values assigned to the corresponding enums are identical to the values already used for the primitive and attachment shading rates in the KHR extension. For those enums, bits 0 and 1 specify the base two logarithm of the fragment height and bits 2 and 3 specify the base two logarithm of the fragment width. For the new shading rates added by this extension, we chose to use 11 through 14 (10 plus the base two logarithm of the invocation count) for the supersample rates and 15 for the “no invocations” rate. None of those values are supported as primitive or attachment shading rates by the KHR extension.
3. Between this extension, VK_KHR_fragment_shading_rate, and VK_NV_shading_rate_image, there are three different ways to specify shading rate state in a pipeline. How should we handle this? **RESOLVED** : We do not allow the concurrent use of VK_NV_shading_rate_image and VK_KHR_fragment_shading_rate; it is an error to enable shading rate features from both extensions. But we do allow applications to enable this extension together with VK_KHR_fragment_shading_rate together. While we expect that applications will never attach pipeline CreateInfo structures for both this extension and the KHR extension concurrently, Vulkan does not have any precedent forbidding such behavior and instead typically treats a pipeline created without an extension-specific CreateInfo structure as equivalent to one containing default values specified by the extension. Rather than adding such a rule considering the presence or absence of our new CreateInfo structure, we instead included a `shadingRateType` member to [`PipelineFragmentShadingRateEnumStateCreateInfoNV`] that selects between using state specified by that structure and state specified by [`PipelineFragmentShadingRateStateCreateInfoKHR`].

# Version history
- Revision 1, 2020-09-02 (pbrown)  - Internal revisions

# Other information
* 2020-09-02
*   - Pat Brown, NVIDIA  - Jeff Bolz, NVIDIA

# Related
- [`FragmentShadingRateNV`]
- [`FragmentShadingRateTypeNV`]
- [`PhysicalDeviceFragmentShadingRateEnumsFeaturesNV`]
- [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`]
- [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
- [`cmd_set_fragment_shading_rate_enum_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        