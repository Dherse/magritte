use crate::extensions::khr_ray_tracing_pipeline::SHADER_UNUSED_KHR;
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_RAY_TRACING_SPEC_VERSION")]
pub const NV_RAY_TRACING_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_RAY_TRACING_EXTENSION_NAME")]
pub const NV_RAY_TRACING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_ray_tracing");
///[VK_SHADER_UNUSED_KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_SHADER_UNUSED_KHR.html) - Sentinel for an unused shader index
///# C Specifications
///[`SHADER_UNUSED_KHR`] is a special shader index used to indicate that a
///ray generation, miss, or callable shader member is not used.
///```c
///#define VK_SHADER_UNUSED_KHR              (~0U)
///```
///or the equivalent
///```c
///#define VK_SHADER_UNUSED_NV               VK_SHADER_UNUSED_KHR
///```
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
/// - [`VK_NV_ray_tracing`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_SHADER_UNUSED_NV")]
pub const SHADER_UNUSED_NV: u32 = SHADER_UNUSED_KHR;
///[VkAccelerationStructureMemoryRequirementsTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMemoryRequirementsTypeNV.html) - Acceleration structure memory requirement type
///# C Specifications
///Possible values of `type` in
///[`AccelerationStructureMemoryRequirementsInfoNV`] are:,
///```c
///// Provided by VK_NV_ray_tracing
///typedef enum VkAccelerationStructureMemoryRequirementsTypeNV {
///    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV = 0,
///    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV = 1,
///    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV = 2,
///} VkAccelerationStructureMemoryRequirementsTypeNV;
///```
///# Description
/// - [`ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT`]
///requests the memory requirement for the [`AccelerationStructureNV`]
///backing store.
/// - [`ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH`]
///requests the memory requirement for scratch space during the initial
///build.
/// - [`ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH`]
///requests the memory requirement for scratch space during an update.
///# Related
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureMemoryRequirementsInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureMemoryRequirementsTypeNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsTypeNV(i32);
impl const Default for AccelerationStructureMemoryRequirementsTypeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for AccelerationStructureMemoryRequirementsTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("AccelerationStructureMemoryRequirementsTypeNV")
            .field(match *self {
                Self::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT => {
                    &"ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT"
                },
                Self::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH => {
                    &"ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH"
                },
                Self::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH => {
                    &"ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH"
                },
                other => unreachable!(
                    "invalid value for `AccelerationStructureMemoryRequirementsTypeNV`: {:?}",
                    other
                ),
            })
            .finish()
    }
}
impl AccelerationStructureMemoryRequirementsTypeNV {
    ///[`ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT`]
    ///requests the memory requirement for the [`AccelerationStructureNV`]
    ///backing store.
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT: Self = Self(0);
    ///[`ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH`]
    ///requests the memory requirement for scratch space during the initial
    ///build.
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH: Self = Self(1);
    ///[`ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH`]
    ///requests the memory requirement for scratch space during an update.
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH: Self = Self(2);
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
}
