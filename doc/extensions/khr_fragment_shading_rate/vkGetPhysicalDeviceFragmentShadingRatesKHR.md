[vkGetPhysicalDeviceFragmentShadingRatesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html) - Get available shading rates for a physical device

# C Specifications
To query available shading rates, call:
```c
// Provided by VK_KHR_fragment_shading_rate
VkResult vkGetPhysicalDeviceFragmentShadingRatesKHR(
    VkPhysicalDevice                            physicalDevice,
    uint32_t*                                   pFragmentShadingRateCount,
    VkPhysicalDeviceFragmentShadingRateKHR*     pFragmentShadingRates);
```

# Parameters
- [`physical_device`] is the handle to the physical device whose properties will be queried.
- [`p_fragment_shading_rate_count`] is a pointer to an integer related to the number of fragment shading rates available or queried, as described below.
- [`p_fragment_shading_rates`] is either `NULL` or a pointer to an array of [`PhysicalDeviceFragmentShadingRateKHR`] structures.

# Description
If [`p_fragment_shading_rates`] is `NULL`, then the number of fragment
shading rates available is returned in [`p_fragment_shading_rate_count`].
Otherwise, [`p_fragment_shading_rate_count`] **must**  point to a variable set by
the user to the number of elements in the [`p_fragment_shading_rates`] array,
and on return the variable is overwritten with the number of structures
actually written to [`p_fragment_shading_rates`].
If [`p_fragment_shading_rate_count`] is less than the number of fragment
shading rates available, at most [`p_fragment_shading_rate_count`] structures
will be written, and `VK_INCOMPLETE` will be returned instead of
`VK_SUCCESS`, to indicate that not all the available fragment shading
rates were returned.The returned array of fragment shading rates  **must**  be ordered from largest
`fragmentSize.width` value to smallest, and each set of fragment shading
rates with the same `fragmentSize.width` value  **must**  be ordered from
largest `fragmentSize.height` to smallest.
Any two entries in the array  **must**  not have the same `fragmentSize`
values.For any entry in the array, the following rules also apply:
- The value of `fragmentSize.width` **must**  be less than or equal to [`maxFragmentSize.width`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentSize).
- The value of `fragmentSize.width` **must**  be greater than or equal to `1`.
- The value of `fragmentSize.width` **must**  be a power-of-two.
- The value of `fragmentSize.height` **must**  be less than or equal to [`maxFragmentSize.height`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentSize).
- The value of `fragmentSize.height` **must**  be greater than or equal to `1`.
- The value of `fragmentSize.height` **must**  be a power-of-two.
- The highest sample count in `sampleCounts` **must**  be less than or equal to [`maxFragmentShadingRateRasterizationSamples`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateRasterizationSamples).
- The product of `fragmentSize.width`, `fragmentSize.height`, and the highest sample count in `sampleCounts` **must**  be less than or equal to [`maxFragmentShadingRateCoverageSamples`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateCoverageSamples).
Implementations  **must**  support at least the following shading rates:If [`framebufferColorSampleCounts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-framebufferColorSampleCounts), includes `VK_SAMPLE_COUNT_2_BIT`,
the required rates  **must**  also include `VK_SAMPLE_COUNT_2_BIT`.The returned set of fragment shading rates  **must**  be returned in the native
(rotated) coordinate system.
For rasterization using render pass `transform` not equal to
`VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR`, the application  **must**  transform
the returned fragment shading rates into the current (unrotated) coordinate
system to get the supported rates for that transform.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_fragment_shading_rate_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_fragment_shading_rate_count`] is not `0`, and [`p_fragment_shading_rates`] is not `NULL`, [`p_fragment_shading_rates`] **must**  be a valid pointer to an array of [`p_fragment_shading_rate_count`][`PhysicalDeviceFragmentShadingRateKHR`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`khr_fragment_shading_rate`]
- [`PhysicalDevice`]
- [`PhysicalDeviceFragmentShadingRateKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        