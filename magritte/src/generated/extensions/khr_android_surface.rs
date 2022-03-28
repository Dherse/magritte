//![VK_KHR_android_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_android_surface.html) - instance extension
//!# Description
//!The [`VK_KHR_android_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to an
//![`ANativeWindow`], Android’s native surface type.
//!The [`ANativeWindow`] represents the producer endpoint of any buffer
//!queue, regardless of consumer endpoint.
//!Common consumer endpoints for `ANativeWindows` are the system window
//!compositor, video encoders, and application-specific compositors importing
//!the images through a `SurfaceTexture`.
//!# Revision
//!6
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_android_surface]
//!   @critsec%0A<<Here describe the issue or question you have about the VK_KHR_android_surface
//!   extension>>)
//!# New functions & commands
//! - [`CreateAndroidSurfaceKHR`]
//!# New structures
//! - [`AndroidSurfaceCreateInfoKHR`]
//!# New bitmasks
//! - [`AndroidSurfaceCreateFlagsKHR`]
//!# New constants
//! - [`KHR_ANDROID_SURFACE_EXTENSION_NAME`]
//! - [`KHR_ANDROID_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Does Android need a way to query for compatibility between a particular
//!physical device (and queue family?) and a specific Android display? **RESOLVED** : No.
//!Currently on Android, any physical device is expected to be able to present
//!to the system compositor, and all queue families must support the necessary
//!image layout transitions and synchronization operations.
//!# Version History
//! - Revision 1, 2015-09-23 (Jesse Hall)  - Initial draft.
//! - Revision 2, 2015-10-26 (Ian Elliott)  - Renamed from VK_EXT_KHR_android_surface to
//!   VK_KHR_android_surface.
//! - Revision 3, 2015-11-03 (Daniel Rakos)  - Added allocation callbacks to surface creation
//!   function.
//! - Revision 4, 2015-11-10 (Jesse Hall)  - Removed VK_ERROR_INVALID_ANDROID_WINDOW_KHR.
//! - Revision 5, 2015-11-28 (Daniel Rakos)  - Updated the surface create function to take a
//!   pCreateInfo structure.
//! - Revision 6, 2016-01-14 (James Jones)  - Moved VK_ERROR_NATIVE_WINDOW_IN_USE_KHR from the
//!   VK_KHR_android_surface to the VK_KHR_surface extension.
//!# Other info
//! * 2016-01-14
//! * No known IP claims.
//! * - Patrick Doane, Blizzard  - Jason Ekstrand, Intel  - Ian Elliott, LunarG  - Courtney
//!   Goeltzenleuchter, LunarG  - Jesse Hall, Google  - James Jones, NVIDIA  - Antoine Labour,
//!   Google  - Jon Leech, Khronos  - David Mao, AMD  - Norbert Nopper, Freescale  - Alon Or-bach,
//!   Samsung  - Daniel Rakos, AMD  - Graham Sellers, AMD  - Ray Smith, ARM  - Jeff Vigil, Qualcomm
//!   - Chia-I Wu, LunarG
//!# Related
//! - [`ANativeWindow`]
//! - [`AndroidSurfaceCreateFlagsKHR`]
//! - [`AndroidSurfaceCreateInfoKHR`]
//! - [`CreateAndroidSurfaceKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
///[VkAndroidSurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidSurfaceCreateFlagsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_android_surface
///typedef VkFlags VkAndroidSurfaceCreateFlagsKHR;
///```
///# Related
/// - [`VK_KHR_android_surface`]
/// - [`AndroidSurfaceCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct AndroidSurfaceCreateFlagsKHR(u32);
impl const Default for AndroidSurfaceCreateFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for AndroidSurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(AndroidSurfaceCreateFlagsKHR))
            .field(&self.0)
            .finish()
    }
}
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
///## Valid Usage
/// - [`window`] **must**  point to a valid Android [`ANativeWindow`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
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
#[doc(alias = "VkAndroidSurfaceCreateInfoKHR")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AndroidSurfaceCreateInfoKHR<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: AndroidSurfaceCreateFlagsKHR,
    ///[`window`] is a pointer to the [`ANativeWindow`] to associate the
    ///surface with.
    pub window: *mut ANativeWindow,
}
impl<'lt> Default for AndroidSurfaceCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            window: std::ptr::null_mut(),
        }
    }
}
impl<'lt> AndroidSurfaceCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::window`]
    pub fn window_raw(&self) -> &*mut ANativeWindow {
        &self.window
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::window`]
    pub fn set_window_raw(&mut self, value: *mut ANativeWindow) -> &mut Self {
        self.window = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> AndroidSurfaceCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::window`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn window(&self) -> &ANativeWindow {
        &*self.window
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut AndroidSurfaceCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::window`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn window_mut(&mut self) -> &mut ANativeWindow {
        &mut *self.window
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
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_android_surface::AndroidSurfaceCreateFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::window`]
    pub fn set_window(&mut self, value: &'lt mut crate::extensions::khr_android_surface::ANativeWindow) -> &mut Self {
        self.window = value as *mut _;
        self
    }
}
