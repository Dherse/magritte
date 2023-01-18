[vkGetQueueCheckpointData2NV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointData2NV.html) - Retrieve diagnostic checkpoint data

# C Specifications
If the device encounters an error during execution, the implementation will
return a `VK_ERROR_DEVICE_LOST` error to the application at some point
during host execution.
When this happens, the application  **can**  call
[`get_queue_checkpoint_data2_nv`] to retrieve information on the most recent
diagnostic checkpoints that were executed by the device.
```c
// Provided by VK_KHR_synchronization2 with VK_NV_device_diagnostic_checkpoints
void vkGetQueueCheckpointData2NV(
    VkQueue                                     queue,
    uint32_t*                                   pCheckpointDataCount,
    VkCheckpointData2NV*                        pCheckpointData);
```

# Parameters
- [`queue`] is the [`Queue`] object the caller would like to retrieve checkpoint data for
- [`p_checkpoint_data_count`] is a pointer to an integer related to the number of checkpoint markers available or queried, as described below.
- [`p_checkpoint_data`] is either `NULL` or a pointer to an array of [`CheckpointData2NV`] structures.

# Description
If [`p_checkpoint_data`] is `NULL`, then the number of checkpoint markers
available is returned in [`p_checkpoint_data_count`].
Otherwise, [`p_checkpoint_data_count`] **must**  point to a variable set by the
user to the number of elements in the [`p_checkpoint_data`] array, and on
return the variable is overwritten with the number of structures actually
written to [`p_checkpoint_data`].If [`p_checkpoint_data_count`] is less than the number of checkpoint markers
available, at most [`p_checkpoint_data_count`] structures will be written.
## Valid Usage
-    The device that [`queue`] belongs to  **must**  be in the lost state

## Valid Usage (Implicit)
-  [`queue`] **must**  be a valid [`Queue`] handle
-  [`p_checkpoint_data_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_checkpoint_data_count`] is not `0`, and [`p_checkpoint_data`] is not `NULL`, [`p_checkpoint_data`] **must**  be a valid pointer to an array of [`p_checkpoint_data_count`][`CheckpointData2NV`] structures

# Related
- [`VK_KHR_synchronization2`]
- [`VK_NV_device_diagnostic_checkpoints`]
- [`CheckpointData2NV`]
- [`Queue`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        