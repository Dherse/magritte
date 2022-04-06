//![VK_EXT_filter_cubic](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_filter_cubic.html) - device extension
//!# Description
//![`VK_EXT_filter_cubic`] extends [`VK_IMG_filter_cubic`].It documents cubic filtering of other
//! image view types.
//!It adds new structures that  **can**  be added to the `pNext` chain of
//![`PhysicalDeviceImageFormatInfo2`] and [`ImageFormatProperties2`]
//!that  **can**  be used to determine which image types and which image view types
//!support cubic filtering.
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Bill Licea-Kane [wwlk](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_filter_cubic]
//!   @wwlk%0A<<Here describe the issue or question you have about the VK_EXT_filter_cubic
//!   extension>>)
//!# New structures
//! - Extending [`ImageFormatProperties2`]:  - [`FilterCubicImageViewImageFormatPropertiesEXT`]
//! - Extending [`PhysicalDeviceImageFormatInfo2`]:  - [`PhysicalDeviceImageViewImageFormatInfoEXT`]
//!# New constants
//! - [`EXT_FILTER_CUBIC_EXTENSION_NAME`]
//! - [`EXT_FILTER_CUBIC_SPEC_VERSION`]
//! - Extending [`Filter`]:  - `VK_FILTER_CUBIC_EXT`
//! - Extending [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT`
//!# Version History
//! - Revision 3, 2019-12-13 (wwlk)  - Delete requirement to cubic filter the formats
//!   USCALED_PACKED32, SSCALED_PACKED32, UINT_PACK32, and SINT_PACK32 (cut/paste error)
//! - Revision 2, 2019-06-05 (wwlk)  - Clarify 1D optional
//! - Revision 1, 2019-01-24 (wwlk)  - Initial version
//!# Other info
//! * 2019-12-13
//! * - Bill Licea-Kane, Qualcomm Technologies, Inc.  - Andrew Garrard, Samsung  - Daniel Koch,
//!   NVIDIA  - Donald Scorgie, Imagination Technologies  - Graeme Leese, Broadcom  - Jan-Herald
//!   Fredericksen, ARM  - Jeff Leger, Qualcomm Technologies, Inc.  - Tobias Hector, AMD  - Tom
//!   Olson, ARM  - Stuart Smith, Imagination Technologies
//!# Related
//! - [`FilterCubicImageViewImageFormatPropertiesEXT`]
//! - [`PhysicalDeviceImageViewImageFormatInfoEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, Bool32, ImageViewType, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FILTER_CUBIC_SPEC_VERSION")]
pub const EXT_FILTER_CUBIC_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FILTER_CUBIC_EXTENSION_NAME")]
pub const EXT_FILTER_CUBIC_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_filter_cubic");
///[VkPhysicalDeviceImageViewImageFormatInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewImageFormatInfoEXT.html) - Structure for providing image view type
///# C Specifications
///The [`PhysicalDeviceImageViewImageFormatInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_filter_cubic
///typedef struct VkPhysicalDeviceImageViewImageFormatInfoEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkImageViewType    imageViewType;
///} VkPhysicalDeviceImageViewImageFormatInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image_view_type`] is a [`ImageViewType`] value specifying the type of the image view.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT`
/// - [`image_view_type`] **must**  be a valid [`ImageViewType`] value
///# Related
/// - [`VK_EXT_filter_cubic`]
/// - [`ImageViewType`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceImageViewImageFormatInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`image_view_type`] is a [`ImageViewType`] value specifying the type
    ///of the image view.
    pub image_view_type: ImageViewType,
}
impl<'lt> Default for PhysicalDeviceImageViewImageFormatInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT,
            p_next: std::ptr::null_mut(),
            image_view_type: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceImageViewImageFormatInfoEXT<'lt> {
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
    ///Gets the value of [`Self::image_view_type`]
    pub fn image_view_type(&self) -> ImageViewType {
        self.image_view_type
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
    ///Gets a mutable reference to the value of [`Self::image_view_type`]
    pub fn image_view_type_mut(&mut self) -> &mut ImageViewType {
        &mut self.image_view_type
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
    ///Sets the value of [`Self::image_view_type`]
    pub fn set_image_view_type(mut self, value: crate::vulkan1_0::ImageViewType) -> Self {
        self.image_view_type = value;
        self
    }
}
///[VkFilterCubicImageViewImageFormatPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFilterCubicImageViewImageFormatPropertiesEXT.html) - Structure for querying cubic filtering capabilities of an image view type
///# C Specifications
///The [`FilterCubicImageViewImageFormatPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_filter_cubic
///typedef struct VkFilterCubicImageViewImageFormatPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           filterCubic;
///    VkBool32           filterCubicMinmax;
///} VkFilterCubicImageViewImageFormatPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`filter_cubic`] tells if image format, image type and image view type  **can**  be used with
///   cubic filtering. This field is set by the implementation. User-specified value is ignored.
/// - [`filter_cubic_minmax`] tells if image format, image type and image view type  **can**  be
///   used with cubic filtering and minmax filtering. This field is set by the implementation.
///   User-specified value is ignored.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT`
///
///## Valid Usage
/// - If the [`p_next`] chain of the [`ImageFormatProperties2`] structure includes a
///   [`FilterCubicImageViewImageFormatPropertiesEXT`] structure, the [`p_next`] chain of the
///   [`PhysicalDeviceImageFormatInfo2`] structure  **must**  include a
///   [`PhysicalDeviceImageViewImageFormatInfoEXT`] structure with an `imageViewType` that is
///   compatible with `imageType`
///# Related
/// - [`VK_EXT_filter_cubic`]
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
#[doc(alias = "VkFilterCubicImageViewImageFormatPropertiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct FilterCubicImageViewImageFormatPropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`filter_cubic`] tells if image format, image type and image view type
    /// **can**  be used with cubic filtering.
    ///This field is set by the implementation.
    ///User-specified value is ignored.
    pub filter_cubic: Bool32,
    ///[`filter_cubic_minmax`] tells if image format, image type and image view
    ///type  **can**  be used with cubic filtering and minmax filtering.
    ///This field is set by the implementation.
    ///User-specified value is ignored.
    pub filter_cubic_minmax: Bool32,
}
impl<'lt> Default for FilterCubicImageViewImageFormatPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            filter_cubic: 0,
            filter_cubic_minmax: 0,
        }
    }
}
impl<'lt> FilterCubicImageViewImageFormatPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::filter_cubic`]
    pub fn filter_cubic_raw(&self) -> Bool32 {
        self.filter_cubic
    }
    ///Gets the raw value of [`Self::filter_cubic_minmax`]
    pub fn filter_cubic_minmax_raw(&self) -> Bool32 {
        self.filter_cubic_minmax
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::filter_cubic`]
    pub fn set_filter_cubic_raw(mut self, value: Bool32) -> Self {
        self.filter_cubic = value;
        self
    }
    ///Sets the raw value of [`Self::filter_cubic_minmax`]
    pub fn set_filter_cubic_minmax_raw(mut self, value: Bool32) -> Self {
        self.filter_cubic_minmax = value;
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
    ///Gets the value of [`Self::filter_cubic`]
    pub fn filter_cubic(&self) -> bool {
        unsafe { std::mem::transmute(self.filter_cubic as u8) }
    }
    ///Gets the value of [`Self::filter_cubic_minmax`]
    pub fn filter_cubic_minmax(&self) -> bool {
        unsafe { std::mem::transmute(self.filter_cubic_minmax as u8) }
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
    ///Gets a mutable reference to the value of [`Self::filter_cubic`]
    pub fn filter_cubic_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.filter_cubic as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.filter_cubic as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::filter_cubic_minmax`]
    pub fn filter_cubic_minmax_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.filter_cubic_minmax as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.filter_cubic_minmax as *mut Bool32)
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
    ///Sets the value of [`Self::filter_cubic`]
    pub fn set_filter_cubic(mut self, value: bool) -> Self {
        self.filter_cubic = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::filter_cubic_minmax`]
    pub fn set_filter_cubic_minmax(mut self, value: bool) -> Self {
        self.filter_cubic_minmax = value as u8 as u32;
        self
    }
}
