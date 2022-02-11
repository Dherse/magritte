#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION")]
pub const NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME")]
pub const NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_ray_tracing_motion_blur");
///[VkAccelerationStructureMotionInstanceTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceTypeNV.html) - Enum specifying a type of acceleration structure motion instance data for building into an acceleration structure geometry
///# C Specifications
///The [`AccelerationStructureMotionInstanceTypeNV`] enumeration is defined
///as:
///```c
///// Provided by VK_NV_ray_tracing_motion_blur
///typedef enum VkAccelerationStructureMotionInstanceTypeNV {
///    VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV = 0,
///    VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV = 1,
///    VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV = 2,
///} VkAccelerationStructureMotionInstanceTypeNV;
///```
///# Description
/// - [`ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC`] specifies
///that the instance is a static instance with no instance motion.
/// - [`ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION`]
///specifies that the instance is a motion instance with motion specified
///by interpolation between two matrices.
/// - [`ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION`]
///specifies that the instance is a motion instance with motion specified
///by interpolation in the SRT decomposition.
///# Related
/// - [`VK_NV_ray_tracing_motion_blur`]
/// - [`AccelerationStructureMotionInstanceNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureMotionInstanceTypeNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureMotionInstanceTypeNV(i32);
impl const Default for AccelerationStructureMotionInstanceTypeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for AccelerationStructureMotionInstanceTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("AccelerationStructureMotionInstanceTypeNV")
            .field(match *self {
                Self::ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC => {
                    &"ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC"
                },
                Self::ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION => {
                    &"ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION"
                },
                Self::ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION => {
                    &"ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION"
                },
                other => unreachable!(
                    "invalid value for `AccelerationStructureMotionInstanceTypeNV`: {:?}",
                    other
                ),
            })
            .finish()
    }
}
impl AccelerationStructureMotionInstanceTypeNV {
    ///[`ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC`] specifies
    ///that the instance is a static instance with no instance motion.
    pub const ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC: Self = Self(0);
    ///[`ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION`]
    ///specifies that the instance is a motion instance with motion specified
    ///by interpolation between two matrices.
    pub const ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION: Self = Self(1);
    ///[`ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION`]
    ///specifies that the instance is a motion instance with motion specified
    ///by interpolation in the SRT decomposition.
    pub const ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION: Self = Self(2);
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
