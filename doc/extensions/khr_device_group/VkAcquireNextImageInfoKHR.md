[VkAcquireNextImageInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAcquireNextImageInfoKHR.html) - Structure specifying parameters of the acquire

# C Specifications
The [`AcquireNextImageInfoKHR`] structure is defined as:
```c
// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
typedef struct VkAcquireNextImageInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    VkSwapchainKHR     swapchain;
    uint64_t           timeout;
    VkSemaphore        semaphore;
    VkFence            fence;
    uint32_t           deviceMask;
} VkAcquireNextImageInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`swapchain`] is a non-retired swapchain from which an image is acquired.
- [`timeout`] specifies how long the function waits, in nanoseconds, if no image is available.
- [`semaphore`] is [`crate::Handle::null`] or a semaphore to signal.
- [`fence`] is [`crate::Handle::null`] or a fence to signal.
- [`device_mask`] is a mask of physical devices for which the swapchain image will be ready to use when the semaphore or fence is signaled.

# Description
If [`acquire_next_image_khr`] is used, the device mask is considered to
include all physical devices in the logical device.
## Valid Usage
-  [`swapchain`] **must**  not be in the retired state
-    If [`semaphore`] is not [`crate::Handle::null`] it  **must**  be unsignaled
-    If [`semaphore`] is not [`crate::Handle::null`] it  **must**  not have any uncompleted signal or wait operations pending
-    If [`fence`] is not [`crate::Handle::null`] it  **must**  be unsignaled and  **must**  not be associated with any other queue command that has not yet completed execution on that queue
-  [`semaphore`] and [`fence`] **must**  not both be equal to [`crate::Handle::null`]
-  [`device_mask`] **must**  be a valid device mask
-  [`device_mask`] **must**  not be zero
-  [`semaphore`] **must**  have a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
-    If [`semaphore`] is not [`crate::Handle::null`], [`semaphore`] **must**  be a valid [`Semaphore`] handle
-    If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  be a valid [`Fence`] handle
-    Each of [`fence`], [`semaphore`], and [`swapchain`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Host Synchronization
- Host access to [`swapchain`] **must**  be externally synchronized
- Host access to [`semaphore`] **must**  be externally synchronized
- Host access to [`fence`] **must**  be externally synchronized

# Related
- [`khr_device_group`]
- [`khr_swapchain`]
- [`crate::vulkan1_1`]
- [`Fence`]
- [`Semaphore`]
- [`StructureType`]
- [`SwapchainKHR`]
- [`acquire_next_image2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        