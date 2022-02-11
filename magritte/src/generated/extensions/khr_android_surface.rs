use std::ffi::{c_void, CStr};
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
