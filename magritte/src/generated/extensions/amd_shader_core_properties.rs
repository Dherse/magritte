//![VK_AMD_shader_core_properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_core_properties.html) - device extension
//!# Description
//!This extension exposes shader core properties for a target physical device
//!through the `[`khr_get_physical_device_properties2`]` extension.
//!Please refer to the example below for proper usage.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Martin Dinkov [mdinkov](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_shader_core_properties]
//!   @mdinkov%0A<<Here describe the issue or question you have about the
//!   VK_AMD_shader_core_properties extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceShaderCorePropertiesAMD`]
//!# New constants
//! - [`AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME`]
//! - [`AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD`
//!# Version history
//! - Revision 2, 2019-06-25 (Matthaeus G. Chajdas)  - Clarified the meaning of a few fields.
//! - Revision 1, 2018-02-15 (Martin Dinkov)  - Initial draft.
//!# Other information
//! * 2019-06-25
//! * No known IP claims.
//! * - Martin Dinkov, AMD  - Matthaeus G. Chajdas, AMD
//!# Related
//! - [`PhysicalDeviceShaderCorePropertiesAMD`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
///   **may**  be lower. A compute unit houses a set of SIMDs along with a sequencer module and a
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
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD`
///# Related
/// - [`amd_shader_core_properties`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceShaderCorePropertiesAMD")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceShaderCorePropertiesAMD<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_engine_count`] is an unsigned
    ///integer value indicating the number of shader engines found inside the
    ///shader core of the physical device.
    pub shader_engine_count: u32,
    ///[`shader_arrays_per_engine_count`]
    ///is an unsigned integer value indicating the number of shader arrays
    ///inside a shader engine.
    ///Each shader array has its own scan converter, set of compute units, and
    ///a render back end (color and depth attachments).
    ///Shader arrays within a shader engine share shader processor input (wave
    ///launcher) and shader export (export buffer) units.
    ///Currently, a shader engine can have one or two shader arrays.
    pub shader_arrays_per_engine_count: u32,
    ///[`compute_units_per_shader_array`]
    ///is an unsigned integer value indicating the physical number of compute
    ///units within a shader array.
    ///The active number of compute units in a shader array  **may**  be lower.
    ///A compute unit houses a set of SIMDs along with a sequencer module and a
    ///local data store.
    pub compute_units_per_shader_array: u32,
    ///[`simd_per_compute_unit`] is an unsigned
    ///integer value indicating the number of SIMDs inside a compute unit.
    ///Each SIMD processes a single instruction at a time.
    pub simd_per_compute_unit: u32,
    ///No documentation found
    pub wavefronts_per_simd: u32,
    ///[`wavefront_size`] is an unsigned integer
    ///value indicating the maximum size of a subgroup.
    pub wavefront_size: u32,
    ///[`sgprs_per_simd`] is an unsigned integer value
    ///indicating the number of physical Scalar General Purpose Registers
    ///(SGPRs) per SIMD.
    pub sgprs_per_simd: u32,
    ///[`min_sgpr_allocation`] is an unsigned
    ///integer value indicating the minimum number of SGPRs allocated for a
    ///wave.
    pub min_sgpr_allocation: u32,
    ///[`max_sgpr_allocation`] is an unsigned
    ///integer value indicating the maximum number of SGPRs allocated for a
    ///wave.
    pub max_sgpr_allocation: u32,
    ///[`sgpr_allocation_granularity`] is
    ///an unsigned integer value indicating the granularity of SGPR allocation
    ///for a wave.
    pub sgpr_allocation_granularity: u32,
    ///[`vgprs_per_simd`] is an unsigned integer value
    ///indicating the number of physical Vector General Purpose Registers
    ///(VGPRs) per SIMD.
    pub vgprs_per_simd: u32,
    ///[`min_vgpr_allocation`] is an unsigned
    ///integer value indicating the minimum number of VGPRs allocated for a
    ///wave.
    pub min_vgpr_allocation: u32,
    ///[`max_vgpr_allocation`] is an unsigned
    ///integer value indicating the maximum number of VGPRs allocated for a
    ///wave.
    pub max_vgpr_allocation: u32,
    ///[`vgpr_allocation_granularity`] is
    ///an unsigned integer value indicating the granularity of VGPR allocation
    ///for a wave.
    pub vgpr_allocation_granularity: u32,
}
impl<'lt> Default for PhysicalDeviceShaderCorePropertiesAMD<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD,
            p_next: std::ptr::null_mut(),
            shader_engine_count: 0,
            shader_arrays_per_engine_count: 0,
            compute_units_per_shader_array: 0,
            simd_per_compute_unit: 0,
            wavefronts_per_simd: 0,
            wavefront_size: 0,
            sgprs_per_simd: 0,
            min_sgpr_allocation: 0,
            max_sgpr_allocation: 0,
            sgpr_allocation_granularity: 0,
            vgprs_per_simd: 0,
            min_vgpr_allocation: 0,
            max_vgpr_allocation: 0,
            vgpr_allocation_granularity: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderCorePropertiesAMD<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::shader_engine_count`]
    pub fn shader_engine_count(&self) -> u32 {
        self.shader_engine_count
    }
    ///Gets the value of [`Self::shader_arrays_per_engine_count`]
    pub fn shader_arrays_per_engine_count(&self) -> u32 {
        self.shader_arrays_per_engine_count
    }
    ///Gets the value of [`Self::compute_units_per_shader_array`]
    pub fn compute_units_per_shader_array(&self) -> u32 {
        self.compute_units_per_shader_array
    }
    ///Gets the value of [`Self::simd_per_compute_unit`]
    pub fn simd_per_compute_unit(&self) -> u32 {
        self.simd_per_compute_unit
    }
    ///Gets the value of [`Self::wavefronts_per_simd`]
    pub fn wavefronts_per_simd(&self) -> u32 {
        self.wavefronts_per_simd
    }
    ///Gets the value of [`Self::wavefront_size`]
    pub fn wavefront_size(&self) -> u32 {
        self.wavefront_size
    }
    ///Gets the value of [`Self::sgprs_per_simd`]
    pub fn sgprs_per_simd(&self) -> u32 {
        self.sgprs_per_simd
    }
    ///Gets the value of [`Self::min_sgpr_allocation`]
    pub fn min_sgpr_allocation(&self) -> u32 {
        self.min_sgpr_allocation
    }
    ///Gets the value of [`Self::max_sgpr_allocation`]
    pub fn max_sgpr_allocation(&self) -> u32 {
        self.max_sgpr_allocation
    }
    ///Gets the value of [`Self::sgpr_allocation_granularity`]
    pub fn sgpr_allocation_granularity(&self) -> u32 {
        self.sgpr_allocation_granularity
    }
    ///Gets the value of [`Self::vgprs_per_simd`]
    pub fn vgprs_per_simd(&self) -> u32 {
        self.vgprs_per_simd
    }
    ///Gets the value of [`Self::min_vgpr_allocation`]
    pub fn min_vgpr_allocation(&self) -> u32 {
        self.min_vgpr_allocation
    }
    ///Gets the value of [`Self::max_vgpr_allocation`]
    pub fn max_vgpr_allocation(&self) -> u32 {
        self.max_vgpr_allocation
    }
    ///Gets the value of [`Self::vgpr_allocation_granularity`]
    pub fn vgpr_allocation_granularity(&self) -> u32 {
        self.vgpr_allocation_granularity
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::shader_engine_count`]
    pub fn shader_engine_count_mut(&mut self) -> &mut u32 {
        &mut self.shader_engine_count
    }
    ///Gets a mutable reference to the value of [`Self::shader_arrays_per_engine_count`]
    pub fn shader_arrays_per_engine_count_mut(&mut self) -> &mut u32 {
        &mut self.shader_arrays_per_engine_count
    }
    ///Gets a mutable reference to the value of [`Self::compute_units_per_shader_array`]
    pub fn compute_units_per_shader_array_mut(&mut self) -> &mut u32 {
        &mut self.compute_units_per_shader_array
    }
    ///Gets a mutable reference to the value of [`Self::simd_per_compute_unit`]
    pub fn simd_per_compute_unit_mut(&mut self) -> &mut u32 {
        &mut self.simd_per_compute_unit
    }
    ///Gets a mutable reference to the value of [`Self::wavefronts_per_simd`]
    pub fn wavefronts_per_simd_mut(&mut self) -> &mut u32 {
        &mut self.wavefronts_per_simd
    }
    ///Gets a mutable reference to the value of [`Self::wavefront_size`]
    pub fn wavefront_size_mut(&mut self) -> &mut u32 {
        &mut self.wavefront_size
    }
    ///Gets a mutable reference to the value of [`Self::sgprs_per_simd`]
    pub fn sgprs_per_simd_mut(&mut self) -> &mut u32 {
        &mut self.sgprs_per_simd
    }
    ///Gets a mutable reference to the value of [`Self::min_sgpr_allocation`]
    pub fn min_sgpr_allocation_mut(&mut self) -> &mut u32 {
        &mut self.min_sgpr_allocation
    }
    ///Gets a mutable reference to the value of [`Self::max_sgpr_allocation`]
    pub fn max_sgpr_allocation_mut(&mut self) -> &mut u32 {
        &mut self.max_sgpr_allocation
    }
    ///Gets a mutable reference to the value of [`Self::sgpr_allocation_granularity`]
    pub fn sgpr_allocation_granularity_mut(&mut self) -> &mut u32 {
        &mut self.sgpr_allocation_granularity
    }
    ///Gets a mutable reference to the value of [`Self::vgprs_per_simd`]
    pub fn vgprs_per_simd_mut(&mut self) -> &mut u32 {
        &mut self.vgprs_per_simd
    }
    ///Gets a mutable reference to the value of [`Self::min_vgpr_allocation`]
    pub fn min_vgpr_allocation_mut(&mut self) -> &mut u32 {
        &mut self.min_vgpr_allocation
    }
    ///Gets a mutable reference to the value of [`Self::max_vgpr_allocation`]
    pub fn max_vgpr_allocation_mut(&mut self) -> &mut u32 {
        &mut self.max_vgpr_allocation
    }
    ///Gets a mutable reference to the value of [`Self::vgpr_allocation_granularity`]
    pub fn vgpr_allocation_granularity_mut(&mut self) -> &mut u32 {
        &mut self.vgpr_allocation_granularity
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::shader_engine_count`]
    pub fn set_shader_engine_count(&mut self, value: u32) -> &mut Self {
        self.shader_engine_count = value;
        self
    }
    ///Sets the value of [`Self::shader_arrays_per_engine_count`]
    pub fn set_shader_arrays_per_engine_count(&mut self, value: u32) -> &mut Self {
        self.shader_arrays_per_engine_count = value;
        self
    }
    ///Sets the value of [`Self::compute_units_per_shader_array`]
    pub fn set_compute_units_per_shader_array(&mut self, value: u32) -> &mut Self {
        self.compute_units_per_shader_array = value;
        self
    }
    ///Sets the value of [`Self::simd_per_compute_unit`]
    pub fn set_simd_per_compute_unit(&mut self, value: u32) -> &mut Self {
        self.simd_per_compute_unit = value;
        self
    }
    ///Sets the value of [`Self::wavefronts_per_simd`]
    pub fn set_wavefronts_per_simd(&mut self, value: u32) -> &mut Self {
        self.wavefronts_per_simd = value;
        self
    }
    ///Sets the value of [`Self::wavefront_size`]
    pub fn set_wavefront_size(&mut self, value: u32) -> &mut Self {
        self.wavefront_size = value;
        self
    }
    ///Sets the value of [`Self::sgprs_per_simd`]
    pub fn set_sgprs_per_simd(&mut self, value: u32) -> &mut Self {
        self.sgprs_per_simd = value;
        self
    }
    ///Sets the value of [`Self::min_sgpr_allocation`]
    pub fn set_min_sgpr_allocation(&mut self, value: u32) -> &mut Self {
        self.min_sgpr_allocation = value;
        self
    }
    ///Sets the value of [`Self::max_sgpr_allocation`]
    pub fn set_max_sgpr_allocation(&mut self, value: u32) -> &mut Self {
        self.max_sgpr_allocation = value;
        self
    }
    ///Sets the value of [`Self::sgpr_allocation_granularity`]
    pub fn set_sgpr_allocation_granularity(&mut self, value: u32) -> &mut Self {
        self.sgpr_allocation_granularity = value;
        self
    }
    ///Sets the value of [`Self::vgprs_per_simd`]
    pub fn set_vgprs_per_simd(&mut self, value: u32) -> &mut Self {
        self.vgprs_per_simd = value;
        self
    }
    ///Sets the value of [`Self::min_vgpr_allocation`]
    pub fn set_min_vgpr_allocation(&mut self, value: u32) -> &mut Self {
        self.min_vgpr_allocation = value;
        self
    }
    ///Sets the value of [`Self::max_vgpr_allocation`]
    pub fn set_max_vgpr_allocation(&mut self, value: u32) -> &mut Self {
        self.max_vgpr_allocation = value;
        self
    }
    ///Sets the value of [`Self::vgpr_allocation_granularity`]
    pub fn set_vgpr_allocation_granularity(&mut self, value: u32) -> &mut Self {
        self.vgpr_allocation_granularity = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::shader_engine_count`]
    pub fn with_shader_engine_count(mut self, value: u32) -> Self {
        self.shader_engine_count = value;
        self
    }
    ///Sets the value of [`Self::shader_arrays_per_engine_count`]
    pub fn with_shader_arrays_per_engine_count(mut self, value: u32) -> Self {
        self.shader_arrays_per_engine_count = value;
        self
    }
    ///Sets the value of [`Self::compute_units_per_shader_array`]
    pub fn with_compute_units_per_shader_array(mut self, value: u32) -> Self {
        self.compute_units_per_shader_array = value;
        self
    }
    ///Sets the value of [`Self::simd_per_compute_unit`]
    pub fn with_simd_per_compute_unit(mut self, value: u32) -> Self {
        self.simd_per_compute_unit = value;
        self
    }
    ///Sets the value of [`Self::wavefronts_per_simd`]
    pub fn with_wavefronts_per_simd(mut self, value: u32) -> Self {
        self.wavefronts_per_simd = value;
        self
    }
    ///Sets the value of [`Self::wavefront_size`]
    pub fn with_wavefront_size(mut self, value: u32) -> Self {
        self.wavefront_size = value;
        self
    }
    ///Sets the value of [`Self::sgprs_per_simd`]
    pub fn with_sgprs_per_simd(mut self, value: u32) -> Self {
        self.sgprs_per_simd = value;
        self
    }
    ///Sets the value of [`Self::min_sgpr_allocation`]
    pub fn with_min_sgpr_allocation(mut self, value: u32) -> Self {
        self.min_sgpr_allocation = value;
        self
    }
    ///Sets the value of [`Self::max_sgpr_allocation`]
    pub fn with_max_sgpr_allocation(mut self, value: u32) -> Self {
        self.max_sgpr_allocation = value;
        self
    }
    ///Sets the value of [`Self::sgpr_allocation_granularity`]
    pub fn with_sgpr_allocation_granularity(mut self, value: u32) -> Self {
        self.sgpr_allocation_granularity = value;
        self
    }
    ///Sets the value of [`Self::vgprs_per_simd`]
    pub fn with_vgprs_per_simd(mut self, value: u32) -> Self {
        self.vgprs_per_simd = value;
        self
    }
    ///Sets the value of [`Self::min_vgpr_allocation`]
    pub fn with_min_vgpr_allocation(mut self, value: u32) -> Self {
        self.min_vgpr_allocation = value;
        self
    }
    ///Sets the value of [`Self::max_vgpr_allocation`]
    pub fn with_max_vgpr_allocation(mut self, value: u32) -> Self {
        self.max_vgpr_allocation = value;
        self
    }
    ///Sets the value of [`Self::vgpr_allocation_granularity`]
    pub fn with_vgpr_allocation_granularity(mut self, value: u32) -> Self {
        self.vgpr_allocation_granularity = value;
        self
    }
}
