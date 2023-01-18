[VkDeviceGroupPresentCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentCapabilitiesKHR.html) - Present capabilities from other physical devices

# C Specifications
The [`DeviceGroupPresentCapabilitiesKHR`] structure is defined as:
```c
// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
typedef struct VkDeviceGroupPresentCapabilitiesKHR {
    VkStructureType                     sType;
    void*                               pNext;
    uint32_t                            presentMask[VK_MAX_DEVICE_GROUP_SIZE];
    VkDeviceGroupPresentModeFlagsKHR    modes;
} VkDeviceGroupPresentCapabilitiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`present_mask`] is an array of [`MAX_DEVICE_GROUP_SIZE`]`uint32_t` masks, where the mask at element i is non-zero if physical device i has a presentation engine, and where bit j is set in element i if physical device i **can**  present swapchain images from physical device j. If element i is non-zero, then bit i **must**  be set.
- [`modes`] is a bitmask of [`DeviceGroupPresentModeFlagBitsKHR`] indicating which device group presentation modes are supported.

# Description
[`modes`] always has `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR` set.The present mode flags are also used when presenting an image, in
[`DeviceGroupPresentInfoKHR::mode`].If a device group only includes a single physical device, then [`modes`] **must**  equal `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_KHR_device_group`]
- [`VK_KHR_surface`]
- [`VK_KHR_swapchain`]
- [`crate::vulkan1_1`]
- [`DeviceGroupPresentModeFlagsKHR`]
- [`StructureType`]
- [`get_device_group_present_capabilities_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        