//![VK_NV_ray_tracing_motion_blur](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_ray_tracing_motion_blur.html) - device extension
//!# Description
//!Ray tracing support in the API provides an efficient mechanism to intersect
//!rays against static geometry, but rendering algorithms often want to support
//!motion, which is more efficiently supported with motion-specific algorithms.
//!This extension adds a set of mechanisms to support fast tracing of moving
//!geometry:
//! - A ray pipeline trace call which takes a time parameter
//! - Flags to enable motion support in an acceleration structure
//! - Support for time-varying vertex positions in a geometry
//! - Motion instances to move existing instances over time
//!The motion represented here is parameterized across a normalized timestep
//!between 0.0 and 1.0.
//!A motion trace using `OpTraceRayMotionNV` provides a time within that
//!normalized range to be used when intersecting that ray with geometry.
//!The geometry can be provided with motion by a combination of adding a second
//!vertex position for time of 1.0 using
//![`AccelerationStructureGeometryMotionTrianglesDataNV`] and providing
//!multiple transforms in the instance using
//![`AccelerationStructureMotionInstanceNV`].
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_ray_tracing_pipeline`]`
//!# Contacts
//! - Eric Werness
//!# New structures
//! - [`AccelerationStructureMatrixMotionInstanceNV`]
//! - [`AccelerationStructureMotionInstanceNV`]
//! - [`AccelerationStructureSrtMotionInstanceNV`]
//! - [`SrtDataNV`]
//! - Extending [`AccelerationStructureCreateInfoKHR`]:
//! - [`AccelerationStructureMotionInfoNV`]
//!
//! - Extending [`AccelerationStructureGeometryTrianglesDataKHR`]:
//! - [`AccelerationStructureGeometryMotionTrianglesDataNV`]
//!
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceRayTracingMotionBlurFeaturesNV`]
//!# New enums
//! - [`AccelerationStructureMotionInstanceTypeNV`]
//!# New bitmasks
//! - [`AccelerationStructureMotionInfoFlagsNV`]
//! - [`AccelerationStructureMotionInstanceFlagsNV`]
//!# New constants
//! - [`NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME`]
//! - [`NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION`]
//! - Extending [`AccelerationStructureCreateFlagBitsKHR`]:
//! - `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV`
//!
//! - Extending [`BuildAccelerationStructureFlagBitsKHR`]:
//! - `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV`
//!
//! - Extending [`PipelineCreateFlagBits`]:
//! - `VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV`
//! - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV`
//!# Known issues & F.A.Q
//!(1) What size is VkAccelerationStructureMotionInstanceNV?
//! - Added a note on the structure size and made the stride explicit in the
//!language.
//!(2) Allow arrayOfPointers for motion TLAS?
//! - Yes, with a packed encoding to minimize the amount of data sent for
//!metadata.
//!# Version History
//! - Revision 1, 2020-06-16 (Eric Werness, Ashwin Lele)
//! - Initial external release
//!# Other info
//! * 2021-06-16
//!*
//! - This extension requires
//![`SPV_NV_ray_tracing_motion_blur`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_ray_tracing_motion_blur.html)
//! - This extension provides API support for
//![`GL_NV_ray_tracing_motion_blur`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_ray_tracing_motion_blur.txt)
//!
//!*
//! - Eric Werness, NVIDIA
//! - Ashwin Lele, NVIDIA
//!# Related
//! - [`AccelerationStructureGeometryMotionTrianglesDataNV`]
//! - [`AccelerationStructureMatrixMotionInstanceNV`]
//! - [`AccelerationStructureMotionInfoFlagsNV`]
//! - [`AccelerationStructureMotionInfoNV`]
//! - [`AccelerationStructureMotionInstanceDataNV`]
//! - [`AccelerationStructureMotionInstanceFlagsNV`]
//! - [`AccelerationStructureMotionInstanceNV`]
//! - [`AccelerationStructureMotionInstanceTypeNV`]
//! - [`AccelerationStructureSrtMotionInstanceNV`]
//! - [`PhysicalDeviceRayTracingMotionBlurFeaturesNV`]
//! - [`SrtDataNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
