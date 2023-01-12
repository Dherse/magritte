[vkGetDeferredOperationResultKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html) - Query the result of a deferred operation

# C Specifications
The [`get_deferred_operation_result_khr`] function is defined as:
```c
// Provided by VK_KHR_deferred_host_operations
VkResult vkGetDeferredOperationResultKHR(
    VkDevice                                    device,
    VkDeferredOperationKHR                      operation);
```

# Parameters
- [`device`] is the device which owns [`operation`].
- [`operation`] is the operation whose deferred result is being queried.

# Description
If no command has been deferred on [`operation`],
[`get_deferred_operation_result_khr`] returns `VK_SUCCESS`.If the deferred operation is pending, [`get_deferred_operation_result_khr`]
returns `VK_NOT_READY`.If the deferred operation is complete, it returns the appropriate return
value from the original command.
This value  **must**  be one of the [`VulkanResultCodes`] values which could have been
returned by the original command if the operation had not been deferred.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`operation`] **must**  be a valid [`DeferredOperationKHR`] handle
-  [`operation`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_NOT_READY`

# Related
- [`khr_deferred_host_operations`]
- [`DeferredOperationKHR`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        