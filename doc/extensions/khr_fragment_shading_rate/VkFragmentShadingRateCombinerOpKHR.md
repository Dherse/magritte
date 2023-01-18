[VkFragmentShadingRateCombinerOpKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateCombinerOpKHR.html) - Control how fragment shading rates are combined

# C Specifications
The equation used for each combiner operation is defined by
[`FragmentShadingRateCombinerOpKHR`]:
```c
// Provided by VK_KHR_fragment_shading_rate
typedef enum VkFragmentShadingRateCombinerOpKHR {
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR = 0,
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR = 1,
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR = 2,
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR = 3,
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR = 4,
} VkFragmentShadingRateCombinerOpKHR;
```

# Description
- [`KEEP`] specifies a combiner operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>.
- [`REPLACE`] specifies a combiner operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = B<sub>xy</sub>.
- [`MIN`] specifies a combiner operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = min(A<sub>xy</sub>,B<sub>xy</sub>).
- [`MAX`] specifies a combiner operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = max(A<sub>xy</sub>,B<sub>xy</sub>).
- [`MUL`] specifies a combiner operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>*B<sub>xy</sub>.
where combine(A<sub>xy</sub>,B<sub>xy</sub>) is the combine operation, and A<sub>xy</sub>
and B<sub>xy</sub> are the inputs to the operation.If [`fragmentShadingRateStrictMultiplyCombiner`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-fragmentShadingRateStrictMultiplyCombiner) is [`FALSE`], using
[`MUL`] with values of 1 for both
A and B in the same dimension results in the value 2 being produced for that
dimension.
See the definition of [`fragmentShadingRateStrictMultiplyCombiner`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-fragmentShadingRateStrictMultiplyCombiner) for more information.These operations are performed in a component-wise fashion.

# Related
- [`VK_KHR_fragment_shading_rate`]
- [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
- [`PipelineFragmentShadingRateStateCreateInfoKHR`]
- [`cmd_set_fragment_shading_rate_enum_nv`]
- [`cmd_set_fragment_shading_rate_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        