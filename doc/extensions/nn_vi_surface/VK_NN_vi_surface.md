[VK_NN_vi_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NN_vi_surface.html) - instance extension

# Description
The [`VK_NN_vi_surface`] extension is an instance extension.
It provides a mechanism to create a [`SurfaceKHR`] object (defined by
the `[`VK_KHR_surface`]` extension) associated with an
`nn`::`vi`::`Layer`.

# Registered extension number
63

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_surface`]`

# Contacts
- Mathias Heyer mheyer

# New commands
- [`create_vi_surface_nn`]

# New structures
- [`ViSurfaceCreateInfoNN`]

# New bitmasks
- [`ViSurfaceCreateFlagsNN`]

# New constants
- [`NN_VI_SURFACE_EXTENSION_NAME`]
- [`NN_VI_SURFACE_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN`

# Known issues & F.A.Q.
1) Does VI need a way to query for compatibility between a particular
physical device (and queue family?) and a specific VI display? **RESOLVED** : No.
It is currently always assumed that the device and display will always be
compatible.2) [`ViSurfaceCreateInfoNN`]`::pWindow` is intended to store an
`nn`::`vi`::`NativeWindowHandle`, but its declared type is a bare
`void*` to store the window handle.
Why the discrepancy? **RESOLVED** : It is for C compatibility.
The definition for the VI native window handle type is defined inside the
`nn`::`vi` C++ namespace.
This prevents its use in C source files.
`nn`::`vi`::`NativeWindowHandle` is always defined to be
`void*`, so this extension uses `void*` to match.

# Version history
- Revision 1, 2016-12-2 (Michael Chock)  - Initial draft.

# Other information
* 2016-12-02
* No known IP claims.
*   - Mathias Heyer, NVIDIA  - Michael Chock, NVIDIA  - Yasuhiro Yoshioka, Nintendo  - Daniel Koch, NVIDIA

# Related
- [`ViSurfaceCreateFlagsNN`]
- [`ViSurfaceCreateInfoNN`]
- [`create_vi_surface_nn`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        