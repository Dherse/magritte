[vkResetDescriptorPool](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html) - Resets a descriptor pool object

# C Specifications
To return all descriptor sets allocated from a given pool to the pool,
rather than freeing individual descriptor sets, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkResetDescriptorPool(
    VkDevice                                    device,
    VkDescriptorPool                            descriptorPool,
    VkDescriptorPoolResetFlags                  flags);
```

# Parameters
- [`device`] is the logical device that owns the descriptor pool.
- [`descriptor_pool`] is the descriptor pool to be reset.
- [`flags`] is reserved for future use.

# Description
Resetting a descriptor pool recycles all of the resources from all of the
descriptor sets allocated from the descriptor pool back to the descriptor
pool, and the descriptor sets are implicitly freed.
## Valid Usage
-    All uses of [`descriptor_pool`] (via any allocated descriptor sets)  **must**  have completed execution

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`descriptor_pool`] **must**  be a valid [`DescriptorPool`] handle
-  [`flags`] **must**  be `0`
-  [`descriptor_pool`] **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`descriptor_pool`] **must**  be externally synchronized
- Host access to any [`DescriptorSet`] objects allocated from [`descriptor_pool`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS`

# Related
- [`crate::vulkan1_0`]
- [`DescriptorPool`]
- [`DescriptorPoolResetFlags`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        