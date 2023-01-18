[VkTimeDomainEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTimeDomainEXT.html) - Supported time domains

# C Specifications
The set of supported time domains consists of:
```c
// Provided by VK_EXT_calibrated_timestamps
typedef enum VkTimeDomainEXT {
    VK_TIME_DOMAIN_DEVICE_EXT = 0,
    VK_TIME_DOMAIN_CLOCK_MONOTONIC_EXT = 1,
    VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT = 2,
    VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT = 3,
} VkTimeDomainEXT;
```

# Description
- [`DEVICE`] specifies the device time domain. Timestamp values in this time domain use the same units and are comparable with device timestamp values captured using [`cmd_write_timestamp`] or [`cmd_write_timestamp2`] and are defined to be incrementing according to the [timestampPeriod](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-timestampPeriod) of the device.
- [`CLOCK_MONOTONIC`] specifies the CLOCK_MONOTONIC time domain available on POSIX platforms. Timestamp values in this time domain are in units of nanoseconds and are comparable with platform timestamp values captured using the POSIX clock_gettime API as computed by this example:

```c
struct timespec tv;
clock_gettime(CLOCK_MONOTONIC, &tv);
return tv.tv_nsec + tv.tv_sec*1000000000ull;
```

- [`CLOCK_MONOTONIC_RAW`] specifies the CLOCK_MONOTONIC_RAW time domain available on POSIX platforms. Timestamp values in this time domain are in units of nanoseconds and are comparable with platform timestamp values captured using the POSIX clock_gettime API as computed by this example:

```c
struct timespec tv;
clock_gettime(CLOCK_MONOTONIC_RAW, &tv);
return tv.tv_nsec + tv.tv_sec*1000000000ull;
```

- [`QUERY_PERFORMANCE_COUNTER`] specifies the performance counter (QPC) time domain available on Windows. Timestamp values in this time domain are in the same units as those provided by the Windows QueryPerformanceCounter API and are comparable with platform timestamp values captured using that API as computed by this example:

```c
LARGE_INTEGER counter;
QueryPerformanceCounter(&counter);
return counter.QuadPart;
```

# Related
- [`VK_EXT_calibrated_timestamps`]
- [`CalibratedTimestampInfoEXT`]
- [`get_physical_device_calibrateable_time_domains_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        