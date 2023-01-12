[VkCommandBufferInheritanceViewportScissorInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceViewportScissorInfoNV.html) - Structure specifying command buffer inheritance information

# C Specifications
The [`CommandBufferInheritanceViewportScissorInfoNV`] structure is
defined as:
```c
// Provided by VK_NV_inherited_viewport_scissor
typedef struct VkCommandBufferInheritanceViewportScissorInfoNV {
    VkStructureType      sType;
    const void*          pNext;
    VkBool32             viewportScissor2D;
    uint32_t             viewportDepthCount;
    const VkViewport*    pViewportDepths;
} VkCommandBufferInheritanceViewportScissorInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`viewport_scissor2_d`] specifies whether the listed dynamic state is inherited.
- [`viewport_depth_count`] specifies the maximum number of viewports to inherit. When [`viewport_scissor2_d`] is `VK_FALSE`, the behavior is as if this value is zero.
- [`viewport_depths`] is a pointer to a [`Viewport`] structure specifying the expected depth range for each inherited viewport.

# Description
If the [`p_next`] chain of [`CommandBufferInheritanceInfo`] includes a
[`CommandBufferInheritanceViewportScissorInfoNV`] structure, then that
structure controls whether a command buffer  **can**  inherit the following state
from other command buffers:
- `VK_DYNAMIC_STATE_SCISSOR`
- `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT`
- `VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT`
as well as the following state, with restrictions on inherited depth values
and viewport count:
- `VK_DYNAMIC_STATE_VIEWPORT`
- `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`
If [`viewport_scissor2_d`] is `VK_FALSE`, then the command buffer does
not inherit the listed dynamic state, and  **should**  set this state itself.
If this structure is not present, the behavior is as if
[`viewport_scissor2_d`] is `VK_FALSE`.If [`viewport_scissor2_d`] is `VK_TRUE`, then the listed dynamic state
is inherited, and the command buffer  **must**  not set this
state, except that the viewport and scissor count  **may**  be set by binding a
graphics pipeline that does not specify this state as dynamic.When the command buffer is executed as part of a the execution of a
[`cmd_execute_commands`] command, the inherited state (if enabled) is
determined by the following procedure, performed separately for each dynamic
state, and separately for each value for dynamic state that consists of
multiple values (e.g. multiple viewports).
- With i being the index of the executed command buffer in the `pCommandBuffers` array of [`cmd_execute_commands`], if i > 0 and any secondary command buffer from index 0 to i-1 modifies the state, the inherited state is provisionally set to the final value set by the last such secondary command buffer. Binding a graphics pipeline defining the state statically is equivalent to setting the state to an undefined value.
- Otherwise, the tentatative inherited state is that of the primary command buffer at the point the [`cmd_execute_commands`] command was recorded; if the state is undefined, then so is the provisional inherited state.
- If the provisional inherited state is an undefined value, then the state is not inherited.
- If the provisional inherited state is a viewport, with n being its viewport index, then if n ≥ [`viewport_depth_count`], or if either [`Viewport::min_depth`] or [`Viewport::max_depth`] are not equal to the respective values of the n<sup>th</sup> element of [`viewport_depths`], then the state is not inherited.
- If the provisional inherited state passes both checks, then it becomes the actual inherited state.

## Valid Usage
-    If the [inherited viewport scissor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-inheritedViewportScissor2D) feature is not enabled, [`viewport_scissor2_d`] **must**  be `VK_FALSE`
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled and [`viewport_scissor2_d`] is `VK_TRUE`, then [`viewport_depth_count`] **must**  be `1`
-    If [`viewport_scissor2_d`] is `VK_TRUE`, then [`viewport_depth_count`] **must**  be greater than `0`
-    If [`viewport_scissor2_d`] is `VK_TRUE`, then [`viewport_depths`] **must**  be a valid pointer to an array of [`viewport_depth_count`] valid [`Viewport`] structures, except any requirements on `x`, `y`, `width`, and `height` do not apply
-    If [`viewport_scissor2_d`] is `VK_TRUE`, then the command buffer  **must**  be recorded with the `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV`

# Related
- [`nv_inherited_viewport_scissor`]
- [`Bool32`]
- [`StructureType`]
- [`Viewport`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        