use crate::{
    extensions::khr_acceleration_structure::{
        DeviceOrHostAddressConstKHR, GeometryInstanceFlagsKHR, TransformMatrixKHR,
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
        AccelerationStructureMotionInstanceTypeStaticNv
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
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
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
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceRayTracingMotionBlurFeaturesNV`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`ray_tracing_motion_blur`] indicates
    ///whether the implementation supports the motion blur feature.
    ray_tracing_motion_blur: Bool32,
    ///[`ray_tracing_motion_blur_pipeline_trace_rays_indirect`] indicates whether
    ///the implementation supports indirect ray tracing commands with the
    ///motion blur feature enabled.
    ray_tracing_motion_blur_pipeline_trace_rays_indirect: Bool32,
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
/// vertex position data.Valid Usage (Implicit)
/// - [`s_type`]**must** be
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
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureGeometryMotionTrianglesDataNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`vertex_data`] is a pointer to vertex data for this geometry at time
    ///1.0
    vertex_data: DeviceOrHostAddressConstKHR<'lt>,
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
/// - [`max_instances`] is the maximum number of instances that **may** be used in the motion
///   top-level acceleration structure.
/// - [`flags`] is 0 and reserved for future use.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV`
/// - [`flags`]**must** be `0`
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureMotionInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`max_instances`] is the maximum number of instances that **may** be used
    ///in the motion top-level acceleration structure.
    max_instances: u32,
    ///[`flags`] is 0 and reserved for future use.
    flags: AccelerationStructureMotionInfoFlagsNV,
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
/// class="katex-html" aria-hidden="true"><span class="base"><span
/// style="height:0.68333em;vertical-align:0em;" class="strut"></span><span class="mord mathdefault"
/// style="margin-right:0.05764em;">S</span><span class="mspace"
/// style="margin-right:0.2777777777777778em;"></span><span class="mrel">=</span><span
/// style="margin-right:0.2777777777777778em;" class="mspace"></span></span><span class="base"><span
/// style="height:3.60004em;vertical-align:-1.55002em;" class="strut"></span><span
/// class="minner"><span class="mopen"><span class="delimsizing mult"><span class="vlist-t
/// vlist-t2"><span class="vlist-r"><span class="vlist" style="height:2.05002em;"><span
/// style="top:-2.2500000000000004em;"><span style="height:3.1550000000000002em;"
/// class="pstrut"></span><span class="delimsizinginner
/// delim-size4"><span>⎝</span></span></span><span style="top:-2.8100000000000005em;"><span
/// class="pstrut" style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎜</span></span></span><span style="top:-4.05002em;"><span
/// style="height:3.1550000000000002em;" class="pstrut"></span><span class="delimsizinginner
/// delim-size4"><span>⎛</span></span></span></span><span class="vlist-s">​</span></span><span
/// class="vlist-r"><span class="vlist"
/// style="height:1.55002em;"><span></span></span></span></span></span></span><span
/// class="mord"><span class="mtable"><span class="col-align-c"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span class="vlist" style="height:2.05em;"><span style="top:-4.21em;"><span
/// class="pstrut" style="height:3em;"></span><span class="mord"><span class="mord
/// mathdefault">s</span><span class="mord mathdefault">x</span></span></span><span
/// style="top:-3.0099999999999993em;"><span class="pstrut" style="height:3em;"></span><span
/// class="mord"><span class="mord">0</span></span></span><span
/// style="top:-1.8099999999999994em;"><span style="height:3em;" class="pstrut"></span><span
/// class="mord"><span class="mord">0</span></span></span></span><span
/// class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.5500000000000007em;"
/// class="vlist"><span></span></span></span></span></span><span style="width:0.5em;"
/// class="arraycolsep"></span><span class="arraycolsep" style="width:0.5em;"></span><span
/// class="col-align-c"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist"
/// style="height:2.05em;"><span style="top:-4.21em;"><span class="pstrut"
/// style="height:3em;"></span><span class="mord"><span class="mord
/// mathdefault">a</span></span></span><span style="top:-3.0099999999999993em;"><span class="pstrut"
/// style="height:3em;"></span><span class="mord"><span class="mord mathdefault">s</span><span
/// class="mord mathdefault" style="margin-right:0.03588em;">y</span></span></span><span
/// style="top:-1.8099999999999994em;"><span style="height:3em;" class="pstrut"></span><span
/// class="mord"><span class="mord">0</span></span></span></span><span
/// class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
/// style="height:1.5500000000000007em;"><span></span></span></span></span></span><span
/// style="width:0.5em;" class="arraycolsep"></span><span style="width:0.5em;"
/// class="arraycolsep"></span><span class="col-align-c"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span class="vlist" style="height:2.05em;"><span style="top:-4.21em;"><span
/// style="height:3em;" class="pstrut"></span><span class="mord"><span class="mord
/// mathdefault">b</span></span></span><span style="top:-3.0099999999999993em;"><span class="pstrut"
/// style="height:3em;"></span><span class="mord"><span class="mord
/// mathdefault">c</span></span></span><span style="top:-1.8099999999999994em;"><span
/// style="height:3em;" class="pstrut"></span><span class="mord"><span class="mord
/// mathdefault">s</span><span style="margin-right:0.04398em;" class="mord
/// mathdefault">z</span></span></span></span><span class="vlist-s">​</span></span><span
/// class="vlist-r"><span class="vlist"
/// style="height:1.5500000000000007em;"><span></span></span></span></span></span><span
/// class="arraycolsep" style="width:0.5em;"></span><span style="width:0.5em;"
/// class="arraycolsep"></span><span class="col-align-c"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span style="height:2.05em;" class="vlist"><span style="top:-4.21em;"><span
/// style="height:3em;" class="pstrut"></span><span class="mord"><span class="mord
/// mathdefault">p</span><span style="margin-right:0.03588em;" class="mord
/// mathdefault">v</span><span class="mord mathdefault">x</span></span></span><span
/// style="top:-3.0099999999999993em;"><span class="pstrut" style="height:3em;"></span><span
/// class="mord"><span class="mord mathdefault">p</span><span class="mord mathdefault"
/// style="margin-right:0.03588em;">v</span><span class="mord mathdefault"
/// style="margin-right:0.03588em;">y</span></span></span><span
/// style="top:-1.8099999999999994em;"><span style="height:3em;" class="pstrut"></span><span
/// class="mord"><span class="mord mathdefault">p</span><span style="margin-right:0.03588em;"
/// class="mord mathdefault">v</span><span style="margin-right:0.04398em;" class="mord
/// mathdefault">z</span></span></span></span><span class="vlist-s">​</span></span><span
/// class="vlist-r"><span style="height:1.5500000000000007em;"
/// class="vlist"><span></span></span></span></span></span></span></span><span class="mclose"><span
/// class="delimsizing mult"><span class="vlist-t vlist-t2"><span class="vlist-r"><span
/// style="height:2.05002em;" class="vlist"><span style="top:-2.2500000000000004em;"><span
/// style="height:3.1550000000000002em;" class="pstrut"></span><span class="delimsizinginner
/// delim-size4"><span>⎠</span></span></span><span style="top:-2.8100000000000005em;"><span
/// class="pstrut" style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎟</span></span></span><span style="top:-4.05002em;"><span class="pstrut"
/// style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎞</span></span></span></span><span class="vlist-s">​</span></span><span
/// class="vlist-r"><span class="vlist"
/// style="height:1.55002em;"><span></span></span></span></span></span></span></span></span></
/// span></span>The rotation quaternion is defined as:
/// * `R` = [ [`qx`], [`qy`], [`qz`], [`qw`] ]
///This is a rotation around a conceptual normalized axis [ ax, ay, az ]
///of amount `theta` such that:
/// * [ [`qx`], [`qy`], [`qz`] ] = sin(`theta`/2) × [ `ax`, `ay`, `az` ]
///and
/// * [`qw`] = cos(`theta`/2)
///Finally, the transform has a translation T constructed from the parameters
///above by:<span class="katex"><span aria-hidden="true" class="katex-html"><span
/// class="base"><span class="strut" style="height:0.68333em;vertical-align:0em;"></span><span
/// class="mord mathdefault" style="margin-right:0.13889em;">T</span><span
/// style="margin-right:0.2777777777777778em;" class="mspace"></span><span
/// class="mrel">=</span><span class="mspace"
/// style="margin-right:0.2777777777777778em;"></span></span><span class="base"><span class="strut"
/// style="height:3.60004em;vertical-align:-1.55002em;"></span><span class="minner"><span
/// class="mopen"><span class="delimsizing mult"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span style="height:2.05002em;" class="vlist"><span
/// style="top:-2.2500000000000004em;"><span class="pstrut"
/// style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎝</span></span></span><span style="top:-2.8100000000000005em;"><span
/// style="height:3.1550000000000002em;" class="pstrut"></span><span class="delimsizinginner
/// delim-size4"><span>⎜</span></span></span><span style="top:-4.05002em;"><span class="pstrut"
/// style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎛</span></span></span></span><span class="vlist-s">​</span></span><span
/// class="vlist-r"><span class="vlist"
/// style="height:1.55002em;"><span></span></span></span></span></span></span><span
/// class="mord"><span class="mtable"><span class="col-align-c"><span class="vlist-t vlist-t2"><span
/// class="vlist-r"><span class="vlist" style="height:2.05em;"><span style="top:-4.21em;"><span
/// style="height:3em;" class="pstrut"></span><span class="mord"><span
/// class="mord">1</span></span></span><span style="top:-3.0099999999999993em;"><span class="pstrut"
/// style="height:3em;"></span><span class="mord"><span class="mord">0</span></span></span><span
/// style="top:-1.8099999999999994em;"><span class="pstrut" style="height:3em;"></span><span
/// class="mord"><span class="mord">0</span></span></span></span><span
/// class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.5500000000000007em;"
/// class="vlist"><span></span></span></span></span></span><span class="arraycolsep"
/// style="width:0.5em;"></span><span class="arraycolsep" style="width:0.5em;"></span><span
/// class="col-align-c"><span class="vlist-t vlist-t2"><span class="vlist-r"><span
/// style="height:2.05em;" class="vlist"><span style="top:-4.21em;"><span class="pstrut"
/// style="height:3em;"></span><span class="mord"><span class="mord">0</span></span></span><span
/// style="top:-3.0099999999999993em;"><span class="pstrut" style="height:3em;"></span><span
/// class="mord"><span class="mord">1</span></span></span><span
/// style="top:-1.8099999999999994em;"><span class="pstrut" style="height:3em;"></span><span
/// class="mord"><span class="mord">0</span></span></span></span><span
/// class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.5500000000000007em;"
/// class="vlist"><span></span></span></span></span></span><span style="width:0.5em;"
/// class="arraycolsep"></span><span style="width:0.5em;" class="arraycolsep"></span><span
/// class="col-align-c"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist"
/// style="height:2.05em;"><span style="top:-4.21em;"><span style="height:3em;"
/// class="pstrut"></span><span class="mord"><span class="mord">0</span></span></span><span
/// style="top:-3.0099999999999993em;"><span class="pstrut" style="height:3em;"></span><span
/// class="mord"><span class="mord">0</span></span></span><span
/// style="top:-1.8099999999999994em;"><span style="height:3em;" class="pstrut"></span><span
/// class="mord"><span class="mord">1</span></span></span></span><span
/// class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
/// style="height:1.5500000000000007em;"><span></span></span></span></span></span><span
/// class="arraycolsep" style="width:0.5em;"></span><span style="width:0.5em;"
/// class="arraycolsep"></span><span class="col-align-c"><span class="vlist-t vlist-t2"><span
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
/// class="vlist" style="height:2.05002em;"><span style="top:-2.2500000000000004em;"><span
/// class="pstrut" style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎠</span></span></span><span style="top:-2.8100000000000005em;"><span
/// class="pstrut" style="height:3.1550000000000002em;"></span><span class="delimsizinginner
/// delim-size4"><span>⎟</span></span></span><span style="top:-4.05002em;"><span
/// style="height:3.1550000000000002em;" class="pstrut"></span><span class="delimsizinginner
/// delim-size4"><span>⎞</span></span></span></span><span class="vlist-s">​</span></span><span
/// class="vlist-r"><span class="vlist"
/// style="height:1.55002em;"><span></span></span></span></span></span></span></span></span></
/// span></span>The effective derived transform is then given by
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
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SrtDataNV {
    ///[`sx`] is the x component of the scale of the transform
    sx: f32,
    ///[`a`] is one component of the shear for the transform
    a: f32,
    ///[`b`] is one component of the shear for the transform
    b: f32,
    ///[`pvx`] is the x component of the pivot point of the transform
    pvx: f32,
    ///[`sy`] is the y component of the scale of the transform
    sy: f32,
    ///[`c`] is one component of the shear for the transform
    c: f32,
    ///[`pvy`] is the y component of the pivot point of the transform
    pvy: f32,
    ///[`sz`] is the z component of the scale of the transform
    sz: f32,
    ///[`pvz`] is the z component of the pivot point of the transform
    pvz: f32,
    ///[`qx`] is the x component of the rotation quaternion
    qx: f32,
    ///[`qy`] is the y component of the rotation quaternion
    qy: f32,
    ///[`qz`] is the z component of the rotation quaternion
    qz: f32,
    ///[`qw`] is the w component of the rotation quaternion
    qw: f32,
    ///[`tx`] is the x component of the post-rotation translation
    tx: f32,
    ///[`ty`] is the y component of the post-rotation translation
    ty: f32,
    ///[`tz`] is the z component of the post-rotation translation
    tz: f32,
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
/// - [`mask`] is an 8-bit visibility mask for the geometry. The instance **may** only be hit if
///   `Cull Mask & instance.mask != 0`
/// - [`instance_shader_binding_table_record_offset`] is a 24-bit offset used in calculating the hit
///   shader binding table index.
/// - [`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`] values to apply to this
///   instance.
/// - [`acceleration_structure_reference`] is either:  - a device address containing the value
///   obtained from [`GetAccelerationStructureDeviceAddressKHR`] or
///   [`GetAccelerationStructureHandleNV`]      (used by device operations which reference
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
///**must** employ another method to set values according to the correct bit
///pattern.The transform for a SRT motion instance at a point in time is derived from
///component-wise linear interpolation of the two SRT transforms.
///That is, for a `time` in [0,1] the resulting transform is
/// * [`transform_t_0`] × (1 - `time`) +  [`transform_t_1`] × `time`
///Valid Usage (Implicit)
/// - [`flags`]**must** be a valid combination of [`GeometryInstanceFlagBitsKHR`] values
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
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureSrtMotionInstanceNV {
    ///[`transform_t_0`] is a [`SrtDataNV`] structure describing a
    ///transformation to be applied to the acceleration structure at time 0.
    transform_t_0: SrtDataNV,
    ///[`transform_t_1`] is a [`SrtDataNV`] structure describing a
    ///transformation to be applied to the acceleration structure at time 1.
    transform_t_1: SrtDataNV,
    ///[`instance_custom_index`] is a 24-bit user-specified index value
    ///accessible to ray shaders in the `InstanceCustomIndexKHR` built-in.
    instance_custom_index: u32,
    ///[`mask`] is an 8-bit visibility mask for the geometry.
    ///The instance **may** only be hit if `Cull Mask & instance.mask != 0`
    mask: u32,
    ///[`instance_shader_binding_table_record_offset`] is a 24-bit offset used in
    ///calculating the hit shader binding table index.
    instance_shader_binding_table_record_offset: u32,
    ///[`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`]
    ///values to apply to this instance.
    flags: GeometryInstanceFlagsKHR,
    ///[`acceleration_structure_reference`] is either:
    /// - a device address containing the value obtained from
    ///   [`GetAccelerationStructureDeviceAddressKHR`] or [`GetAccelerationStructureHandleNV`]
    ///   (used by device operations which reference acceleration structures) or,
    /// - a [`AccelerationStructureKHR`] object (used by host operations which reference
    ///   acceleration structures).
    acceleration_structure_reference: u64,
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
/// - [`mask`] is an 8-bit visibility mask for the geometry. The instance **may** only be hit if
///   `Cull Mask & instance.mask != 0`
/// - [`instance_shader_binding_table_record_offset`] is a 24-bit offset used in calculating the hit
///   shader binding table index.
/// - [`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`] values to apply to this
///   instance.
/// - [`acceleration_structure_reference`] is either:  - a device address containing the value
///   obtained from [`GetAccelerationStructureDeviceAddressKHR`] or
///   [`GetAccelerationStructureHandleNV`]      (used by device operations which reference
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
///**must** employ another method to set values according to the correct bit
///pattern.The transform for a matrix motion instance at a point in time is derived by
///component-wise linear interpolation of the two transforms.
///That is, for a `time` in [0,1] the resulting transform is
/// * [`transform_t_0`] × (1 - `time`) +  [`transform_t_1`] × `time`
///Valid Usage (Implicit)
/// - [`flags`]**must** be a valid combination of [`GeometryInstanceFlagBitsKHR`] values
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
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureMatrixMotionInstanceNV {
    ///[`transform_t_0`] is a [`TransformMatrixKHR`] structure describing a
    ///transformation to be applied to the acceleration structure at time 0.
    transform_t_0: TransformMatrixKHR,
    ///[`transform_t_1`] is a [`TransformMatrixKHR`] structure describing a
    ///transformation to be applied to the acceleration structure at time 1.
    transform_t_1: TransformMatrixKHR,
    ///[`instance_custom_index`] is a 24-bit user-specified index value
    ///accessible to ray shaders in the `InstanceCustomIndexKHR` built-in.
    instance_custom_index: u32,
    ///[`mask`] is an 8-bit visibility mask for the geometry.
    ///The instance **may** only be hit if `Cull Mask & instance.mask != 0`
    mask: u32,
    ///[`instance_shader_binding_table_record_offset`] is a 24-bit offset used in
    ///calculating the hit shader binding table index.
    instance_shader_binding_table_record_offset: u32,
    ///[`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`]
    ///values to apply to this instance.
    flags: GeometryInstanceFlagsKHR,
    ///[`acceleration_structure_reference`] is either:
    /// - a device address containing the value obtained from
    ///   [`GetAccelerationStructureDeviceAddressKHR`] or [`GetAccelerationStructureHandleNV`]
    ///   (used by device operations which reference acceleration structures) or,
    /// - a [`AccelerationStructureKHR`] object (used by host operations which reference
    ///   acceleration structures).
    acceleration_structure_reference: u64,
}
///[VkAccelerationStructureMotionInstanceNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceNV.html) - Structure specifying a single acceleration structure motion instance for building into an acceleration structure geometry
///# C Specifications
///*Acceleration structure motion instances***can** be built into top-level
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
///Valid Usage (Implicit)
/// - [`type_`]**must** be a valid [`AccelerationStructureMotionInstanceTypeNV`] value
/// - [`flags`]**must** be `0`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV`, the
///   `staticInstance` member of [`data`]**must** be a valid [`AccelerationStructureInstanceKHR`]
///   structure
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV`, the
///   `matrixMotionInstance` member of [`data`]**must** be a valid
///   [`AccelerationStructureMatrixMotionInstanceNV`] structure
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV`, the
///   `srtMotionInstance` member of [`data`]**must** be a valid
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
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureMotionInstanceNV {
    ///[`type_`] is a [`AccelerationStructureMotionInstanceTypeNV`]
    ///enumerant identifying which type of motion instance this is and which
    ///type of the union is valid.
    type_: AccelerationStructureMotionInstanceTypeNV,
    ///[`flags`] is currently unused, but is required to keep natural
    ///alignment of [`data`].
    flags: AccelerationStructureMotionInstanceFlagsNV,
    ///[`data`] is a [`AccelerationStructureMotionInstanceDataNV`]
    ///containing motion instance data for this instance.
    data: AccelerationStructureMotionInstanceDataNV,
}
