[VK_NV_fill_rectangle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_fill_rectangle.html) - device extension

# Description
This extension adds a new [`PolygonMode`]`enum` where a triangle is
rasterized by computing and filling its axis-aligned screen-space bounding
box, disregarding the actual triangle edges.
This can be useful for drawing a rectangle without being split into two
triangles with an internal edge.
It is also useful to minimize the number of primitives that need to be
drawn, particularly for a user interface.

# Registered extension number
154

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_fill_rectangle] @jeffbolznv%0A<<Here describe the issue or question you have about the VK_NV_fill_rectangle extension>>)

# New constants
- [`NV_FILL_RECTANGLE_EXTENSION_NAME`]
- [`NV_FILL_RECTANGLE_SPEC_VERSION`]
- Extending [`PolygonMode`]:  - `VK_POLYGON_MODE_FILL_RECTANGLE_NV`

# Version history
- Revision 1, 2017-05-22 (Jeff Bolz)  - Internal revisions

# Other information
* 2017-05-22
*   - Jeff Bolz, NVIDIA
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        