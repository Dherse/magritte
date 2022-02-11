use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION")]
pub const KHR_SYNCHRONIZATION_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME")]
pub const KHR_SYNCHRONIZATION_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_synchronization2");
///[VkFlags64](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFlags64.html) - Vulkan 64-bit bitmasks
///# C Specifications
///A collection of 64-bit flags is represented by a bitmask using the type
///[`Flags64`]:
///```c
///// Provided by VK_KHR_synchronization2
///typedef uint64_t VkFlags64;
///```
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`Flags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFlags64")]
pub type Flags64 = u64;
