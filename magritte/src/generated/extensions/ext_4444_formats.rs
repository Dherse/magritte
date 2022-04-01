//![VK_EXT_4444_formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_4444_formats.html) - device extension
//!# Description
//!This extension defines the `VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT` and
//!`VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT` formats which are defined in other
//!current graphics APIs.This extension may be useful for building translation layers for those
//! APIs
//!or for porting applications that use these formats without having to resort
//!to swizzles.When VK_EXT_custom_border_color is used, these formats are not subject to
//!the same restrictions for border color without format as with
//!VK_FORMAT_B4G4R4A4_UNORM_PACK16.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Joshua Ashton [Joshua-Ashton](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_4444_formats]
//!   @Joshua-Ashton%0A<<Here describe the issue or question you have about the VK_EXT_4444_formats
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevice4444FormatsFeaturesEXT`]
//!# New constants
//! - [`EXT_4444_FORMATS_EXTENSION_NAME`]
//! - [`EXT_4444_FORMATS_SPEC_VERSION`]
//! - Extending [`Format`]:  - `VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT`  -
//!   `VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2020-07-04 (Joshua Ashton)  - Initial draft
//!# Other info
//! * 2020-07-28
//! * - Promoted to Vulkan 1.3 Core
//! * No known IP claims.
//! * - Joshua Ashton, Valve  - Jason Ekstrand, Intel
//!# Related
//! - [`PhysicalDevice4444FormatsFeaturesEXT`]
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
#[doc(alias = "VK_EXT_4444_FORMATS_SPEC_VERSION")]
pub const EXT_4444_FORMATS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_4444_FORMATS_EXTENSION_NAME")]
pub const EXT_4444_FORMATS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_4444_formats");
///[VkPhysicalDevice4444FormatsFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice4444FormatsFeaturesEXT.html) - Structure describing additional 4444 formats supported by an implementation
///# C Specifications
///The [`PhysicalDevice4444FormatsFeaturesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_4444_formats
///typedef struct VkPhysicalDevice4444FormatsFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           formatA4R4G4B4;
///    VkBool32           formatA4B4G4R4;
///} VkPhysicalDevice4444FormatsFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format_a_4_r_4_g_4_b_4`] indicates that the implementation  **must**  support using a
///   [`Format`] of `VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT` with at least the following
///   [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`  -
///   `VK_FORMAT_FEATURE_BLIT_SRC_BIT`  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - [`format_a_4_b_4_g_4_r_4`] indicates that the implementation  **must**  support using a
///   [`Format`] of `VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT` with at least the following
///   [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`  -
///   `VK_FORMAT_FEATURE_BLIT_SRC_BIT`  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
///If the [`PhysicalDevice4444FormatsFeaturesEXT`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevice4444FormatsFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT`
///# Related
/// - [`VK_EXT_4444_formats`]
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
#[doc(alias = "VkPhysicalDevice4444FormatsFeaturesEXT")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDevice4444FormatsFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`format_a_4_r_4_g_4_b_4`] indicates that the
    ///implementation  **must**  support using a [`Format`] of
    ///`VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT` with at least the following
    ///[`FormatFeatureFlagBits`]:
    /// - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`
    /// - `VK_FORMAT_FEATURE_BLIT_SRC_BIT`
    /// - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    pub format_a_4_r_4_g_4_b_4: Bool32,
    ///[`format_a_4_b_4_g_4_r_4`] indicates that the
    ///implementation  **must**  support using a [`Format`] of
    ///`VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT` with at least the following
    ///[`FormatFeatureFlagBits`]:
    /// - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`
    /// - `VK_FORMAT_FEATURE_BLIT_SRC_BIT`
    /// - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    pub format_a_4_b_4_g_4_r_4: Bool32,
}
impl<'lt> Default for PhysicalDevice4444FormatsFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDevice4444FormatsFeaturesExt,
            p_next: std::ptr::null_mut(),
            format_a_4_r_4_g_4_b_4: 0,
            format_a_4_b_4_g_4_r_4: 0,
        }
    }
}
impl<'lt> PhysicalDevice4444FormatsFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::format_a_4_r_4_g_4_b_4`]
    pub fn format_a_4_r_4_g_4_b_4_raw(&self) -> Bool32 {
        self.format_a_4_r_4_g_4_b_4
    }
    ///Gets the raw value of [`Self::format_a_4_b_4_g_4_r_4`]
    pub fn format_a_4_b_4_g_4_r_4_raw(&self) -> Bool32 {
        self.format_a_4_b_4_g_4_r_4
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::format_a_4_r_4_g_4_b_4`]
    pub fn set_format_a_4_r_4_g_4_b_4_raw(&mut self, value: Bool32) -> &mut Self {
        self.format_a_4_r_4_g_4_b_4 = value;
        self
    }
    ///Sets the raw value of [`Self::format_a_4_b_4_g_4_r_4`]
    pub fn set_format_a_4_b_4_g_4_r_4_raw(&mut self, value: Bool32) -> &mut Self {
        self.format_a_4_b_4_g_4_r_4 = value;
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
    ///Gets the value of [`Self::format_a_4_r_4_g_4_b_4`]
    pub fn format_a_4_r_4_g_4_b_4(&self) -> bool {
        unsafe { std::mem::transmute(self.format_a_4_r_4_g_4_b_4 as u8) }
    }
    ///Gets the value of [`Self::format_a_4_b_4_g_4_r_4`]
    pub fn format_a_4_b_4_g_4_r_4(&self) -> bool {
        unsafe { std::mem::transmute(self.format_a_4_b_4_g_4_r_4 as u8) }
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
    ///Gets a mutable reference to the value of [`Self::format_a_4_r_4_g_4_b_4`]
    pub fn format_a_4_r_4_g_4_b_4_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.format_a_4_r_4_g_4_b_4 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.format_a_4_r_4_g_4_b_4 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::format_a_4_b_4_g_4_r_4`]
    pub fn format_a_4_b_4_g_4_r_4_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.format_a_4_b_4_g_4_r_4 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.format_a_4_b_4_g_4_r_4 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the raw value of [`Self::format_a_4_r_4_g_4_b_4`]
    pub fn set_format_a_4_r_4_g_4_b_4(&mut self, value: bool) -> &mut Self {
        self.format_a_4_r_4_g_4_b_4 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::format_a_4_b_4_g_4_r_4`]
    pub fn set_format_a_4_b_4_g_4_r_4(&mut self, value: bool) -> &mut Self {
        self.format_a_4_b_4_g_4_r_4 = value as u8 as u32;
        self
    }
}
