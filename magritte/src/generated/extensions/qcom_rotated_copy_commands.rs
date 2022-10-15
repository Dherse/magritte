//![VK_QCOM_rotated_copy_commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QCOM_rotated_copy_commands.html) - device extension
//!# Description
//!This extension extends adds an optional rotation transform to copy commands
//![`cmd_blit_image2_khr`], [`cmd_copy_image_to_buffer2_khr`] and
//![`cmd_copy_buffer_to_image2_khr`].
//!When copying between two resources, where one resource contains rotated
//!content and the other does not, a rotated copy may be desired.
//!This extension may be used in combination with VK_QCOM_render_pass_transform
//!which adds rotated render passes.This extension adds an extension structure to the following
//! commands:
//!vkCmdBlitImage2KHR, vkCmdCopyImageToBuffer2KHR and
//!vkCmdCopyBufferToImage2KHR
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_swapchain`]`
//! - Requires `[`khr_copy_commands2`]`
//!# Contacts
//! - Jeff Leger [jackohound](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_QCOM_rotated_copy_commands]
//!   @jackohound%0A<<Here describe the issue or question you have about the
//!   VK_QCOM_rotated_copy_commands extension>>)
//!# New structures
//! - Extending [`BufferImageCopy2`], [`ImageBlit2`]:  - [`CopyCommandTransformInfoQCOM`]
//!# New constants
//! - [`QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME`]
//! - [`QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM`
//!# Known issues & F.A.Q.
//!1) What is an appropriate name for the added extension structure? The style
//!guide says “Structures which extend other structures through the
//!`pNext` chain should reflect the name of the base structure they
//!extend.”, but in this case a single extension structure is used to extend
//!three base structures (vkCmdBlitImage2KHR, vkCmdCopyImageToBuffer2KHR and
//!vkCmdCopyBufferToImage2KHR).
//!Creating three identical structures with unique names seemed undesirable. **RESOLVED** : Deviate
//! from the style guide for extension structure naming.2) Should this extension add a rotation
//! capability to vkCmdCopyImage2KHR? **RESOLVED** : No.
//!Use of rotated vkCmdBlitImage2KHR can fully address this use-case.3) Should this extension add a
//! rotation capability to vkCmdResolveImage2KHR? **RESOLVED**  No.
//!Use of vkCmdResolveImage2KHR is very slow and extremely bandwidth intensive
//!on Qualcomm’s GPU architecture and use of pResolveAttachments in
//!vkRenderPass is the strongly preferred approach.
//!Therefore, we choose not to introduce a rotation capability to
//!vkCmdResolveImage2KHR.
//!# Version history
//! - Revision 1, 2020-09-19 (Jeff Leger)
//!# Other information
//! * 2020-09-18
//! * - None
//! * - Jeff Leger, Qualcomm Technologies, Inc.
//!# Related
//! - [`CopyCommandTransformInfoQCOM`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION")]
pub const QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME")]
pub const QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_QCOM_rotated_copy_commands");
///[VkCopyCommandTransformInfoQCOM](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyCommandTransformInfoQCOM.html) - Structure describing transform parameters of rotated copy command
///# C Specifications
///The [`RenderPassTransformBeginInfoQCOM`] structure is defined as:
///```c
///// Provided by VK_QCOM_rotated_copy_commands
///typedef struct VkCopyCommandTransformInfoQCOM {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkSurfaceTransformFlagBitsKHR    transform;
///} VkCopyCommandTransformInfoQCOM;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`transform`] is a [`SurfaceTransformFlagBitsKHR`] value describing the transform to be
///   applied.
/// # Description
/// ## Valid Usage
/// - [`transform`] **must**  be `VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR`,
///   `VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR`, `VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR`, or
///   `VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR`
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM`
/// # Related
/// - [`qcom_rotated_copy_commands`]
/// - [`StructureType`]
/// - [`SurfaceTransformFlagBitsKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCopyCommandTransformInfoQCOM")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct CopyCommandTransformInfoQCOM<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`transform`] is a [`SurfaceTransformFlagBitsKHR`] value
    ///describing the transform to be applied.
    pub transform: SurfaceTransformFlagBitsKHR,
}
impl<'lt> Default for CopyCommandTransformInfoQCOM<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::COPY_COMMAND_TRANSFORM_INFO_QCOM,
            p_next: std::ptr::null(),
            transform: Default::default(),
        }
    }
}
impl<'lt> CopyCommandTransformInfoQCOM<'lt> {
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
    ///Gets the value of [`Self::transform`]
    pub fn transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.transform
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::transform`]
    pub fn transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.transform
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
    ///Sets the value of [`Self::transform`]
    pub fn set_transform(&mut self, value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> &mut Self {
        self.transform = value;
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
    ///Sets the value of [`Self::transform`]
    pub fn with_transform(mut self, value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> Self {
        self.transform = value;
        self
    }
}
