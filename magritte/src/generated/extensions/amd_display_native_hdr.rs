use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION")]
pub const AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME")]
pub const AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_display_native_hdr");
///[VkDisplayNativeHdrSurfaceCapabilitiesAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html) - Structure describing display native HDR specific capabilities of a surface
///# C Specifications
///The [`DisplayNativeHdrSurfaceCapabilitiesAMD`] structure is defined as:
///```c
///// Provided by VK_AMD_display_native_hdr
///typedef struct VkDisplayNativeHdrSurfaceCapabilitiesAMD {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           localDimmingSupport;
///} VkDisplayNativeHdrSurfaceCapabilitiesAMD;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`local_dimming_support`] specifies whether the surface supports local dimming. If this is
///   [`TRUE`], [`SwapchainDisplayNativeHdrCreateInfoAMD`]**can** be used to explicitly enable or
///   disable local dimming for the surface. Local dimming may also be overriden by
///   [`SetLocalDimmingAMD`] during the lifetime of the swapchain.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD`
///# Related
/// - [`VK_AMD_display_native_hdr`]
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
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`local_dimming_support`] specifies whether the surface supports local
    ///dimming.
    ///If this is [`TRUE`], [`SwapchainDisplayNativeHdrCreateInfoAMD`]**can** be used to explicitly
    /// enable or disable local dimming for the surface.
    ///Local dimming may also be overriden by [`SetLocalDimmingAMD`] during
    ///the lifetime of the swapchain.
    local_dimming_support: Bool32,
}
///[VkSwapchainDisplayNativeHdrCreateInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainDisplayNativeHdrCreateInfoAMD.html) - Structure specifying display native HDR parameters of a newly created swapchain object
///# C Specifications
///If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] includes a
///[`SwapchainDisplayNativeHdrCreateInfoAMD`] structure, then that
///structure includes additional swapchain creation parameters specific to
///display native HDR support.The [`SwapchainDisplayNativeHdrCreateInfoAMD`] structure is defined
/// as:
///```c
///// Provided by VK_AMD_display_native_hdr
///typedef struct VkSwapchainDisplayNativeHdrCreateInfoAMD {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           localDimmingEnable;
///} VkSwapchainDisplayNativeHdrCreateInfoAMD;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`local_dimming_enable`] specifies whether local dimming is enabled for the swapchain.
///# Description
///If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] does not include
///this structure, the default value for [`local_dimming_enable`] is
///[`TRUE`], meaning local dimming is initially enabled for the swapchain.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD`
///Valid Usage
/// - It is only valid to set [`local_dimming_enable`] to [`TRUE`] if
///   [`DisplayNativeHdrSurfaceCapabilitiesAMD::local_dimming_support`] is supported
///# Related
/// - [`VK_AMD_display_native_hdr`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`local_dimming_enable`] specifies whether local dimming is enabled for
    ///the swapchain.
    local_dimming_enable: Bool32,
}
