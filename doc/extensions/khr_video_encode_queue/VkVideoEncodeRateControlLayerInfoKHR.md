[VkVideoEncodeRateControlLayerInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlLayerInfoKHR.html) - Structure to set encode per-layer rate control parameters

# C Specifications
The [`VideoEncodeRateControlLayerInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_encode_queue
typedef struct VkVideoEncodeRateControlLayerInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           averageBitrate;
    uint32_t           maxBitrate;
    uint32_t           frameRateNumerator;
    uint32_t           frameRateDenominator;
    uint32_t           virtualBufferSizeInMs;
    uint32_t           initialVirtualBufferSizeInMs;
} VkVideoEncodeRateControlLayerInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is a pointer to a structure extending this structure.
- [`average_bitrate`] is the average bitrate in bits/second. Valid when rate control mode is not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
- [`max_bitrate`] is the peak bitrate in bits/second. Valid when rate control mode is `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR`.
- [`frame_rate_numerator`] is the numerator of the frame rate. Valid when rate control mode is not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
- [`frame_rate_denominator`] is the denominator of the frame rate. Valid when rate control mode is not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
- [`virtual_buffer_size_in_ms`] is the leaky bucket model virtual buffer size in milliseconds, with respect to peak bitrate. Valid when rate control mode is not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`. For example, virtual buffer size is ([`virtual_buffer_size_in_ms`] * [`max_bitrate`] / 1000).
- [`initial_virtual_buffer_size_in_ms`] is the initial occupancy in milliseconds of the virtual buffer in the leaky bucket model. Valid when the rate control mode is not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.

# Description
A codec-specific structure specifying additional per-layer rate control
settings  **must**  be chained to [`VideoEncodeRateControlLayerInfoKHR`].
If multiple rate control layers are enabled
([`VideoEncodeRateControlInfoKHR::layer_count`] is greater than 1),
then the chained codec-specific extension structure also identifies the
specific video coding layer its parent
[`VideoEncodeRateControlLayerInfoKHR`] applies to.
If multiple rate control layers are enabled, the number of rate control
layers  **must**  match the number of video coding layers.
The specification for an encode codec-specific extension would describe how
multiple video coding layers are enabled for the corresponding codec.Per-layer rate control settings for all enabled rate control layers  **must**  be
initialized or re-initialized whenever stream rate control settings are
provided via [`VideoEncodeRateControlInfoKHR`].
This is done by specifying settings for all enabled rate control layers in
[`VideoEncodeRateControlInfoKHR::layer_configs`].To adjust rate control settings for an individual layer at runtime, add a
[`VideoEncodeRateControlLayerInfoKHR`] structure to the [`p_next`]
chain of the [`VideoCodingControlInfoKHR`] structure passed to the
[`cmd_control_video_coding_khr`] command.
This adjustment only impacts the specified layer without impacting the rate
control settings or implementation rate control algorithm behavior for any
other enabled rate control layers.
The adjustment takes effect whenever the corresponding
[`cmd_control_video_coding_khr`] is executed, and only impacts
[`cmd_encode_video_khr`] operations pertaining to the corresponding video
coding layer that follow in execution order.It is possible for an application to enable multiple video coding layers
(via codec-specific extensions to encoding operations) while only enabling a
single layer of rate control for the entire video stream.
To achieve this, `layerCount` in [`VideoEncodeRateControlInfoKHR`] **must**  be set to 1, and the single [`VideoEncodeRateControlLayerInfoKHR`]
provided in `pLayerConfigs` would apply to all encoded segments of the
video stream, regardless of which codec-defined video coding layer they
belong to.
In this case, the implementation decides bitrate distribution across video
coding layers (if applicable to the specified stream rate control mode).
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR`

# Related
- [`khr_video_encode_queue`]
- [`StructureType`]
- [`VideoEncodeRateControlInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        