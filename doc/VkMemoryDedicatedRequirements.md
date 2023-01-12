[VkMemoryDedicatedRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedRequirements.html) - Structure describing dedicated allocation requirements of buffer and image resources

# C Specifications
The [`MemoryDedicatedRequirements`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkMemoryDedicatedRequirements {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           prefersDedicatedAllocation;
    VkBool32           requiresDedicatedAllocation;
} VkMemoryDedicatedRequirements;
```
or the equivalent
```c
// Provided by VK_KHR_dedicated_allocation
typedef VkMemoryDedicatedRequirements VkMemoryDedicatedRequirementsKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`prefers_dedicated_allocation`] specifies that the implementation would prefer a dedicated allocation for this resource. The application is still free to suballocate the resource but it  **may**  get better performance if a dedicated allocation is used.
- [`requires_dedicated_allocation`] specifies that a dedicated allocation is required for this resource.

# Description
To determine the dedicated allocation requirements of a buffer or image
resource, add a [`MemoryDedicatedRequirements`] structure to the
[`p_next`] chain of the [`MemoryRequirements2`] structure passed as the
`pMemoryRequirements` parameter of [`get_buffer_memory_requirements2`]
or [`get_image_memory_requirements2`], respectively.Constraints on the values returned for buffer resources are:
- [`requires_dedicated_allocation`] **may**  be `VK_TRUE` if the [`p_next`] chain of [`BufferCreateInfo`] for the call to [`create_buffer`] used to create the buffer being queried included a [`ExternalMemoryBufferCreateInfo`] structure, and any of the handle types specified in [`ExternalMemoryBufferCreateInfo::handle_types`] requires dedicated allocation, as reported by [`get_physical_device_external_buffer_properties`] in [`ExternalBufferProperties`]::`externalMemoryProperties.externalMemoryFeatures`. Otherwise, [`requires_dedicated_allocation`] will be `VK_FALSE`.
- When the implementation sets [`requires_dedicated_allocation`] to `VK_TRUE`, it  **must**  also set [`prefers_dedicated_allocation`] to `VK_TRUE`.
- If `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` was set in [`BufferCreateInfo::flags`] when `buffer` was created, then both [`prefers_dedicated_allocation`] and [`requires_dedicated_allocation`] will be `VK_FALSE`.
Constraints on the values returned for image resources are:
- [`requires_dedicated_allocation`] **may**  be `VK_TRUE` if the [`p_next`] chain of [`ImageCreateInfo`] for the call to [`create_image`] used to create the image being queried included a [`ExternalMemoryImageCreateInfo`] structure, and any of the handle types specified in [`ExternalMemoryImageCreateInfo::handle_types`] requires dedicated allocation, as reported by [`get_physical_device_image_format_properties2`] in [`ExternalImageFormatProperties`]::`externalMemoryProperties.externalMemoryFeatures`. Otherwise, [`requires_dedicated_allocation`] will be `VK_FALSE`.
- If `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` was set in [`ImageCreateInfo::flags`] when `image` was created, then both [`prefers_dedicated_allocation`] and [`requires_dedicated_allocation`] will be `VK_FALSE`.

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS`

# Related
- [`crate::vulkan1_1`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        