[VkDescriptorSetLayoutCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateInfo.html) - Structure specifying parameters of a newly created descriptor set layout

# C Specifications
Information about the descriptor set layout is passed in a
[`DescriptorSetLayoutCreateInfo`] structure:
```c
// Provided by VK_VERSION_1_0
typedef struct VkDescriptorSetLayoutCreateInfo {
    VkStructureType                        sType;
    const void*                            pNext;
    VkDescriptorSetLayoutCreateFlags       flags;
    uint32_t                               bindingCount;
    const VkDescriptorSetLayoutBinding*    pBindings;
} VkDescriptorSetLayoutCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`DescriptorSetLayoutCreateFlagBits`] specifying options for descriptor set layout creation.
- [`binding_count`] is the number of elements in [`bindings`].
- [`bindings`] is a pointer to an array of [`DescriptorSetLayoutBinding`] structures.

# Description
## Valid Usage
-    The [`DescriptorSetLayoutBinding::binding`] members of the elements of the [`bindings`] array  **must**  each have different values
-    If [`flags`] contains `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`, then all elements of [`bindings`] **must**  not have a `descriptorType` of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
-    If [`flags`] contains `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`, then all elements of [`bindings`] **must**  not have a `descriptorType` of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`
-    If [`flags`] contains `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`, then the total number of elements of all bindings  **must**  be less than or equal to [`PhysicalDevicePushDescriptorPropertiesKHR::max_push_descriptors`]
-    If [`flags`] contains `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`, [`flags`] **must**  not contain `VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE`
-    If [`flags`] contains `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`, [`bindings`] **must**  not have a `descriptorType` of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
-    If any binding has the `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` bit set, [`flags`] **must**  include `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`
-    If any binding has the `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` bit set, then all bindings  **must**  not have `descriptorType` of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
-    If [`flags`] contains `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`, [`flags`] **must**  not contain `VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE`
-    If any binding has a `descriptorType` of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, then a [`MutableDescriptorTypeCreateInfoVALVE`] **must**  be present in the [`p_next`] chain
-    If a binding has a `descriptorType` value of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, then `pImmutableSamplers` **must**  be `NULL`
-    If [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE::mutable_descriptor_type`] is not enabled, [`bindings`] **must**  not contain a `descriptorType` of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
-    If [`flags`] contains `VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE`, [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE::mutable_descriptor_type`] **must**  be enabled

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`DescriptorSetLayoutBindingFlagsCreateInfo`] or [`MutableDescriptorTypeCreateInfoVALVE`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`DescriptorSetLayoutCreateFlagBits`] values
-    If [`binding_count`] is not `0`, [`bindings`] **must**  be a valid pointer to an array of [`binding_count`] valid [`DescriptorSetLayoutBinding`] structures

# Related
- [`crate::vulkan1_0`]
- [`DescriptorSetLayoutBinding`]
- [VkDescriptorSetLayoutCreateFlags]()
- [`StructureType`]
- [`create_descriptor_set_layout`]
- [`get_descriptor_set_layout_support`]
- [`get_descriptor_set_layout_support_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        