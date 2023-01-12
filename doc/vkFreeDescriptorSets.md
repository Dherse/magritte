[vkFreeDescriptorSets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html) - Free one or more descriptor sets

# C Specifications
To free allocated descriptor sets, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkFreeDescriptorSets(
    VkDevice                                    device,
    VkDescriptorPool                            descriptorPool,
    uint32_t                                    descriptorSetCount,
    const VkDescriptorSet*                      pDescriptorSets);
```

# Parameters
- [`device`] is the logical device that owns the descriptor pool.
- [`descriptor_pool`] is the descriptor pool from which the descriptor sets were allocated.
- [`descriptor_set_count`] is the number of elements in the [`p_descriptor_sets`] array.
- [`p_descriptor_sets`] is a pointer to an array of handles to [`DescriptorSet`] objects.

# Description
After calling [`free_descriptor_sets`], all descriptor sets in
[`p_descriptor_sets`] are invalid.
## Valid Usage
-    All submitted commands that refer to any element of [`p_descriptor_sets`] **must**  have completed execution
-  [`p_descriptor_sets`] **must**  be a valid pointer to an array of [`descriptor_set_count`][`DescriptorSet`] handles, each element of which  **must**  either be a valid handle or [`crate::Handle::null`]
-  [`descriptor_pool`] **must**  have been created with the `VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT` flag

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`descriptor_pool`] **must**  be a valid [`DescriptorPool`] handle
-  [`descriptor_set_count`] **must**  be greater than `0`
-  [`descriptor_pool`] **must**  have been created, allocated, or retrieved from [`device`]
-    Each element of [`p_descriptor_sets`] that is a valid handle  **must**  have been created, allocated, or retrieved from [`descriptor_pool`]

## Host Synchronization
- Host access to [`descriptor_pool`] **must**  be externally synchronized
- Host access to each member of [`p_descriptor_sets`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS`

# Related
- [`crate::vulkan1_0`]
- [`DescriptorPool`]
- [`DescriptorSet`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        