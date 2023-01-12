[vkCreateSampler](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html) - Create a new sampler object

# C Specifications
To create a sampler object, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkCreateSampler(
    VkDevice                                    device,
    const VkSamplerCreateInfo*                  pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkSampler*                                  pSampler);
```

# Parameters
- [`device`] is the logical device that creates the sampler.
- [`p_create_info`] is a pointer to a [`SamplerCreateInfo`] structure specifying the state of the sampler object.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_sampler`] is a pointer to a [`Sampler`] handle in which the resulting sampler object is returned.

# Description
## Valid Usage
-    There  **must**  be less than [`PhysicalDeviceLimits::max_sampler_allocation_count`][`Sampler`] objects currently created on the device

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`SamplerCreateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_sampler`] **must**  be a valid pointer to a [`Sampler`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Device`]
- [`Sampler`]
- [`SamplerCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        