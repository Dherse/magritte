[vkCreateBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html) - Create a new buffer object

# C Specifications
To create buffers, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkCreateBuffer(
    VkDevice                                    device,
    const VkBufferCreateInfo*                   pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkBuffer*                                   pBuffer);
```

# Parameters
- [`device`] is the logical device that creates the buffer object.
- [`p_create_info`] is a pointer to a [`BufferCreateInfo`] structure containing parameters affecting creation of the buffer.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_buffer`] is a pointer to a [`Buffer`] handle in which the resulting buffer object is returned.

# Description
## Valid Usage
-    If the `flags` member of [`p_create_info`] includes `VK_BUFFER_CREATE_SPARSE_BINDING_BIT`, creating this [`Buffer`] **must**  not cause the total required sparse memory for all currently valid sparse resources on the device to exceed [`PhysicalDeviceLimits::sparse_address_space_size`]
-    If using the [`Buffer`] for an import operation from a [`BufferCollectionFUCHSIA`] where a [`BufferCollectionBufferCreateInfoFUCHSIA`] has been chained to `pNext`, [`p_create_info`] **must**  match the [`BufferConstraintsInfoFUCHSIA::create_info`] used when setting the constraints on the buffer collection with [`set_buffer_collection_buffer_constraints_fuchsia`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`BufferCreateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_buffer`] **must**  be a valid pointer to a [`Buffer`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Buffer`]
- [`BufferCreateInfo`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        