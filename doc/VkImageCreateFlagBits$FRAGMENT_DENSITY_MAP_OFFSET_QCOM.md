[`FRAGMENT_DENSITY_MAP_OFFSET_QCOM`] specifies
that an image  **can**  be used in a render pass with non-zero
[fragment density map offsets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapoffsets).
In a renderpass with non-zero offsets, fragment density map attachments,
input attachments, color attachments, depth/stencil attachment, resolve
attachments, and preserve attachments  **must**  be created with
VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM.