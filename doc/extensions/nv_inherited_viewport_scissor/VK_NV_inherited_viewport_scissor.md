[VK_NV_inherited_viewport_scissor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_inherited_viewport_scissor.html) - device extension

# Description
This extension adds the ability for a secondary command buffer to inherit
the dynamic viewport and scissor state from a primary command buffer, or a
previous secondary command buffer executed within the same
[`cmd_execute_commands`] call.
It addresses a frequent scenario in applications that deal with window
resizing and want to improve utilization of re-usable secondary command
buffers.
The functionality is provided through
[`CommandBufferInheritanceViewportScissorInfoNV`].
Viewport inheritance is effectively limited to the 2D rectangle; secondary
command buffers must re-specify the inherited depth range values.

# Registered extension number
279

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- David Zhao Akeley [akeley98](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_inherited_viewport_scissor] @akeley98%0A<<Here describe the issue or question you have about the VK_NV_inherited_viewport_scissor extension>>)

# New structures
- Extending [`CommandBufferInheritanceInfo`]:  - [`CommandBufferInheritanceViewportScissorInfoNV`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceInheritedViewportScissorFeaturesNV`]

# New constants
- `VK_NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME`
- `VK_NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV`

# Known issues & F.A.Q.
(1) Why are viewport depth values configured in the
[`CommandBufferInheritanceViewportScissorInfoNV`] struct, rather than by
a `vkCmd…​` function? **DISCUSSION** :We considered both adding a new `vkCmdSetViewportDepthNV` function, and
modifying [`cmd_set_viewport`] to ignore the `x`, `y`,
`width`, and `height` values when called with a secondary command
buffer that activates this extension.The primary design considerations for this extension are debuggability and
easy integration into existing applications.
The main issue with adding a new `vkCmdSetViewportDepthNV` function is
reducing ease-of-integration.
A new function pointer will have to be loaded, but more importantly, a new
function would require changes to be supported in graphics debuggers; this
would delay widespread adoption of the extension.The proposal to modify [`cmd_set_viewport`] would avoid these issues.
However, we expect that the intent of applications using this extension is
to have the viewport values used for drawing exactly match the inherited
values; thus, it would be better for debuggability if no function for
modifying the viewport depth alone is provided.
By specifying viewport depth values when starting secondary command buffer
recording, and requiring the specified depth values to match the inherited
depth values, we allow for validation layers that flag depth changes as
errors.This design also better matches the hardware model.
In fact, there is no need to re-execute a depth-setting command.
The graphics device retains the viewport depth state; it is the CPU-side
state of [`CommandBuffer`] that must be re-initialized.(2) Why are viewport depth values specified as a partial [`Viewport`]
struct, rather than a leaner depth-only struct? **DISCUSSION** :We considered adding a new `VkViewportDepthNV` struct containing only
`minDepth` and `maxDepth`.
However, as application developers would need to maintain both a
[`nv_inherited_viewport_scissor`] code path and a fallback code path (at
least in the short term), we ultimately chose to continue using the existing
[`Viewport`] structure.
Doing so would allow application developers to reuse the same
[`Viewport`] array for both code paths, rather than constructing
separate `VkViewportDepthNV` and [`Viewport`] arrays for each code
path.

# Version history
- Revision 1, 2020-02-04 (David Zhao Akeley)  - Internal revisions

# Other information
* 2021-02-04
*   - David Zhao Akeley, NVIDIA  - Jeff Bolz, NVIDIA  - Piers Daniell, NVIDIA  - Christoph Kubisch, NVIDIA

# Related
- [`CommandBufferInheritanceViewportScissorInfoNV`]
- [`PhysicalDeviceInheritedViewportScissorFeaturesNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        