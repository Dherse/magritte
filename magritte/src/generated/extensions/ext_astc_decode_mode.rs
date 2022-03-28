//![VK_EXT_astc_decode_mode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_astc_decode_mode.html) - device extension
//!# Description
//!The existing specification requires that low dynamic range (LDR) ASTC
//!textures are decompressed to FP16 values per component.
//!In many cases, decompressing LDR textures to a lower precision intermediate
//!result gives acceptable image quality.
//!Source material for LDR textures is typically authored as 8-bit UNORM
//!values, so decoding to FP16 values adds little value.
//!On the other hand, reducing precision of the decoded result reduces the size
//!of the decompressed data, potentially improving texture cache performance
//!and saving power.The goal of this extension is to enable this efficiency gain on existing
//!ASTC texture data.
//!This is achieved by giving the application the ability to select the
//!intermediate decoding precision.Three decoding options are provided:
//! - Decode to `VK_FORMAT_R16G16B16A16_SFLOAT` precision: This is the default, and matches the
//!   required behavior in the core API.
//! - Decode to `VK_FORMAT_R8G8B8A8_UNORM` precision: This is provided as an option in LDR mode.
//! - Decode to `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32` precision: This is provided as an option in both
//!   LDR and HDR mode. In this mode, negative values cannot be represented and are clamped to zero.
//!   The alpha component is ignored, and the results are as if alpha was 1.0. This decode mode is
//!   optional and support can be queried via the physical device properties.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jan-Harald Fredriksen [janharaldfredriksen-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_astc_decode_mode]
//!   @janharaldfredriksen-arm%0A<<Here describe the issue or question you have about the
//!   VK_EXT_astc_decode_mode extension>>)
//!# New structures
//! - Extending [`ImageViewCreateInfo`]:  - [`ImageViewAstcDecodeModeEXT`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceAstcDecodeFeaturesEXT`]
//!# New constants
//! - [`EXT_ASTC_DECODE_MODE_EXTENSION_NAME`]
//! - [`EXT_ASTC_DECODE_MODE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT`
//!# Known issues & F.A.Q
//!1) Are implementations allowed to decode at a higher precision than what is
//!requested?
//!```c
//!RESOLUTION: No.
//!If we allow this, then this extension could be exposed on all
//!implementations that support ASTC.
//!But developers would have no way of knowing what precision was actually
//!used, and thus whether the image quality is sufficient at reduced
//!precision.
//!```
//!2) Should the decode mode be image view state and/or sampler state?
//!```c
//!RESOLUTION: Image view state only.
//!Some implementations treat the different decode modes as different
//!texture formats.
//!```
//!# Version History
//! - Revision 1, 2018-08-07 (Jan-Harald Fredriksen)  - Initial revision
//!# Other info
//! * 2018-08-07
//! * - Jan-Harald Fredriksen, Arm
//!# Related
//! - [`ImageViewAstcDecodeModeEXT`]
//! - [`PhysicalDeviceAstcDecodeFeaturesEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Format, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_ASTC_DECODE_MODE_SPEC_VERSION")]
pub const EXT_ASTC_DECODE_MODE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_ASTC_DECODE_MODE_EXTENSION_NAME")]
pub const EXT_ASTC_DECODE_MODE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_astc_decode_mode");
///[VkImageViewASTCDecodeModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewASTCDecodeModeEXT.html) - Structure describing the ASTC decode mode for an image view
///# C Specifications
///If the [`p_next`] chain includes a [`ImageViewAstcDecodeModeEXT`]
///structure, then that structure includes a parameter specifying the decode
///mode for image views using ASTC compressed formats.The [`ImageViewAstcDecodeModeEXT`] structure
/// is defined as:
///```c
///// Provided by VK_EXT_astc_decode_mode
///typedef struct VkImageViewASTCDecodeModeEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkFormat           decodeMode;
///} VkImageViewASTCDecodeModeEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`decode_mode`] is the intermediate format used to decode ASTC compressed formats.
///# Description
///## Valid Usage
/// - [`decode_mode`] **must**  be one of `VK_FORMAT_R16G16B16A16_SFLOAT`,
///   `VK_FORMAT_R8G8B8A8_UNORM`, or `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32`
/// - If the [`decodeModeSharedExponent`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-astc-decodeModeSharedExponent)
///   feature is not enabled, [`decode_mode`] **must**  not be `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32`
/// - If [`decode_mode`] is `VK_FORMAT_R8G8B8A8_UNORM` the image view  **must**  not include blocks
///   using any of the ASTC HDR modes
/// - `format` of the image view  **must**  be one of the [ASTC Compressed Image Formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#appendix-compressedtex-astc)
///If `format` uses sRGB encoding then the [`decode_mode`] has no effect.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT`
/// - [`decode_mode`] **must**  be a valid [`Format`] value
///# Related
/// - [`VK_EXT_astc_decode_mode`]
/// - [`Format`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageViewAstcDecodeModeEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`decode_mode`] is the intermediate format used to decode ASTC
    ///compressed formats.
    decode_mode: Format,
}
impl<'lt> Default for ImageViewAstcDecodeModeEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            decode_mode: Default::default(),
        }
    }
}
impl<'lt> ImageViewAstcDecodeModeEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::decode_mode`]
    pub fn decode_mode(&self) -> Format {
        self.decode_mode
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::decode_mode`]
    pub fn decode_mode_mut(&mut self) -> &mut Format {
        &mut self.decode_mode
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
    ///Sets the raw value of [`Self::decode_mode`]
    pub fn set_decode_mode(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.decode_mode = value;
        self
    }
}
///[VkPhysicalDeviceASTCDecodeFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceASTCDecodeFeaturesEXT.html) - Structure describing ASTC decode mode features
///# C Specifications
///The [`PhysicalDeviceAstcDecodeFeaturesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_astc_decode_mode
///typedef struct VkPhysicalDeviceASTCDecodeFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           decodeModeSharedExponent;
///} VkPhysicalDeviceASTCDecodeFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`decode_mode_shared_exponent`] indicates whether the implementation supports decoding ASTC
///   compressed formats to `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32` internal precision.
///If the [`PhysicalDeviceAstcDecodeFeaturesEXT`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceAstcDecodeFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT`
///# Related
/// - [`VK_EXT_astc_decode_mode`]
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceAstcDecodeFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`decode_mode_shared_exponent`] indicates whether the implementation
    ///supports decoding ASTC compressed formats to
    ///`VK_FORMAT_E5B9G9R9_UFLOAT_PACK32` internal precision.
    decode_mode_shared_exponent: Bool32,
}
impl<'lt> Default for PhysicalDeviceAstcDecodeFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            decode_mode_shared_exponent: 0,
        }
    }
}
impl<'lt> PhysicalDeviceAstcDecodeFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::decode_mode_shared_exponent`]
    pub fn decode_mode_shared_exponent_raw(&self) -> Bool32 {
        self.decode_mode_shared_exponent
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::decode_mode_shared_exponent`]
    pub fn set_decode_mode_shared_exponent_raw(&mut self, value: Bool32) -> &mut Self {
        self.decode_mode_shared_exponent = value;
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
    ///Gets the value of [`Self::decode_mode_shared_exponent`]
    pub fn decode_mode_shared_exponent(&self) -> bool {
        unsafe { std::mem::transmute(self.decode_mode_shared_exponent as u8) }
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
    ///Gets a mutable reference to the value of [`Self::decode_mode_shared_exponent`]
    pub fn decode_mode_shared_exponent_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.decode_mode_shared_exponent as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.decode_mode_shared_exponent as *mut Bool32)
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
    ///Sets the raw value of [`Self::decode_mode_shared_exponent`]
    pub fn set_decode_mode_shared_exponent(&mut self, value: bool) -> &mut Self {
        self.decode_mode_shared_exponent = value as u8 as u32;
        self
    }
}
