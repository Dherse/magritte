[vkUpdateDescriptorSetWithTemplate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html) - Update the contents of a descriptor set object using an update template

# C Specifications
Once a [`DescriptorUpdateTemplate`] has been created, descriptor sets
 **can**  be updated by calling:
```c
// Provided by VK_VERSION_1_1
void vkUpdateDescriptorSetWithTemplate(
    VkDevice                                    device,
    VkDescriptorSet                             descriptorSet,
    VkDescriptorUpdateTemplate                  descriptorUpdateTemplate,
    const void*                                 pData);
```
or the equivalent command
```c
// Provided by VK_KHR_descriptor_update_template
void vkUpdateDescriptorSetWithTemplateKHR(
    VkDevice                                    device,
    VkDescriptorSet                             descriptorSet,
    VkDescriptorUpdateTemplate                  descriptorUpdateTemplate,
    const void*                                 pData);
```

# Parameters
- [`device`] is the logical device that updates the descriptor set.
- [`descriptor_set`] is the descriptor set to update
- [`descriptor_update_template`] is a [`DescriptorUpdateTemplate`] object specifying the update mapping between [`p_data`] and the descriptor set to update.
- [`p_data`] is a pointer to memory containing one or more     [`DescriptorImageInfo`], [`DescriptorBufferInfo`], or     [`BufferView`] structures or [`AccelerationStructureKHR`] or [`AccelerationStructureNV`] handles     used to write the descriptors.

# Description
## Valid Usage
-  [`p_data`] **must**  be a valid pointer to a memory containing one or more valid instances of [`DescriptorImageInfo`], [`DescriptorBufferInfo`], or [`BufferView`] in a layout defined by [`descriptor_update_template`] when it was created with [`create_descriptor_update_template`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`descriptor_set`] **must**  be a valid [`DescriptorSet`] handle
-  [`descriptor_update_template`] **must**  be a valid [`DescriptorUpdateTemplate`] handle
-  [`descriptor_update_template`] **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`descriptor_set`] **must**  be externally synchronized

## API example
```c
struct AppBufferView {
    VkBufferView bufferView;
    uint32_t     applicationRelatedInformation;
};

struct AppDataStructure
{
    VkDescriptorImageInfo  imageInfo;          // a single image info
    VkDescriptorBufferInfo bufferInfoArray[3]; // 3 buffer infos in an array
    AppBufferView          bufferView[2];      // An application defined structure containing a bufferView
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
    },

    // binding to an array of buffer descriptors
    {
        1,                                           // binding
        0,                                           // dstArrayElement
        3,                                           // descriptorCount
        VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,           // descriptorType
        offsetof(AppDataStructure, bufferInfoArray), // offset
        sizeof(VkDescriptorBufferInfo)               // stride, descriptor buffer infos are compact
    },

    // binding to an array of buffer views
    {
        2,                                           // binding
        0,                                           // dstArrayElement
        2,                                           // descriptorCount
        VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER,     // descriptorType
        offsetof(AppDataStructure, bufferView) +
          offsetof(AppBufferView, bufferView),       // offset
        sizeof(AppBufferView)                        // stride, bufferViews do not have to be compact
    },
};

// create a descriptor update template for descriptor set updates
const VkDescriptorUpdateTemplateCreateInfo createInfo =
{
    VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,  // sType
    NULL,                                                      // pNext
    0,                                                         // flags
    3,                                                         // descriptorUpdateEntryCount
    descriptorUpdateTemplateEntries,                           // pDescriptorUpdateEntries
    VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET,         // templateType
    myLayout,                                                  // descriptorSetLayout
    0,                                                         // pipelineBindPoint, ignored by given templateType
    0,                                                         // pipelineLayout, ignored by given templateType
    0,                                                         // set, ignored by given templateType
};

VkDescriptorUpdateTemplate myDescriptorUpdateTemplate;
myResult = vkCreateDescriptorUpdateTemplate(
    myDevice,
    &createInfo,
    NULL,
    &myDescriptorUpdateTemplate);

AppDataStructure appData;

// fill appData here or cache it in your engine
vkUpdateDescriptorSetWithTemplate(myDevice, myDescriptorSet, myDescriptorUpdateTemplate, &appData);
```

# Related
- [`crate::vulkan1_1`]
- [`DescriptorSet`]
- [`DescriptorUpdateTemplate`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        