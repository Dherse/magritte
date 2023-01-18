[`ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ`] specifies read
access to a [depth/stencil attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass), via
[depth or stencil operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-ds-state) or via certain
[subpass load operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops).
Such access occurs in the
`VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT` or
`VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT` pipeline stages.