//![VK_EXT_ycbcr_2plane_444_formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_ycbcr_2plane_444_formats.html) - device extension
//!# Description
//!This extension adds some Y′C<sub>B</sub>C<sub>R</sub> formats that are in common use for video
//!encode and decode, but were not part of the
//!`[`khr_sampler_ycbcr_conversion`]` extension.
# ! [doc = concat ! ("# " , "Revision")]
//!1
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
//! - Requires `[`khr_sampler_ycbcr_conversion`]`
# ! [doc = concat ! ("# " , "Deprecation State")]
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
# ! [doc = concat ! ("# " , "Contacts")]
//! - Tony Zlatinski [tzlatinski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_ycbcr_2plane_444_formats]
//!   @tzlatinski%0A<<Here describe the issue or question you have about the
//!   VK_EXT_ycbcr_2plane_444_formats extension>>)
# ! [doc = concat ! ("# " , "New structures")]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME`]
//! - [`EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION`]
//! - Extending [`Format`]:  - `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT`  -
//!   `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT`  -
//!   `VK_FORMAT_G16_B16R16_2PLANE_444_UNORM_EXT`  - `VK_FORMAT_G8_B8R8_2PLANE_444_UNORM_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT`
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2020-03-08 (Piers Daniell)  - Initial draft
//!# Other info
//! * 2020-07-28
//! * - Promoted to Vulkan 1.3 Core
//! * No known IP claims.
//! * - Piers Daniell, NVIDIA  - Ping Liu, Intel
//!# Related
//! - [`PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION")]
pub const EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME")]
pub const EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_ycbcr_2plane_444_formats");
///[VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT.html) - Structure describing whether the implementation supports additional 2-plane 444 Y′C<sub>B</sub>C<sub>R</sub> formats
///# C Specifications
///The [`PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_ycbcr_2plane_444_formats
///typedef struct VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           ycbcr2plane444Formats;
///} VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`ycbcr2plane444_formats`] indicates that the implementation supports the following 2-plane
///   444 Y′C<sub>B</sub>C<sub>R</sub> formats:  - `VK_FORMAT_G8_B8R8_2PLANE_444_UNORM`  -
///   `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16`  -
///   `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16`  -
///   `VK_FORMAT_G16_B16R16_2PLANE_444_UNORM`
///If the [`PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT`] **can**  also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT`
///# Related
/// - [`ext_ycbcr_2plane_444_formats`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`ycbcr2plane444_formats`] indicates
    ///that the implementation supports the following 2-plane 444 Y′C<sub>B</sub>C<sub>R</sub>
    ///formats:
    /// - `VK_FORMAT_G8_B8R8_2PLANE_444_UNORM`
    /// - `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16`
    /// - `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16`
    /// - `VK_FORMAT_G16_B16R16_2PLANE_444_UNORM`
    pub ycbcr2plane444_formats: Bool32,
}
impl<'lt> Default for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_YCBCR2_PLANE_444_FORMATS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            ycbcr2plane444_formats: 0,
        }
    }
}
impl<'lt> PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::ycbcr2plane444_formats`]
    pub fn ycbcr2plane444_formats_raw(&self) -> Bool32 {
        self.ycbcr2plane444_formats
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::ycbcr2plane444_formats`]
    pub fn set_ycbcr2plane444_formats_raw(&mut self, value: Bool32) -> &mut Self {
        self.ycbcr2plane444_formats = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::ycbcr2plane444_formats`]
    pub fn with_ycbcr2plane444_formats_raw(mut self, value: Bool32) -> Self {
        self.ycbcr2plane444_formats = value;
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
    ///Gets the value of [`Self::ycbcr2plane444_formats`]
    pub fn ycbcr2plane444_formats(&self) -> bool {
        unsafe { std::mem::transmute(self.ycbcr2plane444_formats as u8) }
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
    ///Gets a mutable reference to the value of [`Self::ycbcr2plane444_formats`]
    pub fn ycbcr2plane444_formats_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.ycbcr2plane444_formats as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.ycbcr2plane444_formats as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::ycbcr2plane444_formats`]
    pub fn set_ycbcr2plane444_formats(&mut self, value: bool) -> &mut Self {
        self.ycbcr2plane444_formats = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::ycbcr2plane444_formats`]
    pub fn with_ycbcr2plane444_formats(mut self, value: bool) -> Self {
        self.ycbcr2plane444_formats = value as u8 as u32;
        self
    }
}
