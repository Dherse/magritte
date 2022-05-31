//![VK_EXT_image_view_min_lod](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_view_min_lod.html) - device extension
//!# Description
//!This extension allows applications to clamp the minimum LOD value during
//![Image Level(s) Selection](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-image-level-selection) and
//![Integer Texel Coordinate
//!Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-integer-coordinate-operations) with a given [`ImageView`] by
//![`ImageViewMinLodCreateInfoEXT::min_lod`].This extension may be useful to restrict a
//! [`ImageView`] to only mips
//!which have been uploaded, and the use of fractional `minLod` can be
//!useful for smoothly introducing new mip levels when using linear mipmap
//!filtering.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Joshua Ashton [Joshua-Ashton](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_image_view_min_lod]
//!   @Joshua-Ashton%0A<<Here describe the issue or question you have about the
//!   VK_EXT_image_view_min_lod extension>>)
//!# New structures
//! - Extending [`ImageViewCreateInfo`]:  - [`ImageViewMinLodCreateInfoEXT`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceImageViewMinLodFeaturesEXT`]
//!# New constants
//! - [`EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME`]
//! - [`EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2021-07-06 (Joshua Ashton)  - Initial version
//!# Other info
//! * 2021-11-09
//! * No known IP claims.
//! * - Joshua Ashton, Valve  - Hans-Kristian Arntzen, Valve  - Samuel Iglesias Gonsalvez, Igalia  -
//!   Tobias Hector, AMD  - Jason Ekstrand, Intel  - Tom Olson, ARM
//!# Related
//! - [`ImageViewMinLodCreateInfoEXT`]
//! - [`PhysicalDeviceImageViewMinLodFeaturesEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION")]
pub const EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME")]
pub const EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_image_view_min_lod");
///[VkPhysicalDeviceImageViewMinLodFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewMinLodFeaturesEXT.html) - Structure describing whether clamping the min lod of a image view is supported by the implementation
///# C Specifications
///The [`PhysicalDeviceImageViewMinLodFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_image_view_min_lod
///typedef struct VkPhysicalDeviceImageViewMinLodFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           minLod;
///} VkPhysicalDeviceImageViewMinLodFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`min_lod`] indicates whether the implementation supports clamping the minimum LOD value during [Image Level(s) Selection](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-image-level-selection) and [Integer Texel Coordinate Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-integer-coordinate-operations) with a given [`ImageView`] by [`ImageViewMinLodCreateInfoEXT`]::[`min_lod`].
///If the [`PhysicalDeviceImageViewMinLodFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceImageViewMinLodFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT`
///# Related
/// - [`ext_image_view_min_lod`]
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
#[doc(alias = "VkPhysicalDeviceImageViewMinLodFeaturesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceImageViewMinLodFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT`
    pub s_type: StructureType,
    ///No documentation found
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`min_lod`] indicates whether the implementation
    ///supports clamping the minimum LOD value during
    ///[Image Level(s) Selection](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-image-level-selection) and
    ///[Integer Texel Coordinate
    ///Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-integer-coordinate-operations) with a given [`ImageView`] by
    ///[`ImageViewMinLodCreateInfoEXT`]::[`min_lod`].
    pub min_lod: Bool32,
}
impl<'lt> Default for PhysicalDeviceImageViewMinLodFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            min_lod: 0,
        }
    }
}
impl<'lt> PhysicalDeviceImageViewMinLodFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::min_lod`]
    pub fn min_lod_raw(&self) -> Bool32 {
        self.min_lod
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::min_lod`]
    pub fn set_min_lod_raw(&mut self, value: Bool32) -> &mut Self {
        self.min_lod = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::min_lod`]
    pub fn with_min_lod_raw(mut self, value: Bool32) -> Self {
        self.min_lod = value;
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
    ///Gets the value of [`Self::min_lod`]
    pub fn min_lod(&self) -> bool {
        unsafe { std::mem::transmute(self.min_lod as u8) }
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
    ///Gets a mutable reference to the value of [`Self::min_lod`]
    pub fn min_lod_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.min_lod as *mut Bool32).cast::<u32>().cast::<u8>().cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.min_lod as *mut Bool32)
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
    ///Sets the value of [`Self::min_lod`]
    pub fn set_min_lod(&mut self, value: bool) -> &mut Self {
        self.min_lod = value as u8 as u32;
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
    ///Sets the value of [`Self::min_lod`]
    pub fn with_min_lod(mut self, value: bool) -> Self {
        self.min_lod = value as u8 as u32;
        self
    }
}
///[VkImageViewMinLodCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewMinLodCreateInfoEXT.html) - Structure describing the minimum lod of an image view
///# C Specifications
///If the [`p_next`] chain includes a [`ImageViewMinLodCreateInfoEXT`]
///structure, then that structure includes a parameter specifying a value to
///clamp the minimum LOD value during [Image
///Level(s) Selection](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-image-level-selection) and [Integer
///Texel Coordinate Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-integer-coordinate-operations).The [`ImageViewMinLodCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_image_view_min_lod
///typedef struct VkImageViewMinLodCreateInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    float              minLod;
///} VkImageViewMinLodCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`min_lod`] is the value to clamp the minimum LOD accessible by this [`ImageView`].
///# Description
///## Valid Usage
/// - If the [[`min_lod`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-minLod)
///   feature is not enabled, [`min_lod`] **must**  be `0.0`.
/// - [`min_lod`] **must**  be less or equal to the index of the last mipmap level accessible to the
///   view.
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT`
///# Related
/// - [`ext_image_view_min_lod`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageViewMinLodCreateInfoEXT")]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct ImageViewMinLodCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`min_lod`] is the value to clamp the minimum LOD accessible by this
    ///[`ImageView`].
    pub min_lod: f32,
}
impl<'lt> Default for ImageViewMinLodCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            min_lod: 0.0,
        }
    }
}
impl<'lt> ImageViewMinLodCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::min_lod`]
    pub fn min_lod(&self) -> f32 {
        self.min_lod
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::min_lod`]
    pub fn min_lod_mut(&mut self) -> &mut f32 {
        &mut self.min_lod
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::min_lod`]
    pub fn set_min_lod(&mut self, value: f32) -> &mut Self {
        self.min_lod = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::min_lod`]
    pub fn with_min_lod(mut self, value: f32) -> Self {
        self.min_lod = value;
        self
    }
}
