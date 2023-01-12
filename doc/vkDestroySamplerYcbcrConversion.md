[vkDestroySamplerYcbcrConversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html) - Destroy a created Y′C<sub>B</sub>C<sub>R</sub> conversion

# C Specifications
To destroy a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion, call:
```c
// Provided by VK_VERSION_1_1
void vkDestroySamplerYcbcrConversion(
    VkDevice                                    device,
    VkSamplerYcbcrConversion                    ycbcrConversion,
    const VkAllocationCallbacks*                pAllocator);
```
or the equivalent command
```c
// Provided by VK_KHR_sampler_ycbcr_conversion
void vkDestroySamplerYcbcrConversionKHR(
    VkDevice                                    device,
    VkSamplerYcbcrConversion                    ycbcrConversion,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the logical device that destroys the Y′C<sub>B</sub>C<sub>R</sub> conversion.
- [`ycbcr_conversion`] is the conversion to destroy.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`ycbcr_conversion`] is not [`crate::Handle::null`], [`ycbcr_conversion`] **must**  be a valid [`SamplerYcbcrConversion`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`ycbcr_conversion`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`ycbcr_conversion`] **must**  be externally synchronized

# Related
- [`crate::vulkan1_1`]
- [`AllocationCallbacks`]
- [`Device`]
- [`SamplerYcbcrConversion`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        