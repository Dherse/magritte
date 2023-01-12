[VkDeviceGroupBindSparseInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupBindSparseInfo.html) - Structure indicating which instances are bound

# C Specifications
If the [`p_next`] chain of [`BindSparseInfo`] includes a
[`DeviceGroupBindSparseInfo`] structure, then that structure includes
device indices specifying which instance of the resources and memory are
bound.The [`DeviceGroupBindSparseInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkDeviceGroupBindSparseInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           resourceDeviceIndex;
    uint32_t           memoryDeviceIndex;
} VkDeviceGroupBindSparseInfo;
```
or the equivalent
```c
// Provided by VK_KHR_device_group
typedef VkDeviceGroupBindSparseInfo VkDeviceGroupBindSparseInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`resource_device_index`] is a device index indicating which instance of the resource is bound.
- [`memory_device_index`] is a device index indicating which instance of the memory the resource instance is bound to.

# Description
These device indices apply to all buffer and image memory binds included in
the batch pointing to this structure.
The semaphore waits and signals for the batch are executed only by the
physical device specified by the [`resource_device_index`].If this structure is not present, [`resource_device_index`] and
[`memory_device_index`] are assumed to be zero.
## Valid Usage
-  [`resource_device_index`] and [`memory_device_index`] **must**  both be valid device indices
-    Each memory allocation bound in this batch  **must**  have allocated an instance for [`memory_device_index`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO`

# Related
- [`crate::vulkan1_1`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        