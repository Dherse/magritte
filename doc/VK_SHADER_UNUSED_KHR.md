[VK_SHADER_UNUSED_KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_SHADER_UNUSED_KHR.html) - Sentinel for an unused shader index

# C Specifications
[`SHADER_UNUSED_KHR`] is a special shader index used to indicate that a
ray generation, miss, or callable shader member is not used.
```c
#define VK_SHADER_UNUSED_KHR              (~0U)
```
or the equivalent
```c
#define VK_SHADER_UNUSED_NV               VK_SHADER_UNUSED_KHR
```

# Related
- [`VK_KHR_ray_tracing_pipeline`]
- [`VK_NV_ray_tracing`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        