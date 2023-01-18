[VkDeviceQueueInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueInfo2.html) - Structure specifying the parameters used for device queue creation

# C Specifications
The [`DeviceQueueInfo2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkDeviceQueueInfo2 {
    VkStructureType             sType;
    const void*                 pNext;
    VkDeviceQueueCreateFlags    flags;
    uint32_t                    queueFamilyIndex;
    uint32_t                    queueIndex;
} VkDeviceQueueInfo2;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure. The [`p_next`] chain of [`DeviceQueueInfo2`] **can**  be used to provide additional device queue parameters to [`get_device_queue2`].
- [`flags`] is a [`DeviceQueueCreateFlags`] value indicating the flags used to create the device queue.
- [`queue_family_index`] is the index of the queue family to which the queue belongs.
- [`queue_index`] is the index within this queue family of the queue to retrieve.

# Description
The queue returned by [`get_device_queue2`] **must**  have the same
[`flags`] value from this structure as that used at device creation time
in a [`DeviceQueueCreateInfo`] structure.
If no matching [`flags`] were specified at device creation time, then the
handle returned in `pQueue` **must**  be `NULL`.
## Valid Usage
-  [`queue_family_index`] **must**  be one of the queue family indices specified when `device` was created, via the [`DeviceQueueCreateInfo`] structure
-  [`flags`] **must**  be equal to [`DeviceQueueCreateInfo`]::[`flags`] for a [`DeviceQueueCreateInfo`] structure for the queue family indicated by [`queue_family_index`] when `device` was created
-  [`queue_index`] **must**  be less than [`DeviceQueueCreateInfo::queue_count`] for the corresponding queue family and flags indicated by [`queue_family_index`] and [`flags`] when `device` was created

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be a valid combination of [`DeviceQueueCreateFlagBits`] values

# Related
- [`crate::vulkan1_1`]
- [`DeviceQueueCreateFlags`]
- [`StructureType`]
- [`get_device_queue2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        