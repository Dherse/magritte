//![VK_EXT_rgba10x6_formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_rgba10x6_formats.html) - device extension
//!# Description
//!This extension enables the
//!`VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16` format to be used without
//!a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) enabled.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_sampler_ycbcr_conversion`]`
//!# Contacts
//! - Jan-Harald Fredriksen [janharaldfredriksen-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_rgba10x6_formats]
//!   @janharaldfredriksen-arm%0A<<Here describe the issue or question you have about the
//!   VK_EXT_rgba10x6_formats extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceRgba10X6FormatsFeaturesEXT`]
//!# New constants
//! - [`EXT_RGBA10X6_FORMATS_EXTENSION_NAME`]
//! - [`EXT_RGBA10X6_FORMATS_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT`
//!# Known issues & F.A.Q
//!1) Should we reuse the existing format enumeration or introduce a new one? **RESOLVED** : We
//! reuse an existing format enumeration,
//!`VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16`, that was previously
//!exclusively used for YCbCr and therefore had a set of limitations related to
//!that usage.
//!The alternative was to introduce a new format token with exactly the same
//!bit representation as the existing token, but without the limitations.2) Should we only
//! introduce
//!`VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16` or also 1-3 component
//!variations? **RESOLVED** : Only the 4-component format is introduced because the 1- and 2-
//!component variations are already not exclusive to YCbCr, and the 3-component
//!variation is not a good match for hardware capabilities.
//!# Version History
//! - Revision 1, 2021-09-29 (Jan-Harald Fredriksen)  - Initial EXT version
//!# Other info
//! * 2021-09-29
//! * No known IP claims.
//! * - Jan-Harald Fredriksen, Arm  - Graeme Leese, Broadcom  - Spencer Fricke, Samsung
//!# Related
//! - [`PhysicalDeviceRgba10X6FormatsFeaturesEXT`]
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
#[doc(alias = "VK_EXT_RGBA10X6_FORMATS_SPEC_VERSION")]
pub const EXT_RGBA10X6_FORMATS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_RGBA10X6_FORMATS_EXTENSION_NAME")]
pub const EXT_RGBA10X6_FORMATS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_rgba10x6_formats");
///[VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT.html) - Structure describing whether rendering to VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16 formats can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceRgba10X6FormatsFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_rgba10x6_formats
///typedef struct VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           formatRgba10x6WithoutYCbCrSampler;
///} VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT;
///```
///# Members
///The members of the [`PhysicalDeviceRgba10X6FormatsFeaturesEXT`]
///structure describe the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format_rgba_10_x_6_without_y_cb_cr_sampler`] indicates that `VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16` **can**  be used with a [`ImageView`] with `subresourceRange.aspectMask` equal to `VK_IMAGE_ASPECT_COLOR_BIT` without a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) enabled.
///If the [`PhysicalDeviceRgba10X6FormatsFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceRgba10X6FormatsFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT`
///# Related
/// - [`VK_EXT_rgba10x6_formats`]
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
#[doc(alias = "VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceRgba10X6FormatsFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`format_rgba_10_x_6_without_y_cb_cr_sampler`] indicates that
    ///`VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16` **can**  be used with a
    ///[`ImageView`] with `subresourceRange.aspectMask` equal to
    ///`VK_IMAGE_ASPECT_COLOR_BIT` without a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) enabled.
    pub format_rgba_10_x_6_without_y_cb_cr_sampler: Bool32,
}
impl<'lt> Default for PhysicalDeviceRgba10X6FormatsFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_RGBA10_X_6_FORMATS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            format_rgba_10_x_6_without_y_cb_cr_sampler: 0,
        }
    }
}
impl<'lt> PhysicalDeviceRgba10X6FormatsFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::format_rgba_10_x_6_without_y_cb_cr_sampler`]
    pub fn format_rgba_10_x_6_without_y_cb_cr_sampler_raw(&self) -> Bool32 {
        self.format_rgba_10_x_6_without_y_cb_cr_sampler
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::format_rgba_10_x_6_without_y_cb_cr_sampler`]
    pub fn set_format_rgba_10_x_6_without_y_cb_cr_sampler_raw(mut self, value: Bool32) -> Self {
        self.format_rgba_10_x_6_without_y_cb_cr_sampler = value;
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
    ///Gets the value of [`Self::format_rgba_10_x_6_without_y_cb_cr_sampler`]
    pub fn format_rgba_10_x_6_without_y_cb_cr_sampler(&self) -> bool {
        unsafe { std::mem::transmute(self.format_rgba_10_x_6_without_y_cb_cr_sampler as u8) }
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
    ///Gets a mutable reference to the value of
    /// [`Self::format_rgba_10_x_6_without_y_cb_cr_sampler`]
    pub fn format_rgba_10_x_6_without_y_cb_cr_sampler_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.format_rgba_10_x_6_without_y_cb_cr_sampler as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.format_rgba_10_x_6_without_y_cb_cr_sampler as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the value of [`Self::format_rgba_10_x_6_without_y_cb_cr_sampler`]
    pub fn set_format_rgba_10_x_6_without_y_cb_cr_sampler(mut self, value: bool) -> Self {
        self.format_rgba_10_x_6_without_y_cb_cr_sampler = value as u8 as u32;
        self
    }
}
