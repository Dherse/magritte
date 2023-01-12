[vkGetPhysicalDeviceCalibrateableTimeDomainsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html) - Query calibrateable time domains

# C Specifications
To query the set of time domains for which a physical device supports
timestamp calibration, call:
```c
// Provided by VK_EXT_calibrated_timestamps
VkResult vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(
    VkPhysicalDevice                            physicalDevice,
    uint32_t*                                   pTimeDomainCount,
    VkTimeDomainEXT*                            pTimeDomains);
```

# Parameters
- [`physical_device`] is the physical device from which to query the set of calibrateable time domains.
- [`p_time_domain_count`] is a pointer to an integer related to the number of calibrateable time domains available or queried, as described below.
- [`p_time_domains`] is either `NULL` or a pointer to an array of [`TimeDomainEXT`] values, indicating the supported calibrateable time domains.

# Description
If [`p_time_domains`] is `NULL`, then the number of calibrateable time
domains supported for the given [`physical_device`] is returned in
[`p_time_domain_count`].
Otherwise, [`p_time_domain_count`] **must**  point to a variable set by the user
to the number of elements in the [`p_time_domains`] array, and on return the
variable is overwritten with the number of values actually written to
[`p_time_domains`].
If the value of [`p_time_domain_count`] is less than the number of
calibrateable time domains supported, at most [`p_time_domain_count`] values
will be written to [`p_time_domains`], and `VK_INCOMPLETE` will be
returned instead of `VK_SUCCESS`, to indicate that not all the available
time domains were returned.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_time_domain_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_time_domain_count`] is not `0`, and [`p_time_domains`] is not `NULL`, [`p_time_domains`] **must**  be a valid pointer to an array of [`p_time_domain_count`][`TimeDomainEXT`] values

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`ext_calibrated_timestamps`]
- [`PhysicalDevice`]
- [`TimeDomainEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        