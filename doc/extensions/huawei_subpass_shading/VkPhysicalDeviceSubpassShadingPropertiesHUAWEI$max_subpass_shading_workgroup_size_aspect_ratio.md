[`max_subpass_shading_workgroup_size_aspect_ratio`] indicates the maximum
ratio between the width and height of the portion of the subpass shading
shader workgroup size.
[`max_subpass_shading_workgroup_size_aspect_ratio`] **must**  be a power-of-two
value, and  **must**  be less than or equal to max(`WorkgroupSize.x` /
`WorkgroupSize.y`, `WorkgroupSize.y` / `WorkgroupSize.x`).