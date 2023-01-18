[vkCopyAccelerationStructureToMemoryKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html) - Serialize an acceleration structure on the host

# C Specifications
To copy an acceleration structure to host accessible memory, call:
```c
// Provided by VK_KHR_acceleration_structure
VkResult vkCopyAccelerationStructureToMemoryKHR(
    VkDevice                                    device,
    VkDeferredOperationKHR                      deferredOperation,
    const VkCopyAccelerationStructureToMemoryInfoKHR* pInfo);
```

# Parameters
- [`device`] is the device which owns `pInfo->src`.
- [`deferred_operation`] is an optional [`DeferredOperationKHR`] to [request deferral](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#deferred-host-operations-requesting) for this command.
- [`p_info`] is a pointer to a [`CopyAccelerationStructureToMemoryInfoKHR`] structure defining the copy operation.

# Description
This command fulfills the same task as
[`cmd_copy_acceleration_structure_to_memory_khr`] but is executed by the host.This command produces the same results as
[`cmd_copy_acceleration_structure_to_memory_khr`], but writes its result
directly to a host pointer, and is executed on the host rather than the
device.
The output  **may**  not necessarily be bit-for-bit identical, but it can be
equally used by either [`cmd_copy_memory_to_acceleration_structure_khr`] or
[`copy_memory_to_acceleration_structure_khr`].
## Valid Usage
-    If [`deferred_operation`] is not [`crate::Handle::null`], it  **must**  be a valid [`DeferredOperationKHR`] object
-    Any previous deferred operation that was associated with [`deferred_operation`] **must**  be complete
-    The `buffer` used to create `pInfo->src` **must**  be bound to host-visible device memory
-  `pInfo->dst.hostAddress` **must**  be a valid host pointer
-  `pInfo->dst.hostAddress` **must**  be aligned to 16 bytes
-    The [[`PhysicalDeviceAccelerationStructureFeaturesKHR::acceleration_structure_host_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-accelerationStructureHostCommands) feature  **must**  be enabled
-    The `buffer` used to create `pInfo->src` **must**  be bound to memory that was not allocated with multiple instances

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`deferred_operation`] is not [`crate::Handle::null`], [`deferred_operation`] **must**  be a valid [`DeferredOperationKHR`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`CopyAccelerationStructureToMemoryInfoKHR`] structure
-    If [`deferred_operation`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_OPERATION_DEFERRED_KHR`  - `VK_OPERATION_NOT_DEFERRED_KHR` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`VK_KHR_acceleration_structure`]
- [`CopyAccelerationStructureToMemoryInfoKHR`]
- [`DeferredOperationKHR`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        