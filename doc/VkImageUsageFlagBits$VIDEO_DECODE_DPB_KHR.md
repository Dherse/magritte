[`VIDEO_DECODE_DPB_KHR`] specifies that
[video decode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations) **can**  use the image
as a [DPB Video Picture Resource](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources),
representing a [reference picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture).
If an implementation requires separate allocations for DPB and decode
output, indicating this by returning `VK_ERROR_FORMAT_NOT_SUPPORTED`
to any [`get_physical_device_video_format_properties_khr`] call with both
[`VIDEO_DECODE_DPB_KHR`] and
[`VIDEO_DECODE_DST_KHR`] usage bits, then
[`VIDEO_DECODE_DPB_KHR`] **must**  not be combined with
any other VK_IMAGE_USAGE_* flags.
Otherwise, [`VIDEO_DECODE_DPB_KHR`] **must**  be
combined with [`VIDEO_DECODE_DST_KHR`], if the DPB
image is required to coincide with the decoded output picture.
In the case where DPB coincides with the decoded output picture, image
resources  **can**  be used as [reference pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture) only
after acting as targets for video decode operations, where its image
view  **must**  be set to both
[`VideoDecodeInfoKHR`]::`pSetupReferenceSlot->pPictureResource->imageViewBinding`
and
[`VideoDecodeInfoKHR`]::`dstPictureResource.imageViewBinding`.