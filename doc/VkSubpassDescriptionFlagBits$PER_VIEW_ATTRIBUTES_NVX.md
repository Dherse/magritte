[`PER_VIEW_ATTRIBUTES_NVX`] specifies that
shaders compiled for this subpass write the attributes for all views in
a single invocation of each
[pre-rasterization shader
stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization).
All pipelines compiled against a subpass that includes this bit  **must** 
write per-view attributes to the `*PerViewNV[]` shader outputs, in
addition to the non-per-view (e.g. `Position`) outputs.