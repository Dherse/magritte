[vkSetBufferCollectionImageConstraintsFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html) - Set image-based constraints for a buffer collection

# C Specifications
Setting the constraints on the buffer collection initiates the format
negotiation and allocation of the buffer collection.
To set the constraints on a [`Image`] buffer collection, call:
```c
// Provided by VK_FUCHSIA_buffer_collection
VkResult vkSetBufferCollectionImageConstraintsFUCHSIA(
    VkDevice                                    device,
    VkBufferCollectionFUCHSIA                   collection,
    const VkImageConstraintsInfoFUCHSIA*        pImageConstraintsInfo);
```

# Parameters
- [`device`] is the logical device
- [`collection`] is the [`BufferCollectionFUCHSIA`] handle
- [`p_image_constraints_info`] is a pointer to a [`ImageConstraintsInfoFUCHSIA`] structure

# Description
[`set_buffer_collection_image_constraints_fuchsia`] **may**  fail if
[`p_image_constraints_info`]`::formatConstraintsCount` is larger than the
implementation-defined limit.
If that occurs, [`set_buffer_collection_image_constraints_fuchsia`] will
return VK_ERROR_INITIALIZATION_FAILED.[`set_buffer_collection_image_constraints_fuchsia`] **may**  fail if the
implementation does not support any of the formats described by the
[`p_image_constraints_info`] structure.
If that occurs, [`set_buffer_collection_image_constraints_fuchsia`] will
return `VK_ERROR_FORMAT_NOT_SUPPORTED`.
## Valid Usage
-  [`set_buffer_collection_image_constraints_fuchsia`] or [`set_buffer_collection_buffer_constraints_fuchsia`] **must**  not have already been called on [`collection`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
-  [`p_image_constraints_info`] **must**  be a valid pointer to a valid [`ImageConstraintsInfoFUCHSIA`] structure
-  [`collection`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_FORMAT_NOT_SUPPORTED`

# Related
- [`fuchsia_buffer_collection`]
- [`BufferCollectionFUCHSIA`]
- [`Device`]
- [`ImageConstraintsInfoFUCHSIA`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        