[`PIPELINE_STAGE2_LATE_FRAGMENT_TESTS`] specifies the stage of
the pipeline where late fragment tests (depth and stencil tests after
fragment shading) are performed.
This stage also includes [subpass store
operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops) for framebuffer attachments with a depth/stencil format.