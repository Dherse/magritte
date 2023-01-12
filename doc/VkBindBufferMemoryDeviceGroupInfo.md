[VkBindBufferMemoryDeviceGroupInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryDeviceGroupInfo.html) - Structure specifying device within a group to bind to

# C Specifications
The [`BindBufferMemoryDeviceGroupInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkBindBufferMemoryDeviceGroupInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           deviceIndexCount;
    const uint32_t*    pDeviceIndices;
} VkBindBufferMemoryDeviceGroupInfo;
```
or the equivalent
```c
// Provided by VK_KHR_bind_memory2 with VK_KHR_device_group
typedef VkBindBufferMemoryDeviceGroupInfo VkBindBufferMemoryDeviceGroupInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`device_index_count`] is the number of elements in [`device_indices`].
- [`device_indices`] is a pointer to an array of device indices.

# Description
If the [`p_next`] chain of [`BindBufferMemoryInfo`] includes a
[`BindBufferMemoryDeviceGroupInfo`] structure, then that structure
determines how memory is bound to buffers across multiple devices in a
device group.If [`device_index_count`] is greater than zero, then on device index i
the buffer is attached to the instance of `memory` on the physical
device with device index [`device_indices`][i].If [`device_index_count`] is zero and `memory` comes from a memory heap
with the `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` bit set, then it is as if
[`device_indices`] contains consecutive indices from zero to the number of
physical devices in the logical device, minus one.
In other words, by default each physical device attaches to its own instance
of `memory`.If [`device_index_count`] is zero and `memory` comes from a memory heap
without the `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` bit set, then it is as
if [`device_indices`] contains an array of zeros.
In other words, by default each physical device attaches to instance zero.
## Valid Usage
-  [`device_index_count`] **must**  either be zero or equal to the number of physical devices in the logical device
-    All elements of [`device_indices`] **must**  be valid device indices

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO`
-    If [`device_index_count`] is not `0`, [`device_indices`] **must**  be a valid pointer to an array of [`device_index_count`]`uint32_t` values

# Related
- [`crate::vulkan1_1`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        