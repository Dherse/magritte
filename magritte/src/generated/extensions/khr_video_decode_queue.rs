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
///operation bits, the [`VideoDecodeCapabilitiesKHR`] structure **must** be
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoDecodeCapabilitiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoDecodeCapabilityFlagBitsKHR`]
    ///describing supported decoding features.
    flags: VideoDecodeCapabilityFlagsKHR,
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
///   **must** be chained here and pass to decode session with the function call
///   [`CmdDecodeVideoKHR`].
/// - [`flags`] is a bitmask of [`VideoDecodeFlagBitsKHR`] specifying decode flags, reserved for
///   future versions of this specification.
/// - [`coded_offset`] is the coded offset of the decode operations. The purpose of this field is
///   interpreted based on the codec extension. When decoding content in H.264 field mode, the
///   [`coded_offset`] specifies the line or picture field’s offset within the image.
/// - [`coded_extent`] is the coded size of the decode operations.
/// - [`src_buffer`] is the source buffer that holds the encoded bitstream.
/// - [`src_buffer_offset`] is the buffer offset where the valid encoded bitstream starts in
///   srcBuffer. It **must** meet the alignment requirement `minBitstreamBufferOffsetAlignment`
///   within [`VideoCapabilitiesKHR`] queried with the [`GetPhysicalDeviceVideoCapabilitiesKHR`]
///   function.
/// - [`src_buffer_range`] is the size of the srcBuffer with valid encoded bitstream, starting from
///   [`src_buffer_offset`]. It **must** meet the alignment requirement
///   `minBitstreamBufferSizeAlignment` within [`VideoCapabilitiesKHR`] queried with the
///   [`GetPhysicalDeviceVideoCapabilitiesKHR`] function.
/// - [`dst_picture_resource`] is the destination [Decoded Output Picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture)
///   Resource.
/// - [`p_setup_reference_slot`] is `NULL` or a pointer to a [`VideoReferenceSlotKHR`] structure
///   used for generating a DPB reference slot and Picture Resource.
///   `pSetupReferenceSlot->slotIndex` specifies the slot index number to use as a target for
///   producing the DPB data. `slotIndex`**must** reference a valid entry as specified in
///   [`VideoBeginCodingInfoKHR`] via the [`p_reference_slots`] within the
///   [`CmdBeginVideoCodingKHR`] command that established the Vulkan Video Decode Context for this
///   command.
/// - [`reference_slot_count`] is the number of the DPB Reference Pictures that will be used when
///   this decoding operation is executing.
/// - [`p_reference_slots`] is a pointer to an array of [`VideoReferenceSlotKHR`] structures
///   specifying the DPB Reference pictures that will be used when this decoding operation is
///   executing.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoDecodeH264PictureInfoEXT`] or
///   [`VideoDecodeH265PictureInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`VideoDecodeFlagBitsKHR`] values
/// - [`src_buffer`]**must** be a valid [`Buffer`] handle
/// - [`dst_picture_resource`]**must** be a valid [`VideoPictureResourceKHR`] structure
/// - [`p_setup_reference_slot`]**must** be a valid pointer to a valid [`VideoReferenceSlotKHR`]
///   structure
/// - If [`reference_slot_count`] is not `0`, [`p_reference_slots`]**must** be a valid pointer to an
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoDecodeInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    ///All the codec specific structures related to each frame(picture
    ///parameters, quantization matrix, etc.) **must** be chained here and pass to
    ///decode session with the function call [`CmdDecodeVideoKHR`].
    p_next: *mut BaseInStructure<'lt>,
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
    ///It **must** meet the alignment requirement
    ///`minBitstreamBufferOffsetAlignment` within
    ///[`VideoCapabilitiesKHR`] queried with the
    ///[`GetPhysicalDeviceVideoCapabilitiesKHR`] function.
    src_buffer_offset: DeviceSize,
    ///[`src_buffer_range`] is the size of the srcBuffer with valid encoded
    ///bitstream, starting from [`src_buffer_offset`].
    ///It **must** meet the alignment requirement
    ///`minBitstreamBufferSizeAlignment` within
    ///[`VideoCapabilitiesKHR`] queried with the
    ///[`GetPhysicalDeviceVideoCapabilitiesKHR`] function.
    src_buffer_range: DeviceSize,
    ///[`dst_picture_resource`] is the destination
    ///[Decoded Output Picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture) Resource.
    dst_picture_resource: VideoPictureResourceKHR<'lt>,
    ///[`p_setup_reference_slot`] is `NULL` or a pointer to a
    ///[`VideoReferenceSlotKHR`] structure used for generating a DPB
    ///reference slot and Picture Resource.
    ///`pSetupReferenceSlot->slotIndex` specifies the slot index number to
    ///use as a target for producing the DPB data.
    ///`slotIndex`**must** reference a valid entry as specified in
    ///[`VideoBeginCodingInfoKHR`] via the [`p_reference_slots`] within the
    ///[`CmdBeginVideoCodingKHR`] command that established the Vulkan Video
    ///Decode Context for this command.
    p_setup_reference_slot: *mut VideoReferenceSlotKHR<'lt>,
    ///[`reference_slot_count`] is the number of the DPB Reference Pictures
    ///that will be used when this decoding operation is executing.
    reference_slot_count: u32,
    ///[`p_reference_slots`] is a pointer to an array of
    ///[`VideoReferenceSlotKHR`] structures specifying the DPB Reference
    ///pictures that will be used when this decoding operation is executing.
    p_reference_slots: *mut VideoReferenceSlotKHR<'lt>,
}
