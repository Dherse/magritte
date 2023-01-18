[vkCopyAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureKHR.html) - Copy an acceleration structure on the host

# C Specifications
To copy or compact an acceleration structure on the host, call:
```c
// Provided by VK_KHR_acceleration_structure
VkResult vkCopyAccelerationStructureKHR(
    VkDevice                                    device,
    VkDeferredOperationKHR                      deferredOperation,
    const VkCopyAccelerationStructureInfoKHR*   pInfo);
```

# Parameters
- [`device`] is the device which owns the acceleration structures.
- [`deferred_operation`] is an optional [`DeferredOperationKHR`] to [request deferral](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#deferred-host-operations-requesting) for this command.
- [`p_info`] is a pointer to a [`CopyAccelerationStructureInfoKHR`] structure defining the copy operation.

# Description
This command fulfills the same task as
[`cmd_copy_acceleration_structure_khr`] but is executed by the host.
## Valid Usage
-    If [`deferred_operation`] is not [`crate::Handle::null`], it  **must**  be a valid [`DeferredOperationKHR`] object
-    Any previous deferred operation that was associated with [`deferred_operation`] **must**  be complete
-    The `buffer` used to create `pInfo->src` **must**  be bound to host-visible device memory
-    The `buffer` used to create `pInfo->dst` **must**  be bound to host-visible device memory
-    The [[`PhysicalDeviceAccelerationStructureFeaturesKHR::acceleration_structure_host_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-accelerationStructureHostCommands) feature  **must**  be enabled
-    The `buffer` used to create `pInfo->src` **must**  be bound to memory that was not allocated with multiple instances
-    The `buffer` used to create `pInfo->dst` **must**  be bound to memory that was not allocated with multiple instances

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`deferred_operation`] is not [`crate::Handle::null`], [`deferred_operation`] **must**  be a valid [`DeferredOperationKHR`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`CopyAccelerationStructureInfoKHR`] structure
-    If [`deferred_operation`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_OPERATION_DEFERRED_KHR`  - `VK_OPERATION_NOT_DEFERRED_KHR` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`VK_KHR_acceleration_structure`]
- [`CopyAccelerationStructureInfoKHR`]
- [`DeferredOperationKHR`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        