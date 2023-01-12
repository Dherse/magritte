[VkDeviceGroupCommandBufferBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupCommandBufferBeginInfo.html) - Set the initial device mask for a command buffer

# C Specifications
If the [`p_next`] chain of [`CommandBufferBeginInfo`] includes a
[`DeviceGroupCommandBufferBeginInfo`] structure, then that structure
includes an initial device mask for the command buffer.The [`DeviceGroupCommandBufferBeginInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkDeviceGroupCommandBufferBeginInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           deviceMask;
} VkDeviceGroupCommandBufferBeginInfo;
```
or the equivalent
```c
// Provided by VK_KHR_device_group
typedef VkDeviceGroupCommandBufferBeginInfo VkDeviceGroupCommandBufferBeginInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`device_mask`] is the initial value of the command buffer’s device mask.

# Description
The initial device mask also acts as an upper bound on the set of devices
that  **can**  ever be in the device mask in the command buffer.If this structure is not present, the initial value of a command buffer’s
device mask is set to include all physical devices in the logical device
when the command buffer begins recording.
## Valid Usage
-  [`device_mask`] **must**  be a valid device mask value
-  [`device_mask`] **must**  not be zero

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO`

# Related
- [`crate::vulkan1_1`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        