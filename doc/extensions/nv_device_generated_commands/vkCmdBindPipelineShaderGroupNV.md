[vkCmdBindPipelineShaderGroupNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html) - Bind a pipeline object

# C Specifications
For pipelines that were created with the support of multiple shader groups
(see [Graphics Pipeline Shader Groups](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#graphics-shadergroups)), the regular
[`cmd_bind_pipeline`] command will bind Shader Group `0`.
To explicitly bind a shader group use:
```c
// Provided by VK_NV_device_generated_commands
void vkCmdBindPipelineShaderGroupNV(
    VkCommandBuffer                             commandBuffer,
    VkPipelineBindPoint                         pipelineBindPoint,
    VkPipeline                                  pipeline,
    uint32_t                                    groupIndex);
```

# Parameters
- [`command_buffer`] is the command buffer that the pipeline will be bound to.
- [`pipeline_bind_point`] is a [`PipelineBindPoint`] value specifying the bind point to which the pipeline will be bound.
- [`pipeline`] is the pipeline to be bound.
- [`group_index`] is the shader group to be bound.

# Description
## Valid Usage
-  [`group_index`] **must**  be `0` or less than the effective [`GraphicsPipelineShaderGroupsCreateInfoNV::group_count`] including the referenced pipelines
-    The [`pipeline_bind_point`] **must**  be `VK_PIPELINE_BIND_POINT_GRAPHICS`
-    The same restrictions as [`cmd_bind_pipeline`] apply as if the bound pipeline was created only with the Shader Group from the [`group_index`] information
-    The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
-  [`pipeline`] **must**  be a valid [`Pipeline`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-    Both of [`command_buffer`], and [`pipeline`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_NV_device_generated_commands`]
- [`CommandBuffer`]
- [`Pipeline`]
- [`PipelineBindPoint`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        