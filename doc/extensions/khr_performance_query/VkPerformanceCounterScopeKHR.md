[VkPerformanceCounterScopeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterScopeKHR.html) - Supported counter scope types

# C Specifications
Performance counters have an associated scope.
This scope describes the granularity of a performance counter.The performance counter scope types which  **may**  be returned in
[`PerformanceCounterKHR::scope`] are:
```c
// Provided by VK_KHR_performance_query
typedef enum VkPerformanceCounterScopeKHR {
    VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR = 0,
    VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR = 1,
    VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR = 2,
    VK_QUERY_SCOPE_COMMAND_BUFFER_KHR = VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR,
    VK_QUERY_SCOPE_RENDER_PASS_KHR = VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR,
    VK_QUERY_SCOPE_COMMAND_KHR = VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR,
} VkPerformanceCounterScopeKHR;
```

# Description
- [`COMMAND_BUFFER`] - the performance counter scope is a single complete command buffer.
- [`RENDER_PASS`] - the performance counter scope is zero or more complete render passes. The performance query containing the performance counter  **must**  begin and end outside a render pass instance.
- [`COMMAND`] - the performance counter scope is zero or more commands.

# Related
- [`VK_KHR_performance_query`]
- [`PerformanceCounterKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        