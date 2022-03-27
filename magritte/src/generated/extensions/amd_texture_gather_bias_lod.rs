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
///argument values **can** be used with a given image format, add a
///[`TextureLodGatherFormatPropertiesAMD`] structure to the [`p_next`]
///chain of the [`ImageFormatProperties2`] structure in a call to
///[`GetPhysicalDeviceImageFormatProperties2`].The [`TextureLodGatherFormatPropertiesAMD`]
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
///   gather bias/LOD functions, as introduced by the `[`VK_AMD_texture_gather_bias_lod`]`
///   extension. This field is set by the implementation. User-specified value is ignored.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD`
///# Related
/// - [`VK_AMD_texture_gather_bias_lod`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct TextureLodGatherFormatPropertiesAMD<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`supports_texture_gather_lod_bias_amd`] tells if the image format can be
    ///used with texture gather bias/LOD functions, as introduced by the
    ///`[`VK_AMD_texture_gather_bias_lod`]` extension.
    ///This field is set by the implementation.
    ///User-specified value is ignored.
    supports_texture_gather_lod_bias_amd: Bool32,
}
