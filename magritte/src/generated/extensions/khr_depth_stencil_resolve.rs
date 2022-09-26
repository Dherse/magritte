//![VK_KHR_depth_stencil_resolve](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_depth_stencil_resolve.html) - device extension
//!# Description
//!This extension adds support for automatically resolving multisampled
//!depth/stencil attachments in a subpass in a similar manner as for color
//!attachments.Multisampled color attachments can be resolved at the end of a subpass by
//!specifying `pResolveAttachments` entries corresponding to the
//!`pColorAttachments` array entries.
//!This does not allow for a way to map the resolve attachments to the
//!depth/stencil attachment.
//!The [`cmd_resolve_image`] command does not allow for depth/stencil images.
//!While there are other ways to resolve the depth/stencil attachment, they can
//!give sub-optimal performance.
//!Extending the [`SubpassDescription2`] in this extension allows an
//!application to add a `pDepthStencilResolveAttachment`, that is similar
//!to the color `pResolveAttachments`, that the
//!`pDepthStencilAttachment` can be resolved into.Depth and stencil samples are resolved to a
//! single value based on the
//!resolve mode.
//!The set of possible resolve modes is defined in the
//![`ResolveModeFlagBits`] enum.
//!The `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT` mode is the only mode that is
//!required of all implementations (that support the extension or support
//!Vulkan 1.2 or higher).
//!Some implementations may also support averaging (the same as color sample
//!resolve) or taking the minimum or maximum sample, which may be more suitable
//!for depth/stencil resolve.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_create_renderpass2`]`
//!# Deprecation State
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Contacts
//! - Jan-Harald Fredriksen [janharald](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_depth_stencil_resolve]
//!   @janharald%0A<<Here describe the issue or question you have about the
//!   VK_KHR_depth_stencil_resolve extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceDepthStencilResolvePropertiesKHR`]
//! - Extending [`SubpassDescription2`]:  - [`SubpassDescriptionDepthStencilResolveKHR`]
//!# New enums
//! - [`ResolveModeFlagBitsKHR`]
//!# New bitmasks
//! - [`ResolveModeFlagsKHR`]
//!# New constants
//! - [`KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME`]
//! - [`KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION`]
//! - Extending [`ResolveModeFlagBits`]:  - `VK_RESOLVE_MODE_AVERAGE_BIT_KHR`  -
//!   `VK_RESOLVE_MODE_MAX_BIT_KHR`  - `VK_RESOLVE_MODE_MIN_BIT_KHR`  - `VK_RESOLVE_MODE_NONE_KHR`
//!   - `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR`
//!# Version history
//! - Revision 1, 2018-04-09 (Jan-Harald Fredriksen)  - Initial revision
//!# Other information
//! * 2018-04-09
//! * - Promoted to Vulkan 1.2 Core
//! * - Jan-Harald Fredriksen, Arm  - Andrew Garrard, Samsung Electronics  - Soowan Park, Samsung
//!   Electronics  - Jeff Bolz, NVIDIA  - Daniel Rakos, AMD
//!# Related
//! - [`PhysicalDeviceDepthStencilResolvePropertiesKHR`]
//! - [`ResolveModeFlagBitsKHR`]
//! - [`ResolveModeFlagsKHR`]
//! - [`SubpassDescriptionDepthStencilResolveKHR`]
//!
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
#[doc(alias = "VK_KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION")]
pub const KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME")]
pub const KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_depth_stencil_resolve");
