use crate::vulkan1_0::{BaseInStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION")]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_surface_protected_capabilities");
///[VkSurfaceProtectedCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceProtectedCapabilitiesKHR.html) - Structure describing capability of a surface to be protected
///# C Specifications
///An application queries if a protected [`SurfaceKHR`] is displayable on a
///specific windowing system using [`SurfaceProtectedCapabilitiesKHR`],
///which **can** be passed in [`p_next`] parameter of
///[`SurfaceCapabilities2KHR`].The [`SurfaceProtectedCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_surface_protected_capabilities
///typedef struct VkSurfaceProtectedCapabilitiesKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           supportsProtected;
///} VkSurfaceProtectedCapabilitiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`supports_protected`] specifies whether a protected swapchain created from
///   [`PhysicalDeviceSurfaceInfo2KHR::surface`] for a particular windowing system **can** be
///   displayed on screen or not. If [`supports_protected`] is [`TRUE`], then creation of swapchains
///   with the `VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR` flag set **must** be supported for `surface`.
///# Description
///If the `[`VK_GOOGLE_surfaceless_query`]` extension is enabled, the value
///returned in [`supports_protected`] will be identical for every valid
///surface created on this physical device, and so in the
///[`GetPhysicalDeviceSurfaceCapabilities2KHR`] call,
///[`PhysicalDeviceSurfaceInfo2KHR::surface`]**can** be
///[`crate::utils::Handle::null`].
///In that case, the contents of
///[`SurfaceCapabilities2KHR::surface_capabilities`] as well as any
///other struct chained to it will be undefined.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR`
///# Related
/// - [`VK_KHR_surface_protected_capabilities`]
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
pub struct SurfaceProtectedCapabilitiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`supports_protected`] specifies whether a protected swapchain created
    ///from [`PhysicalDeviceSurfaceInfo2KHR`]::`surface` for a
    ///particular windowing system **can** be displayed on screen or not.
    ///If [`supports_protected`] is [`TRUE`], then creation of swapchains
    ///with the `VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR` flag set **must** be
    ///supported for `surface`.
    supports_protected: Bool32,
}
