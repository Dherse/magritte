//![VK_EXT_video_decode_h265](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_video_decode_h265.html) - device extension
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_video_decode_queue`]`
//! - **This is a *provisional* extension and  **must**  be used with caution. See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header)
//!   of provisional header files for enablement and stability details.**
//!# Contacts
//! - [peter.fang@amd.com]()
//!# New structures
//! - Extending [`VideoDecodeCapabilitiesKHR`]:  - [`VideoDecodeH265CapabilitiesEXT`]
//! - Extending [`VideoDecodeInfoKHR`]:  - [`VideoDecodeH265PictureInfoEXT`]
//! - Extending [`VideoProfileKHR`], [`QueryPoolCreateInfo`], [`FormatProperties2`],
//!   [`ImageCreateInfo`], [`ImageViewCreateInfo`], [`BufferCreateInfo`]:  -
//!   [`VideoDecodeH265ProfileEXT`]
//! - Extending [`VideoReferenceSlotKHR`]:  - [`VideoDecodeH265DpbSlotInfoEXT`]
//! - Extending [`VideoSessionParametersCreateInfoKHR`]:  -
//!   [`VideoDecodeH265SessionParametersCreateInfoEXT`]
//! - Extending [`VideoSessionParametersUpdateInfoKHR`]:  -
//!   [`VideoDecodeH265SessionParametersAddInfoEXT`]
//!# New constants
//! - [`EXT_VIDEO_DECODE_H265_EXTENSION_NAME`]
//! - [`EXT_VIDEO_DECODE_H265_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_CAPABILITIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PICTURE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PROFILE_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT`
//! - Extending [`VideoCodecOperationFlagBitsKHR`]:  -
//!   `VK_VIDEO_CODEC_OPERATION_DECODE_H265_BIT_EXT`
//!# Version History
//! - Revision 1, 2018-6-11 (Peter Fang)  - Initial draft
//! - Revision 1.6, March 29 2021 (Tony Zlatinski)  - Spec and API updates.
//! - Revision 2, 2022-03-16 (Ahmed Abdelkhalek)  - Relocate Std header version reporting/requesting
//!   from this extension to VK_KHR_video_queue extension.  - Remove the now empty
//!   VkVideoDecodeH265SessionCreateInfoEXT.
//!# Other info
//! * 2022-03-16
//! * No known IP claims.
//! * - Ahmed Abdelkhalek, AMD  - HoHin Lau, AMD  - Jake Beju, AMD  - Peter Fang, AMD  - Ping Liu,
//!   Intel  - Srinath Kumarapuram, NVIDIA  - Tony Zlatinski, NVIDIA
//!# Related
//! - [`VideoDecodeH265CapabilitiesEXT`]
//! - [`VideoDecodeH265DpbSlotInfoEXT`]
//! - [`VideoDecodeH265PictureInfoEXT`]
//! - [`VideoDecodeH265ProfileEXT`]
//! - [`VideoDecodeH265SessionParametersAddInfoEXT`]
//! - [`VideoDecodeH265SessionParametersCreateInfoEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    native::{
        StdVideoDecodeH265PictureInfo, StdVideoDecodeH265ReferenceInfo, StdVideoH265PictureParameterSet,
        StdVideoH265ProfileIdc, StdVideoH265SequenceParameterSet, StdVideoH265VideoParameterSet,
    },
    vulkan1_0::{BaseInStructure, BaseOutStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VIDEO_DECODE_H265_SPEC_VERSION")]
pub const EXT_VIDEO_DECODE_H265_SPEC_VERSION: u32 = 2;
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
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PROFILE_EXT`
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
#[doc(alias = "VkVideoDecodeH265ProfileEXT")]
#[repr(C)]
pub struct VideoDecodeH265ProfileEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`std_profile_idc`] is a [`StdVideoH265ProfileIdc`] value specifying
    ///the H.265 codec profile IDC.
    pub std_profile_idc: StdVideoH265ProfileIdc,
}
impl<'lt> Default for VideoDecodeH265ProfileEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H265_PROFILE_EXT,
            p_next: std::ptr::null(),
            std_profile_idc: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> VideoDecodeH265ProfileEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::std_profile_idc`]
    pub fn std_profile_idc_raw(&self) -> &StdVideoH265ProfileIdc {
        &self.std_profile_idc
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::std_profile_idc`]
    pub fn set_std_profile_idc_raw(mut self, value: StdVideoH265ProfileIdc) -> Self {
        self.std_profile_idc = value;
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
    ///Gets the value of [`Self::std_profile_idc`]
    pub fn std_profile_idc(&self) -> StdVideoH265ProfileIdc {
        self.std_profile_idc
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::std_profile_idc`]
    pub fn std_profile_idc_mut(&mut self) -> &mut StdVideoH265ProfileIdc {
        &mut self.std_profile_idc
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::std_profile_idc`]
    pub fn set_std_profile_idc(mut self, value: crate::native::StdVideoH265ProfileIdc) -> Self {
        self.std_profile_idc = value;
        self
    }
}
///[VkVideoDecodeH265CapabilitiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265CapabilitiesEXT.html) - Structure specifies H.265 decode capabilities parameters when decoding a frame
///# C Specifications
///The [`VideoDecodeH265CapabilitiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_video_decode_h265
///typedef struct VkVideoDecodeH265CapabilitiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxLevel;
///} VkVideoDecodeH265CapabilitiesEXT;
///```
///# Members
///When using [`get_physical_device_video_capabilities_khr`] to query the
///capabilities for the parameter `videoCodecOperation` specified as
///`VK_VIDEO_CODEC_OPERATION_DECODE_H265_BIT_EXT`, a
///[`VideoDecodeH265CapabilitiesEXT`] structure  **can**  be chained to
///[`VideoCapabilitiesKHR`] to return this H.265 extension specific
///capabilities.
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_level`] is the maximum H.265 level supported by the device.
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_CAPABILITIES_EXT`
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
#[doc(alias = "VkVideoDecodeH265CapabilitiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoDecodeH265CapabilitiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_level`] is the maximum H.265 level supported by the device.
    pub max_level: u32,
}
impl<'lt> Default for VideoDecodeH265CapabilitiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H265_CAPABILITIES_EXT,
            p_next: std::ptr::null_mut(),
            max_level: 0,
        }
    }
}
impl<'lt> VideoDecodeH265CapabilitiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::max_level`]
    pub fn max_level(&self) -> u32 {
        self.max_level
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
    ///Gets a mutable reference to the value of [`Self::max_level`]
    pub fn max_level_mut(&mut self) -> &mut u32 {
        &mut self.max_level
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::max_level`]
    pub fn set_max_level(mut self, value: u32) -> Self {
        self.max_level = value;
        self
    }
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
///    uint32_t                                   vpsStdCount;
///    const StdVideoH265VideoParameterSet*       pVpsStd;
///    uint32_t                                   spsStdCount;
///    const StdVideoH265SequenceParameterSet*    pSpsStd;
///    uint32_t                                   ppsStdCount;
///    const StdVideoH265PictureParameterSet*     pPpsStd;
///} VkVideoDecodeH265SessionParametersAddInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`vps_std_count`] is the number of VPS elements in [`vps_std`].
/// - [`vps_std`] is a pointer to an array of [`vps_std_count`][`StdVideoH265VideoParameterSet`]
///   structures representing H.265 video parameter sets.
/// - [`sps_std_count`] is the number of SPS elements in the [`sps_std`]. Its value  **must**  be
///   less than or equal to the value of `maxSpsStdCount`.
/// - [`sps_std`] is a pointer to an array of [`StdVideoH265SequenceParameterSet`] structures
///   representing H.265 sequence parameter sets. Each element of the array  **must**  have a unique
///   H.265 VPS-SPS ID pair.
/// - [`pps_std_count`] is the number of PPS provided in [`pps_std`]. Its value  **must**  be less
///   than or equal to the value of `maxPpsStdCount`.
/// - [`pps_std`] is a pointer to an array of [`StdVideoH265PictureParameterSet`] structures
///   representing H.265 picture parameter sets. Each element of the array entry  **must**  have a
///   unique H.265 VPS-SPS-PPS ID tuple.
///# Description
///## Valid Usage
/// - The values of [`vps_std_count`], [`sps_std_count`] and [`pps_std_count`] **must**  be less
///   than or equal to the values of `maxVpsStdCount`, `maxSpsStdCount` and `maxPpsStdCount`,
///   respectively
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
/// - Each entry to be added  **must**  have a unique, to the rest of the parameter array entries
///   and the existing parameters in the Video Session Parameters Object that is being updated,
///   VPS-SPS-PPS IDs
/// - Parameter entries that already exist in Video Session Parameters object with a particular
///   VPS-SPS-PPS IDs  **cannot**  be replaced nor updated
/// - When creating a new object using a Video Session Parameters as a template, the array’s
///   parameters with the same VPS-SPS-PPS IDs as the ones from the template take precedence
/// - VPS/SPS/PPS parameters  **must**  comply with the limits specified in
///   [`VideoSessionCreateInfoKHR`] during Video Session creation
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT`
/// - If [`vps_std`] is not `NULL`, [`vps_std`] **must**  be a valid pointer to an array of
///   [`vps_std_count`][`StdVideoH265VideoParameterSet`] values
/// - If [`sps_std`] is not `NULL`, [`sps_std`] **must**  be a valid pointer to an array of
///   [`sps_std_count`][`StdVideoH265SequenceParameterSet`] values
/// - If [`pps_std`] is not `NULL`, [`pps_std`] **must**  be a valid pointer to an array of
///   [`pps_std_count`][`StdVideoH265PictureParameterSet`] values
/// - [`vps_std_count`] **must**  be greater than `0`
/// - [`sps_std_count`] **must**  be greater than `0`
/// - [`pps_std_count`] **must**  be greater than `0`
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
#[doc(alias = "VkVideoDecodeH265SessionParametersAddInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoDecodeH265SessionParametersAddInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`vps_std_count`] is the number of VPS elements in [`vps_std`].
    pub vps_std_count: u32,
    ///[`vps_std`] is a pointer to an array of [`vps_std_count`][`StdVideoH265VideoParameterSet`]
    /// structures representing H.265 video parameter sets.
    pub vps_std: *const StdVideoH265VideoParameterSet,
    ///[`sps_std_count`] is the number of SPS elements in the [`sps_std`].
    ///Its value  **must**  be less than or equal to the value of
    ///`maxSpsStdCount`.
    pub sps_std_count: u32,
    ///[`sps_std`] is a pointer to an array of
    ///[`StdVideoH265SequenceParameterSet`] structures representing H.265
    ///sequence parameter sets.
    ///Each element of the array  **must**  have a unique H.265 VPS-SPS ID pair.
    pub sps_std: *const StdVideoH265SequenceParameterSet,
    ///[`pps_std_count`] is the number of PPS provided in [`pps_std`].
    ///Its value  **must**  be less than or equal to the value of
    ///`maxPpsStdCount`.
    pub pps_std_count: u32,
    ///[`pps_std`] is a pointer to an array of
    ///[`StdVideoH265PictureParameterSet`] structures representing H.265
    ///picture parameter sets.
    ///Each element of the array entry  **must**  have a unique H.265 VPS-SPS-PPS ID
    ///tuple.
    pub pps_std: *const StdVideoH265PictureParameterSet,
}
impl<'lt> Default for VideoDecodeH265SessionParametersAddInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT,
            p_next: std::ptr::null(),
            vps_std_count: 0,
            vps_std: std::ptr::null(),
            sps_std_count: 0,
            sps_std: std::ptr::null(),
            pps_std_count: 0,
            pps_std: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH265SessionParametersAddInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::vps_std_count`]
    pub fn vps_std_count(&self) -> u32 {
        self.vps_std_count
    }
    ///Gets the value of [`Self::vps_std`]
    pub fn vps_std(&self) -> *const StdVideoH265VideoParameterSet {
        self.vps_std
    }
    ///Gets the value of [`Self::sps_std_count`]
    pub fn sps_std_count(&self) -> u32 {
        self.sps_std_count
    }
    ///Gets the value of [`Self::sps_std`]
    pub fn sps_std(&self) -> *const StdVideoH265SequenceParameterSet {
        self.sps_std
    }
    ///Gets the value of [`Self::pps_std_count`]
    pub fn pps_std_count(&self) -> u32 {
        self.pps_std_count
    }
    ///Gets the value of [`Self::pps_std`]
    pub fn pps_std(&self) -> *const StdVideoH265PictureParameterSet {
        self.pps_std
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::vps_std_count`]
    pub fn vps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.vps_std_count
    }
    ///Gets a mutable reference to the value of [`Self::sps_std_count`]
    pub fn sps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.sps_std_count
    }
    ///Gets a mutable reference to the value of [`Self::pps_std_count`]
    pub fn pps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.pps_std_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::vps_std_count`]
    pub fn set_vps_std_count(mut self, value: u32) -> Self {
        self.vps_std_count = value;
        self
    }
    ///Sets the value of [`Self::vps_std`]
    pub fn set_vps_std(mut self, value: *const crate::native::StdVideoH265VideoParameterSet) -> Self {
        self.vps_std = value;
        self
    }
    ///Sets the value of [`Self::sps_std_count`]
    pub fn set_sps_std_count(mut self, value: u32) -> Self {
        self.sps_std_count = value;
        self
    }
    ///Sets the value of [`Self::sps_std`]
    pub fn set_sps_std(mut self, value: *const crate::native::StdVideoH265SequenceParameterSet) -> Self {
        self.sps_std = value;
        self
    }
    ///Sets the value of [`Self::pps_std_count`]
    pub fn set_pps_std_count(mut self, value: u32) -> Self {
        self.pps_std_count = value;
        self
    }
    ///Sets the value of [`Self::pps_std`]
    pub fn set_pps_std(mut self, value: *const crate::native::StdVideoH265PictureParameterSet) -> Self {
        self.pps_std = value;
        self
    }
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
///    uint32_t                                               maxVpsStdCount;
///    uint32_t                                               maxSpsStdCount;
///    uint32_t                                               maxPpsStdCount;
///    const VkVideoDecodeH265SessionParametersAddInfoEXT*    pParametersAddInfo;
///} VkVideoDecodeH265SessionParametersCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_vps_std_count`] is the maximum number of entries of type
///   [`StdVideoH265VideoParameterSet`] within [`VideoSessionParametersKHR`].
/// - [`max_sps_std_count`] is the maximum number of SPS parameters that the
///   [`VideoSessionParametersKHR`] can contain.
/// - [`max_pps_std_count`] is the maximum number of PPS parameters that the
///   [`VideoSessionParametersKHR`] can contain.
/// - [`parameters_add_info`] is `NULL` or a pointer to a
///   [`VideoDecodeH265SessionParametersAddInfoEXT`] structure specifying H.265 parameters to add
///   upon object creation.
///# Description
///When a [`VideoSessionParametersKHR`] object contains
///[`max_vps_std_count`][`StdVideoH265VideoParameterSet`] entries, no
///additional [`StdVideoH265VideoParameterSet`] entries can be added to it,
///and `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made
///to add these entries.
///When a [`VideoSessionParametersKHR`] object contains
///[`max_sps_std_count`][`StdVideoH265SequenceParameterSet`] entries, no
///additional [`StdVideoH265SequenceParameterSet`] entries can be added to it,
///and `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made
///to add these entries.
///When a [`VideoSessionParametersKHR`] object contains
///[`max_pps_std_count`][`StdVideoH265PictureParameterSet`] entries, no
///additional [`StdVideoH265PictureParameterSet`] entries can be added to it,
///and `VK_ERROR_TOO_MANY_OBJECTS` will be returned if an attempt is made
///to add these entries.The provided H.265 VPS/SPS/PPS parameters  **must**  be within the limits
///specified during decoder creation for the decoder specified in
///[`VideoSessionParametersCreateInfoKHR`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT`
/// - If [`parameters_add_info`] is not `NULL`, [`parameters_add_info`] **must**  be a valid pointer
///   to a valid [`VideoDecodeH265SessionParametersAddInfoEXT`] structure
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
#[doc(alias = "VkVideoDecodeH265SessionParametersCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoDecodeH265SessionParametersCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`max_vps_std_count`] is the maximum number of entries of type
    ///[`StdVideoH265VideoParameterSet`] within
    ///[`VideoSessionParametersKHR`].
    pub max_vps_std_count: u32,
    ///[`max_sps_std_count`] is the maximum number of SPS parameters that the
    ///[`VideoSessionParametersKHR`] can contain.
    pub max_sps_std_count: u32,
    ///[`max_pps_std_count`] is the maximum number of PPS parameters that the
    ///[`VideoSessionParametersKHR`] can contain.
    pub max_pps_std_count: u32,
    ///[`parameters_add_info`] is `NULL` or a pointer to a
    ///[`VideoDecodeH265SessionParametersAddInfoEXT`] structure specifying
    ///H.265 parameters to add upon object creation.
    pub parameters_add_info: *const VideoDecodeH265SessionParametersAddInfoEXT<'lt>,
}
impl<'lt> Default for VideoDecodeH265SessionParametersCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            max_vps_std_count: 0,
            max_sps_std_count: 0,
            max_pps_std_count: 0,
            parameters_add_info: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH265SessionParametersCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::parameters_add_info`]
    pub fn parameters_add_info_raw(&self) -> *const VideoDecodeH265SessionParametersAddInfoEXT<'lt> {
        self.parameters_add_info
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::parameters_add_info`]
    pub fn set_parameters_add_info_raw(
        mut self,
        value: *const VideoDecodeH265SessionParametersAddInfoEXT<'lt>,
    ) -> Self {
        self.parameters_add_info = value;
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
    ///Gets the value of [`Self::max_vps_std_count`]
    pub fn max_vps_std_count(&self) -> u32 {
        self.max_vps_std_count
    }
    ///Gets the value of [`Self::max_sps_std_count`]
    pub fn max_sps_std_count(&self) -> u32 {
        self.max_sps_std_count
    }
    ///Gets the value of [`Self::max_pps_std_count`]
    pub fn max_pps_std_count(&self) -> u32 {
        self.max_pps_std_count
    }
    ///Gets the value of [`Self::parameters_add_info`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn parameters_add_info(&self) -> &VideoDecodeH265SessionParametersAddInfoEXT<'lt> {
        &*self.parameters_add_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::max_vps_std_count`]
    pub fn max_vps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.max_vps_std_count
    }
    ///Gets a mutable reference to the value of [`Self::max_sps_std_count`]
    pub fn max_sps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.max_sps_std_count
    }
    ///Gets a mutable reference to the value of [`Self::max_pps_std_count`]
    pub fn max_pps_std_count_mut(&mut self) -> &mut u32 {
        &mut self.max_pps_std_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::max_vps_std_count`]
    pub fn set_max_vps_std_count(mut self, value: u32) -> Self {
        self.max_vps_std_count = value;
        self
    }
    ///Sets the value of [`Self::max_sps_std_count`]
    pub fn set_max_sps_std_count(mut self, value: u32) -> Self {
        self.max_sps_std_count = value;
        self
    }
    ///Sets the value of [`Self::max_pps_std_count`]
    pub fn set_max_pps_std_count(mut self, value: u32) -> Self {
        self.max_pps_std_count = value;
        self
    }
    ///Sets the value of [`Self::parameters_add_info`]
    pub fn set_parameters_add_info(
        mut self,
        value: &'lt crate::extensions::ext_video_decode_h_265::VideoDecodeH265SessionParametersAddInfoEXT<'lt>,
    ) -> Self {
        self.parameters_add_info = value as *const _;
        self
    }
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
/// - [`std_picture_info`] is a pointer to a [`StdVideoDecodeH265PictureInfo`] structure specifying
///   codec standard specific picture information from the H.265 specification.
/// - [`slices_count`] is the number of slices in this picture.
/// - [`slices_data_offsets`] is a pointer to an array of [`slices_count`] offsets indicating the
///   start offset of each slice within the bitstream buffer.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PICTURE_INFO_EXT`
/// - [`std_picture_info`] **must**  be a valid pointer to a [`StdVideoDecodeH265PictureInfo`] value
/// - [`slices_data_offsets`] **must**  be a valid pointer to an array of [`slices_count`]`uint32_t`
///   values
/// - [`slices_count`] **must**  be greater than `0`
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
#[doc(alias = "VkVideoDecodeH265PictureInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoDecodeH265PictureInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`std_picture_info`] is a pointer to a
    ///[`StdVideoDecodeH265PictureInfo`] structure specifying codec standard
    ///specific picture information from the H.265 specification.
    pub std_picture_info: *mut StdVideoDecodeH265PictureInfo,
    ///[`slices_count`] is the number of slices in this picture.
    pub slices_count: u32,
    ///[`slices_data_offsets`] is a pointer to an array of [`slices_count`]
    ///offsets indicating the start offset of each slice within the bitstream
    ///buffer.
    pub slices_data_offsets: *const u32,
}
impl<'lt> Default for VideoDecodeH265PictureInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H265_PICTURE_INFO_EXT,
            p_next: std::ptr::null(),
            std_picture_info: std::ptr::null_mut(),
            slices_count: 0,
            slices_data_offsets: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH265PictureInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::slices_data_offsets`]
    pub fn slices_data_offsets_raw(&self) -> *const u32 {
        self.slices_data_offsets
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::slices_data_offsets`]
    pub fn set_slices_data_offsets_raw(mut self, value: *const u32) -> Self {
        self.slices_data_offsets = value;
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
    ///Gets the value of [`Self::std_picture_info`]
    pub fn std_picture_info(&self) -> *mut StdVideoDecodeH265PictureInfo {
        self.std_picture_info
    }
    ///Gets the value of [`Self::slices_count`]
    pub fn slices_count(&self) -> u32 {
        self.slices_count
    }
    ///Gets the value of [`Self::slices_data_offsets`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn slices_data_offsets(&self) -> &[u32] {
        std::slice::from_raw_parts(self.slices_data_offsets, self.slices_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::std_picture_info`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn std_picture_info_mut(&mut self) -> &mut StdVideoDecodeH265PictureInfo {
        &mut *self.std_picture_info
    }
    ///Gets a mutable reference to the value of [`Self::slices_count`]
    pub fn slices_count_mut(&mut self) -> &mut u32 {
        &mut self.slices_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::std_picture_info`]
    pub fn set_std_picture_info(mut self, value: *mut crate::native::StdVideoDecodeH265PictureInfo) -> Self {
        self.std_picture_info = value;
        self
    }
    ///Sets the value of [`Self::slices_count`]
    pub fn set_slices_count(mut self, value: u32) -> Self {
        self.slices_count = value;
        self
    }
    ///Sets the value of [`Self::slices_data_offsets`]
    pub fn set_slices_data_offsets(mut self, value: &'lt [u32]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.slices_data_offsets = value.as_ptr();
        self.slices_count = len_;
        self
    }
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
/// - [`std_reference_info`] is a pointer to a [`StdVideoDecodeH265ReferenceInfo`] structure
///   specifying the codec standard specific picture reference information from the H.264
///   specification.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT`
/// - [`std_reference_info`] **must**  be a valid pointer to a valid
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
#[doc(alias = "VkVideoDecodeH265DpbSlotInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct VideoDecodeH265DpbSlotInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`std_reference_info`] is a pointer to a
    ///[`StdVideoDecodeH265ReferenceInfo`] structure specifying the codec
    ///standard specific picture reference information from the H.264
    ///specification.
    pub std_reference_info: *const StdVideoDecodeH265ReferenceInfo,
}
impl<'lt> Default for VideoDecodeH265DpbSlotInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT,
            p_next: std::ptr::null(),
            std_reference_info: std::ptr::null(),
        }
    }
}
impl<'lt> VideoDecodeH265DpbSlotInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::std_reference_info`]
    pub fn std_reference_info(&self) -> *const StdVideoDecodeH265ReferenceInfo {
        self.std_reference_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::std_reference_info`]
    pub fn set_std_reference_info(mut self, value: *const crate::native::StdVideoDecodeH265ReferenceInfo) -> Self {
        self.std_reference_info = value;
        self
    }
}
