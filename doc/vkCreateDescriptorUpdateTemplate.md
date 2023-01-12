[vkCreateDescriptorUpdateTemplate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplate.html) - Create a new descriptor update template

# C Specifications
Updating a large [`DescriptorSet`] array  **can**  be an expensive operation
since an application  **must**  specify one [`WriteDescriptorSet`] structure
for each descriptor or descriptor array to update, each of which
re-specifies the same state when updating the same descriptor in multiple
descriptor sets.
For cases when an application wishes to update the same set of descriptors
in multiple descriptor sets allocated using the same
[`DescriptorSetLayout`], [`update_descriptor_set_with_template`] **can**  be
used as a replacement for [`update_descriptor_sets`].[`DescriptorUpdateTemplate`] allows implementations to convert a set of
descriptor update operations on a single descriptor set to an internal
format that, in conjunction with [`update_descriptor_set_with_template`]
or [`cmd_push_descriptor_set_with_template_khr`]
,  **can**  be more efficient compared to calling [`update_descriptor_sets`]
or [`cmd_push_descriptor_set_khr`]
.
The descriptors themselves are not specified in the
[`DescriptorUpdateTemplate`], rather, offsets into an application
provided pointer to host memory are specified, which are combined with a
pointer passed to [`update_descriptor_set_with_template`]
or [`cmd_push_descriptor_set_with_template_khr`]
.
This allows large batches of updates to be executed without having to
convert application data structures into a strictly-defined Vulkan data
structure.To create a descriptor update template, call:
```c
// Provided by VK_VERSION_1_1
VkResult vkCreateDescriptorUpdateTemplate(
    VkDevice                                    device,
    const VkDescriptorUpdateTemplateCreateInfo* pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkDescriptorUpdateTemplate*                 pDescriptorUpdateTemplate);
```
or the equivalent command
```c
// Provided by VK_KHR_descriptor_update_template
VkResult vkCreateDescriptorUpdateTemplateKHR(
    VkDevice                                    device,
    const VkDescriptorUpdateTemplateCreateInfo* pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkDescriptorUpdateTemplate*                 pDescriptorUpdateTemplate);
```

# Parameters
- [`device`] is the logical device that creates the descriptor update template.
- [`p_create_info`] is a pointer to a [`DescriptorUpdateTemplateCreateInfo`] structure specifying the set of descriptors to update with a single call to [`cmd_push_descriptor_set_with_template_khr`] or [`update_descriptor_set_with_template`].
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_descriptor_update_template`] is a pointer to a [`DescriptorUpdateTemplate`] handle in which the resulting descriptor update template object is returned.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`DescriptorUpdateTemplateCreateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_descriptor_update_template`] **must**  be a valid pointer to a [`DescriptorUpdateTemplate`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_1`]
- [`AllocationCallbacks`]
- [`DescriptorUpdateTemplate`]
- [`DescriptorUpdateTemplateCreateInfo`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        