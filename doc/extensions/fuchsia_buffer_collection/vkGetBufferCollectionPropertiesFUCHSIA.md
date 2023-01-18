[vkGetBufferCollectionPropertiesFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html) - Retrieve properties from a buffer collection

# C Specifications
After constraints have been set on the buffer collection by calling
[`set_buffer_collection_image_constraints_fuchsia`] or
[`set_buffer_collection_buffer_constraints_fuchsia`], call
[`get_buffer_collection_properties_fuchsia`] to retrieve the negotiated and
finalized properties of the buffer collection.The call to [`get_buffer_collection_properties_fuchsia`] is synchronous.
It waits for the Sysmem format negotiation and buffer collection allocation
to complete before returning.
```c
// Provided by VK_FUCHSIA_buffer_collection
VkResult vkGetBufferCollectionPropertiesFUCHSIA(
    VkDevice                                    device,
    VkBufferCollectionFUCHSIA                   collection,
    VkBufferCollectionPropertiesFUCHSIA*        pProperties);
```

# Parameters
- [`device`] is the logical device handle
- [`collection`] is the [`BufferCollectionFUCHSIA`] handle
- [`p_properties`] is a pointer to the retrieved [`BufferCollectionPropertiesFUCHSIA`] struct

# Description
For image-based buffer collections, upon calling
[`get_buffer_collection_properties_fuchsia`], Sysmem will choose an element
of the [`ImageConstraintsInfoFUCHSIA`]`::pImageCreateInfos`
established by the preceding call to
[`set_buffer_collection_image_constraints_fuchsia`].
The index of the element chosen is stored in and can be retrieved from
[`BufferCollectionPropertiesFUCHSIA::create_info_index`].For buffer-based buffer collections, a single [`BufferCreateInfo`] is
specified as [`BufferConstraintsInfoFUCHSIA::create_info`].
[`BufferCollectionPropertiesFUCHSIA::create_info_index`] will
therefore always be zero.[`get_buffer_collection_properties_fuchsia`] **may**  fail if Sysmem is unable
to resolve the constraints of all of the participants in the buffer
collection.
If that occurs, [`get_buffer_collection_properties_fuchsia`] will return
`VK_ERROR_INITIALIZATION_FAILED`.
## Valid Usage
-    Prior to calling [`get_buffer_collection_properties_fuchsia`], the constraints on the buffer collection  **must**  have been set by either [`set_buffer_collection_image_constraints_fuchsia`] or [`set_buffer_collection_buffer_constraints_fuchsia`].

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
-  [`p_properties`] **must**  be a valid pointer to a [`BufferCollectionPropertiesFUCHSIA`] structure
-  [`collection`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`VK_FUCHSIA_buffer_collection`]
- [`BufferCollectionFUCHSIA`]
- [`BufferCollectionPropertiesFUCHSIA`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        