[VkCommandBufferInheritanceConditionalRenderingInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html) - Structure specifying command buffer inheritance information

# C Specifications
If the [`p_next`] chain of [`CommandBufferInheritanceInfo`] includes a
[`CommandBufferInheritanceConditionalRenderingInfoEXT`] structure, then
that structure controls whether a command buffer  **can**  be executed while
conditional rendering is [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering) in the
primary command buffer.The [`CommandBufferInheritanceConditionalRenderingInfoEXT`] structure is
defined as:
```c
// Provided by VK_EXT_conditional_rendering
typedef struct VkCommandBufferInheritanceConditionalRenderingInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    VkBool32           conditionalRenderingEnable;
} VkCommandBufferInheritanceConditionalRenderingInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`conditional_rendering_enable`] specifies whether the command buffer  **can**  be executed while conditional rendering is active in the primary command buffer. If this is `VK_TRUE`, then this command buffer  **can**  be executed whether the primary command buffer has active conditional rendering or not. If this is `VK_FALSE`, then the primary command buffer  **must**  not have conditional rendering active.

# Description
If this structure is not present, the behavior is as if
[`conditional_rendering_enable`] is `VK_FALSE`.
## Valid Usage
-    If the [inherited conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-inheritedConditionalRendering) feature is not enabled, [`conditional_rendering_enable`] **must**  be `VK_FALSE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT`

# Related
- [`ext_conditional_rendering`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        