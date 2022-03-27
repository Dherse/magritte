use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_ANDROID_SURFACE_SPEC_VERSION")]
pub const KHR_ANDROID_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_ANDROID_SURFACE_EXTENSION_NAME")]
pub const KHR_ANDROID_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_android_surface");
///[ANativeWindow](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/ANativeWindow.html) - Android native window type
///# C Specifications
///To remove an unnecessary compile-time dependency, an incomplete type
///definition of [`ANativeWindow`] is provided in the Vulkan headers:
///```c
///// Provided by VK_KHR_android_surface
///struct ANativeWindow;
///```
///# Related
/// - [`VK_KHR_android_surface`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
pub type ANativeWindow = c_void;
///[VkAndroidSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidSurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Android surface object
///# C Specifications
///The [`AndroidSurfaceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_android_surface
///typedef struct VkAndroidSurfaceCreateInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkAndroidSurfaceCreateFlagsKHR    flags;
///    struct ANativeWindow*             window;
///} VkAndroidSurfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`window`] is a pointer to the [`ANativeWindow`] to associate the surface with.
///# Description
///Valid Usage
/// - [`window`]**must** point to a valid Android [`ANativeWindow`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_KHR_android_surface`]
/// - [`AndroidSurfaceCreateFlagsKHR`]
/// - [`StructureType`]
/// - [`CreateAndroidSurfaceKHR`]
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
pub struct AndroidSurfaceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: AndroidSurfaceCreateFlagsKHR,
    ///[`window`] is a pointer to the [`ANativeWindow`] to associate the
    ///surface with.
    window: *const ANativeWindow,
}
