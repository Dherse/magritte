[VkQueryPool](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPool.html) - Opaque handle to a query pool object

# C Specifications
Queries are managed using *query pool* objects.
Each query pool is a collection of a specific number of queries of a
particular type.Query pools are represented by [`QueryPool`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkQueryPool)
```

# Related
- [`crate::vulkan1_0`]
- [`cmd_begin_query`]
- [`cmd_begin_query_indexed_ext`]
- [`cmd_copy_query_pool_results`]
- [`cmd_end_query`]
- [`cmd_end_query_indexed_ext`]
- [`cmd_reset_query_pool`]
- [`cmd_write_acceleration_structures_properties_khr`]
- [`cmd_write_acceleration_structures_properties_nv`]
- [`cmd_write_timestamp`]
- [`cmd_write_timestamp2`]
- [`cmd_write_timestamp2_khr`]
- [`create_query_pool`]
- [`destroy_query_pool`]
- [`get_query_pool_results`]
- [`reset_query_pool`]
- [`reset_query_pool_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        