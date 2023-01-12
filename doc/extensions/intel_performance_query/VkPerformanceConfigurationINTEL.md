[VkPerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationINTEL.html) - Device configuration for performance queries

# C Specifications
Before submitting command buffers containing performance queries commands to
a device queue, the application must acquire and set a performance query
configuration.
The configuration can be released once all command buffers containing
performance query commands are not in a pending state.
```c
// Provided by VK_INTEL_performance_query
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPerformanceConfigurationINTEL)
```

# Related
- [`intel_performance_query`]
- [`acquire_performance_configuration_intel`]
- [`queue_set_performance_configuration_intel`]
- [`release_performance_configuration_intel`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        