//![VK_EXT_pipeline_creation_feedback](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_pipeline_creation_feedback.html) - device extension
//!# Description
//!This extension adds a mechanism to provide feedback to an application about
//!pipeline creation, with the specific goal of allowing a feedback loop
//!between build systems and in-the-field application executions to ensure
//!effective pipeline caches are shipped to customers.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Deprecation State
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Contacts
//! - Jean-Francois Roy [jfroy](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_pipeline_creation_feedback]
//!   @jfroy%0A<<Here describe the issue or question you have about the
//!   VK_EXT_pipeline_creation_feedback extension>>)
//!# New structures
//! - [`PipelineCreationFeedbackEXT`]
//! - Extending [`GraphicsPipelineCreateInfo`], [`ComputePipelineCreateInfo`],
//!   [`RayTracingPipelineCreateInfoNV`], [`RayTracingPipelineCreateInfoKHR`]:  -
//!   [`PipelineCreationFeedbackCreateInfoEXT`]
//!# New enums
//! - [`PipelineCreationFeedbackFlagBitsEXT`]
//!# New bitmasks
//! - [`PipelineCreationFeedbackFlagsEXT`]
//!# New constants
//! - [`EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME`]
//! - [`EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT`
//!# Version history
//! - Revision 1, 2019-03-12 (Jean-Francois Roy)  - Initial revision
//!# Other information
//! * 2019-03-12
//! * - Promoted to Vulkan 1.3 Core
//! * No known IP claims.
//! * - Jean-Francois Roy, Google  - Hai Nguyen, Google  - Andrew Ellem, Google  - Bob Fraser,
//!   Google  - Sujeevan Rajayogam, Google  - Jan-Harald Fredriksen, ARM  - Jeff Leger, Qualcomm
//!   Technologies, Inc.  - Jeff Bolz, NVIDIA  - Daniel Koch, NVIDIA  - Neil Henning, AMD
//!# Related
//! - [`PipelineCreationFeedbackCreateInfoEXT`]
//! - [`PipelineCreationFeedbackEXT`]
//! - [`PipelineCreationFeedbackFlagBitsEXT`]
//! - [`PipelineCreationFeedbackFlagsEXT`]
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
#[doc(alias = "VK_EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION")]
pub const EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME")]
pub const EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_pipeline_creation_feedback");
