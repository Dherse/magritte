[vkCmdPipelineBarrier2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2.html) - Insert a memory dependency

# C Specifications
To record a pipeline barrier, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdPipelineBarrier2(
    VkCommandBuffer                             commandBuffer,
    const VkDependencyInfo*                     pDependencyInfo);
```
or the equivalent command
```c
// Provided by VK_KHR_synchronization2
void vkCmdPipelineBarrier2KHR(
    VkCommandBuffer                             commandBuffer,
    const VkDependencyInfo*                     pDependencyInfo);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.
- [`p_dependency_info`] is a pointer to a [`DependencyInfo`] structure defining the scopes of this operation.

# Description
When [`cmd_pipeline_barrier2`] is submitted to a queue, it defines memory
dependencies between commands that were submitted before it, and those
submitted after it.The first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes) and
[access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) of each memory
dependency defined by [`p_dependency_info`] are applied to operations that
occurred earlier in [submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order).The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
and [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) of each
memory dependency defined by [`p_dependency_info`] are applied to operations
that occurred later in [submission
order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order).If [`cmd_pipeline_barrier2`] is recorded within a render pass instance,
the synchronization scopes are
[limited to
operations within the same subpass](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-barriers-subpass-self-dependencies).
## Valid Usage
-    If [`cmd_pipeline_barrier2`] is called within a render pass instance, the render pass  **must**  have been created with at least one [`SubpassDependency`] instance in [`RenderPassCreateInfo::dependencies`] that expresses a dependency from the current subpass to itself, with [synchronization scopes]() and [access scopes]() that are all supersets of the scopes defined in this command
-    If [`cmd_pipeline_barrier2`] is called within a render pass instance, it  **must**  not include any buffer memory barriers
-    If [`cmd_pipeline_barrier2`] is called within a render pass instance, the `image` member of any image memory barrier included in this command  **must**  be an attachment used in the current subpass both as an input attachment, and as either a color or depth/stencil attachment
-    If [`cmd_pipeline_barrier2`] is called within a render pass instance, the `oldLayout` and `newLayout` members of any image memory barrier included in this command  **must**  be equal
-    If [`cmd_pipeline_barrier2`] is called within a render pass instance, the `srcQueueFamilyIndex` and `dstQueueFamilyIndex` members of any image memory barrier included in this command  **must**  be equal
-    If [`cmd_pipeline_barrier2`] is called outside of a render pass instance, `VK_DEPENDENCY_VIEW_LOCAL_BIT` **must**  not be included in the dependency flags
-    If [`cmd_pipeline_barrier2`] is called within a render pass instance, the render pass  **must**  not have been started with [`cmd_begin_rendering`]
-    The [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-synchronization2) feature  **must**  be enabled
-    The `srcStageMask` member of any element of the `pMemoryBarriers`, `pBufferMemoryBarriers`, or `pImageMemoryBarriers` members of [`p_dependency_info`] **must**  only include pipeline stages valid for the queue family that was used to create the command pool that [`command_buffer`] was allocated from
-    The `dstStageMask` member of any element of the `pMemoryBarriers`, `pBufferMemoryBarriers`, or `pImageMemoryBarriers` members of [`p_dependency_info`] **must**  only include pipeline stages valid for the queue family that was used to create the command pool that [`command_buffer`] was allocated from

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_dependency_info`] **must**  be a valid pointer to a valid [`DependencyInfo`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support transfer, graphics, or compute operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`khr_synchronization2`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]
- [`DependencyInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        