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
/// - [`setup_reference_slot`] is a pointer to a [`VideoReferenceSlotKHR`] structure used for
///   generating a reconstructed reference slot and Picture Resource.
///   `pSetupReferenceSlot->slotIndex` specifies the slot index number to use as a target for
///   producing the Reconstructed (DPB) data. [`setup_reference_slot`]**must** be one of the entries
///   provided in [`VideoBeginCodingInfoKHR`] via the [`reference_slots`] within the
///   [`CmdBeginVideoCodingKHR`] command that established the Vulkan Video Encode Context for this
///   command.
/// - [`reference_slot_count`] is the number of Reconstructed Reference Pictures that will be used
///   when this encoding operation is executing.
/// - [`reference_slots`] is `NULL` or a pointer to an array of [`VideoReferenceSlotKHR`] structures
///   that will be used when this encoding operation is executing. Each entry in
///   [`reference_slots`]**must** be one of the entries provided in [`VideoBeginCodingInfoKHR`] via
///   the [`reference_slots`] within the [`CmdBeginVideoCodingKHR`] command that established the
///   Vulkan Video Encode Context for this command.
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
/// - [`setup_reference_slot`]**must** be a valid pointer to a valid [`VideoReferenceSlotKHR`]
///   structure
/// - If [`reference_slot_count`] is not `0`, [`reference_slots`]**must** be a valid pointer to an
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is a pointer to a structure extending this structure.
    ///A codec-specific extension structure **must** be chained to specify what
    ///bitstream unit to generate with this encode operation.
    p_next: *const BaseInStructure<'lt>,
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
    ///[`setup_reference_slot`] is a pointer to a
    ///[`VideoReferenceSlotKHR`] structure used for generating a
    ///reconstructed reference slot and Picture Resource.
    ///`pSetupReferenceSlot->slotIndex` specifies the slot index number to
    ///use as a target for producing the Reconstructed (DPB) data.
    ///[`setup_reference_slot`]**must** be one of the entries provided in
    ///[`VideoBeginCodingInfoKHR`] via the [`reference_slots`] within the
    ///[`CmdBeginVideoCodingKHR`] command that established the Vulkan Video
    ///Encode Context for this command.
    setup_reference_slot: *const VideoReferenceSlotKHR<'lt>,
    ///[`reference_slot_count`] is the number of Reconstructed Reference
    ///Pictures that will be used when this encoding operation is executing.
    reference_slot_count: u32,
    ///[`reference_slots`] is `NULL` or a pointer to an array of
    ///[`VideoReferenceSlotKHR`] structures that will be used when this
    ///encoding operation is executing.
    ///Each entry in [`reference_slots`]**must** be one of the entries provided
    ///in [`VideoBeginCodingInfoKHR`] via the [`reference_slots`] within
    ///the [`CmdBeginVideoCodingKHR`] command that established the Vulkan
    ///Video Encode Context for this command.
    reference_slots: *const VideoReferenceSlotKHR<'lt>,
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
impl<'lt> Default for VideoEncodeInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            quality_level: 0,
            coded_extent: Default::default(),
            dst_bitstream_buffer: Default::default(),
            dst_bitstream_buffer_offset: Default::default(),
            dst_bitstream_buffer_max_range: Default::default(),
            src_picture_resource: Default::default(),
            setup_reference_slot: std::ptr::null(),
            reference_slot_count: 0,
            reference_slots: std::ptr::null(),
            preceding_externally_encoded_bytes: 0,
        }
    }
}
impl<'lt> VideoEncodeInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::quality_level`]
    pub fn quality_level_raw(&self) -> u32 {
        self.quality_level
    }
    ///Gets the raw value of [`Self::setup_reference_slot`]
    pub fn setup_reference_slot_raw(&self) -> *const VideoReferenceSlotKHR<'lt> {
        self.setup_reference_slot
    }
    ///Gets the raw value of [`Self::reference_slot_count`]
    pub fn reference_slot_count_raw(&self) -> u32 {
        self.reference_slot_count
    }
    ///Gets the raw value of [`Self::reference_slots`]
    pub fn reference_slots_raw(&self) -> *const VideoReferenceSlotKHR<'lt> {
        self.reference_slots
    }
    ///Gets the raw value of [`Self::preceding_externally_encoded_bytes`]
    pub fn preceding_externally_encoded_bytes_raw(&self) -> u32 {
        self.preceding_externally_encoded_bytes
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::quality_level`]
    pub fn set_quality_level_raw(&mut self, value: u32) -> &mut Self {
        self.quality_level = value;
        self
    }
    ///Sets the raw value of [`Self::setup_reference_slot`]
    pub fn set_setup_reference_slot_raw(&mut self, value: *const VideoReferenceSlotKHR<'lt>) -> &mut Self {
        self.setup_reference_slot = value;
        self
    }
    ///Sets the raw value of [`Self::reference_slot_count`]
    pub fn set_reference_slot_count_raw(&mut self, value: u32) -> &mut Self {
        self.reference_slot_count = value;
        self
    }
    ///Sets the raw value of [`Self::reference_slots`]
    pub fn set_reference_slots_raw(&mut self, value: *const VideoReferenceSlotKHR<'lt>) -> &mut Self {
        self.reference_slots = value;
        self
    }
    ///Sets the raw value of [`Self::preceding_externally_encoded_bytes`]
    pub fn set_preceding_externally_encoded_bytes_raw(&mut self, value: u32) -> &mut Self {
        self.preceding_externally_encoded_bytes = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> VideoEncodeFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::quality_level`]
    pub fn quality_level(&self) -> u32 {
        self.quality_level
    }
    ///Gets the value of [`Self::coded_extent`]
    pub fn coded_extent(&self) -> Extent2D {
        self.coded_extent
    }
    ///Gets the value of [`Self::dst_bitstream_buffer`]
    pub fn dst_bitstream_buffer(&self) -> Buffer {
        self.dst_bitstream_buffer
    }
    ///Gets the value of [`Self::dst_bitstream_buffer_offset`]
    pub fn dst_bitstream_buffer_offset(&self) -> DeviceSize {
        self.dst_bitstream_buffer_offset
    }
    ///Gets the value of [`Self::dst_bitstream_buffer_max_range`]
    pub fn dst_bitstream_buffer_max_range(&self) -> DeviceSize {
        self.dst_bitstream_buffer_max_range
    }
    ///Gets the value of [`Self::src_picture_resource`]
    pub fn src_picture_resource(&self) -> VideoPictureResourceKHR<'lt> {
        self.src_picture_resource
    }
    ///Gets the value of [`Self::setup_reference_slot`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn setup_reference_slot(&self) -> &VideoReferenceSlotKHR<'lt> {
        &*self.setup_reference_slot
    }
    ///Gets the value of [`Self::reference_slot_count`]
    pub fn reference_slot_count(&self) -> u32 {
        self.reference_slot_count
    }
    ///Gets the value of [`Self::reference_slots`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn reference_slots(&self) -> &[VideoReferenceSlotKHR<'lt>] {
        std::slice::from_raw_parts(self.reference_slots, self.reference_slot_count as usize)
    }
    ///Gets the value of [`Self::preceding_externally_encoded_bytes`]
    pub fn preceding_externally_encoded_bytes(&self) -> u32 {
        self.preceding_externally_encoded_bytes
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoEncodeFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::quality_level`]
    pub fn quality_level_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::coded_extent`]
    pub fn coded_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.coded_extent
    }
    ///Gets a mutable reference to the value of [`Self::dst_bitstream_buffer`]
    pub fn dst_bitstream_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.dst_bitstream_buffer
    }
    ///Gets a mutable reference to the value of [`Self::dst_bitstream_buffer_offset`]
    pub fn dst_bitstream_buffer_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.dst_bitstream_buffer_offset
    }
    ///Gets a mutable reference to the value of [`Self::dst_bitstream_buffer_max_range`]
    pub fn dst_bitstream_buffer_max_range_mut(&mut self) -> &mut DeviceSize {
        &mut self.dst_bitstream_buffer_max_range
    }
    ///Gets a mutable reference to the value of [`Self::src_picture_resource`]
    pub fn src_picture_resource_mut(&mut self) -> &mut VideoPictureResourceKHR<'lt> {
        &mut self.src_picture_resource
    }
    ///Gets a mutable reference to the value of [`Self::reference_slot_count`]
    pub fn reference_slot_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::preceding_externally_encoded_bytes`]
    pub fn preceding_externally_encoded_bytes_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::khr_video_encode_queue::VideoEncodeFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::quality_level`]
    pub fn set_quality_level(&mut self, value: u32) -> &mut Self {
        self.quality_level = value;
        self
    }
    ///Sets the raw value of [`Self::coded_extent`]
    pub fn set_coded_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.coded_extent = value;
        self
    }
    ///Sets the raw value of [`Self::dst_bitstream_buffer`]
    pub fn set_dst_bitstream_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.dst_bitstream_buffer = value;
        self
    }
    ///Sets the raw value of [`Self::dst_bitstream_buffer_offset`]
    pub fn set_dst_bitstream_buffer_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.dst_bitstream_buffer_offset = value;
        self
    }
    ///Sets the raw value of [`Self::dst_bitstream_buffer_max_range`]
    pub fn set_dst_bitstream_buffer_max_range(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.dst_bitstream_buffer_max_range = value;
        self
    }
    ///Sets the raw value of [`Self::src_picture_resource`]
    pub fn set_src_picture_resource(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoPictureResourceKHR<'lt>,
    ) -> &mut Self {
        self.src_picture_resource = value;
        self
    }
    ///Sets the raw value of [`Self::setup_reference_slot`]
    pub fn set_setup_reference_slot(
        &mut self,
        value: &'lt crate::extensions::khr_video_queue::VideoReferenceSlotKHR<'lt>,
    ) -> &mut Self {
        self.setup_reference_slot = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::reference_slot_count`]
    pub fn set_reference_slot_count(&mut self, value: u32) -> &mut Self {
        self.reference_slot_count = value;
        self
    }
    ///Sets the raw value of [`Self::reference_slots`]
    pub fn set_reference_slots(
        &mut self,
        value: &'lt [crate::extensions::khr_video_queue::VideoReferenceSlotKHR<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.reference_slots = value.as_ptr();
        self.reference_slot_count = len_;
        self
    }
    ///Sets the raw value of [`Self::preceding_externally_encoded_bytes`]
    pub fn set_preceding_externally_encoded_bytes(&mut self, value: u32) -> &mut Self {
        self.preceding_externally_encoded_bytes = value;
        self
    }
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
/// - [`layer_configs`] is a pointer to an array of [`VideoEncodeRateControlLayerInfoKHR`]
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
/// - [`layer_configs`]**must** be a valid pointer to an array of [`layer_count`] valid
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeRateControlInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoEncodeRateControlFlagBitsKHR`]
    ///specifying encode rate control flags.
    flags: VideoEncodeRateControlFlagsKHR,
    ///[`rate_control_mode`] is a [`VideoEncodeRateControlModeFlagBitsKHR`]
    ///value specifying the encode stream rate control mode.
    rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
    ///[`layer_count`] specifies the number of rate control layers in the
    ///video encode stream.
    layer_count: u8,
    ///[`layer_configs`] is a pointer to an array of
    ///[`VideoEncodeRateControlLayerInfoKHR`] structures specifying the
    ///rate control configurations of [`layer_count`] rate control layers.
    layer_configs: *const VideoEncodeRateControlLayerInfoKHR<'lt>,
}
impl<'lt> Default for VideoEncodeRateControlInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            rate_control_mode: Default::default(),
            layer_count: 0,
            layer_configs: std::ptr::null(),
        }
    }
}
impl<'lt> VideoEncodeRateControlInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::layer_count`]
    pub fn layer_count_raw(&self) -> u8 {
        self.layer_count
    }
    ///Gets the raw value of [`Self::layer_configs`]
    pub fn layer_configs_raw(&self) -> *const VideoEncodeRateControlLayerInfoKHR<'lt> {
        self.layer_configs
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::layer_count`]
    pub fn set_layer_count_raw(&mut self, value: u8) -> &mut Self {
        self.layer_count = value;
        self
    }
    ///Sets the raw value of [`Self::layer_configs`]
    pub fn set_layer_configs_raw(&mut self, value: *const VideoEncodeRateControlLayerInfoKHR<'lt>) -> &mut Self {
        self.layer_configs = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> VideoEncodeRateControlFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::rate_control_mode`]
    pub fn rate_control_mode(&self) -> VideoEncodeRateControlModeFlagBitsKHR {
        self.rate_control_mode
    }
    ///Gets the value of [`Self::layer_count`]
    pub fn layer_count(&self) -> u8 {
        self.layer_count
    }
    ///Gets the value of [`Self::layer_configs`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn layer_configs(&self) -> &[VideoEncodeRateControlLayerInfoKHR<'lt>] {
        std::slice::from_raw_parts(self.layer_configs, self.layer_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoEncodeRateControlFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::rate_control_mode`]
    pub fn rate_control_mode_mut(&mut self) -> &mut VideoEncodeRateControlModeFlagBitsKHR {
        &mut self.rate_control_mode
    }
    ///Gets a mutable reference to the value of [`Self::layer_count`]
    pub fn layer_count_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::rate_control_mode`]
    pub fn set_rate_control_mode(
        &mut self,
        value: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlModeFlagBitsKHR,
    ) -> &mut Self {
        self.rate_control_mode = value;
        self
    }
    ///Sets the raw value of [`Self::layer_count`]
    pub fn set_layer_count(&mut self, value: u8) -> &mut Self {
        self.layer_count = value;
        self
    }
    ///Sets the raw value of [`Self::layer_configs`]
    pub fn set_layer_configs(
        &mut self,
        value: &'lt [crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHR<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.layer_configs = value.as_ptr();
        self.layer_count = len_;
        self
    }
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
///[`VideoEncodeRateControlInfoKHR::layer_configs`].To adjust rate control settings for an
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeRateControlLayerInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is a pointer to a structure extending this structure.
    p_next: *const BaseInStructure<'lt>,
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
impl<'lt> Default for VideoEncodeRateControlLayerInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            average_bitrate: 0,
            max_bitrate: 0,
            frame_rate_numerator: 0,
            frame_rate_denominator: 0,
            virtual_buffer_size_in_ms: 0,
            initial_virtual_buffer_size_in_ms: 0,
        }
    }
}
impl<'lt> VideoEncodeRateControlLayerInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::average_bitrate`]
    pub fn average_bitrate_raw(&self) -> u32 {
        self.average_bitrate
    }
    ///Gets the raw value of [`Self::max_bitrate`]
    pub fn max_bitrate_raw(&self) -> u32 {
        self.max_bitrate
    }
    ///Gets the raw value of [`Self::frame_rate_numerator`]
    pub fn frame_rate_numerator_raw(&self) -> u32 {
        self.frame_rate_numerator
    }
    ///Gets the raw value of [`Self::frame_rate_denominator`]
    pub fn frame_rate_denominator_raw(&self) -> u32 {
        self.frame_rate_denominator
    }
    ///Gets the raw value of [`Self::virtual_buffer_size_in_ms`]
    pub fn virtual_buffer_size_in_ms_raw(&self) -> u32 {
        self.virtual_buffer_size_in_ms
    }
    ///Gets the raw value of [`Self::initial_virtual_buffer_size_in_ms`]
    pub fn initial_virtual_buffer_size_in_ms_raw(&self) -> u32 {
        self.initial_virtual_buffer_size_in_ms
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::average_bitrate`]
    pub fn set_average_bitrate_raw(&mut self, value: u32) -> &mut Self {
        self.average_bitrate = value;
        self
    }
    ///Sets the raw value of [`Self::max_bitrate`]
    pub fn set_max_bitrate_raw(&mut self, value: u32) -> &mut Self {
        self.max_bitrate = value;
        self
    }
    ///Sets the raw value of [`Self::frame_rate_numerator`]
    pub fn set_frame_rate_numerator_raw(&mut self, value: u32) -> &mut Self {
        self.frame_rate_numerator = value;
        self
    }
    ///Sets the raw value of [`Self::frame_rate_denominator`]
    pub fn set_frame_rate_denominator_raw(&mut self, value: u32) -> &mut Self {
        self.frame_rate_denominator = value;
        self
    }
    ///Sets the raw value of [`Self::virtual_buffer_size_in_ms`]
    pub fn set_virtual_buffer_size_in_ms_raw(&mut self, value: u32) -> &mut Self {
        self.virtual_buffer_size_in_ms = value;
        self
    }
    ///Sets the raw value of [`Self::initial_virtual_buffer_size_in_ms`]
    pub fn set_initial_virtual_buffer_size_in_ms_raw(&mut self, value: u32) -> &mut Self {
        self.initial_virtual_buffer_size_in_ms = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::average_bitrate`]
    pub fn average_bitrate(&self) -> u32 {
        self.average_bitrate
    }
    ///Gets the value of [`Self::max_bitrate`]
    pub fn max_bitrate(&self) -> u32 {
        self.max_bitrate
    }
    ///Gets the value of [`Self::frame_rate_numerator`]
    pub fn frame_rate_numerator(&self) -> u32 {
        self.frame_rate_numerator
    }
    ///Gets the value of [`Self::frame_rate_denominator`]
    pub fn frame_rate_denominator(&self) -> u32 {
        self.frame_rate_denominator
    }
    ///Gets the value of [`Self::virtual_buffer_size_in_ms`]
    pub fn virtual_buffer_size_in_ms(&self) -> u32 {
        self.virtual_buffer_size_in_ms
    }
    ///Gets the value of [`Self::initial_virtual_buffer_size_in_ms`]
    pub fn initial_virtual_buffer_size_in_ms(&self) -> u32 {
        self.initial_virtual_buffer_size_in_ms
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::average_bitrate`]
    pub fn average_bitrate_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_bitrate`]
    pub fn max_bitrate_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::frame_rate_numerator`]
    pub fn frame_rate_numerator_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::frame_rate_denominator`]
    pub fn frame_rate_denominator_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::virtual_buffer_size_in_ms`]
    pub fn virtual_buffer_size_in_ms_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::initial_virtual_buffer_size_in_ms`]
    pub fn initial_virtual_buffer_size_in_ms_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::average_bitrate`]
    pub fn set_average_bitrate(&mut self, value: u32) -> &mut Self {
        self.average_bitrate = value;
        self
    }
    ///Sets the raw value of [`Self::max_bitrate`]
    pub fn set_max_bitrate(&mut self, value: u32) -> &mut Self {
        self.max_bitrate = value;
        self
    }
    ///Sets the raw value of [`Self::frame_rate_numerator`]
    pub fn set_frame_rate_numerator(&mut self, value: u32) -> &mut Self {
        self.frame_rate_numerator = value;
        self
    }
    ///Sets the raw value of [`Self::frame_rate_denominator`]
    pub fn set_frame_rate_denominator(&mut self, value: u32) -> &mut Self {
        self.frame_rate_denominator = value;
        self
    }
    ///Sets the raw value of [`Self::virtual_buffer_size_in_ms`]
    pub fn set_virtual_buffer_size_in_ms(&mut self, value: u32) -> &mut Self {
        self.virtual_buffer_size_in_ms = value;
        self
    }
    ///Sets the raw value of [`Self::initial_virtual_buffer_size_in_ms`]
    pub fn set_initial_virtual_buffer_size_in_ms(&mut self, value: u32) -> &mut Self {
        self.initial_virtual_buffer_size_in_ms = value;
        self
    }
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoEncodeCapabilitiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
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
impl<'lt> Default for VideoEncodeCapabilitiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            rate_control_modes: Default::default(),
            rate_control_layer_count: 0,
            quality_level_count: 0,
            input_image_data_fill_alignment: Default::default(),
        }
    }
}
impl<'lt> VideoEncodeCapabilitiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::rate_control_layer_count`]
    pub fn rate_control_layer_count_raw(&self) -> u8 {
        self.rate_control_layer_count
    }
    ///Gets the raw value of [`Self::quality_level_count`]
    pub fn quality_level_count_raw(&self) -> u8 {
        self.quality_level_count
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::rate_control_layer_count`]
    pub fn set_rate_control_layer_count_raw(&mut self, value: u8) -> &mut Self {
        self.rate_control_layer_count = value;
        self
    }
    ///Sets the raw value of [`Self::quality_level_count`]
    pub fn set_quality_level_count_raw(&mut self, value: u8) -> &mut Self {
        self.quality_level_count = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> VideoEncodeCapabilityFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::rate_control_modes`]
    pub fn rate_control_modes(&self) -> VideoEncodeRateControlModeFlagsKHR {
        self.rate_control_modes
    }
    ///Gets the value of [`Self::rate_control_layer_count`]
    pub fn rate_control_layer_count(&self) -> u8 {
        self.rate_control_layer_count
    }
    ///Gets the value of [`Self::quality_level_count`]
    pub fn quality_level_count(&self) -> u8 {
        self.quality_level_count
    }
    ///Gets the value of [`Self::input_image_data_fill_alignment`]
    pub fn input_image_data_fill_alignment(&self) -> Extent2D {
        self.input_image_data_fill_alignment
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoEncodeCapabilityFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::rate_control_modes`]
    pub fn rate_control_modes_mut(&mut self) -> &mut VideoEncodeRateControlModeFlagsKHR {
        &mut self.rate_control_modes
    }
    ///Gets a mutable reference to the value of [`Self::rate_control_layer_count`]
    pub fn rate_control_layer_count_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::quality_level_count`]
    pub fn quality_level_count_mut(&mut self) -> &mut u8 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::input_image_data_fill_alignment`]
    pub fn input_image_data_fill_alignment_mut(&mut self) -> &mut Extent2D {
        &mut self.input_image_data_fill_alignment
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_video_encode_queue::VideoEncodeCapabilityFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::rate_control_modes`]
    pub fn set_rate_control_modes(
        &mut self,
        value: crate::extensions::khr_video_encode_queue::VideoEncodeRateControlModeFlagsKHR,
    ) -> &mut Self {
        self.rate_control_modes = value;
        self
    }
    ///Sets the raw value of [`Self::rate_control_layer_count`]
    pub fn set_rate_control_layer_count(&mut self, value: u8) -> &mut Self {
        self.rate_control_layer_count = value;
        self
    }
    ///Sets the raw value of [`Self::quality_level_count`]
    pub fn set_quality_level_count(&mut self, value: u8) -> &mut Self {
        self.quality_level_count = value;
        self
    }
    ///Sets the raw value of [`Self::input_image_data_fill_alignment`]
    pub fn set_input_image_data_fill_alignment(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.input_image_data_fill_alignment = value;
        self
    }
}
