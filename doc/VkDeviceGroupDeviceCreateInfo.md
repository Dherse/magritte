[VkDeviceGroupDeviceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupDeviceCreateInfo.html) - Create a logical device from multiple physical devices

# C Specifications
A logical device  **can**  be created that connects to one or more physical
devices by adding a [`DeviceGroupDeviceCreateInfo`] structure to the
[`p_next`] chain of [`DeviceCreateInfo`].
The [`DeviceGroupDeviceCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkDeviceGroupDeviceCreateInfo {
    VkStructureType            sType;
    const void*                pNext;
    uint32_t                   physicalDeviceCount;
    const VkPhysicalDevice*    pPhysicalDevices;
} VkDeviceGroupDeviceCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_device_group_creation
typedef VkDeviceGroupDeviceCreateInfo VkDeviceGroupDeviceCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`physical_device_count`] is the number of elements in the [`physical_devices`] array.
- [`physical_devices`] is a pointer to an array of physical device handles belonging to the same device group.

# Description
The elements of the [`physical_devices`] array are an ordered list of the
physical devices that the logical device represents.
These  **must**  be a subset of a single device group, and need not be in the
same order as they were enumerated.
The order of the physical devices in the [`physical_devices`] array
determines the *device index* of each physical device, with element i
being assigned a device index of i.
Certain commands and structures refer to one or more physical devices by
using device indices or *device masks* formed using device indices.A logical device created without using [`DeviceGroupDeviceCreateInfo`],
or with [`physical_device_count`] equal to zero, is equivalent to a
[`physical_device_count`] of one and [`physical_devices`] pointing to the
`physicalDevice` parameter to [`create_device`].
In particular, the device index of that physical device is zero.
## Valid Usage
-    Each element of [`physical_devices`] **must**  be unique
-    All elements of [`physical_devices`] **must**  be in the same device group as enumerated by [`enumerate_physical_device_groups`]
-    If [`physical_device_count`] is not `0`, the `physicalDevice` parameter of [`create_device`] **must**  be an element of [`physical_devices`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO`
-    If [`physical_device_count`] is not `0`, [`physical_devices`] **must**  be a valid pointer to an array of [`physical_device_count`] valid [`PhysicalDevice`] handles

# Related
- [`crate::vulkan1_1`]
- [`PhysicalDevice`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        