[`per_view_attributes`] specifies that shaders compiled for this
pipeline write the attributes for all views in a single invocation of
each vertex processing stage.
All pipelines executed within a render pass instance that includes this
bit  **must**  write per-view attributes to the `*PerViewNV[]` shader
outputs, in addition to the non-per-view (e.g. `Position`) outputs.