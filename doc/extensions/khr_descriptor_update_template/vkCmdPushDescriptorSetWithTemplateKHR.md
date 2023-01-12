[vkCmdPushDescriptorSetWithTemplateKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) - Pushes descriptor updates into a command buffer using a descriptor update template

# C Specifications
It is also possible to use a descriptor update template to specify the push
descriptors to update.
To do so, call:
```c
// Provided by VK_VERSION_1_1 with VK_KHR_push_descriptor, VK_KHR_descriptor_update_template with VK_KHR_push_descriptor
void vkCmdPushDescriptorSetWithTemplateKHR(
    VkCommandBuffer                             commandBuffer,
    VkDescriptorUpdateTemplate                  descriptorUpdateTemplate,
    VkPipelineLayout                            layout,
    uint32_t                                    set,
    const void*                                 pData);
```

# Parameters
- [`command_buffer`] is the command buffer that the descriptors will be recorded in.
- [`descriptor_update_template`] is a descriptor update template defining how to interpret the descriptor information in [`p_data`].
- [`layout`] is a [`PipelineLayout`] object used to program the bindings. It  **must**  be compatible with the layout used to create the [`descriptor_update_template`] handle.
- [`set`] is the set number of the descriptor set in the pipeline layout that will be updated. This  **must**  be the same number used to create the [`descriptor_update_template`] handle.
- [`p_data`] is a pointer to memory containing descriptors for the templated update.

# Description
## Valid Usage
-    The `pipelineBindPoint` specified during the creation of the descriptor update template  **must**  be supported by the [`command_buffer`]’s parent [`CommandPool`]’s queue family
-  [`p_data`] **must**  be a valid pointer to a memory containing one or more valid instances of [`DescriptorImageInfo`], [`DescriptorBufferInfo`], or [`BufferView`] in a layout defined by [`descriptor_update_template`] when it was created with [`create_descriptor_update_template`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`descriptor_update_template`] **must**  be a valid [`DescriptorUpdateTemplate`] handle
-  [`layout`] **must**  be a valid [`PipelineLayout`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-    Each of [`command_buffer`], [`descriptor_update_template`], and [`layout`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties
## API example
```c
struct AppDataStructure
{
    VkDescriptorImageInfo  imageInfo;          // a single image info
    // ... some more application related data
};

const VkDescriptorUpdateTemplateEntry descriptorUpdateTemplateEntries[] =
{
    // binding to a single image descriptor
    {
        0,                                           // binding
        0,                                           // dstArrayElement
        1,                                           // descriptorCount
        VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,   // descriptorType
        offsetof(AppDataStructure, imageInfo),       // offset
        0                                            // stride is not required if descriptorCount is 1
    }
};

// create a descriptor update template for push descriptor set updates
const VkDescriptorUpdateTemplateCreateInfo createInfo =
{
    VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,  // sType
    NULL,                                                      // pNext
    0,                                                         // flags
    1,                                                         // descriptorUpdateEntryCount
    descriptorUpdateTemplateEntries,                           // pDescriptorUpdateEntries
    VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR,   // templateType
    0,                                                         // descriptorSetLayout, ignored by given templateType
    VK_PIPELINE_BIND_POINT_GRAPHICS,                           // pipelineBindPoint
    myPipelineLayout,                                          // pipelineLayout
    0,                                                         // set
};

VkDescriptorUpdateTemplate myDescriptorUpdateTemplate;
myResult = vkCreateDescriptorUpdateTemplate(
    myDevice,
    &createInfo,
    NULL,
    &myDescriptorUpdateTemplate);

AppDataStructure appData;
// fill appData here or cache it in your engine
vkCmdPushDescriptorSetWithTemplateKHR(myCmdBuffer, myDescriptorUpdateTemplate, myPipelineLayout, 0,&appData);
```

# Related
- [`khr_descriptor_update_template`]
- [`khr_push_descriptor`]
- [`crate::vulkan1_1`]
- [`CommandBuffer`]
- [`DescriptorUpdateTemplate`]
- [`PipelineLayout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        