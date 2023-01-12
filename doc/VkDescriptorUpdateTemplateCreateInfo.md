[VkDescriptorUpdateTemplateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateInfo.html) - Structure specifying parameters of a newly created descriptor update template

# C Specifications
The [`DescriptorUpdateTemplateCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkDescriptorUpdateTemplateCreateInfo {
    VkStructureType                           sType;
    const void*                               pNext;
    VkDescriptorUpdateTemplateCreateFlags     flags;
    uint32_t                                  descriptorUpdateEntryCount;
    const VkDescriptorUpdateTemplateEntry*    pDescriptorUpdateEntries;
    VkDescriptorUpdateTemplateType            templateType;
    VkDescriptorSetLayout                     descriptorSetLayout;
    VkPipelineBindPoint                       pipelineBindPoint;
    VkPipelineLayout                          pipelineLayout;
    uint32_t                                  set;
} VkDescriptorUpdateTemplateCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_descriptor_update_template
typedef VkDescriptorUpdateTemplateCreateInfo VkDescriptorUpdateTemplateCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`descriptor_update_entry_count`] is the number of elements in the [`descriptor_update_entries`] array.
- [`descriptor_update_entries`] is a pointer to an array of [`DescriptorUpdateTemplateEntry`] structures describing the descriptors to be updated by the descriptor update template.
- [`template_type`] Specifies the type of the descriptor update template. If set to `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET` it  **can**  only be used to update descriptor sets with a fixed [`descriptor_set_layout`]. If set to `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR` it  **can**  only be used to push descriptor sets using the provided [`pipeline_bind_point`], [`pipeline_layout`], and [`set`] number.
- [`descriptor_set_layout`] is the descriptor set layout used to build the descriptor update template. All descriptor sets which are going to be updated through the newly created descriptor update template  **must**  be created with a layout that matches (is the same as, or defined identically to) this layout. This parameter is ignored if [`template_type`] is not `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`.
- [`pipeline_bind_point`] is a [`PipelineBindPoint`] indicating the type of the pipeline that will use the descriptors. This parameter is ignored if [`template_type`] is not `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
- [`pipeline_layout`] is a [`PipelineLayout`] object used to program the bindings. This parameter is ignored if [`template_type`] is not `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
- [`set`] is the set number of the descriptor set in the pipeline layout that will be updated. This parameter is ignored if [`template_type`] is not `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`

# Description
## Valid Usage
-    If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`, [`descriptor_set_layout`] **must**  be a valid [`DescriptorSetLayout`] handle
-    If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`, [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
-    If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`, [`pipeline_layout`] **must**  be a valid [`PipelineLayout`] handle
-    If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`, [`set`] **must**  be the unique set number in the pipeline layout that uses a descriptor set layout that was created with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`
-    If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`, [`descriptor_set_layout`] **must**  not contain a binding with type `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`
-  [`descriptor_update_entries`] **must**  be a valid pointer to an array of [`descriptor_update_entry_count`] valid [`DescriptorUpdateTemplateEntry`] structures
-  [`template_type`] **must**  be a valid [`DescriptorUpdateTemplateType`] value
-  [`descriptor_update_entry_count`] **must**  be greater than `0`
-    Both of [`descriptor_set_layout`], and [`pipeline_layout`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`crate::vulkan1_1`]
- [`DescriptorSetLayout`]
- [`DescriptorUpdateTemplateCreateFlags`]
- [`DescriptorUpdateTemplateEntry`]
- [`DescriptorUpdateTemplateType`]
- [`PipelineBindPoint`]
- [`PipelineLayout`]
- [`StructureType`]
- [`create_descriptor_update_template`]
- [`create_descriptor_update_template_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        