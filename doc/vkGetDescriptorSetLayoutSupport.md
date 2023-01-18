[vkGetDescriptorSetLayoutSupport](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupport.html) - Query whether a descriptor set layout can be created

# C Specifications
To query information about whether a descriptor set layout  **can**  be created,
call:
```c
// Provided by VK_VERSION_1_1
void vkGetDescriptorSetLayoutSupport(
    VkDevice                                    device,
    const VkDescriptorSetLayoutCreateInfo*      pCreateInfo,
    VkDescriptorSetLayoutSupport*               pSupport);
```
or the equivalent command
```c
// Provided by VK_KHR_maintenance3
void vkGetDescriptorSetLayoutSupportKHR(
    VkDevice                                    device,
    const VkDescriptorSetLayoutCreateInfo*      pCreateInfo,
    VkDescriptorSetLayoutSupport*               pSupport);
```

# Parameters
- [`device`] is the logical device that would create the descriptor set layout.
- [`p_create_info`] is a pointer to a [`DescriptorSetLayoutCreateInfo`] structure specifying the state of the descriptor set layout object.
- [`p_support`] is a pointer to a [`DescriptorSetLayoutSupport`] structure, in which information about support for the descriptor set layout object is returned.

# Description
Some implementations have limitations on what fits in a descriptor set which
are not easily expressible in terms of existing limits like
`maxDescriptorSet`*, for example if all descriptor types share a limited
space in memory but each descriptor is a different size or alignment.
This command returns information about whether a descriptor set satisfies
this limit.
If the descriptor set layout satisfies the
[`PhysicalDeviceMaintenance3Properties::max_per_set_descriptors`]
limit, this command is guaranteed to return [`TRUE`] in
[`DescriptorSetLayoutSupport::supported`].
If the descriptor set layout exceeds the
[`PhysicalDeviceMaintenance3Properties::max_per_set_descriptors`]
limit, whether the descriptor set layout is supported is
implementation-dependent and  **may**  depend on whether the descriptor sizes and
alignments cause the layout to exceed an internal limit.This command does not consider other limits such as
`maxPerStageDescriptor`*, and so a descriptor set layout that is
supported according to this command  **must**  still satisfy the pipeline layout
limits such as `maxPerStageDescriptor`* in order to be used in a
pipeline layout.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`DescriptorSetLayoutCreateInfo`] structure
-  [`p_support`] **must**  be a valid pointer to a [`DescriptorSetLayoutSupport`] structure

# Related
- [`crate::vulkan1_1`]
- [`DescriptorSetLayoutCreateInfo`]
- [`DescriptorSetLayoutSupport`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        