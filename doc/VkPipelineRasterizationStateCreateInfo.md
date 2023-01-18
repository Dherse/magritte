[VkPipelineRasterizationStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateInfo.html) - Structure specifying parameters of a newly created pipeline rasterization state

# C Specifications
The [`PipelineRasterizationStateCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineRasterizationStateCreateInfo {
    VkStructureType                            sType;
    const void*                                pNext;
    VkPipelineRasterizationStateCreateFlags    flags;
    VkBool32                                   depthClampEnable;
    VkBool32                                   rasterizerDiscardEnable;
    VkPolygonMode                              polygonMode;
    VkCullModeFlags                            cullMode;
    VkFrontFace                                frontFace;
    VkBool32                                   depthBiasEnable;
    float                                      depthBiasConstantFactor;
    float                                      depthBiasClamp;
    float                                      depthBiasSlopeFactor;
    float                                      lineWidth;
} VkPipelineRasterizationStateCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`depth_clamp_enable`] controls whether to clamp the fragment’s depth values as described in [Depth Test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-depth). If the pipeline is not created with [`PipelineRasterizationDepthClipStateCreateInfoEXT`] present then enabling depth clamp will also disable clipping primitives to the z planes of the frustrum as described in [Primitive Clipping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-clipping). Otherwise depth clipping is controlled by the state set in [`PipelineRasterizationDepthClipStateCreateInfoEXT`].
- [`rasterizer_discard_enable`] controls whether primitives are discarded immediately before the rasterization stage.
- [`polygon_mode`] is the triangle rendering mode. See [`PolygonMode`].
- [`cull_mode`] is the triangle facing direction used for primitive culling. See [`CullModeFlagBits`].
- [`front_face`] is a [`FrontFace`] value specifying the front-facing triangle orientation to be used for culling.
- [`depth_bias_enable`] controls whether to bias fragment depth values.
- [`depth_bias_constant_factor`] is a scalar factor controlling the constant depth value added to each fragment.
- [`depth_bias_clamp`] is the maximum (or minimum) depth bias of a fragment.
- [`depth_bias_slope_factor`] is a scalar factor applied to a fragment’s slope in depth bias calculations.
- [`line_width`] is the width of rasterized line segments.

# Description
The application  **can**  also add a
[`PipelineRasterizationStateRasterizationOrderAMD`] structure to the
[`p_next`] chain of a [`PipelineRasterizationStateCreateInfo`]
structure.
This structure enables selecting the rasterization order to use when
rendering with the corresponding graphics pipeline as described in
[Rasterization Order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-order).
## Valid Usage
-    If the [depth clamping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-depthClamp) feature is not enabled, [`depth_clamp_enable`] **must**  be [`FALSE`]
-    If the [non-solid fill modes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-fillModeNonSolid) feature is not enabled, [`polygon_mode`] **must**  be `VK_POLYGON_MODE_FILL` or `VK_POLYGON_MODE_FILL_RECTANGLE_NV`
-    If the `[`VK_NV_fill_rectangle`]` extension is not enabled, [`polygon_mode`] **must**  not be `VK_POLYGON_MODE_FILL_RECTANGLE_NV`
-    If the `[`VK_KHR_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::point_polygons`] is [`FALSE`], and [`rasterizer_discard_enable`] is [`FALSE`], [`polygon_mode`] **must**  not be `VK_POLYGON_MODE_POINT`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`PipelineRasterizationConservativeStateCreateInfoEXT`], [`PipelineRasterizationDepthClipStateCreateInfoEXT`], [`PipelineRasterizationLineStateCreateInfoEXT`], [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`], [`PipelineRasterizationStateRasterizationOrderAMD`], or [`PipelineRasterizationStateStreamCreateInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be `0`
-  [`polygon_mode`] **must**  be a valid [`PolygonMode`] value
-  [`cull_mode`] **must**  be a valid combination of [`CullModeFlagBits`] values
-  [`front_face`] **must**  be a valid [`FrontFace`] value

# Related
- [`crate::vulkan1_0`]
- [`Bool32`]
- [`CullModeFlags`]
- [`FrontFace`]
- [`GraphicsPipelineCreateInfo`]
- [`PipelineRasterizationStateCreateFlags`]
- [`PolygonMode`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        