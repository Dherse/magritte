[vkCreateImage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html) - Create a new image object

# C Specifications
To create images, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkCreateImage(
    VkDevice                                    device,
    const VkImageCreateInfo*                    pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkImage*                                    pImage);
```

# Parameters
- [`device`] is the logical device that creates the image.
- [`p_create_info`] is a pointer to a [`ImageCreateInfo`] structure containing parameters to be used to create the image.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_image`] is a pointer to a [`Image`] handle in which the resulting image object is returned.

# Description
## Valid Usage
-    If the `flags` member of [`p_create_info`] includes `VK_IMAGE_CREATE_SPARSE_BINDING_BIT`, creating this [`Image`] **must**  not cause the total required sparse memory for all currently valid sparse resources on the device to exceed [`PhysicalDeviceLimits::sparse_address_space_size`]
-    If a [`BufferCollectionImageCreateInfoFUCHSIA`] has been chained to `pNext`, [`p_create_info`] **must**  match the [Sysmem chosen [`ImageCreateInfo`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sysmem-chosen-create-infos) excepting members [`ImageCreateInfo::extent`] and [`ImageCreateInfo::usage`] in the match criteria

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`ImageCreateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_image`] **must**  be a valid pointer to a [`Image`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Device`]
- [`Image`]
- [`ImageCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        