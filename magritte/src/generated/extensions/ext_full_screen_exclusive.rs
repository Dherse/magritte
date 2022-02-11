#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
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
/// - [`FULL_SCREEN_EXCLUSIVE_DEFAULT`] indicates the implementation
///**should** determine the appropriate full-screen method by whatever means
///it deems appropriate.
/// - [`FULL_SCREEN_EXCLUSIVE_ALLOWED`] indicates the implementation
///**may** use full-screen exclusive mechanisms when available.
///Such mechanisms **may** result in better performance and/or the
///availability of different presentation capabilities, but **may** require a
///more disruptive transition during swapchain initialization, first
///presentation and/or destruction.
/// - [`FULL_SCREEN_EXCLUSIVE_DISALLOWED`] indicates the
///implementation **should** avoid using full-screen mechanisms which rely on
///disruptive transitions.
/// - [`FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED`] indicates the
///application will manage full-screen exclusive mode by using the
///[`AcquireFullScreenExclusiveModeEXT`] and
///[`ReleaseFullScreenExclusiveModeEXT`] commands.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct FullScreenExclusiveEXT(i32);
impl const Default for FullScreenExclusiveEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for FullScreenExclusiveEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("FullScreenExclusiveEXT")
            .field(match *self {
                Self::FULL_SCREEN_EXCLUSIVE_DEFAULT => &"FULL_SCREEN_EXCLUSIVE_DEFAULT",
                Self::FULL_SCREEN_EXCLUSIVE_ALLOWED => &"FULL_SCREEN_EXCLUSIVE_ALLOWED",
                Self::FULL_SCREEN_EXCLUSIVE_DISALLOWED => &"FULL_SCREEN_EXCLUSIVE_DISALLOWED",
                Self::FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED => &"FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED",
                other => unreachable!("invalid value for `FullScreenExclusiveEXT`: {:?}", other),
            })
            .finish()
    }
}
impl FullScreenExclusiveEXT {
    ///[`FULL_SCREEN_EXCLUSIVE_DEFAULT`] indicates the implementation
    ///**should** determine the appropriate full-screen method by whatever means
    ///it deems appropriate.
    pub const FULL_SCREEN_EXCLUSIVE_DEFAULT: Self = Self(0);
    ///[`FULL_SCREEN_EXCLUSIVE_ALLOWED`] indicates the implementation
    ///**may** use full-screen exclusive mechanisms when available.
    ///Such mechanisms **may** result in better performance and/or the
    ///availability of different presentation capabilities, but **may** require a
    ///more disruptive transition during swapchain initialization, first
    ///presentation and/or destruction.
    pub const FULL_SCREEN_EXCLUSIVE_ALLOWED: Self = Self(1);
    ///[`FULL_SCREEN_EXCLUSIVE_DISALLOWED`] indicates the
    ///implementation **should** avoid using full-screen mechanisms which rely on
    ///disruptive transitions.
    pub const FULL_SCREEN_EXCLUSIVE_DISALLOWED: Self = Self(2);
    ///[`FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED`] indicates the
    ///application will manage full-screen exclusive mode by using the
    ///[`AcquireFullScreenExclusiveModeEXT`] and
    ///[`ReleaseFullScreenExclusiveModeEXT`] commands.
    pub const FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED: Self = Self(3);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
