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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`transform`] is a [`SurfaceTransformFlagBitsKHR`] value describing the transform to be
///   applied.
///# Description
///Valid Usage
/// - [`transform`]**must** be `VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR`,
///   `VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR`, `VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR`, or
///   `VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM`
///# Related
/// - [`VK_QCOM_rotated_copy_commands`]
/// - [`StructureType`]
/// - [`SurfaceTransformFlagBitsKHR`]
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
pub struct CopyCommandTransformInfoQCOM<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`transform`] is a [`SurfaceTransformFlagBitsKHR`] value
    ///describing the transform to be applied.
    transform: SurfaceTransformFlagBitsKHR,
}
