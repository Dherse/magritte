[VkPipelineViewportDepthClipControlCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportDepthClipControlCreateInfoEXT.html) - Structure specifying parameters of a newly created pipeline depth clip control state

# C Specifications
The [`PipelineViewportDepthClipControlCreateInfoEXT`] structure is
defined as:
```c
// Provided by VK_EXT_depth_clip_control
typedef struct VkPipelineViewportDepthClipControlCreateInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    VkBool32           negativeOneToOne;
} VkPipelineViewportDepthClipControlCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`negative_one_to_one`] sets the z<sub>m</sub> in the *view volume* to -w<sub>c</sub>

# Description
## Valid Usage
-    If [depthClipControl](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-depthClipControl) is not enabled, [`negative_one_to_one`] **must**  be `VK_FALSE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT`

# Related
- [`ext_depth_clip_control`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        