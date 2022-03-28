use crate::{
    extensions::khr_video_queue::{VideoPictureResourceKHR, VideoReferenceSlotKHR},
    vulkan1_0::{BaseInStructure, BaseOutStructure, Buffer, DeviceSize, Extent2D, Offset2D, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_video_decode_queue");
///[VkVideoDecodeCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilitiesKHR.html) - Structure specifying decode capabilities
///# C Specifications
///When calling [`GetPhysicalDeviceVideoCapabilitiesKHR`] with
///`pVideoProfile->videoCodecOperation` specified as one of the decode
///operation bits, the [`VideoDecodeCapabilitiesKHR`] structure  **must**  be
///included in the [`p_next`] chain of the [`VideoCapabilitiesKHR`]
///structure to retrieve capabilities specific to video decoding.The [`VideoDecodeCapabilitiesKHR`]
/// structure is defined as:
///```c
///// Provided by VK_KHR_video_decode_queue
///typedef struct VkVideoDecodeCapabilitiesKHR {
///    VkStructureType                    sType;
///    void*                              pNext;
///    VkVideoDecodeCapabilityFlagsKHR    flags;
///} VkVideoDecodeCapabilitiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`VideoDecodeCapabilityFlagBitsKHR`] describing supported decoding
///   features.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR`
///# Related
/// - [`VK_KHR_video_decode_queue`]
/// - [`StructureType`]
/// - [`VideoDecodeCapabilityFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VideoDecodeCapabilitiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoDecodeCapabilityFlagBitsKHR`]
    ///describing supported decoding features.
    flags: VideoDecodeCapabilityFlagsKHR,
}
impl<'lt> Default for VideoDecodeCapabilitiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
        }
    }
}
impl<'lt> VideoDecodeCapabilitiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> VideoDecodeCapabilityFlagsKHR {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoDecodeCapabilityFlagsKHR {
        &mut self.flags
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_video_decode_queue::VideoDecodeCapabilityFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
}
///[VkVideoDecodeInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeInfoKHR.html) - Structure specifying parameters of decoding a frame
///# C Specifications
///The [`VideoDecodeInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_decode_queue
///typedef struct VkVideoDecodeInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkVideoDecodeFlagsKHR             flags;
///    VkOffset2D                        codedOffset;
///    VkExtent2D                        codedExtent;
///    VkBuffer                          srcBuffer;
///    VkDeviceSize                      srcBufferOffset;
///    VkDeviceSize                      srcBufferRange;
///    VkVideoPictureResourceKHR         dstPictureResource;
///    const VkVideoReferenceSlotKHR*    pSetupReferenceSlot;
///    uint32_t                          referenceSlotCount;
///    const VkVideoReferenceSlotKHR*    pReferenceSlots;
///} VkVideoDecodeInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure. All the codec
///   specific structures related to each frame(picture parameters, quantization matrix, etc.)
///   **must**  be chained here and pass to decode session with the function call
///   [`CmdDecodeVideoKHR`].
/// - [`flags`] is a bitmask of [`VideoDecodeFlagBitsKHR`] specifying decode flags, reserved for
///   future versions of this specification.
/// - [`coded_offset`] is the coded offset of the decode operations. The purpose of this field is
///   interpreted based on the codec extension. When decoding content in H.264 field mode, the
///   [`coded_offset`] specifies the line or picture field’s offset within the image.
/// - [`coded_extent`] is the coded size of the decode operations.
/// - [`src_buffer`] is the source buffer that holds the encoded bitstream.
/// - [`src_buffer_offset`] is the buffer offset where the valid encoded bitstream starts in
///   srcBuffer. It  **must**  meet the alignment requirement `minBitstreamBufferOffsetAlignment`
///   within [`VideoCapabilitiesKHR`] queried with the [`GetPhysicalDeviceVideoCapabilitiesKHR`]
///   function.
/// - [`src_buffer_range`] is the size of the srcBuffer with valid encoded bitstream, starting from
///   [`src_buffer_offset`]. It  **must**  meet the alignment requirement
///   `minBitstreamBufferSizeAlignment` within [`VideoCapabilitiesKHR`] queried with the
///   [`GetPhysicalDeviceVideoCapabilitiesKHR`] function.
/// - [`dst_picture_resource`] is the destination [Decoded Output Picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture)
///   Resource.
/// - [`setup_reference_slot`] is `NULL` or a pointer to a [`VideoReferenceSlotKHR`] structure used
///   for generating a DPB reference slot and Picture Resource. `pSetupReferenceSlot->slotIndex`
///   specifies the slot index number to use as a target for producing the DPB data. `slotIndex`
///   **must**  reference a valid entry as specified in [`VideoBeginCodingInfoKHR`] via the
///   [`reference_slots`] within the [`CmdBeginVideoCodingKHR`] command that established the Vulkan
///   Video Decode Context for this command.
/// - [`reference_slot_count`] is the number of the DPB Reference Pictures that will be used when
///   this decoding operation is executing.
/// - [`reference_slots`] is a pointer to an array of [`VideoReferenceSlotKHR`] structures
///   specifying the DPB Reference pictures that will be used when this decoding operation is
///   executing.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoDecodeH264PictureInfoEXT`] or
///   [`VideoDecodeH265PictureInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`flags`] **must**  be a valid combination of [`VideoDecodeFlagBitsKHR`] values
/// - [`src_buffer`] **must**  be a valid [`Buffer`] handle
/// - [`dst_picture_resource`] **must**  be a valid [`VideoPictureResourceKHR`] structure
/// - [`setup_reference_slot`] **must**  be a valid pointer to a valid [`VideoReferenceSlotKHR`]
///   structure
/// - If [`reference_slot_count`] is not `0`, [`reference_slots`] **must**  be a valid pointer to an
///   array of [`reference_slot_count`] valid [`VideoReferenceSlotKHR`] structures
///# Related
/// - [`VK_KHR_video_decode_queue`]
/// - [`Buffer`]
/// - [`DeviceSize`]
/// - [`Extent2D`]
/// - [`Offset2D`]
/// - [`StructureType`]
/// - [`VideoDecodeFlagsKHR`]
/// - [`VideoPictureResourceKHR`]
/// - [`VideoReferenceSlotKHR`]
/// - [`CmdDecodeVideoKHR`]
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
pub struct VideoDecodeInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    ///All the codec specific structures related to each frame(picture
    ///parameters, quantization matrix, etc.)  **must**  be chained here and pass to
    ///decode session with the function call [`CmdDecodeVideoKHR`].
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoDecodeFlagBitsKHR`] specifying
    ///decode flags, reserved for future versions of this specification.
    flags: VideoDecodeFlagsKHR,
    ///[`coded_offset`] is the coded offset of the decode operations.
    ///The purpose of this field is interpreted based on the codec extension.
    ///When decoding content in H.264 field mode, the [`coded_offset`]
    ///specifies the line or picture field’s offset within the image.
    coded_offset: Offset2D,
    ///[`coded_extent`] is the coded size of the decode operations.
    coded_extent: Extent2D,
    ///[`src_buffer`] is the source buffer that holds the encoded bitstream.
    src_buffer: Buffer,
    ///[`src_buffer_offset`] is the buffer offset where the valid encoded
    ///bitstream starts in srcBuffer.
    ///It  **must**  meet the alignment requirement
    ///`minBitstreamBufferOffsetAlignment` within
    ///[`VideoCapabilitiesKHR`] queried with the
    ///[`GetPhysicalDeviceVideoCapabilitiesKHR`] function.
    src_buffer_offset: DeviceSize,
    ///[`src_buffer_range`] is the size of the srcBuffer with valid encoded
    ///bitstream, starting from [`src_buffer_offset`].
    ///It  **must**  meet the alignment requirement
    ///`minBitstreamBufferSizeAlignment` within
    ///[`VideoCapabilitiesKHR`] queried with the
    ///[`GetPhysicalDeviceVideoCapabilitiesKHR`] function.
    src_buffer_range: DeviceSize,
    ///[`dst_picture_resource`] is the destination
    ///[Decoded Output Picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture) Resource.
    dst_picture_resource: VideoPictureResourceKHR<'lt>,
    ///[`setup_reference_slot`] is `NULL` or a pointer to a
    ///[`VideoReferenceSlotKHR`] structure used for generating a DPB
    ///reference slot and Picture Resource.
    ///`pSetupReferenceSlot->slotIndex` specifies the slot index number to
    ///use as a target for producing the DPB data.
    ///`slotIndex` **must**  reference a valid entry as specified in
    ///[`VideoBeginCodingInfoKHR`] via the [`reference_slots`] within the
    ///[`CmdBeginVideoCodingKHR`] command that established the Vulkan Video
    ///Decode Context for this command.
    setup_reference_slot: *const VideoReferenceSlotKHR<'lt>,
    ///[`reference_slot_count`] is the number of the DPB Reference Pictures
    ///that will be used when this decoding operation is executing.
    reference_slot_count: u32,
    ///[`reference_slots`] is a pointer to an array of
    ///[`VideoReferenceSlotKHR`] structures specifying the DPB Reference
    ///pictures that will be used when this decoding operation is executing.
    reference_slots: *const VideoReferenceSlotKHR<'lt>,
}
impl<'lt> Default for VideoDecodeInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            coded_offset: Default::default(),
            coded_extent: Default::default(),
            src_buffer: Default::default(),
            src_buffer_offset: Default::default(),
            src_buffer_range: Default::default(),
            dst_picture_resource: Default::default(),
            setup_reference_slot: std::ptr::null(),
            reference_slot_count: 0,
            reference_slots: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::setup_reference_slot`]
    pub fn setup_reference_slot_raw(&self) -> *const VideoReferenceSlotKHR<'lt> {
        self.setup_reference_slot
    }
    ///Gets the raw value of [`Self::reference_slots`]
    pub fn reference_slots_raw(&self) -> *const VideoReferenceSlotKHR<'lt> {
        self.reference_slots
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::setup_reference_slot`]
    pub fn set_setup_reference_slot_raw(&mut self, value: *const VideoReferenceSlotKHR<'lt>) -> &mut Self {
        self.setup_reference_slot = value;
        self
    }
    ///Sets the raw value of [`Self::reference_slots`]
    pub fn set_reference_slots_raw(&mut self, value: *const VideoReferenceSlotKHR<'lt>) -> &mut Self {
        self.reference_slots = value;
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
    pub fn flags(&self) -> VideoDecodeFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::coded_offset`]
    pub fn coded_offset(&self) -> Offset2D {
        self.coded_offset
    }
    ///Gets the value of [`Self::coded_extent`]
    pub fn coded_extent(&self) -> Extent2D {
        self.coded_extent
    }
    ///Gets the value of [`Self::src_buffer`]
    pub fn src_buffer(&self) -> Buffer {
        self.src_buffer
    }
    ///Gets the value of [`Self::src_buffer_offset`]
    pub fn src_buffer_offset(&self) -> DeviceSize {
        self.src_buffer_offset
    }
    ///Gets the value of [`Self::src_buffer_range`]
    pub fn src_buffer_range(&self) -> DeviceSize {
        self.src_buffer_range
    }
    ///Gets the value of [`Self::dst_picture_resource`]
    pub fn dst_picture_resource(&self) -> VideoPictureResourceKHR<'lt> {
        self.dst_picture_resource
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
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoDecodeFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::coded_offset`]
    pub fn coded_offset_mut(&mut self) -> &mut Offset2D {
        &mut self.coded_offset
    }
    ///Gets a mutable reference to the value of [`Self::coded_extent`]
    pub fn coded_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.coded_extent
    }
    ///Gets a mutable reference to the value of [`Self::src_buffer`]
    pub fn src_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.src_buffer
    }
    ///Gets a mutable reference to the value of [`Self::src_buffer_offset`]
    pub fn src_buffer_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.src_buffer_offset
    }
    ///Gets a mutable reference to the value of [`Self::src_buffer_range`]
    pub fn src_buffer_range_mut(&mut self) -> &mut DeviceSize {
        &mut self.src_buffer_range
    }
    ///Gets a mutable reference to the value of [`Self::dst_picture_resource`]
    pub fn dst_picture_resource_mut(&mut self) -> &mut VideoPictureResourceKHR<'lt> {
        &mut self.dst_picture_resource
    }
    ///Gets a mutable reference to the value of [`Self::reference_slot_count`]
    pub fn reference_slot_count_mut(&mut self) -> &mut u32 {
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
    pub fn set_flags(&mut self, value: crate::extensions::khr_video_decode_queue::VideoDecodeFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::coded_offset`]
    pub fn set_coded_offset(&mut self, value: crate::vulkan1_0::Offset2D) -> &mut Self {
        self.coded_offset = value;
        self
    }
    ///Sets the raw value of [`Self::coded_extent`]
    pub fn set_coded_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.coded_extent = value;
        self
    }
    ///Sets the raw value of [`Self::src_buffer`]
    pub fn set_src_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.src_buffer = value;
        self
    }
    ///Sets the raw value of [`Self::src_buffer_offset`]
    pub fn set_src_buffer_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.src_buffer_offset = value;
        self
    }
    ///Sets the raw value of [`Self::src_buffer_range`]
    pub fn set_src_buffer_range(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.src_buffer_range = value;
        self
    }
    ///Sets the raw value of [`Self::dst_picture_resource`]
    pub fn set_dst_picture_resource(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoPictureResourceKHR<'lt>,
    ) -> &mut Self {
        self.dst_picture_resource = value;
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
}
