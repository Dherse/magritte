[`TRANSFER_SRC_OPTIMAL`] **must**  only be used as a
source image of a transfer command (see the definition of
[`VK_PIPELINE_STAGE_TRANSFER_BIT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-transfer)).
This layout is valid only for image subresources of images created with
the `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` usage bit enabled.