[VkPipelineStageFlags2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlags2.html) - 64-bit mask of pipeline stage flags

# C Specifications
[`PipelineStageFlags2`] is a bitmask type for setting a mask of zero or
more [`PipelineStageFlagBits2`] flags:
```c
// Provided by VK_VERSION_1_3
typedef VkFlags64 VkPipelineStageFlags2;
```
or the equivalent
```c
// Provided by VK_KHR_synchronization2
typedef VkPipelineStageFlags2 VkPipelineStageFlags2KHR;
```

# Related
- [`VK_KHR_synchronization2`]
- [`crate::vulkan1_3`]
- [`BufferMemoryBarrier2`]
- [`CheckpointData2NV`]
- [`ImageMemoryBarrier2`]
- [`MemoryBarrier2`]
- [`QueueFamilyCheckpointProperties2NV`]
- [`SemaphoreSubmitInfo`]
- [`cmd_reset_event2`]
- [`cmd_reset_event2_khr`]
- [`cmd_write_buffer_marker2_amd`]
- [`cmd_write_timestamp2`]
- [`cmd_write_timestamp2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        