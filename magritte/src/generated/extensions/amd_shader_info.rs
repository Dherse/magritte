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
//! - [`get_shader_info_amd`]
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
//! - [`get_shader_info_amd`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{Device, Pipeline, ShaderStageFlagBits, ShaderStageFlags, VulkanResultCodes},
    AsRaw, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::{c_void, CStr};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_INFO_SPEC_VERSION")]
pub const AMD_SHADER_INFO_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_INFO_EXTENSION_NAME")]
pub const AMD_SHADER_INFO_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_shader_info");
///[vkGetShaderInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html) - Get information about a shader in a pipeline
///# C Specifications
///Information about a particular shader that has been compiled as part of a
///pipeline object can be extracted by calling:
///```c
///// Provided by VK_AMD_shader_info
///VkResult vkGetShaderInfoAMD(
///    VkDevice                                    device,
///    VkPipeline                                  pipeline,
///    VkShaderStageFlagBits                       shaderStage,
///    VkShaderInfoTypeAMD                         infoType,
///    size_t*                                     pInfoSize,
///    void*                                       pInfo);
///```
///# Parameters
/// - [`device`] is the device that created [`pipeline`].
/// - [`pipeline`] is the target of the query.
/// - [`shader_stage`] is a [`ShaderStageFlagBits`] specifying the particular shader within the
///   pipeline about which information is being queried.
/// - [`info_type`] describes what kind of information is being queried.
/// - [`p_info_size`] is a pointer to a value related to the amount of data the query returns, as
///   described below.
/// - [`p_info`] is either `NULL` or a pointer to a buffer.
///# Description
///If [`p_info`] is `NULL`, then the maximum size of the information that  **can**
///be retrieved about the shader, in bytes, is returned in [`p_info_size`].
///Otherwise, [`p_info_size`] **must**  point to a variable set by the user to the
///size of the buffer, in bytes, pointed to by [`p_info`], and on return the
///variable is overwritten with the amount of data actually written to
///[`p_info`].
///If [`p_info_size`] is less than the maximum size that  **can**  be retrieved by
///the pipeline cache, then at most [`p_info_size`] bytes will be written to
///[`p_info`], and `VK_INCOMPLETE` will be returned, instead of
///`VK_SUCCESS`, to indicate that not all required of the pipeline cache
///was returned.Not all information is available for every shader and implementations may
///not support all kinds of information for any shader.
///When a certain type of information is unavailable, the function returns
///`VK_ERROR_FEATURE_NOT_PRESENT`.If information is successfully and fully queried, the function
/// will return
///`VK_SUCCESS`.For [`info_type`]`VK_SHADER_INFO_TYPE_STATISTICS_AMD`, a
///[`ShaderStatisticsInfoAMD`] structure will be written to the buffer
///pointed to by [`p_info`].
///This structure will be populated with statistics regarding the physical
///device resources used by that shader along with other miscellaneous
///information and is described in further detail below.For
/// [`info_type`]`VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD`, [`p_info`] is
///a pointer to a UTF-8 null-terminated string containing human-readable
///disassembly.
///The exact formatting and contents of the disassembly string are
///vendor-specific.The formatting and contents of all other types of information, including
///[`info_type`]`VK_SHADER_INFO_TYPE_BINARY_AMD`, are left to the vendor
///and are not further specified by this extension.
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
/// - [`shader_stage`] **must**  be a valid [`ShaderStageFlagBits`] value
/// - [`info_type`] **must**  be a valid [`ShaderInfoTypeAMD`] value
/// - [`p_info_size`] **must**  be a valid pointer to a `size_t` value
/// - If the value referenced by [`p_info_size`] is not `0`, and [`p_info`] is not `NULL`,
///   [`p_info`] **must**  be a valid pointer to an array of [`p_info_size`] bytes
/// - [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_FEATURE_NOT_PRESENT`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
///# Related
/// - [`VK_AMD_shader_info`]
/// - [`Device`]
/// - [`Pipeline`]
/// - [`ShaderInfoTypeAMD`]
/// - [`ShaderStageFlagBits`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetShaderInfoAMD")]
pub type FNGetShaderInfoAmd = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlagBits,
        info_type: ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: *mut c_void,
    ) -> VulkanResultCodes,
>;
///[VkShaderInfoTypeAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderInfoTypeAMD.html) - Enum specifying which type of shader information to query
///# C Specifications
///Possible values of [`get_shader_info_amd`]`::infoType`, specifying the
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
/// - [`STATISTICS`] specifies that device resources used by a shader will be queried.
/// - [`BINARY`] specifies that implementation-specific information will be queried.
/// - [`DISASSEMBLY`] specifies that human-readable dissassembly of a shader.
///# Related
/// - [`VK_AMD_shader_info`]
/// - [`get_shader_info_amd`]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ShaderInfoTypeAMD(i32);
impl const Default for ShaderInfoTypeAMD {
    fn default() -> Self {
        Self(0)
    }
}
impl ShaderInfoTypeAMD {
    ///[`STATISTICS`] specifies that device resources
    ///used by a shader will be queried.
    pub const STATISTICS: Self = Self(0);
    ///[`BINARY`] specifies that
    ///implementation-specific information will be queried.
    pub const BINARY: Self = Self(1);
    ///[`DISASSEMBLY`] specifies that human-readable
    ///dissassembly of a shader.
    pub const DISASSEMBLY: Self = Self(2);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
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
        &mut self.num_used_vgprs
    }
    ///Gets a mutable reference to the value of [`Self::num_used_sgprs`]
    pub fn num_used_sgprs_mut(&mut self) -> &mut u32 {
        &mut self.num_used_sgprs
    }
    ///Gets a mutable reference to the value of [`Self::lds_size_per_local_work_group`]
    pub fn lds_size_per_local_work_group_mut(&mut self) -> &mut u32 {
        &mut self.lds_size_per_local_work_group
    }
    ///Gets a mutable reference to the value of [`Self::lds_usage_size_in_bytes`]
    pub fn lds_usage_size_in_bytes_mut(&mut self) -> &mut usize {
        &mut self.lds_usage_size_in_bytes
    }
    ///Gets a mutable reference to the value of [`Self::scratch_mem_usage_in_bytes`]
    pub fn scratch_mem_usage_in_bytes_mut(&mut self) -> &mut usize {
        &mut self.scratch_mem_usage_in_bytes
    }
    ///Sets the value of [`Self::num_used_vgprs`]
    pub fn set_num_used_vgprs(mut self, value: u32) -> Self {
        self.num_used_vgprs = value;
        self
    }
    ///Sets the value of [`Self::num_used_sgprs`]
    pub fn set_num_used_sgprs(mut self, value: u32) -> Self {
        self.num_used_sgprs = value;
        self
    }
    ///Sets the value of [`Self::lds_size_per_local_work_group`]
    pub fn set_lds_size_per_local_work_group(mut self, value: u32) -> Self {
        self.lds_size_per_local_work_group = value;
        self
    }
    ///Sets the value of [`Self::lds_usage_size_in_bytes`]
    pub fn set_lds_usage_size_in_bytes(mut self, value: usize) -> Self {
        self.lds_usage_size_in_bytes = value;
        self
    }
    ///Sets the value of [`Self::scratch_mem_usage_in_bytes`]
    pub fn set_scratch_mem_usage_in_bytes(mut self, value: usize) -> Self {
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
///[`get_shader_info_amd`], the same output information  **may**  be returned for
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
    pub compute_work_group_size: [u32; 3 as usize],
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
            compute_work_group_size: [0; 3 as usize],
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
    pub fn compute_work_group_size(&self) -> &[u32; 3 as usize] {
        &self.compute_work_group_size
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
        &mut self.num_physical_vgprs
    }
    ///Gets a mutable reference to the value of [`Self::num_physical_sgprs`]
    pub fn num_physical_sgprs_mut(&mut self) -> &mut u32 {
        &mut self.num_physical_sgprs
    }
    ///Gets a mutable reference to the value of [`Self::num_available_vgprs`]
    pub fn num_available_vgprs_mut(&mut self) -> &mut u32 {
        &mut self.num_available_vgprs
    }
    ///Gets a mutable reference to the value of [`Self::num_available_sgprs`]
    pub fn num_available_sgprs_mut(&mut self) -> &mut u32 {
        &mut self.num_available_sgprs
    }
    ///Gets a mutable reference to the value of [`Self::compute_work_group_size`]
    pub fn compute_work_group_size_mut(&mut self) -> &mut [u32; 3 as usize] {
        &mut self.compute_work_group_size
    }
    ///Sets the value of [`Self::shader_stage_mask`]
    pub fn set_shader_stage_mask(mut self, value: crate::vulkan1_0::ShaderStageFlags) -> Self {
        self.shader_stage_mask = value;
        self
    }
    ///Sets the value of [`Self::resource_usage`]
    pub fn set_resource_usage(mut self, value: crate::extensions::amd_shader_info::ShaderResourceUsageAMD) -> Self {
        self.resource_usage = value;
        self
    }
    ///Sets the value of [`Self::num_physical_vgprs`]
    pub fn set_num_physical_vgprs(mut self, value: u32) -> Self {
        self.num_physical_vgprs = value;
        self
    }
    ///Sets the value of [`Self::num_physical_sgprs`]
    pub fn set_num_physical_sgprs(mut self, value: u32) -> Self {
        self.num_physical_sgprs = value;
        self
    }
    ///Sets the value of [`Self::num_available_vgprs`]
    pub fn set_num_available_vgprs(mut self, value: u32) -> Self {
        self.num_available_vgprs = value;
        self
    }
    ///Sets the value of [`Self::num_available_sgprs`]
    pub fn set_num_available_sgprs(mut self, value: u32) -> Self {
        self.num_available_sgprs = value;
        self
    }
    ///Sets the value of [`Self::compute_work_group_size`]
    pub fn set_compute_work_group_size(mut self, value: [u32; 3 as usize]) -> Self {
        self.compute_work_group_size = value;
        self
    }
}
impl Device {
    ///[vkGetShaderInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html) - Get information about a shader in a pipeline
    ///# C Specifications
    ///Information about a particular shader that has been compiled as part of a
    ///pipeline object can be extracted by calling:
    ///```c
    ///// Provided by VK_AMD_shader_info
    ///VkResult vkGetShaderInfoAMD(
    ///    VkDevice                                    device,
    ///    VkPipeline                                  pipeline,
    ///    VkShaderStageFlagBits                       shaderStage,
    ///    VkShaderInfoTypeAMD                         infoType,
    ///    size_t*                                     pInfoSize,
    ///    void*                                       pInfo);
    ///```
    ///# Parameters
    /// - [`device`] is the device that created [`pipeline`].
    /// - [`pipeline`] is the target of the query.
    /// - [`shader_stage`] is a [`ShaderStageFlagBits`] specifying the particular shader within the
    ///   pipeline about which information is being queried.
    /// - [`info_type`] describes what kind of information is being queried.
    /// - [`p_info_size`] is a pointer to a value related to the amount of data the query returns,
    ///   as described below.
    /// - [`p_info`] is either `NULL` or a pointer to a buffer.
    ///# Description
    ///If [`p_info`] is `NULL`, then the maximum size of the information that  **can**
    ///be retrieved about the shader, in bytes, is returned in [`p_info_size`].
    ///Otherwise, [`p_info_size`] **must**  point to a variable set by the user to the
    ///size of the buffer, in bytes, pointed to by [`p_info`], and on return the
    ///variable is overwritten with the amount of data actually written to
    ///[`p_info`].
    ///If [`p_info_size`] is less than the maximum size that  **can**  be retrieved by
    ///the pipeline cache, then at most [`p_info_size`] bytes will be written to
    ///[`p_info`], and `VK_INCOMPLETE` will be returned, instead of
    ///`VK_SUCCESS`, to indicate that not all required of the pipeline cache
    ///was returned.Not all information is available for every shader and implementations may
    ///not support all kinds of information for any shader.
    ///When a certain type of information is unavailable, the function returns
    ///`VK_ERROR_FEATURE_NOT_PRESENT`.If information is successfully and fully queried, the
    /// function will return
    ///`VK_SUCCESS`.For [`info_type`]`VK_SHADER_INFO_TYPE_STATISTICS_AMD`, a
    ///[`ShaderStatisticsInfoAMD`] structure will be written to the buffer
    ///pointed to by [`p_info`].
    ///This structure will be populated with statistics regarding the physical
    ///device resources used by that shader along with other miscellaneous
    ///information and is described in further detail below.For
    /// [`info_type`]`VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD`, [`p_info`] is
    ///a pointer to a UTF-8 null-terminated string containing human-readable
    ///disassembly.
    ///The exact formatting and contents of the disassembly string are
    ///vendor-specific.The formatting and contents of all other types of information, including
    ///[`info_type`]`VK_SHADER_INFO_TYPE_BINARY_AMD`, are left to the vendor
    ///and are not further specified by this extension.
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
    /// - [`shader_stage`] **must**  be a valid [`ShaderStageFlagBits`] value
    /// - [`info_type`] **must**  be a valid [`ShaderInfoTypeAMD`] value
    /// - [`p_info_size`] **must**  be a valid pointer to a `size_t` value
    /// - If the value referenced by [`p_info_size`] is not `0`, and [`p_info`] is not `NULL`,
    ///   [`p_info`] **must**  be a valid pointer to an array of [`p_info_size`] bytes
    /// - [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_FEATURE_NOT_PRESENT`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///# Related
    /// - [`VK_AMD_shader_info`]
    /// - [`Device`]
    /// - [`Pipeline`]
    /// - [`ShaderInfoTypeAMD`]
    /// - [`ShaderStageFlagBits`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetShaderInfoAMD")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_shader_info_amd(
        self: &Unique<Device>,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlagBits,
        info_type: ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: Option<*mut c_void>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .amd_shader_info()
            .and_then(|vtable| vtable.get_shader_info_amd())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .amd_shader_info()
            .and_then(|vtable| vtable.get_shader_info_amd())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            pipeline,
            shader_stage,
            info_type,
            p_info_size,
            p_info.unwrap_or_else(std::ptr::null_mut),
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_AMD_shader_info`
pub struct DeviceAmdShaderInfoVTable {
    ///See [`FNGetShaderInfoAmd`] for more information.
    pub get_shader_info_amd: FNGetShaderInfoAmd,
}
impl DeviceAmdShaderInfoVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            get_shader_info_amd: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetShaderInfoAMD").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_shader_info_amd`]. See [`FNGetShaderInfoAmd`] for more information.
    pub fn get_shader_info_amd(&self) -> FNGetShaderInfoAmd {
        self.get_shader_info_amd
    }
}
