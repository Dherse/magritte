//![VK_AMD_shader_info](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_info.html) - device extension
//!# Description
//!This extension adds a way to query certain information about a compiled
//!shader which is part of a pipeline.
//!This information may include shader disassembly, shader binary and various
//!statistics about a shader’s resource usage.While this extension provides a mechanism for
//! extracting this information,
//!the details regarding the contents or format of this information are not
//!specified by this extension and may be provided by the vendor externally.Furthermore, all
//! information types are optionally supported, and users
//!should not assume every implementation supports querying every type of
//!information.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jaakko Konttinen [jaakkoamd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_shader_info]
//!   @jaakkoamd%0A<<Here describe the issue or question you have about the VK_AMD_shader_info
//!   extension>>)
//!# New functions & commands
//! - [`GetShaderInfoAMD`]
//!# New structures
//! - [`ShaderResourceUsageAMD`]
//! - [`ShaderStatisticsInfoAMD`]
//!# New enums
//! - [`ShaderInfoTypeAMD`]
//!# New constants
//! - [`AMD_SHADER_INFO_EXTENSION_NAME`]
//! - [`AMD_SHADER_INFO_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2017-10-09 (Jaakko Konttinen)  - Initial revision
//!# Other info
//! * 2017-10-09
//! * No known IP claims.
//! * - Jaakko Konttinen, AMD
//!# Related
//! - [`ShaderInfoTypeAMD`]
//! - [`ShaderResourceUsageAMD`]
//! - [`ShaderStatisticsInfoAMD`]
//! - [`GetShaderInfoAMD`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::ShaderStageFlags;
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_INFO_SPEC_VERSION")]
pub const AMD_SHADER_INFO_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_INFO_EXTENSION_NAME")]
pub const AMD_SHADER_INFO_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_shader_info");
///[VkShaderInfoTypeAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderInfoTypeAMD.html) - Enum specifying which type of shader information to query
///# C Specifications
///Possible values of [`GetShaderInfoAMD`]`::infoType`, specifying the
///information being queried from a shader, are:
///```c
///// Provided by VK_AMD_shader_info
///typedef enum VkShaderInfoTypeAMD {
///    VK_SHADER_INFO_TYPE_STATISTICS_AMD = 0,
///    VK_SHADER_INFO_TYPE_BINARY_AMD = 1,
///    VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD = 2,
///} VkShaderInfoTypeAMD;
///```
///# Description
/// - [`ShaderInfoTypeStatisticsAmd`] specifies that device resources used by a shader will be
///   queried.
/// - [`ShaderInfoTypeBinaryAmd`] specifies that implementation-specific information will be
///   queried.
/// - [`ShaderInfoTypeDisassemblyAmd`] specifies that human-readable dissassembly of a shader.
///# Related
/// - [`VK_AMD_shader_info`]
/// - [`GetShaderInfoAMD`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkShaderInfoTypeAMD")]
#[doc(alias = "VkShaderInfoTypeAMD")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum ShaderInfoTypeAMD {
    ///[`ShaderInfoTypeStatisticsAmd`] specifies that device resources
    ///used by a shader will be queried.
    ShaderInfoTypeStatisticsAmd = 0,
    ///[`ShaderInfoTypeBinaryAmd`] specifies that
    ///implementation-specific information will be queried.
    ShaderInfoTypeBinaryAmd = 1,
    ///[`ShaderInfoTypeDisassemblyAmd`] specifies that human-readable
    ///dissassembly of a shader.
    ShaderInfoTypeDisassemblyAmd = 2,
}
impl const Default for ShaderInfoTypeAMD {
    fn default() -> Self {
        Self::ShaderInfoTypeStatisticsAmd
    }
}
impl ShaderInfoTypeAMD {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkShaderResourceUsageAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderResourceUsageAMD.html) - Resource usage information about a particular shader within a pipeline
///# C Specifications
///The [`ShaderResourceUsageAMD`] structure is defined as:
///```c
///// Provided by VK_AMD_shader_info
///typedef struct VkShaderResourceUsageAMD {
///    uint32_t    numUsedVgprs;
///    uint32_t    numUsedSgprs;
///    uint32_t    ldsSizePerLocalWorkGroup;
///    size_t      ldsUsageSizeInBytes;
///    size_t      scratchMemUsageInBytes;
///} VkShaderResourceUsageAMD;
///```
///# Members
/// - [`num_used_vgprs`] is the number of vector instruction general-purpose registers used by this
///   shader.
/// - [`num_used_sgprs`] is the number of scalar instruction general-purpose registers used by this
///   shader.
/// - [`lds_size_per_local_work_group`] is the maximum local data store size per work group in
///   bytes.
/// - [`lds_usage_size_in_bytes`] is the LDS usage size in bytes per work group by this shader.
/// - [`scratch_mem_usage_in_bytes`] is the scratch memory usage in bytes by this shader.
///# Related
/// - [`VK_AMD_shader_info`]
/// - [`ShaderStatisticsInfoAMD`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkShaderResourceUsageAMD")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ShaderResourceUsageAMD {
    ///[`num_used_vgprs`] is the number of vector instruction general-purpose
    ///registers used by this shader.
    pub num_used_vgprs: u32,
    ///[`num_used_sgprs`] is the number of scalar instruction general-purpose
    ///registers used by this shader.
    pub num_used_sgprs: u32,
    ///[`lds_size_per_local_work_group`] is the maximum local data store size per
    ///work group in bytes.
    pub lds_size_per_local_work_group: u32,
    ///[`lds_usage_size_in_bytes`] is the LDS usage size in bytes per work group
    ///by this shader.
    pub lds_usage_size_in_bytes: usize,
    ///[`scratch_mem_usage_in_bytes`] is the scratch memory usage in bytes by
    ///this shader.
    pub scratch_mem_usage_in_bytes: usize,
}
impl Default for ShaderResourceUsageAMD {
    fn default() -> Self {
        Self {
            num_used_vgprs: 0,
            num_used_sgprs: 0,
            lds_size_per_local_work_group: 0,
            lds_usage_size_in_bytes: 0,
            scratch_mem_usage_in_bytes: 0,
        }
    }
}
impl ShaderResourceUsageAMD {
    ///Gets the value of [`Self::num_used_vgprs`]
    pub fn num_used_vgprs(&self) -> u32 {
        self.num_used_vgprs
    }
    ///Gets the value of [`Self::num_used_sgprs`]
    pub fn num_used_sgprs(&self) -> u32 {
        self.num_used_sgprs
    }
    ///Gets the value of [`Self::lds_size_per_local_work_group`]
    pub fn lds_size_per_local_work_group(&self) -> u32 {
        self.lds_size_per_local_work_group
    }
    ///Gets the value of [`Self::lds_usage_size_in_bytes`]
    pub fn lds_usage_size_in_bytes(&self) -> usize {
        self.lds_usage_size_in_bytes
    }
    ///Gets the value of [`Self::scratch_mem_usage_in_bytes`]
    pub fn scratch_mem_usage_in_bytes(&self) -> usize {
        self.scratch_mem_usage_in_bytes
    }
    ///Gets a mutable reference to the value of [`Self::num_used_vgprs`]
    pub fn num_used_vgprs_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::num_used_sgprs`]
    pub fn num_used_sgprs_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::lds_size_per_local_work_group`]
    pub fn lds_size_per_local_work_group_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::lds_usage_size_in_bytes`]
    pub fn lds_usage_size_in_bytes_mut(&mut self) -> &mut usize {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::scratch_mem_usage_in_bytes`]
    pub fn scratch_mem_usage_in_bytes_mut(&mut self) -> &mut usize {
        &mut getter
    }
    ///Sets the raw value of [`Self::num_used_vgprs`]
    pub fn set_num_used_vgprs(&mut self, value: u32) -> &mut Self {
        self.num_used_vgprs = value;
        self
    }
    ///Sets the raw value of [`Self::num_used_sgprs`]
    pub fn set_num_used_sgprs(&mut self, value: u32) -> &mut Self {
        self.num_used_sgprs = value;
        self
    }
    ///Sets the raw value of [`Self::lds_size_per_local_work_group`]
    pub fn set_lds_size_per_local_work_group(&mut self, value: u32) -> &mut Self {
        self.lds_size_per_local_work_group = value;
        self
    }
    ///Sets the raw value of [`Self::lds_usage_size_in_bytes`]
    pub fn set_lds_usage_size_in_bytes(&mut self, value: usize) -> &mut Self {
        self.lds_usage_size_in_bytes = value;
        self
    }
    ///Sets the raw value of [`Self::scratch_mem_usage_in_bytes`]
    pub fn set_scratch_mem_usage_in_bytes(&mut self, value: usize) -> &mut Self {
        self.scratch_mem_usage_in_bytes = value;
        self
    }
}
///[VkShaderStatisticsInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderStatisticsInfoAMD.html) - Statistical information about a particular shader within a pipeline
///# C Specifications
///The [`ShaderStatisticsInfoAMD`] structure is defined as:
///```c
///// Provided by VK_AMD_shader_info
///typedef struct VkShaderStatisticsInfoAMD {
///    VkShaderStageFlags          shaderStageMask;
///    VkShaderResourceUsageAMD    resourceUsage;
///    uint32_t                    numPhysicalVgprs;
///    uint32_t                    numPhysicalSgprs;
///    uint32_t                    numAvailableVgprs;
///    uint32_t                    numAvailableSgprs;
///    uint32_t                    computeWorkGroupSize[3];
///} VkShaderStatisticsInfoAMD;
///```
///# Members
/// - [`shader_stage_mask`] are the combination of logical shader stages contained within this
///   shader.
/// - [`resource_usage`] is a [`ShaderResourceUsageAMD`] structure describing internal physical
///   device resources used by this shader.
/// - [`num_physical_vgprs`] is the maximum number of vector instruction general-purpose registers
///   (VGPRs) available to the physical device.
/// - [`num_physical_sgprs`] is the maximum number of scalar instruction general-purpose registers
///   (SGPRs) available to the physical device.
/// - [`num_available_vgprs`] is the maximum limit of VGPRs made available to the shader compiler.
/// - [`num_available_sgprs`] is the maximum limit of SGPRs made available to the shader compiler.
/// - [`compute_work_group_size`] is the local workgroup size of this shader in { X, Y, Z }
///   dimensions.
///# Description
///Some implementations may merge multiple logical shader stages together in a
///single shader.
///In such cases, [`shader_stage_mask`] will contain a bitmask of all of the
///stages that are active within that shader.
///Consequently, if specifying those stages as input to
///[`GetShaderInfoAMD`], the same output information  **may**  be returned for
///all such shader stage queries.The number of available VGPRs and SGPRs ([`num_available_vgprs`]
/// and
///[`num_available_sgprs`] respectively) are the shader-addressable subset of
///physical registers that is given as a limit to the compiler for register
///assignment.
///These values  **may**  further be limited by implementations due to performance
///optimizations where register pressure is a bottleneck.
///# Related
/// - [`VK_AMD_shader_info`]
/// - [`ShaderResourceUsageAMD`]
/// - [`ShaderStageFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkShaderStatisticsInfoAMD")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ShaderStatisticsInfoAMD {
    ///[`shader_stage_mask`] are the combination of logical shader stages
    ///contained within this shader.
    pub shader_stage_mask: ShaderStageFlags,
    ///[`resource_usage`] is a [`ShaderResourceUsageAMD`] structure
    ///describing internal physical device resources used by this shader.
    pub resource_usage: ShaderResourceUsageAMD,
    ///[`num_physical_vgprs`] is the maximum number of vector instruction
    ///general-purpose registers (VGPRs) available to the physical device.
    pub num_physical_vgprs: u32,
    ///[`num_physical_sgprs`] is the maximum number of scalar instruction
    ///general-purpose registers (SGPRs) available to the physical device.
    pub num_physical_sgprs: u32,
    ///[`num_available_vgprs`] is the maximum limit of VGPRs made available to
    ///the shader compiler.
    pub num_available_vgprs: u32,
    ///[`num_available_sgprs`] is the maximum limit of SGPRs made available to
    ///the shader compiler.
    pub num_available_sgprs: u32,
    ///[`compute_work_group_size`] is the local workgroup size of this shader in
    ///{ X, Y, Z } dimensions.
    pub compute_work_group_size: [u32; 3],
}
impl Default for ShaderStatisticsInfoAMD {
    fn default() -> Self {
        Self {
            shader_stage_mask: Default::default(),
            resource_usage: Default::default(),
            num_physical_vgprs: 0,
            num_physical_sgprs: 0,
            num_available_vgprs: 0,
            num_available_sgprs: 0,
            compute_work_group_size: [0; 3],
        }
    }
}
impl ShaderStatisticsInfoAMD {
    ///Gets the value of [`Self::shader_stage_mask`]
    pub fn shader_stage_mask(&self) -> ShaderStageFlags {
        self.shader_stage_mask
    }
    ///Gets the value of [`Self::resource_usage`]
    pub fn resource_usage(&self) -> ShaderResourceUsageAMD {
        self.resource_usage
    }
    ///Gets the value of [`Self::num_physical_vgprs`]
    pub fn num_physical_vgprs(&self) -> u32 {
        self.num_physical_vgprs
    }
    ///Gets the value of [`Self::num_physical_sgprs`]
    pub fn num_physical_sgprs(&self) -> u32 {
        self.num_physical_sgprs
    }
    ///Gets the value of [`Self::num_available_vgprs`]
    pub fn num_available_vgprs(&self) -> u32 {
        self.num_available_vgprs
    }
    ///Gets the value of [`Self::num_available_sgprs`]
    pub fn num_available_sgprs(&self) -> u32 {
        self.num_available_sgprs
    }
    ///Gets the value of [`Self::compute_work_group_size`]
    pub fn compute_work_group_size(&self) -> &[u32; 3] {
        &getter
    }
    ///Gets a mutable reference to the value of [`Self::shader_stage_mask`]
    pub fn shader_stage_mask_mut(&mut self) -> &mut ShaderStageFlags {
        &mut self.shader_stage_mask
    }
    ///Gets a mutable reference to the value of [`Self::resource_usage`]
    pub fn resource_usage_mut(&mut self) -> &mut ShaderResourceUsageAMD {
        &mut self.resource_usage
    }
    ///Gets a mutable reference to the value of [`Self::num_physical_vgprs`]
    pub fn num_physical_vgprs_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::num_physical_sgprs`]
    pub fn num_physical_sgprs_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::num_available_vgprs`]
    pub fn num_available_vgprs_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::num_available_sgprs`]
    pub fn num_available_sgprs_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::compute_work_group_size`]
    pub fn compute_work_group_size_mut(&mut self) -> &mut [u32; 3] {
        &mut getter
    }
    ///Sets the raw value of [`Self::shader_stage_mask`]
    pub fn set_shader_stage_mask(&mut self, value: crate::vulkan1_0::ShaderStageFlags) -> &mut Self {
        self.shader_stage_mask = value;
        self
    }
    ///Sets the raw value of [`Self::resource_usage`]
    pub fn set_resource_usage(
        &mut self,
        value: crate::extensions::amd_shader_info::ShaderResourceUsageAMD,
    ) -> &mut Self {
        self.resource_usage = value;
        self
    }
    ///Sets the raw value of [`Self::num_physical_vgprs`]
    pub fn set_num_physical_vgprs(&mut self, value: u32) -> &mut Self {
        self.num_physical_vgprs = value;
        self
    }
    ///Sets the raw value of [`Self::num_physical_sgprs`]
    pub fn set_num_physical_sgprs(&mut self, value: u32) -> &mut Self {
        self.num_physical_sgprs = value;
        self
    }
    ///Sets the raw value of [`Self::num_available_vgprs`]
    pub fn set_num_available_vgprs(&mut self, value: u32) -> &mut Self {
        self.num_available_vgprs = value;
        self
    }
    ///Sets the raw value of [`Self::num_available_sgprs`]
    pub fn set_num_available_sgprs(&mut self, value: u32) -> &mut Self {
        self.num_available_sgprs = value;
        self
    }
    ///Sets the raw value of [`Self::compute_work_group_size`]
    pub fn set_compute_work_group_size(&mut self, value: [u32; 3]) -> &mut Self {
        self.compute_work_group_size = value;
        self
    }
}
