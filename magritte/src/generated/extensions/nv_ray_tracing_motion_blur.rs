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
//! - Extending [`AccelerationStructureCreateInfoKHR`]:  - [`AccelerationStructureMotionInfoNV`]
//! - Extending [`AccelerationStructureGeometryTrianglesDataKHR`]:  -
//!   [`AccelerationStructureGeometryMotionTrianglesDataNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceRayTracingMotionBlurFeaturesNV`]
//!# New enums
//! - [`AccelerationStructureMotionInstanceTypeNV`]
//!# New bitmasks
//! - [`AccelerationStructureMotionInfoFlagsNV`]
//! - [`AccelerationStructureMotionInstanceFlagsNV`]
//!# New constants
//! - [`NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME`]
//! - [`NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION`]
//! - Extending [`AccelerationStructureCreateFlagBitsKHR`]:  -
//!   `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV`
//! - Extending [`BuildAccelerationStructureFlagBitsKHR`]:  -
//!   `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV`
//! - Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV`  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV`
//!# Known issues & F.A.Q
//!(1) What size is VkAccelerationStructureMotionInstanceNV?
//! - Added a note on the structure size and made the stride explicit in the language.
//!(2) Allow arrayOfPointers for motion TLAS?
//! - Yes, with a packed encoding to minimize the amount of data sent for metadata.
//!# Version History
//! - Revision 1, 2020-06-16 (Eric Werness, Ashwin Lele)  - Initial external release
//!# Other info
//! * 2021-06-16
//! * - This extension requires [`SPV_NV_ray_tracing_motion_blur`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_ray_tracing_motion_blur.html)
//!   - This extension provides API support for [`GL_NV_ray_tracing_motion_blur`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_ray_tracing_motion_blur.txt)
//! * - Eric Werness, NVIDIA  - Ashwin Lele, NVIDIA
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
use crate::{
    extensions::khr_acceleration_structure::{
        AccelerationStructureInstanceKHR, DeviceOrHostAddressConstKHR, GeometryInstanceFlagsKHR, TransformMatrixKHR,
    },
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
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
/// - [`AccelerationStructureMotionInstanceTypeStaticNv`] specifies that the instance is a static
///   instance with no instance motion.
/// - [`AccelerationStructureMotionInstanceTypeMatrixMotionNv`] specifies that the instance is a
///   motion instance with motion specified by interpolation between two matrices.
/// - [`AccelerationStructureMotionInstanceTypeSrtMotionNv`] specifies that the instance is a motion
///   instance with motion specified by interpolation in the SRT decomposition.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum AccelerationStructureMotionInstanceTypeNV {
    ///[`AccelerationStructureMotionInstanceTypeStaticNv`] specifies
    ///that the instance is a static instance with no instance motion.
    AccelerationStructureMotionInstanceTypeStaticNv = 0,
    ///[`AccelerationStructureMotionInstanceTypeMatrixMotionNv`]
    ///specifies that the instance is a motion instance with motion specified
    ///by interpolation between two matrices.
    AccelerationStructureMotionInstanceTypeMatrixMotionNv = 1,
    ///[`AccelerationStructureMotionInstanceTypeSrtMotionNv`]
    ///specifies that the instance is a motion instance with motion specified
    ///by interpolation in the SRT decomposition.
    AccelerationStructureMotionInstanceTypeSrtMotionNv = 2,
}
impl const Default for AccelerationStructureMotionInstanceTypeNV {
    fn default() -> Self {
        Self::AccelerationStructureMotionInstanceTypeStaticNv
    }
}
impl AccelerationStructureMotionInstanceTypeNV {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkAccelerationStructureMotionInfoFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInfoFlagsNV.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_NV_ray_tracing_motion_blur
///typedef VkFlags VkAccelerationStructureMotionInfoFlagsNV;
///```
///# Related
/// - [`VK_NV_ray_tracing_motion_blur`]
/// - [`AccelerationStructureMotionInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct AccelerationStructureMotionInfoFlagsNV(u32);
impl const Default for AccelerationStructureMotionInfoFlagsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for AccelerationStructureMotionInfoFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(AccelerationStructureMotionInfoFlagsNV))
            .field(&self.0)
            .finish()
    }
}
///[VkAccelerationStructureMotionInstanceFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceFlagsNV.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_NV_ray_tracing_motion_blur
///typedef VkFlags VkAccelerationStructureMotionInstanceFlagsNV;
///```
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct AccelerationStructureMotionInstanceFlagsNV(u32);
impl const Default for AccelerationStructureMotionInstanceFlagsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for AccelerationStructureMotionInstanceFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(AccelerationStructureMotionInstanceFlagsNV))
            .field(&self.0)
            .finish()
    }
}
///[VkPhysicalDeviceRayTracingMotionBlurFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingMotionBlurFeaturesNV.html) - Structure describing the ray tracing motion blur features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceRayTracingMotionBlurFeaturesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_ray_tracing_motion_blur
///typedef struct VkPhysicalDeviceRayTracingMotionBlurFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           rayTracingMotionBlur;
///    VkBool32           rayTracingMotionBlurPipelineTraceRaysIndirect;
///} VkPhysicalDeviceRayTracingMotionBlurFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`ray_tracing_motion_blur`] indicates whether the implementation supports the motion blur
///   feature.
/// - [`ray_tracing_motion_blur_pipeline_trace_rays_indirect`] indicates whether the implementation
///   supports indirect ray tracing commands with the motion blur feature enabled.
///If the [`PhysicalDeviceRayTracingMotionBlurFeaturesNV`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceRayTracingMotionBlurFeaturesNV`] **can**  also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV`
///# Related
/// - [`VK_NV_ray_tracing_motion_blur`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceRayTracingMotionBlurFeaturesNV")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`ray_tracing_motion_blur`] indicates
    ///whether the implementation supports the motion blur feature.
    pub ray_tracing_motion_blur: Bool32,
    ///[`ray_tracing_motion_blur_pipeline_trace_rays_indirect`] indicates whether
    ///the implementation supports indirect ray tracing commands with the
    ///motion blur feature enabled.
    pub ray_tracing_motion_blur_pipeline_trace_rays_indirect: Bool32,
}
impl<'lt> Default for PhysicalDeviceRayTracingMotionBlurFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceRayTracingMotionBlurFeaturesNv,
            p_next: std::ptr::null_mut(),
            ray_tracing_motion_blur: 0,
            ray_tracing_motion_blur_pipeline_trace_rays_indirect: 0,
        }
    }
}
impl<'lt> PhysicalDeviceRayTracingMotionBlurFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::ray_tracing_motion_blur`]
    pub fn ray_tracing_motion_blur_raw(&self) -> Bool32 {
        self.ray_tracing_motion_blur
    }
    ///Gets the raw value of [`Self::ray_tracing_motion_blur_pipeline_trace_rays_indirect`]
    pub fn ray_tracing_motion_blur_pipeline_trace_rays_indirect_raw(&self) -> Bool32 {
        self.ray_tracing_motion_blur_pipeline_trace_rays_indirect
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::ray_tracing_motion_blur`]
    pub fn set_ray_tracing_motion_blur_raw(&mut self, value: Bool32) -> &mut Self {
        self.ray_tracing_motion_blur = value;
        self
    }
    ///Sets the raw value of [`Self::ray_tracing_motion_blur_pipeline_trace_rays_indirect`]
    pub fn set_ray_tracing_motion_blur_pipeline_trace_rays_indirect_raw(&mut self, value: Bool32) -> &mut Self {
        self.ray_tracing_motion_blur_pipeline_trace_rays_indirect = value;
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
    ///Gets the value of [`Self::ray_tracing_motion_blur`]
    pub fn ray_tracing_motion_blur(&self) -> bool {
        unsafe { std::mem::transmute(self.ray_tracing_motion_blur as u8) }
    }
    ///Gets the value of [`Self::ray_tracing_motion_blur_pipeline_trace_rays_indirect`]
    pub fn ray_tracing_motion_blur_pipeline_trace_rays_indirect(&self) -> bool {
        unsafe { std::mem::transmute(self.ray_tracing_motion_blur_pipeline_trace_rays_indirect as u8) }
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
    ///Gets a mutable reference to the value of [`Self::ray_tracing_motion_blur`]
    pub fn ray_tracing_motion_blur_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.ray_tracing_motion_blur as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.ray_tracing_motion_blur as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::ray_tracing_motion_blur_pipeline_trace_rays_indirect`]
    pub fn ray_tracing_motion_blur_pipeline_trace_rays_indirect_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.ray_tracing_motion_blur_pipeline_trace_rays_indirect as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.ray_tracing_motion_blur_pipeline_trace_rays_indirect as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::ray_tracing_motion_blur`]
    pub fn set_ray_tracing_motion_blur(&mut self, value: bool) -> &mut Self {
        self.ray_tracing_motion_blur = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::ray_tracing_motion_blur_pipeline_trace_rays_indirect`]
    pub fn set_ray_tracing_motion_blur_pipeline_trace_rays_indirect(&mut self, value: bool) -> &mut Self {
        self.ray_tracing_motion_blur_pipeline_trace_rays_indirect = value as u8 as u32;
        self
    }
}
///[VkAccelerationStructureGeometryMotionTrianglesDataNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryMotionTrianglesDataNV.html) - Structure specifying vertex motion in a bottom-level acceleration structure
///# C Specifications
///The [`AccelerationStructureGeometryMotionTrianglesDataNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_ray_tracing_motion_blur
///typedef struct VkAccelerationStructureGeometryMotionTrianglesDataNV {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkDeviceOrHostAddressConstKHR    vertexData;
///} VkAccelerationStructureGeometryMotionTrianglesDataNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`vertex_data`] is a pointer to vertex data for this geometry at time 1.0
///# Description
///If [`AccelerationStructureGeometryMotionTrianglesDataNV`] is included in
///the [`p_next`] chain of a
///[`AccelerationStructureGeometryTrianglesDataKHR`] structure, the basic
///vertex positions are used for the position of the triangles in the geometry
///at time 0.0 and the [`vertex_data`] in
///[`AccelerationStructureGeometryMotionTrianglesDataNV`] is used for the
///vertex positions at time 1.0, with positions linearly interpolated at
///intermediate times.Indexing for
/// [`AccelerationStructureGeometryMotionTrianglesDataNV`][`vertex_data`] is equivalent to the basic
/// vertex position data.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV`
///# Related
/// - [`VK_NV_ray_tracing_motion_blur`]
/// - [`DeviceOrHostAddressConstKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureGeometryMotionTrianglesDataNV")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AccelerationStructureGeometryMotionTrianglesDataNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`vertex_data`] is a pointer to vertex data for this geometry at time
    ///1.0
    pub vertex_data: DeviceOrHostAddressConstKHR,
}
impl<'lt> Default for AccelerationStructureGeometryMotionTrianglesDataNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::AccelerationStructureGeometryMotionTrianglesDataNv,
            p_next: std::ptr::null(),
            vertex_data: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> AccelerationStructureGeometryMotionTrianglesDataNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::vertex_data`]
    pub fn vertex_data(&self) -> DeviceOrHostAddressConstKHR {
        self.vertex_data
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::vertex_data`]
    pub fn vertex_data_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.vertex_data
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::vertex_data`]
    pub fn set_vertex_data(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    ) -> &mut Self {
        self.vertex_data = value;
        self
    }
}
///[VkAccelerationStructureMotionInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInfoNV.html) - Structure specifying the parameters of a newly created acceleration structure object
///# C Specifications
///The [`AccelerationStructureMotionInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_ray_tracing_motion_blur
///typedef struct VkAccelerationStructureMotionInfoNV {
///    VkStructureType                             sType;
///    const void*                                 pNext;
///    uint32_t                                    maxInstances;
///    VkAccelerationStructureMotionInfoFlagsNV    flags;
///} VkAccelerationStructureMotionInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_instances`] is the maximum number of instances that  **may**  be used in the motion
///   top-level acceleration structure.
/// - [`flags`] is 0 and reserved for future use.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`VK_NV_ray_tracing_motion_blur`]
/// - [`AccelerationStructureMotionInfoFlagsNV`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureMotionInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AccelerationStructureMotionInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`max_instances`] is the maximum number of instances that  **may**  be used
    ///in the motion top-level acceleration structure.
    pub max_instances: u32,
    ///[`flags`] is 0 and reserved for future use.
    pub flags: AccelerationStructureMotionInfoFlagsNV,
}
impl<'lt> Default for AccelerationStructureMotionInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::AccelerationStructureMotionInfoNv,
            p_next: std::ptr::null(),
            max_instances: 0,
            flags: Default::default(),
        }
    }
}
impl<'lt> AccelerationStructureMotionInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::max_instances`]
    pub fn max_instances(&self) -> u32 {
        self.max_instances
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> AccelerationStructureMotionInfoFlagsNV {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::max_instances`]
    pub fn max_instances_mut(&mut self) -> &mut u32 {
        &mut self.max_instances
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut AccelerationStructureMotionInfoFlagsNV {
        &mut self.flags
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::max_instances`]
    pub fn set_max_instances(&mut self, value: u32) -> &mut Self {
        self.max_instances = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInfoFlagsNV,
    ) -> &mut Self {
        self.flags = value;
        self
    }
}
///[VkSRTDataNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSRTDataNV.html) - Structure specifying a transform in SRT decomposition
///# C Specifications
///An acceleration structure SRT transform is defined by the structure:
///```c
///// Provided by VK_NV_ray_tracing_motion_blur
///typedef struct VkSRTDataNV {
///    float    sx;
///    float    a;
///    float    b;
///    float    pvx;
///    float    sy;
///    float    c;
///    float    pvy;
///    float    sz;
///    float    pvz;
///    float    qx;
///    float    qy;
///    float    qz;
///    float    qw;
///    float    tx;
///    float    ty;
///    float    tz;
///} VkSRTDataNV;
///```
///# Members
/// - [`sx`] is the x component of the scale of the transform
/// - [`a`] is one component of the shear for the transform
/// - [`b`] is one component of the shear for the transform
/// - [`pvx`] is the x component of the pivot point of the transform
/// - [`sy`] is the y component of the scale of the transform
/// - [`c`] is one component of the shear for the transform
/// - [`pvy`] is the y component of the pivot point of the transform
/// - [`sz`] is the z component of the scale of the transform
/// - [`pvz`] is the z component of the pivot point of the transform
/// - [`qx`] is the x component of the rotation quaternion
/// - [`qy`] is the y component of the rotation quaternion
/// - [`qz`] is the z component of the rotation quaternion
/// - [`qw`] is the w component of the rotation quaternion
/// - [`tx`] is the x component of the post-rotation translation
/// - [`ty`] is the y component of the post-rotation translation
/// - [`tz`] is the z component of the post-rotation translation
///# Description
///This transform decomposition consists of three elements.
///The first is a matrix S, consisting of a scale, shear, and translation,
///usually used to define the pivot point of the following rotation.
///This matrix is constructed from the parameters above by:<span class="katex"><span
/// class="katex-html" aria-hidden="true"><span class="base"><span class="strut"
/// style="height:0.68333em;vertical-align:0em;"></span><span style="margin-right:0.05764em;"
/// class="mord mathdefault">S</span><span class="mspace"
/// style="margin-right:0.2777777777777778em;"></span><span class="mrel">=</span><span
/// class="mspace" style="margin-right:0.2777777777777778em;"></span></span><span class="base"><span
/// class="strut" style="height:3.60004em;vertical-align:-1.55002em;"></span><span
/// class="minner"><span class="mopen"><span class="delimsizing mult"><span class="vlist-t
/// vlist-t2"><span class="vlist-r"><span style="height:2.05002em;" class="vlist"><span
/// style="top:-2.2500000000000004em;"><span class="pstrut"
/// style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎝</span></span></span><span style="top:-2.8100000000000005em;"><span
/// class="pstrut" style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎜</span></span></span><span style="top:-4.05002em;"><span class="pstrut"
/// style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎛</span></span></span></span><span class="vlist-s">​</span></span><span
/// class="vlist-r"><span style="height:1.55002em;"
/// class="vlist"><span></span></span></span></span></span></span><span class="mord"><span
/// class="mtable"><span class="col-align-c"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span class="vlist" style="height:2.05em;"><span style="top:-4.21em;"><span
/// class="pstrut" style="height:3em;"></span><span class="mord"><span class="mord
/// mathdefault">s</span><span class="mord mathdefault">x</span></span></span><span
/// style="top:-3.0099999999999993em;"><span class="pstrut" style="height:3em;"></span><span
/// class="mord"><span class="mord">0</span></span></span><span
/// style="top:-1.8099999999999994em;"><span style="height:3em;" class="pstrut"></span><span
/// class="mord"><span class="mord">0</span></span></span></span><span
/// class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
/// style="height:1.5500000000000007em;"><span></span></span></span></span></span><span
/// style="width:0.5em;" class="arraycolsep"></span><span class="arraycolsep"
/// style="width:0.5em;"></span><span class="col-align-c"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span style="height:2.05em;" class="vlist"><span style="top:-4.21em;"><span
/// class="pstrut" style="height:3em;"></span><span class="mord"><span class="mord
/// mathdefault">a</span></span></span><span style="top:-3.0099999999999993em;"><span class="pstrut"
/// style="height:3em;"></span><span class="mord"><span class="mord mathdefault">s</span><span
/// class="mord mathdefault" style="margin-right:0.03588em;">y</span></span></span><span
/// style="top:-1.8099999999999994em;"><span class="pstrut" style="height:3em;"></span><span
/// class="mord"><span class="mord">0</span></span></span></span><span
/// class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
/// style="height:1.5500000000000007em;"><span></span></span></span></span></span><span
/// style="width:0.5em;" class="arraycolsep"></span><span style="width:0.5em;"
/// class="arraycolsep"></span><span class="col-align-c"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span style="height:2.05em;" class="vlist"><span style="top:-4.21em;"><span
/// class="pstrut" style="height:3em;"></span><span class="mord"><span class="mord
/// mathdefault">b</span></span></span><span style="top:-3.0099999999999993em;"><span class="pstrut"
/// style="height:3em;"></span><span class="mord"><span class="mord
/// mathdefault">c</span></span></span><span style="top:-1.8099999999999994em;"><span class="pstrut"
/// style="height:3em;"></span><span class="mord"><span class="mord mathdefault">s</span><span
/// class="mord mathdefault" style="margin-right:0.04398em;">z</span></span></span></span><span
/// class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
/// style="height:1.5500000000000007em;"><span></span></span></span></span></span><span
/// class="arraycolsep" style="width:0.5em;"></span><span class="arraycolsep"
/// style="width:0.5em;"></span><span class="col-align-c"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span class="vlist" style="height:2.05em;"><span style="top:-4.21em;"><span
/// class="pstrut" style="height:3em;"></span><span class="mord"><span class="mord
/// mathdefault">p</span><span style="margin-right:0.03588em;" class="mord
/// mathdefault">v</span><span class="mord mathdefault">x</span></span></span><span
/// style="top:-3.0099999999999993em;"><span class="pstrut" style="height:3em;"></span><span
/// class="mord"><span class="mord mathdefault">p</span><span class="mord mathdefault"
/// style="margin-right:0.03588em;">v</span><span class="mord mathdefault"
/// style="margin-right:0.03588em;">y</span></span></span><span
/// style="top:-1.8099999999999994em;"><span class="pstrut" style="height:3em;"></span><span
/// class="mord"><span class="mord mathdefault">p</span><span style="margin-right:0.03588em;"
/// class="mord mathdefault">v</span><span class="mord mathdefault"
/// style="margin-right:0.04398em;">z</span></span></span></span><span
/// class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.5500000000000007em;"
/// class="vlist"><span></span></span></span></span></span></span></span><span class="mclose"><span
/// class="delimsizing mult"><span class="vlist-t vlist-t2"><span class="vlist-r"><span
/// class="vlist" style="height:2.05002em;"><span style="top:-2.2500000000000004em;"><span
/// style="height:3.1550000000000002em;" class="pstrut"></span><span class="delimsizinginner
/// delim-size4"><span>⎠</span></span></span><span style="top:-2.8100000000000005em;"><span
/// class="pstrut" style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎟</span></span></span><span style="top:-4.05002em;"><span
/// style="height:3.1550000000000002em;" class="pstrut"></span><span class="delimsizinginner
/// delim-size4"><span>⎞</span></span></span></span><span class="vlist-s">​</span></span><span
/// class="vlist-r"><span style="height:1.55002em;"
/// class="vlist"><span></span></span></span></span></span></span></span></span></span></span>The
/// rotation quaternion is defined as:
/// * `R` = [ [`qx`], [`qy`], [`qz`], [`qw`] ]
///This is a rotation around a conceptual normalized axis [ ax, ay, az ]
///of amount `theta` such that:
/// * [ [`qx`], [`qy`], [`qz`] ] = sin(`theta`/2) × [ `ax`, `ay`, `az` ]
///and
/// * [`qw`] = cos(`theta`/2)
///Finally, the transform has a translation T constructed from the parameters
///above by:<span class="katex"><span aria-hidden="true" class="katex-html"><span
/// class="base"><span style="height:0.68333em;vertical-align:0em;" class="strut"></span><span
/// style="margin-right:0.13889em;" class="mord mathdefault">T</span><span
/// style="margin-right:0.2777777777777778em;" class="mspace"></span><span
/// class="mrel">=</span><span class="mspace"
/// style="margin-right:0.2777777777777778em;"></span></span><span class="base"><span class="strut"
/// style="height:3.60004em;vertical-align:-1.55002em;"></span><span class="minner"><span
/// class="mopen"><span class="delimsizing mult"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span style="height:2.05002em;" class="vlist"><span
/// style="top:-2.2500000000000004em;"><span style="height:3.1550000000000002em;"
/// class="pstrut"></span><span class="delimsizinginner
/// delim-size4"><span>⎝</span></span></span><span style="top:-2.8100000000000005em;"><span
/// class="pstrut" style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎜</span></span></span><span style="top:-4.05002em;"><span class="pstrut"
/// style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎛</span></span></span></span><span class="vlist-s">​</span></span><span
/// class="vlist-r"><span style="height:1.55002em;"
/// class="vlist"><span></span></span></span></span></span></span><span class="mord"><span
/// class="mtable"><span class="col-align-c"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span class="vlist" style="height:2.05em;"><span style="top:-4.21em;"><span
/// style="height:3em;" class="pstrut"></span><span class="mord"><span
/// class="mord">1</span></span></span><span style="top:-3.0099999999999993em;"><span
/// style="height:3em;" class="pstrut"></span><span class="mord"><span
/// class="mord">0</span></span></span><span style="top:-1.8099999999999994em;"><span
/// style="height:3em;" class="pstrut"></span><span class="mord"><span
/// class="mord">0</span></span></span></span><span class="vlist-s">​</span></span><span
/// class="vlist-r"><span class="vlist"
/// style="height:1.5500000000000007em;"><span></span></span></span></span></span><span
/// style="width:0.5em;" class="arraycolsep"></span><span style="width:0.5em;"
/// class="arraycolsep"></span><span class="col-align-c"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span style="height:2.05em;" class="vlist"><span style="top:-4.21em;"><span
/// class="pstrut" style="height:3em;"></span><span class="mord"><span
/// class="mord">0</span></span></span><span style="top:-3.0099999999999993em;"><span class="pstrut"
/// style="height:3em;"></span><span class="mord"><span class="mord">1</span></span></span><span
/// style="top:-1.8099999999999994em;"><span style="height:3em;" class="pstrut"></span><span
/// class="mord"><span class="mord">0</span></span></span></span><span
/// class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
/// style="height:1.5500000000000007em;"><span></span></span></span></span></span><span
/// style="width:0.5em;" class="arraycolsep"></span><span style="width:0.5em;"
/// class="arraycolsep"></span><span class="col-align-c"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span class="vlist" style="height:2.05em;"><span style="top:-4.21em;"><span
/// style="height:3em;" class="pstrut"></span><span class="mord"><span
/// class="mord">0</span></span></span><span style="top:-3.0099999999999993em;"><span class="pstrut"
/// style="height:3em;"></span><span class="mord"><span class="mord">0</span></span></span><span
/// style="top:-1.8099999999999994em;"><span style="height:3em;" class="pstrut"></span><span
/// class="mord"><span class="mord">1</span></span></span></span><span
/// class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
/// style="height:1.5500000000000007em;"><span></span></span></span></span></span><span
/// style="width:0.5em;" class="arraycolsep"></span><span class="arraycolsep"
/// style="width:0.5em;"></span><span class="col-align-c"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span class="vlist" style="height:2.05em;"><span style="top:-4.21em;"><span
/// class="pstrut" style="height:3em;"></span><span class="mord"><span class="mord
/// mathdefault">t</span><span class="mord mathdefault">x</span></span></span><span
/// style="top:-3.0099999999999993em;"><span class="pstrut" style="height:3em;"></span><span
/// class="mord"><span class="mord mathdefault">t</span><span style="margin-right:0.03588em;"
/// class="mord mathdefault">y</span></span></span><span style="top:-1.8099999999999994em;"><span
/// style="height:3em;" class="pstrut"></span><span class="mord"><span class="mord
/// mathdefault">t</span><span style="margin-right:0.04398em;" class="mord
/// mathdefault">z</span></span></span></span><span class="vlist-s">​</span></span><span
/// class="vlist-r"><span style="height:1.5500000000000007em;"
/// class="vlist"><span></span></span></span></span></span></span></span><span class="mclose"><span
/// class="delimsizing mult"><span class="vlist-t vlist-t2"><span class="vlist-r"><span
/// style="height:2.05002em;" class="vlist"><span style="top:-2.2500000000000004em;"><span
/// style="height:3.1550000000000002em;" class="pstrut"></span><span class="delimsizinginner
/// delim-size4"><span>⎠</span></span></span><span style="top:-2.8100000000000005em;"><span
/// style="height:3.1550000000000002em;" class="pstrut"></span><span class="delimsizinginner
/// delim-size4"><span>⎟</span></span></span><span style="top:-4.05002em;"><span class="pstrut"
/// style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎞</span></span></span></span><span class="vlist-s">​</span></span><span
/// class="vlist-r"><span style="height:1.55002em;"
/// class="vlist"><span></span></span></span></span></span></span></span></span></span></span>The
/// effective derived transform is then given by
/// * `T` × `R` × `S`
///# Related
/// - [`VK_NV_ray_tracing_motion_blur`]
/// - [`AccelerationStructureSrtMotionInstanceNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSRTDataNV")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SrtDataNV {
    ///[`sx`] is the x component of the scale of the transform
    pub sx: f32,
    ///[`a`] is one component of the shear for the transform
    pub a: f32,
    ///[`b`] is one component of the shear for the transform
    pub b: f32,
    ///[`pvx`] is the x component of the pivot point of the transform
    pub pvx: f32,
    ///[`sy`] is the y component of the scale of the transform
    pub sy: f32,
    ///[`c`] is one component of the shear for the transform
    pub c: f32,
    ///[`pvy`] is the y component of the pivot point of the transform
    pub pvy: f32,
    ///[`sz`] is the z component of the scale of the transform
    pub sz: f32,
    ///[`pvz`] is the z component of the pivot point of the transform
    pub pvz: f32,
    ///[`qx`] is the x component of the rotation quaternion
    pub qx: f32,
    ///[`qy`] is the y component of the rotation quaternion
    pub qy: f32,
    ///[`qz`] is the z component of the rotation quaternion
    pub qz: f32,
    ///[`qw`] is the w component of the rotation quaternion
    pub qw: f32,
    ///[`tx`] is the x component of the post-rotation translation
    pub tx: f32,
    ///[`ty`] is the y component of the post-rotation translation
    pub ty: f32,
    ///[`tz`] is the z component of the post-rotation translation
    pub tz: f32,
}
impl Default for SrtDataNV {
    fn default() -> Self {
        Self {
            sx: 0.0,
            a: 0.0,
            b: 0.0,
            pvx: 0.0,
            sy: 0.0,
            c: 0.0,
            pvy: 0.0,
            sz: 0.0,
            pvz: 0.0,
            qx: 0.0,
            qy: 0.0,
            qz: 0.0,
            qw: 0.0,
            tx: 0.0,
            ty: 0.0,
            tz: 0.0,
        }
    }
}
impl SrtDataNV {
    ///Gets the value of [`Self::sx`]
    pub fn sx(&self) -> f32 {
        self.sx
    }
    ///Gets the value of [`Self::a`]
    pub fn a(&self) -> f32 {
        self.a
    }
    ///Gets the value of [`Self::b`]
    pub fn b(&self) -> f32 {
        self.b
    }
    ///Gets the value of [`Self::pvx`]
    pub fn pvx(&self) -> f32 {
        self.pvx
    }
    ///Gets the value of [`Self::sy`]
    pub fn sy(&self) -> f32 {
        self.sy
    }
    ///Gets the value of [`Self::c`]
    pub fn c(&self) -> f32 {
        self.c
    }
    ///Gets the value of [`Self::pvy`]
    pub fn pvy(&self) -> f32 {
        self.pvy
    }
    ///Gets the value of [`Self::sz`]
    pub fn sz(&self) -> f32 {
        self.sz
    }
    ///Gets the value of [`Self::pvz`]
    pub fn pvz(&self) -> f32 {
        self.pvz
    }
    ///Gets the value of [`Self::qx`]
    pub fn qx(&self) -> f32 {
        self.qx
    }
    ///Gets the value of [`Self::qy`]
    pub fn qy(&self) -> f32 {
        self.qy
    }
    ///Gets the value of [`Self::qz`]
    pub fn qz(&self) -> f32 {
        self.qz
    }
    ///Gets the value of [`Self::qw`]
    pub fn qw(&self) -> f32 {
        self.qw
    }
    ///Gets the value of [`Self::tx`]
    pub fn tx(&self) -> f32 {
        self.tx
    }
    ///Gets the value of [`Self::ty`]
    pub fn ty(&self) -> f32 {
        self.ty
    }
    ///Gets the value of [`Self::tz`]
    pub fn tz(&self) -> f32 {
        self.tz
    }
    ///Gets a mutable reference to the value of [`Self::sx`]
    pub fn sx_mut(&mut self) -> &mut f32 {
        &mut self.sx
    }
    ///Gets a mutable reference to the value of [`Self::a`]
    pub fn a_mut(&mut self) -> &mut f32 {
        &mut self.a
    }
    ///Gets a mutable reference to the value of [`Self::b`]
    pub fn b_mut(&mut self) -> &mut f32 {
        &mut self.b
    }
    ///Gets a mutable reference to the value of [`Self::pvx`]
    pub fn pvx_mut(&mut self) -> &mut f32 {
        &mut self.pvx
    }
    ///Gets a mutable reference to the value of [`Self::sy`]
    pub fn sy_mut(&mut self) -> &mut f32 {
        &mut self.sy
    }
    ///Gets a mutable reference to the value of [`Self::c`]
    pub fn c_mut(&mut self) -> &mut f32 {
        &mut self.c
    }
    ///Gets a mutable reference to the value of [`Self::pvy`]
    pub fn pvy_mut(&mut self) -> &mut f32 {
        &mut self.pvy
    }
    ///Gets a mutable reference to the value of [`Self::sz`]
    pub fn sz_mut(&mut self) -> &mut f32 {
        &mut self.sz
    }
    ///Gets a mutable reference to the value of [`Self::pvz`]
    pub fn pvz_mut(&mut self) -> &mut f32 {
        &mut self.pvz
    }
    ///Gets a mutable reference to the value of [`Self::qx`]
    pub fn qx_mut(&mut self) -> &mut f32 {
        &mut self.qx
    }
    ///Gets a mutable reference to the value of [`Self::qy`]
    pub fn qy_mut(&mut self) -> &mut f32 {
        &mut self.qy
    }
    ///Gets a mutable reference to the value of [`Self::qz`]
    pub fn qz_mut(&mut self) -> &mut f32 {
        &mut self.qz
    }
    ///Gets a mutable reference to the value of [`Self::qw`]
    pub fn qw_mut(&mut self) -> &mut f32 {
        &mut self.qw
    }
    ///Gets a mutable reference to the value of [`Self::tx`]
    pub fn tx_mut(&mut self) -> &mut f32 {
        &mut self.tx
    }
    ///Gets a mutable reference to the value of [`Self::ty`]
    pub fn ty_mut(&mut self) -> &mut f32 {
        &mut self.ty
    }
    ///Gets a mutable reference to the value of [`Self::tz`]
    pub fn tz_mut(&mut self) -> &mut f32 {
        &mut self.tz
    }
    ///Sets the raw value of [`Self::sx`]
    pub fn set_sx(&mut self, value: f32) -> &mut Self {
        self.sx = value;
        self
    }
    ///Sets the raw value of [`Self::a`]
    pub fn set_a(&mut self, value: f32) -> &mut Self {
        self.a = value;
        self
    }
    ///Sets the raw value of [`Self::b`]
    pub fn set_b(&mut self, value: f32) -> &mut Self {
        self.b = value;
        self
    }
    ///Sets the raw value of [`Self::pvx`]
    pub fn set_pvx(&mut self, value: f32) -> &mut Self {
        self.pvx = value;
        self
    }
    ///Sets the raw value of [`Self::sy`]
    pub fn set_sy(&mut self, value: f32) -> &mut Self {
        self.sy = value;
        self
    }
    ///Sets the raw value of [`Self::c`]
    pub fn set_c(&mut self, value: f32) -> &mut Self {
        self.c = value;
        self
    }
    ///Sets the raw value of [`Self::pvy`]
    pub fn set_pvy(&mut self, value: f32) -> &mut Self {
        self.pvy = value;
        self
    }
    ///Sets the raw value of [`Self::sz`]
    pub fn set_sz(&mut self, value: f32) -> &mut Self {
        self.sz = value;
        self
    }
    ///Sets the raw value of [`Self::pvz`]
    pub fn set_pvz(&mut self, value: f32) -> &mut Self {
        self.pvz = value;
        self
    }
    ///Sets the raw value of [`Self::qx`]
    pub fn set_qx(&mut self, value: f32) -> &mut Self {
        self.qx = value;
        self
    }
    ///Sets the raw value of [`Self::qy`]
    pub fn set_qy(&mut self, value: f32) -> &mut Self {
        self.qy = value;
        self
    }
    ///Sets the raw value of [`Self::qz`]
    pub fn set_qz(&mut self, value: f32) -> &mut Self {
        self.qz = value;
        self
    }
    ///Sets the raw value of [`Self::qw`]
    pub fn set_qw(&mut self, value: f32) -> &mut Self {
        self.qw = value;
        self
    }
    ///Sets the raw value of [`Self::tx`]
    pub fn set_tx(&mut self, value: f32) -> &mut Self {
        self.tx = value;
        self
    }
    ///Sets the raw value of [`Self::ty`]
    pub fn set_ty(&mut self, value: f32) -> &mut Self {
        self.ty = value;
        self
    }
    ///Sets the raw value of [`Self::tz`]
    pub fn set_tz(&mut self, value: f32) -> &mut Self {
        self.tz = value;
        self
    }
}
///[VkAccelerationStructureSRTMotionInstanceNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureSRTMotionInstanceNV.html) - Structure specifying a single acceleration structure SRT motion instance for building into an acceleration structure geometry
///# C Specifications
///An acceleration structure SRT motion instance is defined by the structure:
///```c
///// Provided by VK_NV_ray_tracing_motion_blur
///typedef struct VkAccelerationStructureSRTMotionInstanceNV {
///    VkSRTDataNV                   transformT0;
///    VkSRTDataNV                   transformT1;
///    uint32_t                      instanceCustomIndex:24;
///    uint32_t                      mask:8;
///    uint32_t                      instanceShaderBindingTableRecordOffset:24;
///    VkGeometryInstanceFlagsKHR    flags:8;
///    uint64_t                      accelerationStructureReference;
///} VkAccelerationStructureSRTMotionInstanceNV;
///```
///# Members
/// - [`transform_t_0`] is a [`SrtDataNV`] structure describing a transformation to be applied to
///   the acceleration structure at time 0.
/// - [`transform_t_1`] is a [`SrtDataNV`] structure describing a transformation to be applied to
///   the acceleration structure at time 1.
/// - [`instance_custom_index`] is a 24-bit user-specified index value accessible to ray shaders in
///   the `InstanceCustomIndexKHR` built-in.
/// - [`mask`] is an 8-bit visibility mask for the geometry. The instance  **may**  only be hit if
///   `Cull Mask & instance.mask != 0`
/// - [`instance_shader_binding_table_record_offset`] is a 24-bit offset used in calculating the hit
///   shader binding table index.
/// - [`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`] values to apply to this
///   instance.
/// - [`acceleration_structure_reference`] is either:  - a device address containing the value
///   obtained from [`get_acceleration_structure_device_address_khr`] or
///   [`get_acceleration_structure_handle_nv`]      (used by device operations which reference
///   acceleration structures) or,  - a [`AccelerationStructureKHR`] object (used by host operations
///   which reference acceleration structures).
///# Description
///The C language specification does not define the ordering of bit-fields, but
///in practice, this struct produces the correct layout with existing
///compilers.
///The intended bit pattern is for the following:
/// - [`instance_custom_index`] and [`mask`] occupy the same memory as if a single `uint32_t` was
///   specified in their place  - [`instance_custom_index`] occupies the 24 least significant bits
///   of that memory  - [`mask`] occupies the 8 most significant bits of that memory
/// - [`instance_shader_binding_table_record_offset`] and [`flags`] occupy the same memory as if a
///   single `uint32_t` was specified in their place  -
///   [`instance_shader_binding_table_record_offset`] occupies the 24 least significant bits of that
///   memory  - [`flags`] occupies the 8 most significant bits of that memory
///If a compiler produces code that diverges from that pattern, applications
/// **must**  employ another method to set values according to the correct bit
///pattern.The transform for a SRT motion instance at a point in time is derived from
///component-wise linear interpolation of the two SRT transforms.
///That is, for a `time` in [0,1] the resulting transform is
/// * [`transform_t_0`] × (1 - `time`) +  [`transform_t_1`] × `time`
///
///## Valid Usage (Implicit)
/// - [`flags`] **must**  be a valid combination of [`GeometryInstanceFlagBitsKHR`] values
///# Related
/// - [`VK_NV_ray_tracing_motion_blur`]
/// - [`AccelerationStructureMotionInstanceDataNV`]
/// - [`GeometryInstanceFlagsKHR`]
/// - [`SrtDataNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureSRTMotionInstanceNV")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureSrtMotionInstanceNV {
    ///[`transform_t_0`] is a [`SrtDataNV`] structure describing a
    ///transformation to be applied to the acceleration structure at time 0.
    pub transform_t_0: SrtDataNV,
    ///[`transform_t_1`] is a [`SrtDataNV`] structure describing a
    ///transformation to be applied to the acceleration structure at time 1.
    pub transform_t_1: SrtDataNV,
    ///[`instance_custom_index`] is a 24-bit user-specified index value
    ///accessible to ray shaders in the `InstanceCustomIndexKHR` built-in.
    pub instance_custom_index: u32,
    ///[`mask`] is an 8-bit visibility mask for the geometry.
    ///The instance  **may**  only be hit if `Cull Mask & instance.mask != 0`
    pub mask: u32,
    ///[`instance_shader_binding_table_record_offset`] is a 24-bit offset used in
    ///calculating the hit shader binding table index.
    pub instance_shader_binding_table_record_offset: u32,
    ///[`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`]
    ///values to apply to this instance.
    pub flags: GeometryInstanceFlagsKHR,
    ///[`acceleration_structure_reference`] is either:
    /// - a device address containing the value obtained from
    ///   [`get_acceleration_structure_device_address_khr`] or
    ///   [`get_acceleration_structure_handle_nv`]      (used by device operations which reference
    ///   acceleration structures) or,
    /// - a [`AccelerationStructureKHR`] object (used by host operations which reference
    ///   acceleration structures).
    pub acceleration_structure_reference: u64,
}
impl Default for AccelerationStructureSrtMotionInstanceNV {
    fn default() -> Self {
        Self {
            transform_t_0: Default::default(),
            transform_t_1: Default::default(),
            instance_custom_index: 0,
            mask: 0,
            instance_shader_binding_table_record_offset: 0,
            flags: Default::default(),
            acceleration_structure_reference: 0,
        }
    }
}
impl AccelerationStructureSrtMotionInstanceNV {
    ///Gets the value of [`Self::transform_t_0`]
    pub fn transform_t_0(&self) -> SrtDataNV {
        self.transform_t_0
    }
    ///Gets the value of [`Self::transform_t_1`]
    pub fn transform_t_1(&self) -> SrtDataNV {
        self.transform_t_1
    }
    ///Gets the value of [`Self::instance_custom_index`]
    pub fn instance_custom_index(&self) -> u32 {
        self.instance_custom_index
    }
    ///Gets the value of [`Self::mask`]
    pub fn mask(&self) -> u32 {
        self.mask
    }
    ///Gets the value of [`Self::instance_shader_binding_table_record_offset`]
    pub fn instance_shader_binding_table_record_offset(&self) -> u32 {
        self.instance_shader_binding_table_record_offset
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> GeometryInstanceFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::acceleration_structure_reference`]
    pub fn acceleration_structure_reference(&self) -> u64 {
        self.acceleration_structure_reference
    }
    ///Gets a mutable reference to the value of [`Self::transform_t_0`]
    pub fn transform_t_0_mut(&mut self) -> &mut SrtDataNV {
        &mut self.transform_t_0
    }
    ///Gets a mutable reference to the value of [`Self::transform_t_1`]
    pub fn transform_t_1_mut(&mut self) -> &mut SrtDataNV {
        &mut self.transform_t_1
    }
    ///Gets a mutable reference to the value of [`Self::instance_custom_index`]
    pub fn instance_custom_index_mut(&mut self) -> &mut u32 {
        &mut self.instance_custom_index
    }
    ///Gets a mutable reference to the value of [`Self::mask`]
    pub fn mask_mut(&mut self) -> &mut u32 {
        &mut self.mask
    }
    ///Gets a mutable reference to the value of
    /// [`Self::instance_shader_binding_table_record_offset`]
    pub fn instance_shader_binding_table_record_offset_mut(&mut self) -> &mut u32 {
        &mut self.instance_shader_binding_table_record_offset
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut GeometryInstanceFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure_reference`]
    pub fn acceleration_structure_reference_mut(&mut self) -> &mut u64 {
        &mut self.acceleration_structure_reference
    }
    ///Sets the raw value of [`Self::transform_t_0`]
    pub fn set_transform_t_0(&mut self, value: crate::extensions::nv_ray_tracing_motion_blur::SrtDataNV) -> &mut Self {
        self.transform_t_0 = value;
        self
    }
    ///Sets the raw value of [`Self::transform_t_1`]
    pub fn set_transform_t_1(&mut self, value: crate::extensions::nv_ray_tracing_motion_blur::SrtDataNV) -> &mut Self {
        self.transform_t_1 = value;
        self
    }
    ///Sets the raw value of [`Self::instance_custom_index`]
    pub fn set_instance_custom_index(&mut self, value: u32) -> &mut Self {
        self.instance_custom_index = value;
        self
    }
    ///Sets the raw value of [`Self::mask`]
    pub fn set_mask(&mut self, value: u32) -> &mut Self {
        self.mask = value;
        self
    }
    ///Sets the raw value of [`Self::instance_shader_binding_table_record_offset`]
    pub fn set_instance_shader_binding_table_record_offset(&mut self, value: u32) -> &mut Self {
        self.instance_shader_binding_table_record_offset = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure_reference`]
    pub fn set_acceleration_structure_reference(&mut self, value: u64) -> &mut Self {
        self.acceleration_structure_reference = value;
        self
    }
}
///[VkAccelerationStructureMatrixMotionInstanceNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMatrixMotionInstanceNV.html) - Structure specifying a single acceleration structure matrix motion instance for building into an acceleration structure geometry
///# C Specifications
///An acceleration structure matrix motion instance is defined by the
///structure:
///```c
///// Provided by VK_NV_ray_tracing_motion_blur
///typedef struct VkAccelerationStructureMatrixMotionInstanceNV {
///    VkTransformMatrixKHR          transformT0;
///    VkTransformMatrixKHR          transformT1;
///    uint32_t                      instanceCustomIndex:24;
///    uint32_t                      mask:8;
///    uint32_t                      instanceShaderBindingTableRecordOffset:24;
///    VkGeometryInstanceFlagsKHR    flags:8;
///    uint64_t                      accelerationStructureReference;
///} VkAccelerationStructureMatrixMotionInstanceNV;
///```
///# Members
/// - [`transform_t_0`] is a [`TransformMatrixKHR`] structure describing a transformation to be
///   applied to the acceleration structure at time 0.
/// - [`transform_t_1`] is a [`TransformMatrixKHR`] structure describing a transformation to be
///   applied to the acceleration structure at time 1.
/// - [`instance_custom_index`] is a 24-bit user-specified index value accessible to ray shaders in
///   the `InstanceCustomIndexKHR` built-in.
/// - [`mask`] is an 8-bit visibility mask for the geometry. The instance  **may**  only be hit if
///   `Cull Mask & instance.mask != 0`
/// - [`instance_shader_binding_table_record_offset`] is a 24-bit offset used in calculating the hit
///   shader binding table index.
/// - [`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`] values to apply to this
///   instance.
/// - [`acceleration_structure_reference`] is either:  - a device address containing the value
///   obtained from [`get_acceleration_structure_device_address_khr`] or
///   [`get_acceleration_structure_handle_nv`]      (used by device operations which reference
///   acceleration structures) or,  - a [`AccelerationStructureKHR`] object (used by host operations
///   which reference acceleration structures).
///# Description
///The C language specification does not define the ordering of bit-fields, but
///in practice, this struct produces the correct layout with existing
///compilers.
///The intended bit pattern is for the following:
/// - [`instance_custom_index`] and [`mask`] occupy the same memory as if a single `uint32_t` was
///   specified in their place  - [`instance_custom_index`] occupies the 24 least significant bits
///   of that memory  - [`mask`] occupies the 8 most significant bits of that memory
/// - [`instance_shader_binding_table_record_offset`] and [`flags`] occupy the same memory as if a
///   single `uint32_t` was specified in their place  -
///   [`instance_shader_binding_table_record_offset`] occupies the 24 least significant bits of that
///   memory  - [`flags`] occupies the 8 most significant bits of that memory
///If a compiler produces code that diverges from that pattern, applications
/// **must**  employ another method to set values according to the correct bit
///pattern.The transform for a matrix motion instance at a point in time is derived by
///component-wise linear interpolation of the two transforms.
///That is, for a `time` in [0,1] the resulting transform is
/// * [`transform_t_0`] × (1 - `time`) +  [`transform_t_1`] × `time`
///
///## Valid Usage (Implicit)
/// - [`flags`] **must**  be a valid combination of [`GeometryInstanceFlagBitsKHR`] values
///# Related
/// - [`VK_NV_ray_tracing_motion_blur`]
/// - [`AccelerationStructureMotionInstanceDataNV`]
/// - [`GeometryInstanceFlagsKHR`]
/// - [`TransformMatrixKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureMatrixMotionInstanceNV")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureMatrixMotionInstanceNV {
    ///[`transform_t_0`] is a [`TransformMatrixKHR`] structure describing a
    ///transformation to be applied to the acceleration structure at time 0.
    pub transform_t_0: TransformMatrixKHR,
    ///[`transform_t_1`] is a [`TransformMatrixKHR`] structure describing a
    ///transformation to be applied to the acceleration structure at time 1.
    pub transform_t_1: TransformMatrixKHR,
    ///[`instance_custom_index`] is a 24-bit user-specified index value
    ///accessible to ray shaders in the `InstanceCustomIndexKHR` built-in.
    pub instance_custom_index: u32,
    ///[`mask`] is an 8-bit visibility mask for the geometry.
    ///The instance  **may**  only be hit if `Cull Mask & instance.mask != 0`
    pub mask: u32,
    ///[`instance_shader_binding_table_record_offset`] is a 24-bit offset used in
    ///calculating the hit shader binding table index.
    pub instance_shader_binding_table_record_offset: u32,
    ///[`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`]
    ///values to apply to this instance.
    pub flags: GeometryInstanceFlagsKHR,
    ///[`acceleration_structure_reference`] is either:
    /// - a device address containing the value obtained from
    ///   [`get_acceleration_structure_device_address_khr`] or
    ///   [`get_acceleration_structure_handle_nv`]      (used by device operations which reference
    ///   acceleration structures) or,
    /// - a [`AccelerationStructureKHR`] object (used by host operations which reference
    ///   acceleration structures).
    pub acceleration_structure_reference: u64,
}
impl Default for AccelerationStructureMatrixMotionInstanceNV {
    fn default() -> Self {
        Self {
            transform_t_0: Default::default(),
            transform_t_1: Default::default(),
            instance_custom_index: 0,
            mask: 0,
            instance_shader_binding_table_record_offset: 0,
            flags: Default::default(),
            acceleration_structure_reference: 0,
        }
    }
}
impl AccelerationStructureMatrixMotionInstanceNV {
    ///Gets the value of [`Self::transform_t_0`]
    pub fn transform_t_0(&self) -> TransformMatrixKHR {
        self.transform_t_0
    }
    ///Gets the value of [`Self::transform_t_1`]
    pub fn transform_t_1(&self) -> TransformMatrixKHR {
        self.transform_t_1
    }
    ///Gets the value of [`Self::instance_custom_index`]
    pub fn instance_custom_index(&self) -> u32 {
        self.instance_custom_index
    }
    ///Gets the value of [`Self::mask`]
    pub fn mask(&self) -> u32 {
        self.mask
    }
    ///Gets the value of [`Self::instance_shader_binding_table_record_offset`]
    pub fn instance_shader_binding_table_record_offset(&self) -> u32 {
        self.instance_shader_binding_table_record_offset
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> GeometryInstanceFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::acceleration_structure_reference`]
    pub fn acceleration_structure_reference(&self) -> u64 {
        self.acceleration_structure_reference
    }
    ///Gets a mutable reference to the value of [`Self::transform_t_0`]
    pub fn transform_t_0_mut(&mut self) -> &mut TransformMatrixKHR {
        &mut self.transform_t_0
    }
    ///Gets a mutable reference to the value of [`Self::transform_t_1`]
    pub fn transform_t_1_mut(&mut self) -> &mut TransformMatrixKHR {
        &mut self.transform_t_1
    }
    ///Gets a mutable reference to the value of [`Self::instance_custom_index`]
    pub fn instance_custom_index_mut(&mut self) -> &mut u32 {
        &mut self.instance_custom_index
    }
    ///Gets a mutable reference to the value of [`Self::mask`]
    pub fn mask_mut(&mut self) -> &mut u32 {
        &mut self.mask
    }
    ///Gets a mutable reference to the value of
    /// [`Self::instance_shader_binding_table_record_offset`]
    pub fn instance_shader_binding_table_record_offset_mut(&mut self) -> &mut u32 {
        &mut self.instance_shader_binding_table_record_offset
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut GeometryInstanceFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure_reference`]
    pub fn acceleration_structure_reference_mut(&mut self) -> &mut u64 {
        &mut self.acceleration_structure_reference
    }
    ///Sets the raw value of [`Self::transform_t_0`]
    pub fn set_transform_t_0(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::TransformMatrixKHR,
    ) -> &mut Self {
        self.transform_t_0 = value;
        self
    }
    ///Sets the raw value of [`Self::transform_t_1`]
    pub fn set_transform_t_1(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::TransformMatrixKHR,
    ) -> &mut Self {
        self.transform_t_1 = value;
        self
    }
    ///Sets the raw value of [`Self::instance_custom_index`]
    pub fn set_instance_custom_index(&mut self, value: u32) -> &mut Self {
        self.instance_custom_index = value;
        self
    }
    ///Sets the raw value of [`Self::mask`]
    pub fn set_mask(&mut self, value: u32) -> &mut Self {
        self.mask = value;
        self
    }
    ///Sets the raw value of [`Self::instance_shader_binding_table_record_offset`]
    pub fn set_instance_shader_binding_table_record_offset(&mut self, value: u32) -> &mut Self {
        self.instance_shader_binding_table_record_offset = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure_reference`]
    pub fn set_acceleration_structure_reference(&mut self, value: u64) -> &mut Self {
        self.acceleration_structure_reference = value;
        self
    }
}
///[VkAccelerationStructureMotionInstanceNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceNV.html) - Structure specifying a single acceleration structure motion instance for building into an acceleration structure geometry
///# C Specifications
///*Acceleration structure motion instances* **can**  be built into top-level
///acceleration structures.
///Each acceleration structure instance is a separate entry in the top-level
///acceleration structure which includes all the geometry of a bottom-level
///acceleration structure at a transformed location including a type of motion
///and parameters to determine the motion of the instance over time.An acceleration structure
/// motion instance is defined by the structure:
///```c
///// Provided by VK_NV_ray_tracing_motion_blur
///typedef struct VkAccelerationStructureMotionInstanceNV {
///    VkAccelerationStructureMotionInstanceTypeNV     type;
///    VkAccelerationStructureMotionInstanceFlagsNV    flags;
///    VkAccelerationStructureMotionInstanceDataNV     data;
///} VkAccelerationStructureMotionInstanceNV;
///```
///# Members
/// - [`type_`] is a [`AccelerationStructureMotionInstanceTypeNV`] enumerant identifying which type
///   of motion instance this is and which type of the union is valid.
/// - [`flags`] is currently unused, but is required to keep natural alignment of [`data`].
/// - [`data`] is a [`AccelerationStructureMotionInstanceDataNV`] containing motion instance data
///   for this instance.
///# Description
///## Valid Usage (Implicit)
/// - [`type_`] **must**  be a valid [`AccelerationStructureMotionInstanceTypeNV`] value
/// - [`flags`] **must**  be `0`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV`, the
///   `staticInstance` member of [`data`] **must**  be a valid [`AccelerationStructureInstanceKHR`]
///   structure
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV`, the
///   `matrixMotionInstance` member of [`data`] **must**  be a valid
///   [`AccelerationStructureMatrixMotionInstanceNV`] structure
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV`, the
///   `srtMotionInstance` member of [`data`] **must**  be a valid
///   [`AccelerationStructureSrtMotionInstanceNV`] structure
///# Related
/// - [`VK_NV_ray_tracing_motion_blur`]
/// - [`AccelerationStructureMotionInstanceDataNV`]
/// - [`AccelerationStructureMotionInstanceFlagsNV`]
/// - [`AccelerationStructureMotionInstanceTypeNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureMotionInstanceNV")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AccelerationStructureMotionInstanceNV {
    ///[`type_`] is a [`AccelerationStructureMotionInstanceTypeNV`]
    ///enumerant identifying which type of motion instance this is and which
    ///type of the union is valid.
    pub type_: AccelerationStructureMotionInstanceTypeNV,
    ///[`flags`] is currently unused, but is required to keep natural
    ///alignment of [`data`].
    pub flags: AccelerationStructureMotionInstanceFlagsNV,
    ///[`data`] is a [`AccelerationStructureMotionInstanceDataNV`]
    ///containing motion instance data for this instance.
    pub data: AccelerationStructureMotionInstanceDataNV,
}
impl Default for AccelerationStructureMotionInstanceNV {
    fn default() -> Self {
        Self {
            type_: Default::default(),
            flags: Default::default(),
            data: unsafe { std::mem::zeroed() },
        }
    }
}
impl AccelerationStructureMotionInstanceNV {
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> AccelerationStructureMotionInstanceTypeNV {
        self.type_
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> AccelerationStructureMotionInstanceFlagsNV {
        self.flags
    }
    ///Gets the value of [`Self::data`]
    pub fn data(&self) -> AccelerationStructureMotionInstanceDataNV {
        self.data
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type_mut(&mut self) -> &mut AccelerationStructureMotionInstanceTypeNV {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut AccelerationStructureMotionInstanceFlagsNV {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::data`]
    pub fn data_mut(&mut self) -> &mut AccelerationStructureMotionInstanceDataNV {
        &mut self.data
    }
    ///Sets the raw value of [`Self::type_`]
    pub fn set_type_(
        &mut self,
        value: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceTypeNV,
    ) -> &mut Self {
        self.type_ = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceFlagsNV,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::data`]
    pub fn set_data(
        &mut self,
        value: crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceDataNV,
    ) -> &mut Self {
        self.data = value;
        self
    }
}
///[VkAccelerationStructureMotionInstanceDataNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceDataNV.html) - Union specifying a acceleration structure motion instance data for building into an acceleration structure geometry
///# C Specifications
///Acceleration structure motion instance is defined by the union:
///```c
///// Provided by VK_NV_ray_tracing_motion_blur
///typedef union VkAccelerationStructureMotionInstanceDataNV {
///    VkAccelerationStructureInstanceKHR               staticInstance;
///    VkAccelerationStructureMatrixMotionInstanceNV    matrixMotionInstance;
///    VkAccelerationStructureSRTMotionInstanceNV       srtMotionInstance;
///} VkAccelerationStructureMotionInstanceDataNV;
///```
///# Members
/// - [`static_instance`] is a [`AccelerationStructureInstanceKHR`] structure containing data for a
///   static instance.
/// - [`matrix_motion_instance`] is a [`AccelerationStructureMatrixMotionInstanceNV`] structure
///   containing data for a matrix motion instance.
/// - [`srt_motion_instance`] is a [`AccelerationStructureSrtMotionInstanceNV`] structure containing
///   data for an SRT motion instance.
///# Related
/// - [`VK_NV_ray_tracing_motion_blur`]
/// - [`AccelerationStructureInstanceKHR`]
/// - [`AccelerationStructureMatrixMotionInstanceNV`]
/// - [`AccelerationStructureMotionInstanceNV`]
/// - [`AccelerationStructureSrtMotionInstanceNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureMotionInstanceDataNV")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub union AccelerationStructureMotionInstanceDataNV {
    ///[`static_instance`] is a [`AccelerationStructureInstanceKHR`]
    ///structure containing data for a static instance.
    pub static_instance: AccelerationStructureInstanceKHR,
    ///[`matrix_motion_instance`] is a
    ///[`AccelerationStructureMatrixMotionInstanceNV`] structure containing
    ///data for a matrix motion instance.
    pub matrix_motion_instance: AccelerationStructureMatrixMotionInstanceNV,
    ///[`srt_motion_instance`] is a
    ///[`AccelerationStructureSrtMotionInstanceNV`] structure containing
    ///data for an SRT motion instance.
    pub srt_motion_instance: AccelerationStructureSrtMotionInstanceNV,
}
impl Default for AccelerationStructureMotionInstanceDataNV {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
