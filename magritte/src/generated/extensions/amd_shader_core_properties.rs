use crate::vulkan1_0::{BaseOutStructure, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION")]
pub const AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME")]
pub const AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_shader_core_properties");
///[VkPhysicalDeviceShaderCorePropertiesAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCorePropertiesAMD.html) - Structure describing shader core properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceShaderCorePropertiesAMD`] structure is defined as:
///```c
///// Provided by VK_AMD_shader_core_properties
///typedef struct VkPhysicalDeviceShaderCorePropertiesAMD {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           shaderEngineCount;
///    uint32_t           shaderArraysPerEngineCount;
///    uint32_t           computeUnitsPerShaderArray;
///    uint32_t           simdPerComputeUnit;
///    uint32_t           wavefrontsPerSimd;
///    uint32_t           wavefrontSize;
///    uint32_t           sgprsPerSimd;
///    uint32_t           minSgprAllocation;
///    uint32_t           maxSgprAllocation;
///    uint32_t           sgprAllocationGranularity;
///    uint32_t           vgprsPerSimd;
///    uint32_t           minVgprAllocation;
///    uint32_t           maxVgprAllocation;
///    uint32_t           vgprAllocationGranularity;
///} VkPhysicalDeviceShaderCorePropertiesAMD;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shader_engine_count`] is an unsigned integer value indicating the number of shader engines
///   found inside the shader core of the physical device.
/// - [`shader_arrays_per_engine_count`] is an unsigned integer value indicating the number of
///   shader arrays inside a shader engine. Each shader array has its own scan converter, set of
///   compute units, and a render back end (color and depth attachments). Shader arrays within a
///   shader engine share shader processor input (wave launcher) and shader export (export buffer)
///   units. Currently, a shader engine can have one or two shader arrays.
/// - [`compute_units_per_shader_array`] is an unsigned integer value indicating the physical number
///   of compute units within a shader array. The active number of compute units in a shader array
///   **may** be lower. A compute unit houses a set of SIMDs along with a sequencer module and a
///   local data store.
/// - [`simd_per_compute_unit`] is an unsigned integer value indicating the number of SIMDs inside a
///   compute unit. Each SIMD processes a single instruction at a time.
/// - [`wavefront_size`] is an unsigned integer value indicating the maximum size of a subgroup.
/// - [`sgprs_per_simd`] is an unsigned integer value indicating the number of physical Scalar
///   General Purpose Registers (SGPRs) per SIMD.
/// - [`min_sgpr_allocation`] is an unsigned integer value indicating the minimum number of SGPRs
///   allocated for a wave.
/// - [`max_sgpr_allocation`] is an unsigned integer value indicating the maximum number of SGPRs
///   allocated for a wave.
/// - [`sgpr_allocation_granularity`] is an unsigned integer value indicating the granularity of
///   SGPR allocation for a wave.
/// - [`vgprs_per_simd`] is an unsigned integer value indicating the number of physical Vector
///   General Purpose Registers (VGPRs) per SIMD.
/// - [`min_vgpr_allocation`] is an unsigned integer value indicating the minimum number of VGPRs
///   allocated for a wave.
/// - [`max_vgpr_allocation`] is an unsigned integer value indicating the maximum number of VGPRs
///   allocated for a wave.
/// - [`vgpr_allocation_granularity`] is an unsigned integer value indicating the granularity of
///   VGPR allocation for a wave.
///# Description
///If the [`PhysicalDeviceShaderCorePropertiesAMD`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD`
///# Related
/// - [`VK_AMD_shader_core_properties`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceShaderCorePropertiesAMD<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_engine_count`] is an unsigned
    ///integer value indicating the number of shader engines found inside the
    ///shader core of the physical device.
    shader_engine_count: u32,
    ///[`shader_arrays_per_engine_count`]
    ///is an unsigned integer value indicating the number of shader arrays
    ///inside a shader engine.
    ///Each shader array has its own scan converter, set of compute units, and
    ///a render back end (color and depth attachments).
    ///Shader arrays within a shader engine share shader processor input (wave
    ///launcher) and shader export (export buffer) units.
    ///Currently, a shader engine can have one or two shader arrays.
    shader_arrays_per_engine_count: u32,
    ///[`compute_units_per_shader_array`]
    ///is an unsigned integer value indicating the physical number of compute
    ///units within a shader array.
    ///The active number of compute units in a shader array **may** be lower.
    ///A compute unit houses a set of SIMDs along with a sequencer module and a
    ///local data store.
    compute_units_per_shader_array: u32,
    ///[`simd_per_compute_unit`] is an unsigned
    ///integer value indicating the number of SIMDs inside a compute unit.
    ///Each SIMD processes a single instruction at a time.
    simd_per_compute_unit: u32,
    ///No documentation found
    wavefronts_per_simd: u32,
    ///[`wavefront_size`] is an unsigned integer
    ///value indicating the maximum size of a subgroup.
    wavefront_size: u32,
    ///[`sgprs_per_simd`] is an unsigned integer value
    ///indicating the number of physical Scalar General Purpose Registers
    ///(SGPRs) per SIMD.
    sgprs_per_simd: u32,
    ///[`min_sgpr_allocation`] is an unsigned
    ///integer value indicating the minimum number of SGPRs allocated for a
    ///wave.
    min_sgpr_allocation: u32,
    ///[`max_sgpr_allocation`] is an unsigned
    ///integer value indicating the maximum number of SGPRs allocated for a
    ///wave.
    max_sgpr_allocation: u32,
    ///[`sgpr_allocation_granularity`] is
    ///an unsigned integer value indicating the granularity of SGPR allocation
    ///for a wave.
    sgpr_allocation_granularity: u32,
    ///[`vgprs_per_simd`] is an unsigned integer value
    ///indicating the number of physical Vector General Purpose Registers
    ///(VGPRs) per SIMD.
    vgprs_per_simd: u32,
    ///[`min_vgpr_allocation`] is an unsigned
    ///integer value indicating the minimum number of VGPRs allocated for a
    ///wave.
    min_vgpr_allocation: u32,
    ///[`max_vgpr_allocation`] is an unsigned
    ///integer value indicating the maximum number of VGPRs allocated for a
    ///wave.
    max_vgpr_allocation: u32,
    ///[`vgpr_allocation_granularity`] is
    ///an unsigned integer value indicating the granularity of VGPR allocation
    ///for a wave.
    vgpr_allocation_granularity: u32,
}
