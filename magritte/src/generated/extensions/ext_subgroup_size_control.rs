//![VK_EXT_subgroup_size_control](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_subgroup_size_control.html) - device extension
//!# Description
//!This extension enables an implementation to control the subgroup size by
//!allowing a varying subgroup size and also specifying a required subgroup
//!size.It extends the subgroup support in Vulkan 1.1 to allow an implementation to
//!expose a varying subgroup size.
//!Previously Vulkan exposed a single subgroup size per physical device, with
//!the expectation that implementations will behave as if all subgroups have
//!the same size.
//!Some implementations  **may**  dispatch shaders with a varying subgroup size for
//!different subgroups.
//!As a result they could implicitly split a large subgroup into smaller
//!subgroups or represent a small subgroup as a larger subgroup, some of whose
//!invocations were inactive on launch.To aid developers in understanding the performance
//! characteristics of their
//!programs, this extension exposes a minimum and maximum subgroup size that a
//!physical device supports and a pipeline create flag to enable that pipeline
//!to vary its subgroup size.
//!If enabled, any `SubgroupSize` decorated variables in the SPIR-V shader
//!modules provided to pipeline creation  **may**  vary between the
//![minimum](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-minSubgroupSize) and [maximum](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxSubgroupSize)
//!subgroup sizes.An implementation is also optionally allowed to support specifying a
//!required subgroup size for a given pipeline stage.
//!Implementations advertise which [stages
//!support a required subgroup size](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-requiredSubgroupSizeStages), and any pipeline of a supported stage
//!can be passed a [`PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT`]
//!structure to set the subgroup size for that shader stage of the pipeline.
//!For compute shaders, this requires the developer to query the
//![`maxComputeWorkgroupSubgroups`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxComputeWorkgroupSubgroups)
//!and ensure that:<span class="katex"><span aria-hidden="true" class="katex-html"><span
//! class="base"><span style="height:0.43056em;vertical-align:0em;" class="strut"></span><span
//! class="mord mathdefault">s</span><span class="mspace"
//! style="margin-right:0.2777777777777778em;"></span><span class="mrel">=</span><span
//! class="mspace" style="margin-right:0.2777777777777778em;"></span></span><span class="base"><span
//! class="strut" style="height:0.8888799999999999em;vertical-align:-0.19444em;"></span><span
//! class="mord"><span style="margin-right:0.13889em;" class="mord mathdefault">W</span><span
//! class="mord mathdefault">o</span><span class="mord mathdefault"
//! style="margin-right:0.02778em;">r</span><span class="mord mathdefault"
//! style="margin-right:0.03148em;">k</span><span class="mord mathdefault">G</span><span
//! style="margin-right:0.02778em;" class="mord mathdefault">r</span><span class="mord
//! mathdefault">o</span><span class="mord mathdefault">u</span><span class="mord
//! mathdefault">p</span><span class="mord mathdefault"
//! style="margin-right:0.05764em;">S</span><span class="mord mathdefault">i</span><span
//! style="margin-right:0.04398em;" class="mord mathdefault">z</span><span class="mord
//! mathdefault">e</span><span class="mord">.</span><span class="mord mathdefault">x</span><span
//! style="margin-right:0.2222222222222222em;" class="mspace"></span><span
//! class="mbin">×</span><span style="margin-right:0.2222222222222222em;"
//! class="mspace"></span><span class="mord mathdefault"
//! style="margin-right:0.13889em;">W</span><span class="mord mathdefault">o</span><span class="mord
//! mathdefault" style="margin-right:0.02778em;">r</span><span style="margin-right:0.03148em;"
//! class="mord mathdefault">k</span><span class="mord mathdefault">G</span><span class="mord
//! mathdefault" style="margin-right:0.02778em;">r</span><span class="mord
//! mathdefault">o</span><span class="mord mathdefault">u</span><span class="mord
//! mathdefault">p</span><span style="margin-right:0.05764em;" class="mord
//! mathdefault">S</span><span class="mord mathdefault">i</span><span
//! style="margin-right:0.04398em;" class="mord mathdefault">z</span><span class="mord
//! mathdefault">e</span><span class="mord">.</span><span style="margin-right:0.03588em;"
//! class="mord mathdefault">y</span><span class="mspace"
//! style="margin-right:0.2222222222222222em;"></span><span class="mbin">×</span><span
//! class="mspace" style="margin-right:0.2222222222222222em;"></span><span
//! style="margin-right:0.13889em;" class="mord mathdefault">W</span><span class="mord
//! mathdefault">o</span><span class="mord mathdefault"
//! style="margin-right:0.02778em;">r</span><span class="mord mathdefault"
//! style="margin-right:0.03148em;">k</span><span class="mord mathdefault"
//! style="margin-right:0.03588em;">g</span><span style="margin-right:0.02778em;" class="mord
//! mathdefault">r</span><span class="mord mathdefault">o</span><span class="mord
//! mathdefault">u</span><span class="mord mathdefault">p</span><span
//! style="margin-right:0.05764em;" class="mord mathdefault">S</span><span class="mord
//! mathdefault">i</span><span style="margin-right:0.04398em;" class="mord
//! mathdefault">z</span><span class="mord mathdefault">e</span><span class="mord">.</span><span
//! style="margin-right:0.04398em;" class="mord mathdefault">z</span><span
//! style="margin-right:0.2777777777777778em;" class="mspace"></span><span
//! class="mrel">≤</span><span class="mspace"
//! style="margin-right:0.2777777777777778em;"></span><span style="margin-right:0.05764em;"
//! class="mord mathdefault">S</span><span class="mord mathdefault">u</span><span class="mord
//! mathdefault">b</span><span class="mord mathdefault"
//! style="margin-right:0.03588em;">g</span><span class="mord mathdefault"
//! style="margin-right:0.02778em;">r</span><span class="mord mathdefault">o</span><span class="mord
//! mathdefault">u</span><span class="mord mathdefault">p</span><span class="mord mathdefault"
//! style="margin-right:0.05764em;">S</span><span class="mord mathdefault">i</span><span class="mord
//! mathdefault" style="margin-right:0.04398em;">z</span><span class="mord
//! mathdefault">e</span><span class="mspace"
//! style="margin-right:0.2222222222222222em;"></span><span class="mbin">×</span><span
//! class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord
//! mathdefault">m</span><span class="mord mathdefault">a</span><span class="mord
//! mathdefault">x</span><span class="mord mathdefault"
//! style="margin-right:0.07153em;">C</span><span class="mord mathdefault">o</span><span class="mord
//! mathdefault">m</span><span class="mord mathdefault">p</span><span class="mord
//! mathdefault">u</span><span class="mord mathdefault">t</span><span class="mord
//! mathdefault">e</span><span class="mord mathdefault"
//! style="margin-right:0.13889em;">W</span><span class="mord mathdefault">o</span><span class="mord
//! mathdefault" style="margin-right:0.02778em;">r</span><span class="mord mathdefault"
//! style="margin-right:0.03148em;">k</span><span style="margin-right:0.03588em;" class="mord
//! mathdefault">g</span><span style="margin-right:0.02778em;" class="mord
//! mathdefault">r</span><span class="mord mathdefault">o</span><span class="mord
//! mathdefault">u</span><span class="mord mathdefault">p</span><span class="mord mathdefault"
//! style="margin-right:0.05764em;">S</span><span class="mord mathdefault">u</span><span class="mord
//! mathdefault">b</span><span style="margin-right:0.03588em;" class="mord
//! mathdefault">g</span><span style="margin-right:0.02778em;" class="mord
//! mathdefault">r</span><span class="mord mathdefault">o</span><span class="mord
//! mathdefault">u</span><span class="mord mathdefault">p</span><span class="mord
//! mathdefault">s</span></span></span></span></span>Developers can also specify a new pipeline
//! shader stage create flag that
//!requires the implementation to have fully populated subgroups within local
//!workgroups.
//!This requires the workgroup size in the X dimension to be a multiple of the
//!subgroup size.
//!# Revision
//!2
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.1
//!# Contacts
//! - Neil Henning [sheredom](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_subgroup_size_control]
//!   @sheredom%0A<<Here describe the issue or question you have about the
//!   VK_EXT_subgroup_size_control extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceSubgroupSizeControlFeaturesEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceSubgroupSizeControlPropertiesEXT`]
//! - Extending [`PipelineShaderStageCreateInfo`]:  -
//!   [`PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT`]
//!# New constants
//! - [`EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME`]
//! - [`EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION`]
//! - Extending [`PipelineShaderStageCreateFlagBits`]:  -
//!   `VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT`  -
//!   `VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2019-03-05 (Neil Henning)  - Initial draft
//! - Revision 2, 2019-07-26 (Jason Ekstrand)  - Add the missing
//!   [`PhysicalDeviceSubgroupSizeControlFeaturesEXT`] for querying subgroup size control features.
//!# Other info
//! * 2019-03-05
//! * - Promoted to Vulkan 1.3 Core
//! * - Jeff Bolz, NVIDIA  - Jason Ekstrand, Intel  - Sławek Grajewski, Intel  - Jesse Hall, Google
//!   - Neil Henning, AMD  - Daniel Koch, NVIDIA  - Jeff Leger, Qualcomm  - Graeme Leese, Broadcom
//!   - Allan MacKinnon, Google  - Mariusz Merecki, Intel  - Graham Wihlidal, Electronic Arts
//!# Related
//! - [`PhysicalDeviceSubgroupSizeControlFeaturesEXT`]
//! - [`PhysicalDeviceSubgroupSizeControlPropertiesEXT`]
//! - [`PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT`]
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
#[doc(alias = "VK_EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION")]
pub const EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME")]
pub const EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_subgroup_size_control");
