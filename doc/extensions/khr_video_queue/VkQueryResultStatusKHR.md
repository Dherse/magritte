[VkQueryResultStatusKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryResultStatusKHR.html) - Specific status codes for operations

# C Specifications
Specific status codes that  **can**  be returned from a query are:
```c
// Provided by VK_KHR_video_queue
typedef enum VkQueryResultStatusKHR {
    VK_QUERY_RESULT_STATUS_ERROR_KHR = -1,
    VK_QUERY_RESULT_STATUS_NOT_READY_KHR = 0,
    VK_QUERY_RESULT_STATUS_COMPLETE_KHR = 1,
} VkQueryResultStatusKHR;
```

# Description
- [`VK_QUERY_RESULT_STATUS_KHR`] specifies that the query result is not yet available.
- [`VK_QUERY_RESULT_STATUS_KHR`] specifies that operations did not complete successfully.
- [`VK_QUERY_RESULT_STATUS_KHR`] specifies that operations completed successfully and the query result is available.

# Related
- [`khr_video_queue`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        