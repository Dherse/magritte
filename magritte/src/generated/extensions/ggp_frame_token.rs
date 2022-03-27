use crate::{
    native::GgpFrameToken,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GGP_FRAME_TOKEN_SPEC_VERSION")]
pub const GGP_FRAME_TOKEN_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GGP_FRAME_TOKEN_EXTENSION_NAME")]
pub const GGP_FRAME_TOKEN_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_GGP_frame_token");
///[VkPresentFrameTokenGGP](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentFrameTokenGGP.html) - The Google Games Platform frame token
///# C Specifications
///When the `[`VK_GGP_frame_token`]` extension is enabled, a Google Games
///Platform frame token **can** be specified when presenting an image to a
///swapchain by adding a [`PresentFrameTokenGGP`] structure to the
///[`p_next`] chain of the [`PresentInfoKHR`] structure.The [`PresentFrameTokenGGP`] structure is
/// defined as:
///```c
///// Provided by VK_GGP_frame_token
///typedef struct VkPresentFrameTokenGGP {
///    VkStructureType    sType;
///    const void*        pNext;
///    GgpFrameToken      frameToken;
///} VkPresentFrameTokenGGP;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`frame_token`] is the Google Games Platform frame token.
///# Description
///Valid Usage
/// - [`frame_token`]**must** be a valid [`GgpFrameToken`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PRESENT_FRAME_TOKEN_GGP`
///# Related
/// - [`VK_GGP_frame_token`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PresentFrameTokenGGP<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`frame_token`] is the Google Games Platform frame token.
    frame_token: GgpFrameToken,
}
