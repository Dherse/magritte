[VkVideoEncodeRateControlInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlInfoKHR.html) - Structure to set encode stream rate control parameters

# C Specifications
The [`VideoEncodeRateControlInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_encode_queue
typedef struct VkVideoEncodeRateControlInfoKHR {
    VkStructureType                                sType;
    const void*                                    pNext;
    VkVideoEncodeRateControlFlagsKHR               flags;
    VkVideoEncodeRateControlModeFlagBitsKHR        rateControlMode;
    uint8_t                                        layerCount;
    const VkVideoEncodeRateControlLayerInfoKHR*    pLayerConfigs;
} VkVideoEncodeRateControlInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`VideoEncodeRateControlFlagBitsKHR`] specifying encode rate control flags.
- [`rate_control_mode`] is a [`VideoEncodeRateControlModeFlagBitsKHR`] value specifying the encode stream rate control mode.
- [`layer_count`] specifies the number of rate control layers in the video encode stream.
- [`layer_configs`] is a pointer to an array of [`VideoEncodeRateControlLayerInfoKHR`] structures specifying the rate control configurations of [`layer_count`] rate control layers.

# Description
In order to provide video encode stream rate control settings, add a
[`VideoEncodeRateControlInfoKHR`] structure to the [`p_next`] chain of
the [`VideoCodingControlInfoKHR`] structure passed to the
[`cmd_control_video_coding_khr`] command.A codec-specific extension structure for further encode stream rate control
parameter settings  **may**  be chained to [`VideoEncodeRateControlInfoKHR`].To ensure that the video session is properly initalized with stream-level
rate control settings, the application  **must**  call
[`cmd_control_video_coding_khr`] with stream-level rate control settings at
least once in execution order before the first [`cmd_encode_video_khr`]
command that is executed after video session reset.
If not provided, default implementation-specific stream rate control
settings will be used.Stream rate control settings  **can**  also be re-initialized during an active
video encoding session.
The re-initialization takes effect whenever the
[`VideoEncodeRateControlInfoKHR`] structure is included in the
[`p_next`] chain of the [`VideoCodingControlInfoKHR`] structure in the
call to [`cmd_control_video_coding_khr`], and only impacts
[`cmd_encode_video_khr`] operations that follow in execution order.
## Valid Usage
-  [`VideoEncodeH264RateControlInfoEXT`] **must**  be included in the [`p_next`] chain of [`VideoEncodeRateControlInfoKHR`] if and only if [`VideoEncodeRateControlInfoKHR`]::[`rate_control_mode`] is not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR` and the bound video session was created with [`VideoProfileKHR::video_codec_operation`] set to `VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT`.
-    If [`VideoEncodeRateControlInfoKHR`]::[`layer_count`] is greater than `1`, then [`VideoEncodeH264RateControlInfoEXT::temporal_layer_count`] **must**  be equal to [`layer_count`].
-  [`VideoEncodeH265RateControlInfoEXT`] **must**  be included in the [`p_next`] chain of [`VideoEncodeRateControlInfoKHR`] if and only if [`VideoEncodeRateControlInfoKHR`]::[`rate_control_mode`] is not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR` and the bound video session was created with [`VideoProfileKHR::video_codec_operation`] set to `VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_EXT`.
-    If [`VideoEncodeRateControlInfoKHR`]::[`layer_count`] is greater than `1`, then [`VideoEncodeH265RateControlInfoEXT::sub_layer_count`] **must**  be equal to [`layer_count`].

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_INFO_KHR`
-  [`rate_control_mode`] **must**  be a valid [`VideoEncodeRateControlModeFlagBitsKHR`] value
-  [`layer_configs`] **must**  be a valid pointer to an array of [`layer_count`] valid [`VideoEncodeRateControlLayerInfoKHR`] structures
-  [`layer_count`] **must**  be greater than `0`

# Related
- [`VK_KHR_video_encode_queue`]
- [`StructureType`]
- [`VideoEncodeRateControlFlagsKHR`]
- [`VideoEncodeRateControlLayerInfoKHR`]
- [`VideoEncodeRateControlModeFlagBitsKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        