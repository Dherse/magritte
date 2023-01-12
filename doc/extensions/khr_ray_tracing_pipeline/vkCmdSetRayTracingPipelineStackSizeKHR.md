[vkCmdSetRayTracingPipelineStackSizeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html) - Set the stack size dynamically for a ray tracing pipeline

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the stack size for a ray
tracing pipeline, call:
```c
// Provided by VK_KHR_ray_tracing_pipeline
void vkCmdSetRayTracingPipelineStackSizeKHR(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    pipelineStackSize);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`pipeline_stack_size`] is the stack size to use for subsequent ray tracing trace commands.

# Description
This command sets the stack size for subsequent ray tracing commands when
the ray tracing pipeline is created with
`VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, the stack size is computed as described in
[Ray Tracing Pipeline Stack](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-pipeline-stack).
## Valid Usage
-  [`pipeline_stack_size`] **must**  be large enough for any dynamic execution through the shaders in the ray tracing pipeline used by a subsequent trace call

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-    This command  **must**  only be called outside of a render pass instance

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`khr_ray_tracing_pipeline`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        