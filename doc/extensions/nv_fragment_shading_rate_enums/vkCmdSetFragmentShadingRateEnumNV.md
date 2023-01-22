[vkCmdSetFragmentShadingRateEnumNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html) - Set pipeline fragment shading rate dynamically for a command buffer using enums

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the pipeline fragment
shading rate and combiner operation, call:
```c
// Provided by VK_NV_fragment_shading_rate_enums
void vkCmdSetFragmentShadingRateEnumNV(
    VkCommandBuffer                             commandBuffer,
    VkFragmentShadingRateNV                     shadingRate,
    const VkFragmentShadingRateCombinerOpKHR    combinerOps[2]);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`shading_rate`] specifies a [`FragmentShadingRateNV`] enum indicating the pipeline fragment shading rate for subsequent drawing commands.
- [`combiner_ops`] specifies a [`FragmentShadingRateCombinerOpKHR`] determining how the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline), [primitive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive), and [attachment shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment) are [combined](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-combining) for fragments generated by subsequent drawing commands.

# Description
This command sets the pipeline fragment shading rate and combiner operation
for subsequent drawing commands when the graphics pipeline is created with
`VK_DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineFragmentShadingRateEnumStateCreateInfoNV`] values used to
create the currently active pipeline.
## Valid Usage
-    If [`pipelineFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineFragmentShadingRate) is not enabled, [`shading_rate`] **must**  be `VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV`
-    If [`supersampleFragmentShadingRates`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-supersampleFragmentShadingRates) is not enabled, [`shading_rate`] **must**  not be `VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV`, `VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV`, `VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV`, or `VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV`
-    If [`noInvocationFragmentShadingRates`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-noInvocationFragmentShadingRates) is not enabled, [`shading_rate`] **must**  not be `VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV`
-  [`fragmentShadingRateEnums`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-fragmentShadingRateEnums) **must**  be enabled
-    One of [`pipelineFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineFragmentShadingRate), [`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate), or [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) **must**  be enabled
-    If the [`primitiveFragmentShadingRate` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) is not enabled, [`combiner_ops`][0]  **must**  be `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR`
-    If the [`attachmentFragmentShadingRate` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) is not enabled, [`combiner_ops`][1]  **must**  be `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR`
-    If the [`fragmentSizeNonTrivialCombinerOps`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-fragmentShadingRateNonTrivialCombinerOps) limit is not supported, elements of [`combiner_ops`] **must**  be either `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR` or `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR`

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`shading_rate`] **must**  be a valid [`FragmentShadingRateNV`] value
-    Any given element of [`combiner_ops`] **must**  be a valid [`FragmentShadingRateCombinerOpKHR`] value
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_NV_fragment_shading_rate_enums`]
- [`CommandBuffer`]
- [`FragmentShadingRateCombinerOpKHR`]
- [`FragmentShadingRateNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        