[`TRIANGLE_FLIP_FACING`] indicates that
the [facing determination](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-culling-face) for geometry in
this instance is inverted.
Because the facing is determined in object space, an instance transform
does not change the winding, but a geometry transform does.