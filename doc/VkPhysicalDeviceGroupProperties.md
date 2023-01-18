[VkPhysicalDeviceGroupProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGroupProperties.html) - Structure specifying physical device group properties

# C Specifications
The [`PhysicalDeviceGroupProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDeviceGroupProperties {
    VkStructureType     sType;
    void*               pNext;
    uint32_t            physicalDeviceCount;
    VkPhysicalDevice    physicalDevices[VK_MAX_DEVICE_GROUP_SIZE];
    VkBool32            subsetAllocation;
} VkPhysicalDeviceGroupProperties;
```
or the equivalent
```c
// Provided by VK_KHR_device_group_creation
typedef VkPhysicalDeviceGroupProperties VkPhysicalDeviceGroupPropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`physical_device_count`] is the number of physical devices in the group.
- [`physical_devices`] is an array of [`MAX_DEVICE_GROUP_SIZE`][`PhysicalDevice`] handles representing all physical devices in the group. The first [`physical_device_count`] elements of the array will be valid.
- [`subset_allocation`] specifies whether logical devices created from the group support allocating device memory on a subset of devices, via the `deviceMask` member of the [`MemoryAllocateFlagsInfo`]. If this is [`FALSE`], then all device memory allocations are made across all physical devices in the group. If [`physical_device_count`] is `1`, then [`subset_allocation`] **must**  be [`FALSE`].

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES`
-  [`p_next`] **must**  be `NULL`

# Related
- [`crate::vulkan1_1`]
- [`Bool32`]
- [`PhysicalDevice`]
- [`StructureType`]
- [`enumerate_physical_device_groups`]
- [`enumerate_physical_device_groups_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        