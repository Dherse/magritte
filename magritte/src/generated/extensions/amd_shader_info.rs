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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        ShaderInfoTypeStatisticsAmd
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ShaderResourceUsageAMD {
    ///[`num_used_vgprs`] is the number of vector instruction general-purpose
    ///registers used by this shader.
    num_used_vgprs: u32,
    ///[`num_used_sgprs`] is the number of scalar instruction general-purpose
    ///registers used by this shader.
    num_used_sgprs: u32,
    ///[`lds_size_per_local_work_group`] is the maximum local data store size per
    ///work group in bytes.
    lds_size_per_local_work_group: u32,
    ///[`lds_usage_size_in_bytes`] is the LDS usage size in bytes per work group
    ///by this shader.
    lds_usage_size_in_bytes: usize,
    ///[`scratch_mem_usage_in_bytes`] is the scratch memory usage in bytes by
    ///this shader.
    scratch_mem_usage_in_bytes: usize,
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
///[`GetShaderInfoAMD`], the same output information **may** be returned for
///all such shader stage queries.The number of available VGPRs and SGPRs ([`num_available_vgprs`]
/// and
///[`num_available_sgprs`] respectively) are the shader-addressable subset of
///physical registers that is given as a limit to the compiler for register
///assignment.
///These values **may** further be limited by implementations due to performance
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ShaderStatisticsInfoAMD {
    ///[`shader_stage_mask`] are the combination of logical shader stages
    ///contained within this shader.
    shader_stage_mask: ShaderStageFlags,
    ///[`resource_usage`] is a [`ShaderResourceUsageAMD`] structure
    ///describing internal physical device resources used by this shader.
    resource_usage: ShaderResourceUsageAMD,
    ///[`num_physical_vgprs`] is the maximum number of vector instruction
    ///general-purpose registers (VGPRs) available to the physical device.
    num_physical_vgprs: u32,
    ///[`num_physical_sgprs`] is the maximum number of scalar instruction
    ///general-purpose registers (SGPRs) available to the physical device.
    num_physical_sgprs: u32,
    ///[`num_available_vgprs`] is the maximum limit of VGPRs made available to
    ///the shader compiler.
    num_available_vgprs: u32,
    ///[`num_available_sgprs`] is the maximum limit of SGPRs made available to
    ///the shader compiler.
    num_available_sgprs: u32,
    ///[`compute_work_group_size`] is the local workgroup size of this shader in
    ///{ X, Y, Z } dimensions.
    compute_work_group_size: [u32; 3],
}
