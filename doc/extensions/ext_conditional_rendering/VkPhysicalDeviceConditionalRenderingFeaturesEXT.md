[VkPhysicalDeviceConditionalRenderingFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html) - Structure describing if a secondary command buffer can be executed if conditional rendering is active in the primary command buffer

# C Specifications
The [`PhysicalDeviceConditionalRenderingFeaturesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_conditional_rendering
typedef struct VkPhysicalDeviceConditionalRenderingFeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           conditionalRendering;
    VkBool32           inheritedConditionalRendering;
} VkPhysicalDeviceConditionalRenderingFeaturesEXT;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`conditional_rendering`] specifies whether conditional rendering is supported.
- [`inherited_conditional_rendering`] specifies whether a secondary command buffer  **can**  be executed while conditional rendering is active in the primary command buffer.
If the [`PhysicalDeviceConditionalRenderingFeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceConditionalRenderingFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT`

# Related
- [`VK_EXT_conditional_rendering`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        