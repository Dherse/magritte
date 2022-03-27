use crate::{
    native::{
        StdVideoDecodeH265PictureInfo, StdVideoDecodeH265ReferenceInfo, StdVideoH265PictureParameterSet,
        StdVideoH265ProfileIdc, StdVideoH265SequenceParameterSet,
    },
    vulkan1_0::{BaseInStructure, BaseOutStructure, ExtensionProperties, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_DECODE_H265_SPEC_VERSION")]
pub const EXT_VIDEO_DECODE_H265_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_DECODE_H265_EXTENSION_NAME")]
pub const EXT_VIDEO_DECODE_H265_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_video_decode_h265");
///[VkVideoDecodeH265ProfileEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265ProfileEXT.html) - Structure specifying H.265 decode profile
///# C Specifications
///The [`VideoDecodeH265ProfileEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h265
///typedef struct VkVideoDecodeH265ProfileEXT {
///    VkStructureType           sType;
///    const void*               pNext;
///    StdVideoH265ProfileIdc    stdProfileIdc;
///} VkVideoDecodeH265ProfileEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`std_profile_idc`] is a [`StdVideoH265ProfileIdc`] value specifying the H.265 codec profile
///   IDC.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PROFILE_EXT`
///# Related
/// - [`VK_EXT_video_decode_h265`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VideoDecodeH265ProfileEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`std_profile_idc`] is a [`StdVideoH265ProfileIdc`] value specifying
    ///the H.265 codec profile IDC.
    std_profile_idc: StdVideoH265ProfileIdc,
}
///[VkVideoDecodeH265CapabilitiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265CapabilitiesEXT.html) - Structure specifies H.265 decode capabilities parameters when decoding a frame
///# C Specifications
///The [`VideoDecodeH265CapabilitiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h265
///typedef struct VkVideoDecodeH265CapabilitiesEXT {
///    VkStructureType          sType;
///    void*                    pNext;
///    uint32_t                 maxLevel;
///    VkExtensionProperties    stdExtensionVersion;
///} VkVideoDecodeH265CapabilitiesEXT;
///```
///# Members
///When using [`GetPhysicalDeviceVideoCapabilitiesKHR`] to query the
///capabilities for the parameter `videoCodecOperation` specified as
///`VK_VIDEO_CODEC_OPERATION_DECODE_H265_BIT_EXT`, a
///[`VideoDecodeH265CapabilitiesEXT`] structure **can** be chained to
///[`VideoCapabilitiesKHR`] to return this H.265 extension specific
///capabilities.
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_level`] is the maximum H.265 level supported by the device.
/// - [`std_extension_version`] is a [`ExtensionProperties`] structure specifying the H.265
///   extension name and version supported by this implementation.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_CAPABILITIES_EXT`
///# Related
/// - [`VK_EXT_video_decode_h265`]
/// - [`ExtensionProperties`]
/// - [`StructureType`]
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
pub struct VideoDecodeH265CapabilitiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`max_level`] is the maximum H.265 level supported by the device.
    max_level: u32,
    ///[`std_extension_version`] is a [`ExtensionProperties`] structure
    ///specifying the H.265 extension name and version supported by this
    ///implementation.
    std_extension_version: ExtensionProperties,
}
///[VkVideoDecodeH265SessionCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265SessionCreateInfoEXT.html) - Structure specifies H.265 decode session creation parameters
///# C Specifications
///The [`VideoDecodeH265SessionCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h265
///typedef struct VkVideoDecodeH265SessionCreateInfoEXT {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkVideoDecodeH265CreateFlagsEXT    flags;
///    const VkExtensionProperties*       pStdExtensionVersion;
///} VkVideoDecodeH265SessionCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`p_std_extension_version`] is a pointer to a [`ExtensionProperties`] structure specifying
///   H.265 codec extensions.
///# Description
///A [`VideoDecodeH265SessionCreateInfoEXT`] structure **can** be chained to
///[`VideoSessionCreateInfoKHR`] when the function
///[`CreateVideoSessionKHR`] is called to create a video session for H.265
///decode operations.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_CREATE_INFO_EXT`
/// - [`flags`]**must** be `0`
/// - [`p_std_extension_version`]**must** be a valid pointer to a valid [`ExtensionProperties`]
///   structure
///# Related
/// - [`VK_EXT_video_decode_h265`]
/// - [`ExtensionProperties`]
/// - [`StructureType`]
/// - [`VideoDecodeH265CreateFlagsEXT`]
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
pub struct VideoDecodeH265SessionCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: VideoDecodeH265CreateFlagsEXT,
    ///[`p_std_extension_version`] is a pointer to a [`ExtensionProperties`]
    ///structure specifying H.265 codec extensions.
    p_std_extension_version: *mut ExtensionProperties,
}
///[VkVideoDecodeH265SessionParametersAddInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265SessionParametersAddInfoEXT.html) - Structure specifies H.265 decoder parameter set information
///# C Specifications
///The [`VideoDecodeH265SessionParametersAddInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_video_decode_h265
///typedef struct VkVideoDecodeH265SessionParametersAddInfoEXT {
///    VkStructureType                            sType;
///    const void*                                pNext;
///    uint32_t                                   spsStdCount;
///    const StdVideoH265SequenceParameterSet*    pSpsStd;
///    uint32_t                                   ppsStdCount;
///    const StdVideoH265PictureParameterSet*     pPpsStd;
///} VkVideoDecodeH265SessionParametersAddInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`sps_std_count`] is the number of SPS elements in the [`p_sps_std`]. Its value **must** be
///   less than or equal to the value of `maxSpsStdCount`.
/// - [`p_sps_std`] is a pointer to an array of [`StdVideoH265SequenceParameterSet`] structures
///   representing H.265 sequence parameter sets. Each element of the array **must** have a unique
///   H.265 VPS-SPS ID pair.
/// - [`pps_std_count`] is the number of PPS provided in [`p_pps_std`]. Its value **must** be less
///   than or equal to the value of `maxPpsStdCount`.
/// - [`p_pps_std`] is a pointer to an array of [`StdVideoH265PictureParameterSet`] structures
///   representing H.265 picture parameter sets. Each element of the array entry **must** have a
///   unique H.265 VPS-SPS-PPS ID tuple.
///# Description
///Valid Usage
/// - The values of `vpsStdCount`, [`sps_std_count`] and [`pps_std_count`]**must** be less than or
///   equal to the values of `maxVpsStdCount`, `maxSpsStdCount` and `maxPpsStdCount`, respectively
/// - When the `maxVpsStdCount` number of parameters of type StdVideoH265VideoParameterSet in the
///   Video Session Parameters object is reached, no additional parameters of that type can be added
///   to the object. `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made to add
///   additional data to this object at this point
/// - When the `maxSpsStdCount` number of parameters of type StdVideoH265SequenceParameterSet in the
///   Video Session Parameters object is reached, no additional parameters of that type can be added
///   to the object. `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made to add
///   additional data to this object at this point
/// - When the `maxPpsStdCount` number of parameters of type StdVideoH265PictureParameterSet in the
///   Video Session Parameters object is reached, no additional parameters of that type can be added
///   to the object. `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made to add
///   additional data to this object at this point
/// - Each entry to be added **must** have a unique, to the rest of the parameter array entries and
///   the existing parameters in the Video Session Parameters Object that is being updated,
///   VPS-SPS-PPS IDs
/// - Parameter entries that already exist in Video Session Parameters object with a particular
///   VPS-SPS-PPS IDs **cannot** be replaced nor updated
/// - When creating a new object using a Video Session Parameters as a template, the array’s
///   parameters with the same VPS-SPS-PPS IDs as the ones from the template take precedence
/// - VPS/SPS/PPS parameters **must** comply with the limits specified in
///   [`VideoSessionCreateInfoKHR`] during Video Session creation
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT`
/// - If [`p_sps_std`] is not `NULL`, [`p_sps_std`]**must** be a valid pointer to an array of
///   [`sps_std_count`][`StdVideoH265SequenceParameterSet`] values
/// - If [`p_pps_std`] is not `NULL`, [`p_pps_std`]**must** be a valid pointer to an array of
///   [`pps_std_count`][`StdVideoH265PictureParameterSet`] values
/// - [`sps_std_count`]**must** be greater than `0`
/// - [`pps_std_count`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_video_decode_h265`]
/// - [`StructureType`]
/// - [`VideoDecodeH265SessionParametersCreateInfoEXT`]
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
pub struct VideoDecodeH265SessionParametersAddInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`sps_std_count`] is the number of SPS elements in the [`p_sps_std`].
    ///Its value **must** be less than or equal to the value of
    ///`maxSpsStdCount`.
    sps_std_count: u32,
    ///[`p_sps_std`] is a pointer to an array of
    ///[`StdVideoH265SequenceParameterSet`] structures representing H.265
    ///sequence parameter sets.
    ///Each element of the array **must** have a unique H.265 VPS-SPS ID pair.
    p_sps_std: *mut StdVideoH265SequenceParameterSet,
    ///[`pps_std_count`] is the number of PPS provided in [`p_pps_std`].
    ///Its value **must** be less than or equal to the value of
    ///`maxPpsStdCount`.
    pps_std_count: u32,
    ///[`p_pps_std`] is a pointer to an array of
    ///[`StdVideoH265PictureParameterSet`] structures representing H.265
    ///picture parameter sets.
    ///Each element of the array entry **must** have a unique H.265 VPS-SPS-PPS ID
    ///tuple.
    p_pps_std: *mut StdVideoH265PictureParameterSet,
}
///[VkVideoDecodeH265SessionParametersCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265SessionParametersCreateInfoEXT.html) - Structure specifies H.265 decoder parameter set information
///# C Specifications
///The [`VideoDecodeH265SessionParametersCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_video_decode_h265
///typedef struct VkVideoDecodeH265SessionParametersCreateInfoEXT {
///    VkStructureType                                        sType;
///    const void*                                            pNext;
///    uint32_t                                               maxSpsStdCount;
///    uint32_t                                               maxPpsStdCount;
///    const VkVideoDecodeH265SessionParametersAddInfoEXT*    pParametersAddInfo;
///} VkVideoDecodeH265SessionParametersCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_sps_std_count`] is the maximum number of SPS parameters that the
///   [`VideoSessionParametersKHR`] can contain.
/// - [`max_pps_std_count`] is the maximum number of PPS parameters that the
///   [`VideoSessionParametersKHR`] can contain.
/// - [`p_parameters_add_info`] is `NULL` or a pointer to a
///   [`VideoDecodeH265SessionParametersAddInfoEXT`] structure specifying H.265 parameters to add
///   upon object creation.
///# Description
///A [`VideoDecodeH265SessionParametersCreateInfoEXT`] structure holding
///one H.265 SPS and at least one H.265 PPS paramater set **must** be chained to
///[`VideoSessionParametersCreateInfoKHR`] when calling
///[`CreateVideoSessionParametersKHR`] to store these parameter set(s) with
///the decoder parameter set object for later reference.
///The provided H.265 SPS/PPS parameters **must** be within the limits specified
///during decoder creation for the decoder specified in
///[`VideoSessionParametersCreateInfoKHR`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT`
/// - If [`p_parameters_add_info`] is not `NULL`, [`p_parameters_add_info`]**must** be a valid
///   pointer to a valid [`VideoDecodeH265SessionParametersAddInfoEXT`] structure
///# Related
/// - [`VK_EXT_video_decode_h265`]
/// - [`StructureType`]
/// - [`VideoDecodeH265SessionParametersAddInfoEXT`]
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
pub struct VideoDecodeH265SessionParametersCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`max_sps_std_count`] is the maximum number of SPS parameters that the
    ///[`VideoSessionParametersKHR`] can contain.
    max_sps_std_count: u32,
    ///[`max_pps_std_count`] is the maximum number of PPS parameters that the
    ///[`VideoSessionParametersKHR`] can contain.
    max_pps_std_count: u32,
    ///[`p_parameters_add_info`] is `NULL` or a pointer to a
    ///[`VideoDecodeH265SessionParametersAddInfoEXT`] structure specifying
    ///H.265 parameters to add upon object creation.
    p_parameters_add_info: *mut VideoDecodeH265SessionParametersAddInfoEXT<'lt>,
}
///[VkVideoDecodeH265PictureInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265PictureInfoEXT.html) - Structure specifies H.265 picture information when decoding a frame
///# C Specifications
///The [`VideoDecodeH265PictureInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h265
///typedef struct VkVideoDecodeH265PictureInfoEXT {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    StdVideoDecodeH265PictureInfo*    pStdPictureInfo;
///    uint32_t                          slicesCount;
///    const uint32_t*                   pSlicesDataOffsets;
///} VkVideoDecodeH265PictureInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`p_std_picture_info`] is a pointer to a [`StdVideoDecodeH265PictureInfo`] structure
///   specifying codec standard specific picture information from the H.265 specification.
/// - [`slices_count`] is the number of slices in this picture.
/// - [`p_slices_data_offsets`] is a pointer to an array of [`slices_count`] offsets indicating the
///   start offset of each slice within the bitstream buffer.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PICTURE_INFO_EXT`
/// - [`p_std_picture_info`]**must** be a valid pointer to a [`StdVideoDecodeH265PictureInfo`] value
/// - [`p_slices_data_offsets`]**must** be a valid pointer to an array of [`slices_count`]`uint32_t`
///   values
/// - [`slices_count`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_video_decode_h265`]
/// - [`StructureType`]
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
pub struct VideoDecodeH265PictureInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`p_std_picture_info`] is a pointer to a
    ///[`StdVideoDecodeH265PictureInfo`] structure specifying codec standard
    ///specific picture information from the H.265 specification.
    p_std_picture_info: *const StdVideoDecodeH265PictureInfo,
    ///[`slices_count`] is the number of slices in this picture.
    slices_count: u32,
    ///[`p_slices_data_offsets`] is a pointer to an array of [`slices_count`]
    ///offsets indicating the start offset of each slice within the bitstream
    ///buffer.
    p_slices_data_offsets: *mut u32,
}
///[VkVideoDecodeH265DpbSlotInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265DpbSlotInfoEXT.html) - Structure specifies H.265 DPB information when decoding a frame
///# C Specifications
///The [`VideoDecodeH265DpbSlotInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h265
///typedef struct VkVideoDecodeH265DpbSlotInfoEXT {
///    VkStructureType                           sType;
///    const void*                               pNext;
///    const StdVideoDecodeH265ReferenceInfo*    pStdReferenceInfo;
///} VkVideoDecodeH265DpbSlotInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`p_std_reference_info`] is a pointer to a [`StdVideoDecodeH265ReferenceInfo`] structure
///   specifying the codec standard specific picture reference information from the H.264
///   specification.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT`
/// - [`p_std_reference_info`]**must** be a valid pointer to a valid
///   [`StdVideoDecodeH265ReferenceInfo`] value
///# Related
/// - [`VK_EXT_video_decode_h265`]
/// - [`StructureType`]
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
pub struct VideoDecodeH265DpbSlotInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`p_std_reference_info`] is a pointer to a
    ///[`StdVideoDecodeH265ReferenceInfo`] structure specifying the codec
    ///standard specific picture reference information from the H.264
    ///specification.
    p_std_reference_info: *mut StdVideoDecodeH265ReferenceInfo,
}
