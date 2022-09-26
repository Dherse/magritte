//![VK_AMD_texture_gather_bias_lod](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_texture_gather_bias_lod.html) - device extension
//!# Description
//!This extension adds two related features.Firstly, support for the following SPIR-V extension in
//! Vulkan is added:
//! - `SPV_AMD_texture_gather_bias_lod`
//!Secondly, the extension allows the application to query which formats can be
//!used together with the new function prototypes introduced by the SPIR-V
//!extension.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Rex Xu [amdrexu](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_texture_gather_bias_lod]
//!   @amdrexu%0A<<Here describe the issue or question you have about the
//!   VK_AMD_texture_gather_bias_lod extension>>)
//!# New structures
//! - Extending [`ImageFormatProperties2`]:  - [`TextureLodGatherFormatPropertiesAMD`]
//!# New constants
//! - [`AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME`]
//! - [`AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD`
//!# Version history
//! - Revision 1, 2017-03-21 (Dominik Witczak)  - Initial draft
//!# Other information
//! * 2017-03-21
//! * No known IP claims.
//! * - This extension requires [`SPV_AMD_texture_gather_bias_lod`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/AMD/SPV_AMD_texture_gather_bias_lod.html)
//!   - This extension provides API support for [`GL_AMD_texture_gather_bias_lod`](https://www.khronos.org/registry/OpenGL/extensions/AMD/AMD_texture_gather_bias_lod.txt)
//! * - Dominik Witczak, AMD  - Daniel Rakos, AMD  - Graham Sellers, AMD  - Matthaeus G. Chajdas,
//!   AMD  - Qun Lin, AMD  - Rex Xu, AMD  - Timothy Lottes, AMD
//!# Related
//! - [`TextureLodGatherFormatPropertiesAMD`]
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
#[doc(alias = "VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION")]
pub const AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME")]
pub const AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_texture_gather_bias_lod");
///[VkTextureLODGatherFormatPropertiesAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTextureLODGatherFormatPropertiesAMD.html) - Structure informing whether or not texture gather bias/LOD functionality is supported for a given image format and a given physical device.
///# C Specifications
///To determine if texture gather functions that take explicit LOD and/or bias
///argument values  **can**  be used with a given image format, add a
///[`TextureLodGatherFormatPropertiesAMD`] structure to the [`p_next`]
///chain of the [`ImageFormatProperties2`] structure in a call to
///[`get_physical_device_image_format_properties2`].The [`TextureLodGatherFormatPropertiesAMD`]
/// structure is defined as:
///```c
///// Provided by VK_AMD_texture_gather_bias_lod
///typedef struct VkTextureLODGatherFormatPropertiesAMD {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           supportsTextureGatherLODBiasAMD;
///} VkTextureLODGatherFormatPropertiesAMD;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`supports_texture_gather_lod_bias_amd`] tells if the image format can be used with texture
///   gather bias/LOD functions, as introduced by the `[`amd_texture_gather_bias_lod`]` extension.
///   This field is set by the implementation. User-specified value is ignored.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD`
///# Related
/// - [`amd_texture_gather_bias_lod`]
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
#[doc(alias = "VkTextureLODGatherFormatPropertiesAMD")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct TextureLodGatherFormatPropertiesAMD<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`supports_texture_gather_lod_bias_amd`] tells if the image format can be
    ///used with texture gather bias/LOD functions, as introduced by the
    ///`[`amd_texture_gather_bias_lod`]` extension.
    ///This field is set by the implementation.
    ///User-specified value is ignored.
    pub supports_texture_gather_lod_bias_amd: Bool32,
}
impl<'lt> Default for TextureLodGatherFormatPropertiesAMD<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD,
            p_next: std::ptr::null_mut(),
            supports_texture_gather_lod_bias_amd: 0,
        }
    }
}
impl<'lt> TextureLodGatherFormatPropertiesAMD<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::supports_texture_gather_lod_bias_amd`]
    pub fn supports_texture_gather_lod_bias_amd_raw(&self) -> Bool32 {
        self.supports_texture_gather_lod_bias_amd
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::supports_texture_gather_lod_bias_amd`]
    pub fn set_supports_texture_gather_lod_bias_amd_raw(&mut self, value: Bool32) -> &mut Self {
        self.supports_texture_gather_lod_bias_amd = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::supports_texture_gather_lod_bias_amd`]
    pub fn with_supports_texture_gather_lod_bias_amd_raw(mut self, value: Bool32) -> Self {
        self.supports_texture_gather_lod_bias_amd = value;
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
    ///Gets the value of [`Self::supports_texture_gather_lod_bias_amd`]
    pub fn supports_texture_gather_lod_bias_amd(&self) -> bool {
        unsafe { std::mem::transmute(self.supports_texture_gather_lod_bias_amd as u8) }
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
    ///Gets a mutable reference to the value of [`Self::supports_texture_gather_lod_bias_amd`]
    pub fn supports_texture_gather_lod_bias_amd_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.supports_texture_gather_lod_bias_amd as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.supports_texture_gather_lod_bias_amd as *mut Bool32)
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
    ///Sets the value of [`Self::supports_texture_gather_lod_bias_amd`]
    pub fn set_supports_texture_gather_lod_bias_amd(&mut self, value: bool) -> &mut Self {
        self.supports_texture_gather_lod_bias_amd = value as u8 as u32;
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
    ///Sets the value of [`Self::supports_texture_gather_lod_bias_amd`]
    pub fn with_supports_texture_gather_lod_bias_amd(mut self, value: bool) -> Self {
        self.supports_texture_gather_lod_bias_amd = value as u8 as u32;
        self
    }
}
