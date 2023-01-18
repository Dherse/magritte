[`PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS`] specifies the stage
of the pipeline where early fragment tests (depth and stencil tests
before fragment shading) are performed.
This stage also includes [subpass load
operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops) for framebuffer attachments with a depth/stencil format.