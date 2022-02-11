use std::ffi::{c_void, CStr};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_ANDROID_external_memory_android_hardware_buffer");
///[AHardwareBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/AHardwareBuffer.html) - Android hardware buffer type
///# C Specifications
///To remove an unnecessary compile-time dependency, an incomplete type
///definition of [`AHardwareBuffer`] is provided in the Vulkan headers:
///```c
///// Provided by VK_ANDROID_external_memory_android_hardware_buffer
///struct AHardwareBuffer;
///```
///# Related
/// - [`VK_ANDROID_external_memory_android_hardware_buffer`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
pub type AHardwareBuffer = c_void;
