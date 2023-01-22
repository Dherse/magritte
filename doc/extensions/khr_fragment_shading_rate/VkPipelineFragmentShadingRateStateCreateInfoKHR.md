[VkPipelineFragmentShadingRateStateCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineFragmentShadingRateStateCreateInfoKHR.html) - Structure specifying parameters controlling the fragment shading rate

# C Specifications
The [`PipelineFragmentShadingRateStateCreateInfoKHR`] structure is
defined as:
```c
// Provided by VK_KHR_fragment_shading_rate
typedef struct VkPipelineFragmentShadingRateStateCreateInfoKHR {
    VkStructureType                       sType;
    const void*                           pNext;
    VkExtent2D                            fragmentSize;
    VkFragmentShadingRateCombinerOpKHR    combinerOps[2];
} VkPipelineFragmentShadingRateStateCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`fragment_size`] specifies a [`Extent2D`] structure containing the fragment size used to define the pipeline fragment shading rate for drawing commands using this pipeline.
- [`combiner_ops`] specifies a [`FragmentShadingRateCombinerOpKHR`] value determining how the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline), [primitive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive), and [attachment shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment) are [combined](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-combining) for fragments generated by drawing commands using the created pipeline.

# Description
If the [`p_next`] chain of [`GraphicsPipelineCreateInfo`] includes a
[`PipelineFragmentShadingRateStateCreateInfoKHR`] structure, then that
structure includes parameters controlling the pipeline fragment shading
rate.If this structure is not present, [`fragment_size`] is considered to be
equal to (1,1), and both elements of [`combiner_ops`] are considered
to be equal to `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR`.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR`
-    Any given element of [`combiner_ops`] **must**  be a valid [`FragmentShadingRateCombinerOpKHR`] value

# Related
- [`VK_KHR_fragment_shading_rate`]
- [`Extent2D`]
- [`FragmentShadingRateCombinerOpKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        