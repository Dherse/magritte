[VkPipelineDiscardRectangleStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateInfoEXT.html) - Structure specifying discard rectangle

# C Specifications
The [`PipelineDiscardRectangleStateCreateInfoEXT`] structure is defined
as:
```c
// Provided by VK_EXT_discard_rectangles
typedef struct VkPipelineDiscardRectangleStateCreateInfoEXT {
    VkStructureType                                  sType;
    const void*                                      pNext;
    VkPipelineDiscardRectangleStateCreateFlagsEXT    flags;
    VkDiscardRectangleModeEXT                        discardRectangleMode;
    uint32_t                                         discardRectangleCount;
    const VkRect2D*                                  pDiscardRectangles;
} VkPipelineDiscardRectangleStateCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`discard_rectangle_mode`] is a [`DiscardRectangleModeEXT`] value determining whether the discard rectangle test is inclusive or exclusive.
- [`discard_rectangle_count`] is the number of discard rectangles to use.
- [`discard_rectangles`] is a pointer to an array of [`Rect2D`] structures defining discard rectangles.

# Description
If the `VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT` dynamic state is enabled
for a pipeline, the [`discard_rectangles`] member is ignored.When this structure is included in the [`p_next`] chain of
[`GraphicsPipelineCreateInfo`], it defines parameters of the discard
rectangle test.
If this structure is not included in the [`p_next`] chain, it is equivalent
to specifying this structure with a [`discard_rectangle_count`] of `0`.
## Valid Usage
-  [`discard_rectangle_count`] **must**  be less than or equal to [`PhysicalDeviceDiscardRectanglePropertiesEXT::max_discard_rectangles`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT`
-  [`flags`] **must**  be `0`
-  [`discard_rectangle_mode`] **must**  be a valid [`DiscardRectangleModeEXT`] value

# Related
- [`ext_discard_rectangles`]
- [`DiscardRectangleModeEXT`]
- [`PipelineDiscardRectangleStateCreateFlagsEXT`]
- [`Rect2D`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        