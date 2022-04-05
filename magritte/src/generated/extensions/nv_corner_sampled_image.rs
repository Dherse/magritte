//![VK_NV_corner_sampled_image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_corner_sampled_image.html) - device extension
//!# Description
//!This extension adds support for a new image organization, which this
//!extension refers to as “corner-sampled” images.
//!A corner-sampled image differs from a conventional image in the following
//!ways:
//! - Texels are centered on integer coordinates. See [Unnormalized Texel Coordinate Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-unnormalized-to-integer)
//! - Normalized coordinates are scaled using coord × (dim - 1) rather than coord × dim, where dim is the size of one dimension of the image. See [normalized texel coordinate transform](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-normalized-to-unnormalized).
//! - Partial derivatives are scaled using coord × (dim - 1) rather than coord × dim. See [Scale Factor Operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-scale-factor).
//! - Calculation of the next higher lod size goes according to ⌈dim / 2⌉ rather than ⌊dim / 2⌋. See
//!   [Image Miplevel Sizing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-miplevel-sizing).
//! - The minimum level size is 2x2 for 2D images and 2x2x2 for 3D images. See [Image Miplevel Sizing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-miplevel-sizing).
//!This image organization is designed to facilitate a system like Ptex with
//!separate textures for each face of a subdivision or polygon mesh.
//!Placing sample locations at pixel corners allows applications to maintain
//!continuity between adjacent patches by duplicating values along shared
//!edges.
//!Additionally, using the modified mipmapping logic along with texture
//!dimensions of the form 2<sup>n</sup>+1 allows continuity across shared edges
//!even if the adjacent patches use different level-of-detail values.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_corner_sampled_image]
//!   @dgkoch%0A<<Here describe the issue or question you have about the VK_NV_corner_sampled_image
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceCornerSampledImageFeaturesNV`]
//!# New constants
//! - [`NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME`]
//! - [`NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION`]
//! - Extending [`ImageCreateFlagBits`]:  - `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV`
//!# Known issues & F.A.Q
//!0. What should this extension be named? **DISCUSSION** : While naming this extension, we chose
//! the most distinctive aspect of the image organization and referred to such images as
//! “corner-sampled images”. As a result, we decided to name the extension NV_corner_sampled_image.
//!1. Do we need a format feature flag so formats can advertise if they support corner-sampling?
//! **DISCUSSION** : Currently NVIDIA supports this for all 2D and 3D formats, but not for cube maps
//! or depth-stencil formats. A format feature might be useful if other vendors would only support
//! this on some formats.
//!2. Do integer texel coordinates have a different range for corner-sampled images? **RESOLVED** :
//! No, these are unchanged.
//!3. Do unnormalized sampler coordinates work with corner-sampled images? Are there any functional
//! differences? **RESOLVED** : Yes. Unnormalized coordinates are treated as already scaled for
//! corner-sample usage.
//!4. Should we have a diagram in the “Image Operations” chapter demonstrating different texel
//! sampling locations? **UNRESOLVED** : Probaby, but later.
//!# Version History
//! - Revision 1, 2018-08-14 (Daniel Koch)  - Internal revisions
//! - Revision 2, 2018-08-14 (Daniel Koch)  - ???
//!# Other info
//! * 2018-08-13
//! * - Jeff Bolz, NVIDIA  - Pat Brown, NVIDIA  - Chris Lentini, NVIDIA
//!# Related
//! - [`PhysicalDeviceCornerSampledImageFeaturesNV`]
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
#[doc(alias = "VK_NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION")]
pub const NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME")]
pub const NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_corner_sampled_image");
///[VkPhysicalDeviceCornerSampledImageFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCornerSampledImageFeaturesNV.html) - Structure describing corner sampled image features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceCornerSampledImageFeaturesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_corner_sampled_image
///typedef struct VkPhysicalDeviceCornerSampledImageFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           cornerSampledImage;
///} VkPhysicalDeviceCornerSampledImageFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`corner_sampled_image`] specifies whether images can be created with a [`ImageCreateInfo::flags`] containing `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV`. See [Corner-Sampled Images](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-images-corner-sampled).
///If the [`PhysicalDeviceCornerSampledImageFeaturesNV`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceCornerSampledImageFeaturesNV`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV`
///# Related
/// - [`VK_NV_corner_sampled_image`]
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
#[doc(alias = "VkPhysicalDeviceCornerSampledImageFeaturesNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`corner_sampled_image`] specifies
    ///whether images can be created with a
    ///[`ImageCreateInfo`]::`flags` containing
    ///`VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV`.
    ///See [Corner-Sampled Images](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-images-corner-sampled).
    pub corner_sampled_image: Bool32,
}
impl<'lt> Default for PhysicalDeviceCornerSampledImageFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceCornerSampledImageFeaturesNv,
            p_next: std::ptr::null_mut(),
            corner_sampled_image: 0,
        }
    }
}
impl<'lt> PhysicalDeviceCornerSampledImageFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::corner_sampled_image`]
    pub fn corner_sampled_image_raw(&self) -> Bool32 {
        self.corner_sampled_image
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::corner_sampled_image`]
    pub fn set_corner_sampled_image_raw(mut self, value: Bool32) -> Self {
        self.corner_sampled_image = value;
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
    ///Gets the value of [`Self::corner_sampled_image`]
    pub fn corner_sampled_image(&self) -> bool {
        unsafe { std::mem::transmute(self.corner_sampled_image as u8) }
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
    ///Gets a mutable reference to the value of [`Self::corner_sampled_image`]
    pub fn corner_sampled_image_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.corner_sampled_image as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.corner_sampled_image as *mut Bool32)
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
    ///Sets the value of [`Self::corner_sampled_image`]
    pub fn set_corner_sampled_image(mut self, value: bool) -> Self {
        self.corner_sampled_image = value as u8 as u32;
        self
    }
}
