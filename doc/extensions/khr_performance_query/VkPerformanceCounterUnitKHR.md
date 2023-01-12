[VkPerformanceCounterUnitKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterUnitKHR.html) - Supported counter unit types

# C Specifications
Performance counters have an associated unit.
This unit describes how to interpret the performance counter result.The performance counter unit types which  **may**  be returned in
[`PerformanceCounterKHR::unit`] are:
```c
// Provided by VK_KHR_performance_query
typedef enum VkPerformanceCounterUnitKHR {
    VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR = 0,
    VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR = 1,
    VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR = 2,
    VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR = 3,
    VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR = 4,
    VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR = 5,
    VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR = 6,
    VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR = 7,
    VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR = 8,
    VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR = 9,
    VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR = 10,
} VkPerformanceCounterUnitKHR;
```

# Description
- [`VK_PERFORMANCE_COUNTER_UNIT_KHR`] - the performance counter unit is a generic data point.
- [`VK_PERFORMANCE_COUNTER_UNIT_KHR`] - the performance counter unit is a percentage (%).
- [`VK_PERFORMANCE_COUNTER_UNIT_KHR`] - the performance counter unit is a value of nanoseconds (ns).
- [`VK_PERFORMANCE_COUNTER_UNIT_KHR`] - the performance counter unit is a value of bytes.
- [`VK_PERFORMANCE_COUNTER_UNIT_KHR`] - the performance counter unit is a value of bytes/s.
- [`VK_PERFORMANCE_COUNTER_UNIT_KHR`] - the performance counter unit is a temperature reported in Kelvin.
- [`VK_PERFORMANCE_COUNTER_UNIT_KHR`] - the performance counter unit is a value of watts (W).
- [`VK_PERFORMANCE_COUNTER_UNIT_KHR`] - the performance counter unit is a value of volts (V).
- [`VK_PERFORMANCE_COUNTER_UNIT_KHR`] - the performance counter unit is a value of amps (A).
- [`VK_PERFORMANCE_COUNTER_UNIT_KHR`] - the performance counter unit is a value of hertz (Hz).
- [`VK_PERFORMANCE_COUNTER_UNIT_KHR`] - the performance counter unit is a value of cycles.

# Related
- [`khr_performance_query`]
- [`PerformanceCounterKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        