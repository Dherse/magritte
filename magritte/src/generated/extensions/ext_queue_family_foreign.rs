use std::ffi::CStr;
///[VK_QUEUE_FAMILY_FOREIGN_EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_FOREIGN_EXT.html) - Foreign queue family index sentinel
///# C Specifications
///The special queue family index [`QUEUE_FAMILY_FOREIGN_EXT`] represents
///any queue external to the resource’s current Vulkan instance, regardless of
///the queue’s underlying physical device or driver version.
///This includes, for example, queues for fixed-function image processing
///devices, media codec devices, and display devices, as well as all queues
///that use the same underlying
///device group or
///physical device, and the same driver version as the resource’s
///[`Device`].
///```c
///#define VK_QUEUE_FAMILY_FOREIGN_EXT       (~2U)
///```
///# Related
/// - [`VK_EXT_queue_family_foreign`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_QUEUE_FAMILY_FOREIGN_EXT")]
pub const QUEUE_FAMILY_FOREIGN_EXT: u32 = !2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION")]
pub const EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME")]
pub const EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_queue_family_foreign");
