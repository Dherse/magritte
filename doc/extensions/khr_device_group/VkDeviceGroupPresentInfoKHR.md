[VkDeviceGroupPresentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentInfoKHR.html) - Mode and mask controlling which physical devices' images are presented

# C Specifications
If the [`p_next`] chain of [`PresentInfoKHR`] includes a
[`DeviceGroupPresentInfoKHR`] structure, then that structure includes an
array of device masks and a device group present mode.The [`DeviceGroupPresentInfoKHR`] structure is defined as:
```c
// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
typedef struct VkDeviceGroupPresentInfoKHR {
    VkStructureType                        sType;
    const void*                            pNext;
    uint32_t                               swapchainCount;
    const uint32_t*                        pDeviceMasks;
    VkDeviceGroupPresentModeFlagBitsKHR    mode;
} VkDeviceGroupPresentInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`swapchain_count`] is zero or the number of elements in [`device_masks`].
- [`device_masks`] is a pointer to an array of device masks, one for each element of [`PresentInfoKHR`]::pSwapchains.
- [`mode`] is a [`DeviceGroupPresentModeFlagBitsKHR`] value specifying the device group present mode that will be used for this present.

# Description
If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`, then each
element of [`device_masks`] selects which instance of the swapchain image
is presented.
Each element of [`device_masks`] **must**  have exactly one bit set, and the
corresponding physical device  **must**  have a presentation engine as reported
by [`DeviceGroupPresentCapabilitiesKHR`].If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR`, then
each element of [`device_masks`] selects which instance of the swapchain
image is presented.
Each element of [`device_masks`] **must**  have exactly one bit set, and some
physical device in the logical device  **must**  include that bit in its
[`DeviceGroupPresentCapabilitiesKHR::present_mask`].If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR`, then each
element of [`device_masks`] selects which instances of the swapchain image
are component-wise summed and the sum of those images is presented.
If the sum in any component is outside the representable range, the value of
that component is undefined.
Each element of [`device_masks`] **must**  have a value for which all set bits
are set in one of the elements of
[`DeviceGroupPresentCapabilitiesKHR::present_mask`].If [`mode`] is
`VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR`, then each
element of [`device_masks`] selects which instance(s) of the swapchain
images are presented.
For each bit set in each element of [`device_masks`], the corresponding
physical device  **must**  have a presentation engine as reported by
[`DeviceGroupPresentCapabilitiesKHR`].If [`DeviceGroupPresentInfoKHR`] is not provided or [`swapchain_count`]
is zero then the masks are considered to be `1`.
If [`DeviceGroupPresentInfoKHR`] is not provided, [`mode`] is
considered to be `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`.
## Valid Usage
-  [`swapchain_count`] **must**  equal `0` or [`PresentInfoKHR`]::[`swapchain_count`]
-    If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`, then each element of [`device_masks`] **must**  have exactly one bit set, and the corresponding element of [`DeviceGroupPresentCapabilitiesKHR::present_mask`] **must**  be non-zero
-    If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR`, then each element of [`device_masks`] **must**  have exactly one bit set, and some physical device in the logical device  **must**  include that bit in its [`DeviceGroupPresentCapabilitiesKHR::present_mask`]
-    If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR`, then each element of [`device_masks`] **must**  have a value for which all set bits are set in one of the elements of [`DeviceGroupPresentCapabilitiesKHR::present_mask`]
-    If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR`, then for each bit set in each element of [`device_masks`], the corresponding element of [`DeviceGroupPresentCapabilitiesKHR::present_mask`] **must**  be non-zero
-    The value of each element of [`device_masks`] **must**  be equal to the device mask passed in [`AcquireNextImageInfoKHR::device_mask`] when the image index was last acquired
-  [`mode`] **must**  have exactly one bit set, and that bit  **must**  have been included in [`DeviceGroupSwapchainCreateInfoKHR::modes`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR`
-    If [`swapchain_count`] is not `0`, [`device_masks`] **must**  be a valid pointer to an array of [`swapchain_count`]`uint32_t` values
-  [`mode`] **must**  be a valid [`DeviceGroupPresentModeFlagBitsKHR`] value

# Related
- [`khr_device_group`]
- [`khr_swapchain`]
- [`crate::vulkan1_1`]
- [`DeviceGroupPresentModeFlagBitsKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        