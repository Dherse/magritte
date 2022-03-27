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
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PresentFrameTokenGGP<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`frame_token`] is the Google Games Platform frame token.
    frame_token: GgpFrameToken,
}
impl<'lt> Default for PresentFrameTokenGGP<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            frame_token: Default::default(),
        }
    }
}
impl<'lt> PresentFrameTokenGGP<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::frame_token`]
    pub fn frame_token_raw(&self) -> &GgpFrameToken {
        &self.frame_token
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::frame_token`]
    pub fn set_frame_token_raw(&mut self, value: GgpFrameToken) -> &mut Self {
        self.frame_token = value;
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
    ///Gets the value of [`Self::frame_token`]
    pub fn frame_token(&self) -> &GgpFrameToken {
        &self.frame_token
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::frame_token`]
    pub fn frame_token_mut(&mut self) -> &mut GgpFrameToken {
        &mut self.frame_token
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
    ///Sets the raw value of [`Self::frame_token`]
    pub fn set_frame_token(&mut self, value: crate::native::GgpFrameToken) -> &mut Self {
        self.frame_token = value;
        self
    }
}
