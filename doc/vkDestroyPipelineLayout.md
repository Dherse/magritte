[vkDestroyPipelineLayout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html) - Destroy a pipeline layout object

# C Specifications
To destroy a pipeline layout, call:
```c
// Provided by VK_VERSION_1_0
void vkDestroyPipelineLayout(
    VkDevice                                    device,
    VkPipelineLayout                            pipelineLayout,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the logical device that destroys the pipeline layout.
- [`pipeline_layout`] is the pipeline layout to destroy.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
## Valid Usage
-    If [`AllocationCallbacks`] were provided when [`pipeline_layout`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`pipeline_layout`] was created, [`p_allocator`] **must**  be `NULL`
-  [`pipeline_layout`] **must**  not have been passed to any `vkCmd*` command for any command buffers that are still in the [recording state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle) when [`destroy_pipeline_layout`] is called

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`pipeline_layout`] is not [`crate::Handle::null`], [`pipeline_layout`] **must**  be a valid [`PipelineLayout`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`pipeline_layout`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`pipeline_layout`] **must**  be externally synchronized

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Device`]
- [`PipelineLayout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        