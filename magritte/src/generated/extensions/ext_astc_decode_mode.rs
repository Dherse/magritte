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
///Valid Usage
/// - [`decode_mode`]**must** be one of `VK_FORMAT_R16G16B16A16_SFLOAT`, `VK_FORMAT_R8G8B8A8_UNORM`,
///   or `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32`
/// - If the [`decodeModeSharedExponent`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-astc-decodeModeSharedExponent)
///   feature is not enabled, [`decode_mode`]**must** not be `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32`
/// - If [`decode_mode`] is `VK_FORMAT_R8G8B8A8_UNORM` the image view **must** not include blocks
///   using any of the ASTC HDR modes
/// - `format` of the image view **must** be one of the [ASTC Compressed Image Formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#appendix-compressedtex-astc)
///If `format` uses sRGB encoding then the [`decode_mode`] has no effect.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT`
/// - [`decode_mode`]**must** be a valid [`Format`] value
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImageViewAstcDecodeModeEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`decode_mode`] is the intermediate format used to decode ASTC
    ///compressed formats.
    decode_mode: Format,
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
///[`PhysicalDeviceAstcDecodeFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceAstcDecodeFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`decode_mode_shared_exponent`] indicates whether the implementation
    ///supports decoding ASTC compressed formats to
    ///`VK_FORMAT_E5B9G9R9_UFLOAT_PACK32` internal precision.
    decode_mode_shared_exponent: Bool32,
}
