[vkCreateSamplerYcbcrConversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html) - Create a new Y′C<sub>B</sub>C<sub>R</sub> conversion

# C Specifications
To create a [`SamplerYcbcrConversion`], call:
```c
// Provided by VK_VERSION_1_1
VkResult vkCreateSamplerYcbcrConversion(
    VkDevice                                    device,
    const VkSamplerYcbcrConversionCreateInfo*   pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkSamplerYcbcrConversion*                   pYcbcrConversion);
```
or the equivalent command
```c
// Provided by VK_KHR_sampler_ycbcr_conversion
VkResult vkCreateSamplerYcbcrConversionKHR(
    VkDevice                                    device,
    const VkSamplerYcbcrConversionCreateInfo*   pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkSamplerYcbcrConversion*                   pYcbcrConversion);
```

# Parameters
- [`device`] is the logical device that creates the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion.
- [`p_create_info`] is a pointer to a [`SamplerYcbcrConversionCreateInfo`] structure specifying the requested sampler Y′C<sub>B</sub>C<sub>R</sub> conversion.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_ycbcr_conversion`] is a pointer to a [`SamplerYcbcrConversion`] handle in which the resulting sampler Y′C<sub>B</sub>C<sub>R</sub> conversion is returned.

# Description
The interpretation of the configured sampler Y′C<sub>B</sub>C<sub>R</sub> conversion is described
in more detail in [the description of
sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-sampler-YCbCr-conversion) in the [Image Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures) chapter.
## Valid Usage
-    The [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-samplerYcbcrConversion) **must**  be enabled

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`SamplerYcbcrConversionCreateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_ycbcr_conversion`] **must**  be a valid pointer to a [`SamplerYcbcrConversion`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_1`]
- [`AllocationCallbacks`]
- [`Device`]
- [`SamplerYcbcrConversion`]
- [`SamplerYcbcrConversionCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        