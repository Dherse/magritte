//![VK_EXT_border_color_swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_border_color_swizzle.html) - device extension
//!# Description
//!After the publication of VK_EXT_custom_border_color, it was discovered that
//!some implementations had undefined behavior when combining a sampler that
//!uses a custom border color with image views whose component mapping is not
//!the identity mapping.Since VK_EXT_custom_border_color has already shipped, this new extension
//!VK_EXT_border_color_swizzle was created to define the interaction between
//!custom border colors and non-identity image view swizzles, and provide a
//!work-around for implementations that must pre-swizzle the sampler border
//!color to match the image view component mapping it is combined with.This extension also defines
//! the behavior between samplers with an opaque
//!black border color and image views with a non-identity component swizzle,
//!which was previously left undefined.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_EXT_custom_border_color`]`
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_border_color_swizzle]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_border_color_swizzle extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`]
//! - Extending [`SamplerCreateInfo`]:  - [`SamplerBorderColorComponentMappingCreateInfoEXT`]
//!# New constants
//! - [`EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME`]
//! - [`EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT`
//!# Known issues & F.A.Q
//!None.
//!# Version History
//! - Revision 1, 2021-10-12 (Piers Daniell)  - Internal revisions.
//!# Other info
//! * 2021-10-12
//! * No known IP claims.
//! * - Graeme Leese, Broadcom  - Jan-Harald Fredriksen, Arm  - Ricardo Garcia, Igalia  - Shahbaz
//!   Youssefi, Google  - Stu Smith, AMD
//!# Related
//! - [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`]
//! - [`SamplerBorderColorComponentMappingCreateInfoEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, ComponentMapping, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION")]
pub const EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME")]
pub const EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_border_color_swizzle");
///[VkSamplerBorderColorComponentMappingCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerBorderColorComponentMappingCreateInfoEXT.html) - Structure specifying the component mapping of the border color
///# C Specifications
///If the sampler is created with `VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK`,
///`VK_BORDER_COLOR_INT_OPAQUE_BLACK`,
///`VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`, or
///`VK_BORDER_COLOR_INT_CUSTOM_EXT``borderColor`, and that sampler
///will be combined with an image view that does not have an
///[identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings), and
///[`PhysicalDeviceBorderColorSwizzleFeaturesEXT::border_color_swizzle_from_image`]
///is not enabled, then it is necessary to specify the component mapping of the
///border color, by including the
///[`SamplerBorderColorComponentMappingCreateInfoEXT`] structure in the
///[`SamplerCreateInfo`]::[`p_next`] chain, to get defined results.The [`SamplerBorderColorComponentMappingCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_border_color_swizzle
///typedef struct VkSamplerBorderColorComponentMappingCreateInfoEXT {
///    VkStructureType       sType;
///    const void*           pNext;
///    VkComponentMapping    components;
///    VkBool32              srgb;
///} VkSamplerBorderColorComponentMappingCreateInfoEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`components`] is a [`ComponentMapping`] structure specifying a remapping of the border color
///   components.
/// - [`srgb`] indicates that the sampler will be combined with an image view that has an image
///   format which is sRGB encoded.
/// # Description
/// The [`ComponentMapping`][`components`] member describes a remapping
/// from components of the border color to components of the vector returned by
/// shader image instructions when the border color is used.
/// ## Valid Usage
/// - The [`borderColorSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-borderColorSwizzle)
///   feature  **must**  be enabled.
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT`
/// - [`components`] **must**  be a valid [`ComponentMapping`] structure
/// # Related
/// - [`VK_EXT_border_color_swizzle`]
/// - [`Bool32`]
/// - [`ComponentMapping`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSamplerBorderColorComponentMappingCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SamplerBorderColorComponentMappingCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`components`] is a [`ComponentMapping`] structure specifying a
    ///remapping of the border color components.
    pub components: ComponentMapping,
    ///[`srgb`] indicates that the sampler will be combined with an image
    ///view that has an image format which is sRGB encoded.
    pub srgb: Bool32,
}
impl<'lt> Default for SamplerBorderColorComponentMappingCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SamplerBorderColorComponentMappingCreateInfoExt,
            p_next: std::ptr::null(),
            components: Default::default(),
            srgb: 0,
        }
    }
}
impl<'lt> SamplerBorderColorComponentMappingCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::srgb`]
    pub fn srgb_raw(&self) -> Bool32 {
        self.srgb
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::srgb`]
    pub fn set_srgb_raw(&mut self, value: Bool32) -> &mut Self {
        self.srgb = value;
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
    ///Gets the value of [`Self::components`]
    pub fn components(&self) -> ComponentMapping {
        self.components
    }
    ///Gets the value of [`Self::srgb`]
    pub fn srgb(&self) -> bool {
        unsafe { std::mem::transmute(self.srgb as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::components`]
    pub fn components_mut(&mut self) -> &mut ComponentMapping {
        &mut self.components
    }
    ///Gets a mutable reference to the value of [`Self::srgb`]
    pub fn srgb_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.srgb as *mut Bool32).cast::<u32>().cast::<u8>().cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.srgb as *mut Bool32)
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
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::components`]
    pub fn set_components(&mut self, value: crate::vulkan1_0::ComponentMapping) -> &mut Self {
        self.components = value;
        self
    }
    ///Sets the raw value of [`Self::srgb`]
    pub fn set_srgb(&mut self, value: bool) -> &mut Self {
        self.srgb = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceBorderColorSwizzleFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBorderColorSwizzleFeaturesEXT.html) - Structure describing whether samplers with custom border colors require the component swizzle specified in order to have defined behavior
///# C Specifications
///The [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_border_color_swizzle
///typedef struct VkPhysicalDeviceBorderColorSwizzleFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           borderColorSwizzle;
///    VkBool32           borderColorSwizzleFromImage;
///} VkPhysicalDeviceBorderColorSwizzleFeaturesEXT;
///```
/// # Members
/// This structure describes the following features:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`border_color_swizzle`] indicates that defined values are returned by sampled image operations when used with a sampler that uses a `VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK`, `VK_BORDER_COLOR_INT_OPAQUE_BLACK`, `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`, or `VK_BORDER_COLOR_INT_CUSTOM_EXT``borderColor` and an image view that uses a non-[identity component mapping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings), when either [`border_color_swizzle_from_image`] is enabled or the [`SamplerBorderColorComponentMappingCreateInfoEXT`] is specified.
/// - [`border_color_swizzle_from_image`] indicates that the implementation will return the correct border color values from sampled image operations under the conditions expressed above, without the application having to specify the border color component mapping when creating the sampler object. If this feature bit is not set, applications  **can**  chain a [`SamplerBorderColorComponentMappingCreateInfoEXT`] structure when creating samplers for use with image views that do not have an [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings) and, when those samplers are combined with image views using the same component mapping, sampled image operations that use opaque black or custom border colors will return the correct border color values.
/// If the [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT`
/// # Related
/// - [`VK_EXT_border_color_swizzle`]
/// - [`Bool32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceBorderColorSwizzleFeaturesEXT")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`border_color_swizzle`] indicates that
    ///defined values are returned by sampled image operations when used with a
    ///sampler that uses a `VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK`,
    ///`VK_BORDER_COLOR_INT_OPAQUE_BLACK`,
    ///`VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`, or
    ///`VK_BORDER_COLOR_INT_CUSTOM_EXT``borderColor` and an image view
    ///that uses a non-[identity
    ///component mapping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings), when either [`border_color_swizzle_from_image`] is
    ///enabled or the [`SamplerBorderColorComponentMappingCreateInfoEXT`]
    ///is specified.
    pub border_color_swizzle: Bool32,
    ///[`border_color_swizzle_from_image`] indicates that the implementation will
    ///return the correct border color values from sampled image operations
    ///under the conditions expressed above, without the application having to
    ///specify the border color component mapping when creating the sampler
    ///object.
    ///If this feature bit is not set, applications  **can**  chain a
    ///[`SamplerBorderColorComponentMappingCreateInfoEXT`] structure when
    ///creating samplers for use with image views that do not have an
    ///[identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings) and, when
    ///those samplers are combined with image views using the same component
    ///mapping, sampled image operations that use opaque black or custom border
    ///colors will return the correct border color values.
    pub border_color_swizzle_from_image: Bool32,
}
impl<'lt> Default for PhysicalDeviceBorderColorSwizzleFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceBorderColorSwizzleFeaturesExt,
            p_next: std::ptr::null_mut(),
            border_color_swizzle: 0,
            border_color_swizzle_from_image: 0,
        }
    }
}
impl<'lt> PhysicalDeviceBorderColorSwizzleFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::border_color_swizzle`]
    pub fn border_color_swizzle_raw(&self) -> Bool32 {
        self.border_color_swizzle
    }
    ///Gets the raw value of [`Self::border_color_swizzle_from_image`]
    pub fn border_color_swizzle_from_image_raw(&self) -> Bool32 {
        self.border_color_swizzle_from_image
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::border_color_swizzle`]
    pub fn set_border_color_swizzle_raw(&mut self, value: Bool32) -> &mut Self {
        self.border_color_swizzle = value;
        self
    }
    ///Sets the raw value of [`Self::border_color_swizzle_from_image`]
    pub fn set_border_color_swizzle_from_image_raw(&mut self, value: Bool32) -> &mut Self {
        self.border_color_swizzle_from_image = value;
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
    ///Gets the value of [`Self::border_color_swizzle`]
    pub fn border_color_swizzle(&self) -> bool {
        unsafe { std::mem::transmute(self.border_color_swizzle as u8) }
    }
    ///Gets the value of [`Self::border_color_swizzle_from_image`]
    pub fn border_color_swizzle_from_image(&self) -> bool {
        unsafe { std::mem::transmute(self.border_color_swizzle_from_image as u8) }
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
    ///Gets a mutable reference to the value of [`Self::border_color_swizzle`]
    pub fn border_color_swizzle_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.border_color_swizzle as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.border_color_swizzle as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::border_color_swizzle_from_image`]
    pub fn border_color_swizzle_from_image_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.border_color_swizzle_from_image as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.border_color_swizzle_from_image as *mut Bool32)
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
    ///Sets the raw value of [`Self::border_color_swizzle`]
    pub fn set_border_color_swizzle(&mut self, value: bool) -> &mut Self {
        self.border_color_swizzle = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::border_color_swizzle_from_image`]
    pub fn set_border_color_swizzle_from_image(&mut self, value: bool) -> &mut Self {
        self.border_color_swizzle_from_image = value as u8 as u32;
        self
    }
}
