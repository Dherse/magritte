[`COLOR_ATTACHMENT_READ`] specifies read access to a
[color attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass), such as via [blending](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blending), [logic operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-logicop), or via certain
[subpass load operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops).
It does not include [advanced blend
operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced).
Such access occurs in the
`VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage.