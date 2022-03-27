use crate::{
    extensions::khr_video_queue::{VideoPictureResourceKHR, VideoReferenceSlotKHR},
    vulkan1_0::{BaseInStructure, Buffer, DeviceSize, Extent2D, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_video_encode_queue");
///[VkVideoEncodeInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeInfoKHR.html) - Structure to chain codec-specific structures to
///# C Specifications
///The [`VideoEncodeInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef struct VkVideoEncodeInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkVideoEncodeFlagsKHR             flags;
///    uint32_t                          qualityLevel;
///    VkExtent2D                        codedExtent;
///    VkBuffer                          dstBitstreamBuffer;
///    VkDeviceSize                      dstBitstreamBufferOffset;
///    VkDeviceSize                      dstBitstreamBufferMaxRange;
///    VkVideoPictureResourceKHR         srcPictureResource;
///    const VkVideoReferenceSlotKHR*    pSetupReferenceSlot;
///    uint32_t                          referenceSlotCount;
///    const VkVideoReferenceSlotKHR*    pReferenceSlots;
///    uint32_t                          precedingExternallyEncodedBytes;
///} VkVideoEncodeInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is a pointer to a structure extending this structure. A codec-specific extension
///   structure **must** be chained to specify what bitstream unit to generate with this encode
///   operation.
/// - [`flags`] is a bitmask of [`VideoEncodeFlagBitsKHR`] specifying encode flags, and is reserved
///   for future versions of this specification.
/// - [`quality_level`] is the coding quality level of the encoding. It is defined by the
///   codec-specific extensions.
/// - [`coded_extent`] is the coded size of the encode operations.
/// - [`dst_bitstream_buffer`] is the buffer where the encoded bitstream output will be produced.
/// - [`dst_bitstream_buffer_offset`] is the offset in the [`dst_bitstream_buffer`] where the
///   encoded bitstream output will start. [`dst_bitstream_buffer_offset`]’s value **must** be
///   aligned to [`VideoCapabilitiesKHR::min_bitstream_buffer_offset_alignment`], as reported by the
///   implementation.
/// - [`dst_bitstream_buffer_max_range`] is the maximum size of the [`dst_bitstream_buffer`] that
///   can be used while the encoded bitstream output is produced.
///   [`dst_bitstream_buffer_max_range`]’s value **must** be aligned to
///   [`VideoCapabilitiesKHR::min_bitstream_buffer_size_alignment`], as reported by the
///   implementation.
/// - [`src_picture_resource`] is the Picture Resource of the [Input Picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#input-encode-picture)
///   to be encoded by the operation.
/// - [`p_setup_reference_slot`] is a pointer to a [`VideoReferenceSlotKHR`] structure used for
///   generating a reconstructed reference slot and Picture Resource.
///   `pSetupReferenceSlot->slotIndex` specifies the slot index number to use as a target for
///   producing the Reconstructed (DPB) data. [`p_setup_reference_slot`]**must** be one of the
///   entries provided in [`VideoBeginCodingInfoKHR`] via the [`p_reference_slots`] within the
///   [`CmdBeginVideoCodingKHR`] command that established the Vulkan Video Encode Context for this
///   command.
/// - [`reference_slot_count`] is the number of Reconstructed Reference Pictures that will be used
///   when this encoding operation is executing.
/// - [`p_reference_slots`] is `NULL` or a pointer to an array of [`VideoReferenceSlotKHR`]
///   structures that will be used when this encoding operation is executing. Each entry in
///   [`p_reference_slots`]**must** be one of the entries provided in [`VideoBeginCodingInfoKHR`]
///   via the [`p_reference_slots`] within the [`CmdBeginVideoCodingKHR`] command that established
///   the Vulkan Video Encode Context for this command.
/// - [`preceding_externally_encoded_bytes`] is the number of bytes externally encoded for insertion
///   in the active video encode session overall bitstream prior to the bitstream that will be
///   generated by the implementation for this instance of [`VideoEncodeInfoKHR`]. Valid when
///   [`VideoEncodeRateControlInfoKHR::rate_control_mode`] is not
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`. The value provided is used to update the
///   implementation’s rate control algorithm for the rate control layer this instance of
///   [`VideoEncodeInfoKHR`] belongs to, by accounting for the bitrate budget consumed by these
///   externally encoded bytes. See [`VideoEncodeRateControlInfoKHR`] for additional information
///   about encode rate control.
///# Description
///Multiple [`CmdEncodeVideoKHR`] commands **may** be recorded within a Vulkan
///Video Encode Context.
///The execution of each [`CmdEncodeVideoKHR`] command will result in
///generating codec-specific bitstream units.
///These bitstream units are generated consecutively into the bitstream buffer
///specified in [`dst_bitstream_buffer`] of a [`VideoEncodeInfoKHR`]
///structure within the [`CmdBeginVideoCodingKHR`] command.
///The produced bitstream is the sum of all these bitstream units, including
///any padding between the bitstream units.
///Any bitstream padding **must** be filled with data compliant to the codec
///standard so as not to cause any syntax errors during decoding of the
///bitstream units with the padding included.
///The range of the bitstream buffer written **can** be queried via
///[video encode bitstream buffer
///range queries](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-video-encode-bitstream-buffer-range).Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`VideoEncodeH264EmitPictureParametersEXT`], [`VideoEncodeH264VclFrameInfoEXT`],
///   [`VideoEncodeH265EmitPictureParametersEXT`], or [`VideoEncodeH265VclFrameInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`VideoEncodeFlagBitsKHR`] values
/// - [`dst_bitstream_buffer`]**must** be a valid [`Buffer`] handle
/// - [`src_picture_resource`]**must** be a valid [`VideoPictureResourceKHR`] structure
/// - [`p_setup_reference_slot`]**must** be a valid pointer to a valid [`VideoReferenceSlotKHR`]
///   structure
/// - If [`reference_slot_count`] is not `0`, [`p_reference_slots`]**must** be a valid pointer to an
///   array of [`reference_slot_count`] valid [`VideoReferenceSlotKHR`] structures
///# Related
/// - [`VK_KHR_video_encode_queue`]
/// - [`Buffer`]
/// - [`DeviceSize`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`VideoEncodeFlagsKHR`]
/// - [`VideoPictureResourceKHR`]
/// - [`VideoReferenceSlotKHR`]
/// - [`CmdEncodeVideoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoEncodeInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is a pointer to a structure extending this structure.
    ///A codec-specific extension structure **must** be chained to specify what
    ///bitstream unit to generate with this encode operation.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeFlagBitsKHR`] specifying
    ///encode flags, and is reserved for future versions of this specification.
    flags: VideoEncodeFlagsKHR,
    ///[`quality_level`] is the coding quality level of the encoding.
    ///It is defined by the codec-specific extensions.
    quality_level: u32,
    ///[`coded_extent`] is the coded size of the encode operations.
    coded_extent: Extent2D,
    ///[`dst_bitstream_buffer`] is the buffer where the encoded bitstream
    ///output will be produced.
    dst_bitstream_buffer: Buffer,
    ///[`dst_bitstream_buffer_offset`] is the offset in the
    ///[`dst_bitstream_buffer`] where the encoded bitstream output will start.
    ///[`dst_bitstream_buffer_offset`]’s value **must** be aligned to
    ///[`VideoCapabilitiesKHR`]::`minBitstreamBufferOffsetAlignment`,
    ///as reported by the implementation.
    dst_bitstream_buffer_offset: DeviceSize,
    ///[`dst_bitstream_buffer_max_range`] is the maximum size of the
    ///[`dst_bitstream_buffer`] that can be used while the encoded bitstream
    ///output is produced.
    ///[`dst_bitstream_buffer_max_range`]’s value **must** be aligned to
    ///[`VideoCapabilitiesKHR`]::`minBitstreamBufferSizeAlignment`, as
    ///reported by the implementation.
    dst_bitstream_buffer_max_range: DeviceSize,
    ///[`src_picture_resource`] is the Picture Resource of the
    ///[Input Picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#input-encode-picture) to be encoded by the operation.
    src_picture_resource: VideoPictureResourceKHR<'lt>,
    ///[`p_setup_reference_slot`] is a pointer to a
    ///[`VideoReferenceSlotKHR`] structure used for generating a
    ///reconstructed reference slot and Picture Resource.
    ///`pSetupReferenceSlot->slotIndex` specifies the slot index number to
    ///use as a target for producing the Reconstructed (DPB) data.
    ///[`p_setup_reference_slot`]**must** be one of the entries provided in
    ///[`VideoBeginCodingInfoKHR`] via the [`p_reference_slots`] within the
    ///[`CmdBeginVideoCodingKHR`] command that established the Vulkan Video
    ///Encode Context for this command.
    p_setup_reference_slot: *mut VideoReferenceSlotKHR<'lt>,
    ///[`reference_slot_count`] is the number of Reconstructed Reference
    ///Pictures that will be used when this encoding operation is executing.
    reference_slot_count: u32,
    ///[`p_reference_slots`] is `NULL` or a pointer to an array of
    ///[`VideoReferenceSlotKHR`] structures that will be used when this
    ///encoding operation is executing.
    ///Each entry in [`p_reference_slots`]**must** be one of the entries provided
    ///in [`VideoBeginCodingInfoKHR`] via the [`p_reference_slots`] within
    ///the [`CmdBeginVideoCodingKHR`] command that established the Vulkan
    ///Video Encode Context for this command.
    p_reference_slots: *mut VideoReferenceSlotKHR<'lt>,
    ///[`preceding_externally_encoded_bytes`] is the number of bytes externally
    ///encoded for insertion in the active video encode session overall
    ///bitstream prior to the bitstream that will be generated by the
    ///implementation for this instance of [`VideoEncodeInfoKHR`].
    ///Valid when [`VideoEncodeRateControlInfoKHR`]::`rateControlMode`
    ///is not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    ///The value provided is used to update the implementation’s rate control
    ///algorithm for the rate control layer this instance of
    ///[`VideoEncodeInfoKHR`] belongs to, by accounting for the bitrate
    ///budget consumed by these externally encoded bytes.
    ///See [`VideoEncodeRateControlInfoKHR`] for additional information
    ///about encode rate control.
    preceding_externally_encoded_bytes: u32,
}
///[VkVideoEncodeRateControlInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlInfoKHR.html) - Structure to set encode stream rate control parameters
///# C Specifications
///The [`VideoEncodeRateControlInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef struct VkVideoEncodeRateControlInfoKHR {
///    VkStructureType                                sType;
///    const void*                                    pNext;
///    VkVideoEncodeRateControlFlagsKHR               flags;
///    VkVideoEncodeRateControlModeFlagBitsKHR        rateControlMode;
///    uint8_t                                        layerCount;
///    const VkVideoEncodeRateControlLayerInfoKHR*    pLayerConfigs;
///} VkVideoEncodeRateControlInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`VideoEncodeRateControlFlagBitsKHR`] specifying encode rate control
///   flags.
/// - [`rate_control_mode`] is a [`VideoEncodeRateControlModeFlagBitsKHR`] value specifying the
///   encode stream rate control mode.
/// - [`layer_count`] specifies the number of rate control layers in the video encode stream.
/// - [`p_layer_configs`] is a pointer to an array of [`VideoEncodeRateControlLayerInfoKHR`]
///   structures specifying the rate control configurations of [`layer_count`] rate control layers.
///# Description
///In order to provide video encode stream rate control settings, add a
///[`VideoEncodeRateControlInfoKHR`] structure to the [`p_next`] chain of
///the [`VideoCodingControlInfoKHR`] structure passed to the
///[`CmdControlVideoCodingKHR`] command.A codec-specific extension structure for further encode
/// stream rate control
///parameter settings **may** be chained to [`VideoEncodeRateControlInfoKHR`].To ensure that the
/// video session is properly initalized with stream-level
///rate control settings, the application **must** call
///[`CmdControlVideoCodingKHR`] with stream-level rate control settings at
///least once in execution order before the first [`CmdEncodeVideoKHR`]
///command that is executed after video session reset.
///If not provided, default implementation-specific stream rate control
///settings will be used.Stream rate control settings **can** also be re-initialized during an
/// active
///video encoding session.
///The re-initialization takes effect whenever the
///[`VideoEncodeRateControlInfoKHR`] structure is included in the
///[`p_next`] chain of the [`VideoCodingControlInfoKHR`] structure in the
///call to [`CmdControlVideoCodingKHR`], and only impacts
///[`CmdEncodeVideoKHR`] operations that follow in execution order.Valid Usage
/// - [`VideoEncodeH264RateControlInfoEXT`]**must** be included in the [`p_next`] chain of
///   [`VideoEncodeRateControlInfoKHR`] if and only if
///   [`VideoEncodeRateControlInfoKHR`]::[`rate_control_mode`] is not
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR` and the bound video session was created with
///   [`VideoProfileKHR::video_codec_operation`] set to
///   `VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT`.
/// - If [`VideoEncodeRateControlInfoKHR`]::[`layer_count`] is greater than `1`, then
///   [`VideoEncodeH264RateControlInfoEXT::temporal_layer_count`]**must** be equal to
///   [`layer_count`].
/// - [`VideoEncodeH265RateControlInfoEXT`]**must** be included in the [`p_next`] chain of
///   [`VideoEncodeRateControlInfoKHR`] if and only if
///   [`VideoEncodeRateControlInfoKHR`]::[`rate_control_mode`] is not
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR` and the bound video session was created with
///   [`VideoProfileKHR::video_codec_operation`] set to
///   `VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_EXT`.
/// - If [`VideoEncodeRateControlInfoKHR`]::[`layer_count`] is greater than `1`, then
///   [`VideoEncodeH265RateControlInfoEXT::sub_layer_count`]**must** be equal to [`layer_count`].
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_INFO_KHR`
/// - [`rate_control_mode`]**must** be a valid [`VideoEncodeRateControlModeFlagBitsKHR`] value
/// - [`p_layer_configs`]**must** be a valid pointer to an array of [`layer_count`] valid
///   [`VideoEncodeRateControlLayerInfoKHR`] structures
/// - [`layer_count`]**must** be greater than `0`
///# Related
/// - [`VK_KHR_video_encode_queue`]
/// - [`StructureType`]
/// - [`VideoEncodeRateControlFlagsKHR`]
/// - [`VideoEncodeRateControlLayerInfoKHR`]
/// - [`VideoEncodeRateControlModeFlagBitsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoEncodeRateControlInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeRateControlFlagBitsKHR`]
    ///specifying encode rate control flags.
    flags: VideoEncodeRateControlFlagsKHR,
    ///[`rate_control_mode`] is a [`VideoEncodeRateControlModeFlagBitsKHR`]
    ///value specifying the encode stream rate control mode.
    rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
    ///[`layer_count`] specifies the number of rate control layers in the
    ///video encode stream.
    layer_count: u8,
    ///[`p_layer_configs`] is a pointer to an array of
    ///[`VideoEncodeRateControlLayerInfoKHR`] structures specifying the
    ///rate control configurations of [`layer_count`] rate control layers.
    p_layer_configs: *mut VideoEncodeRateControlLayerInfoKHR<'lt>,
}
///[VkVideoEncodeRateControlLayerInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlLayerInfoKHR.html) - Structure to set encode per-layer rate control parameters
///# C Specifications
///The [`VideoEncodeRateControlLayerInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef struct VkVideoEncodeRateControlLayerInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           averageBitrate;
///    uint32_t           maxBitrate;
///    uint32_t           frameRateNumerator;
///    uint32_t           frameRateDenominator;
///    uint32_t           virtualBufferSizeInMs;
///    uint32_t           initialVirtualBufferSizeInMs;
///} VkVideoEncodeRateControlLayerInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is a pointer to a structure extending this structure.
/// - [`average_bitrate`] is the average bitrate in bits/second. Valid when rate control mode is not
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
/// - [`max_bitrate`] is the peak bitrate in bits/second. Valid when rate control mode is
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR`.
/// - [`frame_rate_numerator`] is the numerator of the frame rate. Valid when rate control mode is
///   not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
/// - [`frame_rate_denominator`] is the denominator of the frame rate. Valid when rate control mode
///   is not `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
/// - [`virtual_buffer_size_in_ms`] is the leaky bucket model virtual buffer size in milliseconds,
///   with respect to peak bitrate. Valid when rate control mode is not
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`. For example, virtual buffer size is
///   ([`virtual_buffer_size_in_ms`] * [`max_bitrate`] / 1000).
/// - [`initial_virtual_buffer_size_in_ms`] is the initial occupancy in milliseconds of the virtual
///   buffer in the leaky bucket model. Valid when the rate control mode is not
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
///# Description
///A codec-specific structure specifying additional per-layer rate control
///settings **must** be chained to [`VideoEncodeRateControlLayerInfoKHR`].
///If multiple rate control layers are enabled
///([`VideoEncodeRateControlInfoKHR::layer_count`] is greater than 1),
///then the chained codec-specific extension structure also identifies the
///specific video coding layer its parent
///[`VideoEncodeRateControlLayerInfoKHR`] applies to.
///If multiple rate control layers are enabled, the number of rate control
///layers **must** match the number of video coding layers.
///The specification for an encode codec-specific extension would describe how
///multiple video coding layers are enabled for the corresponding codec.Per-layer rate control
/// settings for all enabled rate control layers **must** be
///initialized or re-initialized whenever stream rate control settings are
///provided via [`VideoEncodeRateControlInfoKHR`].
///This is done by specifying settings for all enabled rate control layers in
///[`VideoEncodeRateControlInfoKHR::p_layer_configs`].To adjust rate control settings for an
/// individual layer at runtime, add a
///[`VideoEncodeRateControlLayerInfoKHR`] structure to the [`p_next`]
///chain of the [`VideoCodingControlInfoKHR`] structure passed to the
///[`CmdControlVideoCodingKHR`] command.
///This adjustment only impacts the specified layer without impacting the rate
///control settings or implementation rate control algorithm behavior for any
///other enabled rate control layers.
///The adjustment takes effect whenever the corresponding
///[`CmdControlVideoCodingKHR`] is executed, and only impacts
///[`CmdEncodeVideoKHR`] operations pertaining to the corresponding video
///coding layer that follow in execution order.It is possible for an application to enable multiple
/// video coding layers
///(via codec-specific extensions to encoding operations) while only enabling a
///single layer of rate control for the entire video stream.
///To achieve this, `layerCount` in [`VideoEncodeRateControlInfoKHR`]**must** be set to 1, and the
/// single [`VideoEncodeRateControlLayerInfoKHR`]
///provided in `pLayerConfigs` would apply to all encoded segments of the
///video stream, regardless of which codec-defined video coding layer they
///belong to.
///In this case, the implementation decides bitrate distribution across video
///coding layers (if applicable to the specified stream rate control mode).Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR`
///# Related
/// - [`VK_KHR_video_encode_queue`]
/// - [`StructureType`]
/// - [`VideoEncodeRateControlInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoEncodeRateControlLayerInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is a pointer to a structure extending this structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`average_bitrate`] is the average bitrate in bits/second.
    ///Valid when rate control mode is not
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    average_bitrate: u32,
    ///[`max_bitrate`] is the peak bitrate in bits/second.
    ///Valid when rate control mode is
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR`.
    max_bitrate: u32,
    ///[`frame_rate_numerator`] is the numerator of the frame rate.
    ///Valid when rate control mode is not
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    frame_rate_numerator: u32,
    ///[`frame_rate_denominator`] is the denominator of the frame rate.
    ///Valid when rate control mode is not
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    frame_rate_denominator: u32,
    ///[`virtual_buffer_size_in_ms`] is the leaky bucket model virtual buffer
    ///size in milliseconds, with respect to peak bitrate.
    ///Valid when rate control mode is not
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    ///For example, virtual buffer size is ([`virtual_buffer_size_in_ms`] *
    ///[`max_bitrate`] / 1000).
    virtual_buffer_size_in_ms: u32,
    ///[`initial_virtual_buffer_size_in_ms`] is the initial occupancy in
    ///milliseconds of the virtual buffer in the leaky bucket model.
    ///Valid when the rate control mode is not
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    initial_virtual_buffer_size_in_ms: u32,
}
///[VkVideoEncodeCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeCapabilitiesKHR.html) - Structure specifying encode capabilities
///# C Specifications
///When calling [`GetPhysicalDeviceVideoCapabilitiesKHR`] with
///`pVideoProfile->videoCodecOperation` specified as one of the encode
///operation bits, the [`VideoEncodeCapabilitiesKHR`] structure **must** be
///included in the [`p_next`] chain of the [`VideoCapabilitiesKHR`]
///structure to retrieve capabilities specific to video encoding.The [`VideoEncodeCapabilitiesKHR`]
/// structure is defined as:
///```c
///// Provided by VK_KHR_video_encode_queue
///typedef struct VkVideoEncodeCapabilitiesKHR {
///    VkStructureType                         sType;
///    const void*                             pNext;
///    VkVideoEncodeCapabilityFlagsKHR         flags;
///    VkVideoEncodeRateControlModeFlagsKHR    rateControlModes;
///    uint8_t                                 rateControlLayerCount;
///    uint8_t                                 qualityLevelCount;
///    VkExtent2D                              inputImageDataFillAlignment;
///} VkVideoEncodeCapabilitiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`VideoEncodeCapabilityFlagBitsKHR`] describing supported encoding
///   features.
/// - [`rate_control_modes`] is a bitmask of [`VideoEncodeRateControlModeFlagBitsKHR`] describing
///   supported rate control modes. All implementations **must** support
///   `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
/// - [`rate_control_layer_count`] reports the maximum number of rate control layers supported.
///   Implementations **must** report at least 1.
/// - [`quality_level_count`] is the number of discrete quality levels supported. Implementations
///   **must** report at least 1.
/// - [`input_image_data_fill_alignment`] reports alignment of data that should be filled in the
///   input image horizontally and vertically in pixels before encode operations are performed on
///   the input image.
///# Description
///The input content and encode resolution (specified in
///[`VideoEncodeInfoKHR::coded_extent`]) may not be aligned with the
///codec-specific coding block size.
///For example, the input content may be 1920x1080 and the coding block size
///may be 16x16 pixel blocks.
///In this example, the content is horizontally aligned with the coding block
///size, but not vertically aligned with the coding block size.
///Encoding of the last row of blocks may be impacted by contents of the input
///image in pixel rows 1081 to 1088 (the next vertical alignment with the
///coding block size).
///In general, to ensure efficient encoding for the last row/column of blocks,
///and/or to ensure consistent encoding results between repeated encoding of
///the same input content, these extra pixel rows/columns should be filled to
///known values up to the coding block size alignment before encoding
///operations are performed.
///Some implementations support performing auto-fill of unaligned pixels beyond
///a specific alignment, which is reported in
///[`input_image_data_fill_alignment`].
///For example, if an implementation reports 1x1 in
///[`input_image_data_fill_alignment`], then the implementation will perform
///auto-fill for any unaligned pixels beyond the encode resolution up to the
///next coding block size.
///For a coding block size of 16x16, if the implementation reports 16x16 in
///[`input_image_data_fill_alignment`], then it is the application’s
///responsibility to fill any unaligned pixels, if desired.
///If not, it may impact the encoding efficiency, but it will not affect the
///validity of the generated bitstream.
///If the implementation reports 8x8 in [`input_image_data_fill_alignment`], then
///for the 1920x1080 example, since the content is aligned to 8 pixels
///vertically, the implementation will auto-fill pixel rows 1081 to 1088 (up to
///the 16x16 coding block size in the example).
///The auto-fill value(s) are implementation-specific.
///The auto-fill value(s) are not written to the input image memory, but are
///used as part of the encoding operation on the input image.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_CAPABILITIES_KHR`
/// - [`rate_control_modes`]**must** be a valid combination of
///   [`VideoEncodeRateControlModeFlagBitsKHR`] values
/// - [`rate_control_modes`]**must** not be `0`
///# Related
/// - [`VK_KHR_video_encode_queue`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`VideoEncodeCapabilityFlagsKHR`]
/// - [`VideoEncodeRateControlModeFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoEncodeCapabilitiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeCapabilityFlagBitsKHR`]
    ///describing supported encoding features.
    flags: VideoEncodeCapabilityFlagsKHR,
    ///[`rate_control_modes`] is a bitmask of
    ///[`VideoEncodeRateControlModeFlagBitsKHR`] describing supported rate
    ///control modes.
    ///All implementations **must** support
    ///`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
    rate_control_modes: VideoEncodeRateControlModeFlagsKHR,
    ///[`rate_control_layer_count`] reports the maximum number of rate control
    ///layers supported.
    ///Implementations **must** report at least 1.
    rate_control_layer_count: u8,
    ///[`quality_level_count`] is the number of discrete quality levels
    ///supported.
    ///Implementations **must** report at least 1.
    quality_level_count: u8,
    ///[`input_image_data_fill_alignment`] reports alignment of data that should
    ///be filled in the input image horizontally and vertically in pixels
    ///before encode operations are performed on the input image.
    input_image_data_fill_alignment: Extent2D,
}
