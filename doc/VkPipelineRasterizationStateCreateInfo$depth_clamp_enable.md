[`depth_clamp_enable`] controls whether to clamp the fragment’s depth
values as described in [Depth Test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-depth).
If the pipeline is not created with
[`PipelineRasterizationDepthClipStateCreateInfoEXT`] present then
enabling depth clamp will also disable clipping primitives to the z
planes of the frustrum as described in [Primitive Clipping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-clipping).
Otherwise depth clipping is controlled by the state set in
[`PipelineRasterizationDepthClipStateCreateInfoEXT`].