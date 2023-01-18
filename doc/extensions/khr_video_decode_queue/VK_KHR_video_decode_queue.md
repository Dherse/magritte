[VK_KHR_video_decode_queue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_decode_queue.html) - device extension

# Registered extension number
25

# Revision
3

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_video_queue`]`
- Requires `[`VK_KHR_synchronization2`]`
-  **This is a *provisional* extension and  **must**  be used with caution. See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header) of provisional header files for enablement and stability details.**

# Contacts
- [jake.beju@amd.com]()

# New commands
- [`cmd_decode_video_khr`]

# New structures
- [`VideoDecodeInfoKHR`]
- Extending [`VideoCapabilitiesKHR`]:  - [`VideoDecodeCapabilitiesKHR`]

# New enums
- [`VideoDecodeCapabilityFlagBitsKHR`]
- [`VideoDecodeFlagBitsKHR`]

# New bitmasks
- [`VideoDecodeCapabilityFlagsKHR`]
- [`VideoDecodeFlagsKHR`]

# New constants
- [`KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME`]
- [`KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION`]
- Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR`  - `VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR` 
- Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_VIDEO_DECODE_DST_BIT_KHR`  - `VK_BUFFER_USAGE_VIDEO_DECODE_SRC_BIT_KHR` 
- Extending [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_VIDEO_DECODE_DPB_BIT_KHR`  - `VK_FORMAT_FEATURE_VIDEO_DECODE_OUTPUT_BIT_KHR` 
- Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_VIDEO_DECODE_DPB_KHR`  - `VK_IMAGE_LAYOUT_VIDEO_DECODE_DST_KHR`  - `VK_IMAGE_LAYOUT_VIDEO_DECODE_SRC_KHR` 
- Extending [`ImageUsageFlagBits`]:  - `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR`  - `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`  - `VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR` 
- Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR` 
- Extending [`QueueFlagBits`]:  - `VK_QUEUE_VIDEO_DECODE_BIT_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR` 
If [`VK_KHR_format_feature_flags2`] is supported:
- Extending [`FormatFeatureFlagBits2`]:  - `VK_FORMAT_FEATURE_2_VIDEO_DECODE_DPB_BIT_KHR`  - `VK_FORMAT_FEATURE_2_VIDEO_DECODE_OUTPUT_BIT_KHR`

# Version history
- Revision 1, 2018-6-11 (Peter Fang)  - Initial draft 
- Revision 1.5, Nov 09 2018 (Tony Zlatinski)  - API Updates 
- Revision 1.6, Jan 08 2020 (Tony Zlatinski)  - API unify with the video_encode_queue spec 
- Revision 1.7, March 29 2021 (Tony Zlatinski)  - Spec and API updates. 
- Revision 2, September 30 2021 (Jon Leech)  - Add interaction with `[`VK_KHR_format_feature_flags2`]` to `vk.xml` 
- Revision 3, 2022-02-25 (Ahmed Abdelkhalek)  - Add VkVideoDecodeCapabilitiesKHR with new flags to report support for decode DPB and output coinciding in the same image, or in distinct images.

# Other information
* 2022-02-25
* No known IP claims.
*   - Ahmed Abdelkhalek, AMD  - Jake Beju, AMD  - Olivier Lapicque, NVIDIA  - Peter Fang, AMD  - Piers Daniell, NVIDIA  - Srinath Kumarapuram, NVIDIA  - Tony Zlatinski, NVIDIA

# Related
- [`VideoDecodeCapabilitiesKHR`]
- [`VideoDecodeCapabilityFlagBitsKHR`]
- [`VideoDecodeCapabilityFlagsKHR`]
- [`VideoDecodeFlagBitsKHR`]
- [`VideoDecodeFlagsKHR`]
- [`VideoDecodeInfoKHR`]
- [`cmd_decode_video_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        