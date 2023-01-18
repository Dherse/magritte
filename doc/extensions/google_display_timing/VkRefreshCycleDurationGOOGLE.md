[VkRefreshCycleDurationGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRefreshCycleDurationGOOGLE.html) - Structure containing the RC duration of a display

# C Specifications
The [`RefreshCycleDurationGOOGLE`] structure is defined as:
```c
// Provided by VK_GOOGLE_display_timing
typedef struct VkRefreshCycleDurationGOOGLE {
    uint64_t    refreshDuration;
} VkRefreshCycleDurationGOOGLE;
```

# Members
- [`refresh_duration`] is the number of nanoseconds from the start of one refresh cycle to the next.

# Related
- [`VK_GOOGLE_display_timing`]
- [`get_refresh_cycle_duration_google`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        