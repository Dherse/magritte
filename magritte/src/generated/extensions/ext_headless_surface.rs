//![VK_EXT_headless_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_headless_surface.html) - instance extension
//!# Description
//!The [`VK_EXT_headless_surface`] extension is an instance extension.
//!It provides a mechanism to create [`SurfaceKHR`] objects independently
//!of any window system or display device.
//!The presentation operation for a swapchain created from a headless surface
//!is by default a no-op, resulting in no externally-visible result.Because there is no real
//! presentation target, future extensions can layer on
//!top of the headless surface to introduce arbitrary or customisable sets of
//!restrictions or features.
//!These could include features like saving to a file or restrictions to
//!emulate a particular presentation target.This functionality is expected to be useful for
//! application and driver
//!development because it allows any platform to expose an arbitrary or
//!customisable set of restrictions and features of a presentation engine.
//!This makes it a useful portable test target for applications targeting a
//!wide range of presentation engines where the actual target presentation
//!engines might be scarce, unavailable or otherwise undesirable or
//!inconvenient to use for general Vulkan application development.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Lisa Wu [chengtianww](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_headless_surface]
//!   @chengtianww%0A<<Here describe the issue or question you have about the
//!   VK_EXT_headless_surface extension>>)
//!# New functions & commands
//! - [`CreateHeadlessSurfaceEXT`]
//!# New structures
//! - [`HeadlessSurfaceCreateInfoEXT`]
//!# New bitmasks
//! - [`HeadlessSurfaceCreateFlagsEXT`]
//!# New constants
//! - [`EXT_HEADLESS_SURFACE_EXTENSION_NAME`]
//! - [`EXT_HEADLESS_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2019-03-21 (Ray Smith)  - Initial draft
//!# Other info
//! * 2019-03-21
//! * No known IP claims.
//! * - Ray Smith, Arm
//!# Related
//! - [`HeadlessSurfaceCreateFlagsEXT`]
//! - [`HeadlessSurfaceCreateInfoEXT`]
//! - [`CreateHeadlessSurfaceEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_HEADLESS_SURFACE_SPEC_VERSION")]
pub const EXT_HEADLESS_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_HEADLESS_SURFACE_EXTENSION_NAME")]
pub const EXT_HEADLESS_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_headless_surface");
///[VkHeadlessSurfaceCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkHeadlessSurfaceCreateFlagsEXT.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_EXT_headless_surface
///typedef VkFlags VkHeadlessSurfaceCreateFlagsEXT;
///```
///# Related
/// - [`VK_EXT_headless_surface`]
/// - [`HeadlessSurfaceCreateInfoEXT`]
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
pub struct HeadlessSurfaceCreateFlagsEXT(u32);
impl const Default for HeadlessSurfaceCreateFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for HeadlessSurfaceCreateFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(HeadlessSurfaceCreateFlagsEXT))
            .field(&self.0)
            .finish()
    }
}
///[VkHeadlessSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkHeadlessSurfaceCreateInfoEXT.html) - Structure specifying parameters of a newly created headless surface object
///# C Specifications
///The [`HeadlessSurfaceCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_headless_surface
///typedef struct VkHeadlessSurfaceCreateInfoEXT {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkHeadlessSurfaceCreateFlagsEXT    flags;
///} VkHeadlessSurfaceCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`VK_EXT_headless_surface`]
/// - [`HeadlessSurfaceCreateFlagsEXT`]
/// - [`StructureType`]
/// - [`CreateHeadlessSurfaceEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkHeadlessSurfaceCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct HeadlessSurfaceCreateInfoEXT<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: HeadlessSurfaceCreateFlagsEXT,
}
impl<'lt> Default for HeadlessSurfaceCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
impl<'lt> HeadlessSurfaceCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> HeadlessSurfaceCreateFlagsEXT {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut HeadlessSurfaceCreateFlagsEXT {
        &mut self.flags
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
        value: crate::extensions::ext_headless_surface::HeadlessSurfaceCreateFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
}
