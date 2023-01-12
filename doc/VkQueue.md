[VkQueue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueue.html) - Opaque handle to a queue object

# C Specifications
Creating a logical device also creates the queues associated with that
device.
The queues to create are described by a set of [`DeviceQueueCreateInfo`]
structures that are passed to [`create_device`] in
`pQueueCreateInfos`.Queues are represented by [`Queue`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_HANDLE(VkQueue)
```

# Related
- [`crate::vulkan1_0`]
- [`get_device_queue`]
- [`get_device_queue2`]
- [`get_queue_checkpoint_data2_nv`]
- [`get_queue_checkpoint_data_nv`]
- [`queue_begin_debug_utils_label_ext`]
- [`queue_bind_sparse`]
- [`queue_end_debug_utils_label_ext`]
- [`queue_insert_debug_utils_label_ext`]
- [`queue_present_khr`]
- [`queue_set_performance_configuration_intel`]
- [`queue_submit`]
- [`queue_submit2`]
- [`queue_submit2_khr`]
- [`queue_wait_idle`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        