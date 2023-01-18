[VkPipelineRepresentativeFragmentTestStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.html) - Structure specifying representative fragment test

# C Specifications
If the [`p_next`] chain of [`GraphicsPipelineCreateInfo`] includes a
[`PipelineRepresentativeFragmentTestStateCreateInfoNV`] structure, then
that structure includes parameters controlling the representative fragment
test.The [`PipelineRepresentativeFragmentTestStateCreateInfoNV`] structure is
defined as:
```c
// Provided by VK_NV_representative_fragment_test
typedef struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV {
    VkStructureType    sType;
    const void*        pNext;
    VkBool32           representativeFragmentTestEnable;
} VkPipelineRepresentativeFragmentTestStateCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`representative_fragment_test_enable`] controls whether the representative fragment test is enabled.

# Description
If this structure is not included in the [`p_next`] chain,
[`representative_fragment_test_enable`] is considered to be [`FALSE`],
and the representative fragment test is disabled.If the active fragment shader does not specify the `EarlyFragmentTests`
execution mode, the representative fragment shader test has no effect, even
if enabled.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV`

# Related
- [`VK_NV_representative_fragment_test`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        