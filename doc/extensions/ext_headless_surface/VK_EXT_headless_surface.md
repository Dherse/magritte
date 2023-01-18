[VK_EXT_headless_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_headless_surface.html) - instance extension

# Description
The [`VK_EXT_headless_surface`] extension is an instance extension.
It provides a mechanism to create [`SurfaceKHR`] objects independently
of any window system or display device.
The presentation operation for a swapchain created from a headless surface
is by default a no-op, resulting in no externally-visible result.Because there is no real presentation target, future extensions can layer on
top of the headless surface to introduce arbitrary or customisable sets of
restrictions or features.
These could include features like saving to a file or restrictions to
emulate a particular presentation target.This functionality is expected to be useful for application and driver
development because it allows any platform to expose an arbitrary or
customisable set of restrictions and features of a presentation engine.
This makes it a useful portable test target for applications targeting a
wide range of presentation engines where the actual target presentation
engines might be scarce, unavailable or otherwise undesirable or
inconvenient to use for general Vulkan application development.

# Registered extension number
257

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_surface`]`

# Contacts
- Lisa Wu [chengtianww](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_headless_surface] @chengtianww%0A<<Here describe the issue or question you have about the VK_EXT_headless_surface extension>>)

# New commands
- [`create_headless_surface_ext`]

# New structures
- [`HeadlessSurfaceCreateInfoEXT`]

# New bitmasks
- [`HeadlessSurfaceCreateFlagsEXT`]

# New constants
- [`EXT_HEADLESS_SURFACE_EXTENSION_NAME`]
- [`EXT_HEADLESS_SURFACE_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT`

# Version history
- Revision 1, 2019-03-21 (Ray Smith)  - Initial draft

# Other information
* 2019-03-21
* No known IP claims.
*   - Ray Smith, Arm

# Related
- [`HeadlessSurfaceCreateFlagsEXT`]
- [`HeadlessSurfaceCreateInfoEXT`]
- [`create_headless_surface_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        