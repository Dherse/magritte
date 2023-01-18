[VK_MAX_GLOBAL_PRIORITY_SIZE_KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_GLOBAL_PRIORITY_SIZE_KHR.html) - Length of an array of global queue priorities

# C Specifications
[`MAX_GLOBAL_PRIORITY_SIZE_KHR`] is the length of an array of
[`QueueGlobalPriorityKHR`] enumerants representing supported queue
priorities, as returned in
[`QueueFamilyGlobalPriorityPropertiesKHR::priorities`].
```c
#define VK_MAX_GLOBAL_PRIORITY_SIZE_KHR   16U
```
or the equivalent
```c
#define VK_MAX_GLOBAL_PRIORITY_SIZE_EXT   VK_MAX_GLOBAL_PRIORITY_SIZE_KHR
```

# Related
- [`VK_KHR_global_priority`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        