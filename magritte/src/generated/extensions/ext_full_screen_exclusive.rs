use crate::{
    native::HMONITOR,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_full_screen_exclusive");
///[VkFullScreenExclusiveEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFullScreenExclusiveEXT.html) - Hint values an application can specify affecting full-screen transition behavior
///# C Specifications
///Possible values of
///[`SurfaceFullScreenExclusiveInfoEXT::full_screen_exclusive`] are:
///```c
///// Provided by VK_EXT_full_screen_exclusive
///typedef enum VkFullScreenExclusiveEXT {
///    VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT = 0,
///    VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT = 1,
///    VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT = 2,
///    VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT = 3,
///} VkFullScreenExclusiveEXT;
///```
///# Description
/// - [`FullScreenExclusiveDefaultExt`] indicates the implementation **should** determine the
///   appropriate full-screen method by whatever means it deems appropriate.
/// - [`FullScreenExclusiveAllowedExt`] indicates the implementation **may** use full-screen
///   exclusive mechanisms when available. Such mechanisms **may** result in better performance
///   and/or the availability of different presentation capabilities, but **may** require a more
///   disruptive transition during swapchain initialization, first presentation and/or destruction.
/// - [`FullScreenExclusiveDisallowedExt`] indicates the implementation **should** avoid using
///   full-screen mechanisms which rely on disruptive transitions.
/// - [`FullScreenExclusiveApplicationControlledExt`] indicates the application will manage
///   full-screen exclusive mode by using the [`AcquireFullScreenExclusiveModeEXT`] and
///   [`ReleaseFullScreenExclusiveModeEXT`] commands.
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`SurfaceFullScreenExclusiveInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFullScreenExclusiveEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum FullScreenExclusiveEXT {
    ///[`FullScreenExclusiveDefaultExt`] indicates the implementation
    ///**should** determine the appropriate full-screen method by whatever means
    ///it deems appropriate.
    FullScreenExclusiveDefaultExt = 0,
    ///[`FullScreenExclusiveAllowedExt`] indicates the implementation
    ///**may** use full-screen exclusive mechanisms when available.
    ///Such mechanisms **may** result in better performance and/or the
    ///availability of different presentation capabilities, but **may** require a
    ///more disruptive transition during swapchain initialization, first
    ///presentation and/or destruction.
    FullScreenExclusiveAllowedExt = 1,
    ///[`FullScreenExclusiveDisallowedExt`] indicates the
    ///implementation **should** avoid using full-screen mechanisms which rely on
    ///disruptive transitions.
    FullScreenExclusiveDisallowedExt = 2,
    ///[`FullScreenExclusiveApplicationControlledExt`] indicates the
    ///application will manage full-screen exclusive mode by using the
    ///[`AcquireFullScreenExclusiveModeEXT`] and
    ///[`ReleaseFullScreenExclusiveModeEXT`] commands.
    FullScreenExclusiveApplicationControlledExt = 3,
}
impl const Default for FullScreenExclusiveEXT {
    fn default() -> Self {
        FullScreenExclusiveDefaultExt
    }
}
impl FullScreenExclusiveEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkSurfaceFullScreenExclusiveInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html) - Structure specifying the preferred full-screen transition behavior
///# C Specifications
///If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] includes a
///[`SurfaceFullScreenExclusiveInfoEXT`] structure, then that structure
///specifies the application’s preferred full-screen transition behavior.The
/// [`SurfaceFullScreenExclusiveInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_full_screen_exclusive
///typedef struct VkSurfaceFullScreenExclusiveInfoEXT {
///    VkStructureType             sType;
///    void*                       pNext;
///    VkFullScreenExclusiveEXT    fullScreenExclusive;
///} VkSurfaceFullScreenExclusiveInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`full_screen_exclusive`] is a [`FullScreenExclusiveEXT`] value specifying the preferred
///   full-screen transition behavior.
///# Description
///If this structure is not present, [`full_screen_exclusive`] is considered to
///be `VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT`.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT`
/// - [`full_screen_exclusive`]**must** be a valid [`FullScreenExclusiveEXT`] value
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`FullScreenExclusiveEXT`]
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
pub struct SurfaceFullScreenExclusiveInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`full_screen_exclusive`] is a [`FullScreenExclusiveEXT`] value
    ///specifying the preferred full-screen transition behavior.
    full_screen_exclusive: FullScreenExclusiveEXT,
}
///[VkSurfaceFullScreenExclusiveWin32InfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html) - Structure specifying additional creation parameters specific to Win32 fullscreen exclusive mode
///# C Specifications
///The [`SurfaceFullScreenExclusiveWin32InfoEXT`] structure is defined as:
///```c
///// Provided by VK_KHR_win32_surface with VK_EXT_full_screen_exclusive
///typedef struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    HMONITOR           hmonitor;
///} VkSurfaceFullScreenExclusiveWin32InfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`hmonitor`] is the Win32 [`HMONITOR`] handle identifying the display to create the surface
///   with.
///# Description
///Valid Usage
/// - [`hmonitor`]**must** be a valid [`HMONITOR`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT`
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`VK_KHR_win32_surface`]
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
pub struct SurfaceFullScreenExclusiveWin32InfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`hmonitor`] is the Win32 [`HMONITOR`] handle identifying the display
    ///to create the surface with.
    hmonitor: HMONITOR,
}
///[VkSurfaceCapabilitiesFullScreenExclusiveEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html) - Structure describing full screen exclusive capabilities of a surface
///# C Specifications
///The [`SurfaceCapabilitiesFullScreenExclusiveEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_full_screen_exclusive
///typedef struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           fullScreenExclusiveSupported;
///} VkSurfaceCapabilitiesFullScreenExclusiveEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - `fullScreenExclusiveControlSupported` is a boolean describing whether the surface is able to
///   make use of exclusive full-screen access.
///# Description
///This structure **can** be included in the [`p_next`] chain of
///[`SurfaceCapabilities2KHR`] to determine support for exclusive
///full-screen access.
///If [`full_screen_exclusive_supported`] is [`FALSE`], it indicates that
///exclusive full-screen access is not obtainable for this surface.Applications **must** not
/// attempt to create swapchains with
///`VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT` set if
///[`full_screen_exclusive_supported`] is [`FALSE`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT`
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
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
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    full_screen_exclusive_supported: Bool32,
}
