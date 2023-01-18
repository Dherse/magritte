[vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html) - Query supported sample count combinations

# C Specifications
To query the set of mixed sample combinations of coverage reduction mode,
rasterization samples and color, depth, stencil attachment sample counts
that are supported by a physical device, call:
```c
// Provided by VK_NV_coverage_reduction_mode
VkResult vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(
    VkPhysicalDevice                            physicalDevice,
    uint32_t*                                   pCombinationCount,
    VkFramebufferMixedSamplesCombinationNV*     pCombinations);
```

# Parameters
- [`physical_device`] is the physical device from which to query the set of combinations.
- [`p_combination_count`] is a pointer to an integer related to the number of combinations available or queried, as described below.
- [`p_combinations`] is either `NULL` or a pointer to an array of [`FramebufferMixedSamplesCombinationNV`] values, indicating the supported combinations of coverage reduction mode, rasterization samples, and color, depth, stencil attachment sample counts.

# Description
If [`p_combinations`] is `NULL`, then the number of supported combinations
for the given [`physical_device`] is returned in [`p_combination_count`].
Otherwise, [`p_combination_count`] **must**  point to a variable set by the user
to the number of elements in the [`p_combinations`] array, and on return
the variable is overwritten with the number of values actually written to
[`p_combinations`].
If the value of [`p_combination_count`] is less than the number of
combinations supported for the given [`physical_device`], at most
[`p_combination_count`] values will be written to [`p_combinations`], and
`VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to
indicate that not all the supported values were returned.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_combination_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_combination_count`] is not `0`, and [`p_combinations`] is not `NULL`, [`p_combinations`] **must**  be a valid pointer to an array of [`p_combination_count`][`FramebufferMixedSamplesCombinationNV`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`VK_NV_coverage_reduction_mode`]
- [`FramebufferMixedSamplesCombinationNV`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        