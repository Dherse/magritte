#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
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
#[doc(alias = "VK_SHADER_UNUSED_KHR")]
pub const SHADER_UNUSED_KHR: u32 = !0;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_SPEC_VERSION")]
pub const KHR_RAY_TRACING_PIPELINE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME")]
pub const KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_ray_tracing_pipeline");
///[VkRayTracingShaderGroupTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html) - Shader group types
///# C Specifications
///Possible values of `type` in [`RayTracingShaderGroupCreateInfoKHR`]
///are:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///typedef enum VkRayTracingShaderGroupTypeKHR {
///    VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR = 0,
///    VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR = 1,
///    VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR = 2,
///  // Provided by VK_NV_ray_tracing
///    VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV = VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV =
/// VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV =
/// VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR,
///} VkRayTracingShaderGroupTypeKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkRayTracingShaderGroupTypeKHR VkRayTracingShaderGroupTypeNV;
///```
///# Description
/// - [`RAY_TRACING_SHADER_GROUP_TYPE_GENERAL`] indicates a shader
///group with a single `VK_SHADER_STAGE_RAYGEN_BIT_KHR`,
///`VK_SHADER_STAGE_MISS_BIT_KHR`, or
///`VK_SHADER_STAGE_CALLABLE_BIT_KHR` shader in it.
/// - [`RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP`] specifies
///a shader group that only hits triangles and **must** not contain an
///intersection shader, only closest hit and any-hit shaders.
/// - [`RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP`]
///specifies a shader group that only intersects with custom geometry and
///**must** contain an intersection shader and **may** contain closest hit and
///any-hit shaders.
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
/// - [`VK_NV_ray_tracing`]
/// - [`RayTracingShaderGroupCreateInfoKHR`]
/// - [`RayTracingShaderGroupCreateInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRayTracingShaderGroupTypeKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RayTracingShaderGroupTypeKHR(i32);
impl const Default for RayTracingShaderGroupTypeKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for RayTracingShaderGroupTypeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("RayTracingShaderGroupTypeKHR")
            .field(match *self {
                Self::RAY_TRACING_SHADER_GROUP_TYPE_GENERAL => &"RAY_TRACING_SHADER_GROUP_TYPE_GENERAL",
                Self::RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP => {
                    &"RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP"
                },
                Self::RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP => {
                    &"RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP"
                },
                other => unreachable!("invalid value for `RayTracingShaderGroupTypeKHR`: {:?}", other),
            })
            .finish()
    }
}
impl RayTracingShaderGroupTypeKHR {
    ///[`RAY_TRACING_SHADER_GROUP_TYPE_GENERAL`] indicates a shader
    ///group with a single `VK_SHADER_STAGE_RAYGEN_BIT_KHR`,
    ///`VK_SHADER_STAGE_MISS_BIT_KHR`, or
    ///`VK_SHADER_STAGE_CALLABLE_BIT_KHR` shader in it.
    pub const RAY_TRACING_SHADER_GROUP_TYPE_GENERAL: Self = Self(0);
    ///[`RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP`] specifies
    ///a shader group that only hits triangles and **must** not contain an
    ///intersection shader, only closest hit and any-hit shaders.
    pub const RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP: Self = Self(1);
    ///[`RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP`]
    ///specifies a shader group that only intersects with custom geometry and
    ///**must** contain an intersection shader and **may** contain closest hit and
    ///any-hit shaders.
    pub const RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP: Self = Self(2);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV: Self = Self::VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV: Self =
        Self::VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV: Self =
        Self::VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR;
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
///[VkShaderGroupShaderKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderGroupShaderKHR.html) - Shader group shaders
///# C Specifications
///Possible values of `groupShader` in
///[`GetRayTracingShaderGroupStackSizeKHR`] are:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///typedef enum VkShaderGroupShaderKHR {
///    VK_SHADER_GROUP_SHADER_GENERAL_KHR = 0,
///    VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR = 1,
///    VK_SHADER_GROUP_SHADER_ANY_HIT_KHR = 2,
///    VK_SHADER_GROUP_SHADER_INTERSECTION_KHR = 3,
///} VkShaderGroupShaderKHR;
///```
///# Description
/// - [`SHADER_GROUP_SHADER_GENERAL`] uses the shader specified in
///the group with
///[`RayTracingShaderGroupCreateInfoKHR::general_shader`]
/// - [`SHADER_GROUP_SHADER_CLOSEST_HIT`] uses the shader specified
///in the group with
///[`RayTracingShaderGroupCreateInfoKHR::closest_hit_shader`]
/// - [`SHADER_GROUP_SHADER_ANY_HIT`] uses the shader specified in
///the group with
///[`RayTracingShaderGroupCreateInfoKHR::any_hit_shader`]
/// - [`SHADER_GROUP_SHADER_INTERSECTION`] uses the shader specified
///in the group with
///[`RayTracingShaderGroupCreateInfoKHR::intersection_shader`]
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
/// - [`GetRayTracingShaderGroupStackSizeKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkShaderGroupShaderKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ShaderGroupShaderKHR(i32);
impl const Default for ShaderGroupShaderKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ShaderGroupShaderKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ShaderGroupShaderKHR")
            .field(match *self {
                Self::SHADER_GROUP_SHADER_GENERAL => &"SHADER_GROUP_SHADER_GENERAL",
                Self::SHADER_GROUP_SHADER_CLOSEST_HIT => &"SHADER_GROUP_SHADER_CLOSEST_HIT",
                Self::SHADER_GROUP_SHADER_ANY_HIT => &"SHADER_GROUP_SHADER_ANY_HIT",
                Self::SHADER_GROUP_SHADER_INTERSECTION => &"SHADER_GROUP_SHADER_INTERSECTION",
                other => unreachable!("invalid value for `ShaderGroupShaderKHR`: {:?}", other),
            })
            .finish()
    }
}
impl ShaderGroupShaderKHR {
    ///[`SHADER_GROUP_SHADER_GENERAL`] uses the shader specified in
    ///the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`generalShader`
    pub const SHADER_GROUP_SHADER_GENERAL: Self = Self(0);
    ///[`SHADER_GROUP_SHADER_CLOSEST_HIT`] uses the shader specified
    ///in the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`closestHitShader`
    pub const SHADER_GROUP_SHADER_CLOSEST_HIT: Self = Self(1);
    ///[`SHADER_GROUP_SHADER_ANY_HIT`] uses the shader specified in
    ///the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`anyHitShader`
    pub const SHADER_GROUP_SHADER_ANY_HIT: Self = Self(2);
    ///[`SHADER_GROUP_SHADER_INTERSECTION`] uses the shader specified
    ///in the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`intersectionShader`
    pub const SHADER_GROUP_SHADER_INTERSECTION: Self = Self(3);
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
