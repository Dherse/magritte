[VkQueryResultFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlagBits.html) - Bitmask specifying how and when query results are returned

# C Specifications
Bits which  **can**  be set in [`get_query_pool_results`]`::flags` and
[`cmd_copy_query_pool_results`]`::flags`, specifying how and when
results are returned, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkQueryResultFlagBits {
    VK_QUERY_RESULT_64_BIT = 0x00000001,
    VK_QUERY_RESULT_WAIT_BIT = 0x00000002,
    VK_QUERY_RESULT_WITH_AVAILABILITY_BIT = 0x00000004,
    VK_QUERY_RESULT_PARTIAL_BIT = 0x00000008,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_queue
    VK_QUERY_RESULT_WITH_STATUS_BIT_KHR = 0x00000010,
#endif
} VkQueryResultFlagBits;
```

# Description
- [`VK_QUERY_RESULT_FLAG_BITS`] specifies the results will be written as an array of 64-bit unsigned integer values. If this bit is not set, the results will be written as an array of 32-bit unsigned integer values.
- [`VK_QUERY_RESULT_FLAG_BITS`] specifies that Vulkan will wait for each query’s status to become available before retrieving its results.
- [`VK_QUERY_RESULT_FLAG_BITS`] specifies that the availability status accompanies the results.
- [`VK_QUERY_RESULT_FLAG_BITS`] specifies that returning partial results is acceptable.
- [`WITH_STATUS_KHR`] specifies that the last value returned in the query is a [`QueryResultStatusKHR`] value. See [result status query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-result-status-only) for information on how an application can determine whether the use of this flag bit is supported.

# Related
- [`crate::vulkan1_0`]
- [VkQueryResultFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        