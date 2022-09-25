//![VK_KHR_portability_enumeration](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_portability_enumeration.html) - instance extension
//!# Description
//!This extension allows applications to control whether devices that expose
//!the `[`khr_portability_subset`]` extension are included in the results
//!of physical device enumeration.
//!Since devices which support the `[`khr_portability_subset`]` extension
//!are not fully conformant Vulkan implementations, the Vulkan loader does not
//!report those devices unless the application explicitly asks for them.
//!This prevents applications which may not be aware of non-conformant devices
//!from accidentally using them, as any device which supports the
//!`[`khr_portability_subset`]` extension mandates that the extension
//!must be enabled if that device is used.This extension is implemented in the loader.
# ! [doc = concat ! ("# " , "Revision")]
//!1
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
# ! [doc = concat ! ("# " , "Contacts")]
//! - Charles Giessen [charles-lunarg](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_portability_enumeration]
//!   @charles-lunarg%0A<<Here describe the issue or question you have about the
//!   VK_KHR_portability_enumeration extension>>)
# ! [doc = concat ! ("# " , "New constants")]
//! - [`KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME`]
//! - [`KHR_PORTABILITY_ENUMERATION_SPEC_VERSION`]
//! - Extending [`InstanceCreateFlagBits`]:  - `VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR`
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2021-06-02 (Lenny Komow)  - Initial version
//!# Other info
//! * 2021-06-02
//! * No known IP claims.
//! * - Interacts with `[`khr_portability_subset`]`
//! * - Lenny Komow, LunarG  - Charles Giessen, LunarG
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION")]
pub const KHR_PORTABILITY_ENUMERATION_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME")]
pub const KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_portability_enumeration");
