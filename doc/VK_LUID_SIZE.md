[VK_LUID_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_LUID_SIZE.html) - Length of a locally unique device identifier

# C Specifications
[`LUID_SIZE`] is the length in `uint8_t` values of an array
containing a locally unique device identifier, as returned in
[`PhysicalDeviceIdProperties`]::deviceLUID.
```c
#define VK_LUID_SIZE                      8U
```
or the equivalent
```c
#define VK_LUID_SIZE_KHR                  VK_LUID_SIZE
```

# Related
- [`VK_KHR_external_fence_capabilities`]
- [`VK_KHR_external_memory_capabilities`]
- [`VK_KHR_external_semaphore_capabilities`]
- [`crate::vulkan1_1`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        