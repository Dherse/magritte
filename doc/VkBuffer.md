[VkBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuffer.html) - Opaque handle to a buffer object

# C Specifications
Buffers represent linear arrays of data which are used for various purposes
by binding them to a graphics or compute pipeline via descriptor sets or via
certain commands, or by directly specifying them as parameters to certain
commands.Buffers are represented by [`Buffer`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkBuffer)
```

# Related
- [`crate::vulkan1_0`]
- [`AccelerationStructureCreateInfoKHR`]
- [`BindBufferMemoryInfo`]
- [`BufferDeviceAddressInfo`]
- [`BufferMemoryBarrier`]
- [`BufferMemoryBarrier2`]
- [`BufferMemoryRequirementsInfo2`]
- [`BufferViewCreateInfo`]
- [`ConditionalRenderingBeginInfoEXT`]
- [`CopyBufferInfo2`]
- [`CopyBufferToImageInfo2`]
- [`CopyImageToBufferInfo2`]
- [`DedicatedAllocationMemoryAllocateInfoNV`]
- [`DescriptorBufferInfo`]
- [`GeneratedCommandsInfoNV`]
- [`GeometryAabbNV`]
- [`GeometryTrianglesNV`]
- [`IndirectCommandsStreamNV`]
- [`MemoryDedicatedAllocateInfo`]
- [`SparseBufferMemoryBindInfo`]
- [`VideoDecodeInfoKHR`]
- [`VideoEncodeInfoKHR`]
- [`bind_buffer_memory`]
- [`cmd_begin_transform_feedback_ext`]
- [`cmd_bind_index_buffer`]
- [`cmd_bind_transform_feedback_buffers_ext`]
- [`cmd_bind_vertex_buffers`]
- [`cmd_bind_vertex_buffers2`]
- [`cmd_bind_vertex_buffers2_ext`]
- [`cmd_build_acceleration_structure_nv`]
- [`cmd_copy_buffer`]
- [`cmd_copy_buffer_to_image`]
- [`cmd_copy_image_to_buffer`]
- [`cmd_copy_query_pool_results`]
- [`cmd_dispatch_indirect`]
- [`cmd_draw_indexed_indirect`]
- [`cmd_draw_indexed_indirect_count`]
- [`cmd_draw_indexed_indirect_count_amd`]
- [`cmd_draw_indexed_indirect_count_khr`]
- [`cmd_draw_indirect`]
- [`cmd_draw_indirect_byte_count_ext`]
- [`cmd_draw_indirect_count`]
- [`cmd_draw_indirect_count_amd`]
- [`cmd_draw_indirect_count_khr`]
- [`cmd_draw_mesh_tasks_indirect_count_nv`]
- [`cmd_draw_mesh_tasks_indirect_nv`]
- [`cmd_end_transform_feedback_ext`]
- [`cmd_fill_buffer`]
- [`cmd_trace_rays_nv`]
- [`cmd_update_buffer`]
- [`cmd_write_buffer_marker2_amd`]
- [`cmd_write_buffer_marker_amd`]
- [`create_buffer`]
- [`destroy_buffer`]
- [`get_buffer_memory_requirements`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        