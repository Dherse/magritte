[vkGetCalibratedTimestampsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html) - Query calibrated timestamps

# C Specifications
In order to be able to correlate the time a particular operation took place
at on timelines of different time domains (e.g. a device operation vs a host
operation), Vulkan allows querying calibrated timestamps from multiple time
domains.To query calibrated timestamps from a set of time domains, call:
```c
// Provided by VK_EXT_calibrated_timestamps
VkResult vkGetCalibratedTimestampsEXT(
    VkDevice                                    device,
    uint32_t                                    timestampCount,
    const VkCalibratedTimestampInfoEXT*         pTimestampInfos,
    uint64_t*                                   pTimestamps,
    uint64_t*                                   pMaxDeviation);
```

# Parameters
- [`device`] is the logical device used to perform the query.
- [`timestamp_count`] is the number of timestamps to query.
- [`p_timestamp_infos`] is a pointer to an array of [`timestamp_count`][`CalibratedTimestampInfoEXT`] structures, describing the time domains the calibrated timestamps should be captured from.
- [`p_timestamps`] is a pointer to an array of [`timestamp_count`] 64-bit unsigned integer values in which the requested calibrated timestamp values are returned.
- [`p_max_deviation`] is a pointer to a 64-bit unsigned integer value in which the strictly positive maximum deviation, in nanoseconds, of the calibrated timestamp values is returned.

# Description
Calibrated timestamp values  **can**  be extrapolated to estimate future
coinciding timestamp values, however, depending on the nature of the time
domains and other properties of the platform extrapolating values over a
sufficiently long period of time  **may**  no longer be accurate enough to fit
any particular purpose, so applications are expected to re-calibrate the
timestamps on a regular basis.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_timestamp_infos`] **must**  be a valid pointer to an array of [`timestamp_count`] valid [`CalibratedTimestampInfoEXT`] structures
-  [`p_timestamps`] **must**  be a valid pointer to an array of [`timestamp_count`]`uint64_t` values
-  [`p_max_deviation`] **must**  be a valid pointer to a `uint64_t` value
-  [`timestamp_count`] **must**  be greater than `0`

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`VK_EXT_calibrated_timestamps`]
- [`CalibratedTimestampInfoEXT`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        