use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION")]
pub const EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME")]
pub const EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_conservative_rasterization");
///[VkConservativeRasterizationModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConservativeRasterizationModeEXT.html) - Specify the conservative rasterization mode
///# C Specifications
///Possible values of
///[`PipelineRasterizationConservativeStateCreateInfoEXT::conservative_rasterization_mode`],
///specifying the conservative rasterization mode are:
///```c
///// Provided by VK_EXT_conservative_rasterization
///typedef enum VkConservativeRasterizationModeEXT {
///    VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT = 0,
///    VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT = 1,
///    VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT = 2,
///} VkConservativeRasterizationModeEXT;
///```
///# Description
/// - [`ConservativeRasterizationModeDisabledExt`] specifies that conservative rasterization is
///   disabled and rasterization proceeds as normal.
/// - [`ConservativeRasterizationModeOverestimateExt`] specifies that conservative rasterization is
///   enabled in overestimation mode.
/// - [`ConservativeRasterizationModeUnderestimateExt`] specifies that conservative rasterization is
///   enabled in underestimation mode.
///# Related
/// - [`VK_EXT_conservative_rasterization`]
/// - [`PipelineRasterizationConservativeStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkConservativeRasterizationModeEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ConservativeRasterizationModeEXT {
    ///[`ConservativeRasterizationModeDisabledExt`] specifies that
    ///conservative rasterization is disabled and rasterization proceeds as
    ///normal.
    ConservativeRasterizationModeDisabledExt = 0,
    ///[`ConservativeRasterizationModeOverestimateExt`] specifies that
    ///conservative rasterization is enabled in overestimation mode.
    ConservativeRasterizationModeOverestimateExt = 1,
    ///[`ConservativeRasterizationModeUnderestimateExt`] specifies
    ///that conservative rasterization is enabled in underestimation mode.
    ConservativeRasterizationModeUnderestimateExt = 2,
}
impl const Default for ConservativeRasterizationModeEXT {
    fn default() -> Self {
        ConservativeRasterizationModeDisabledExt
    }
}
impl ConservativeRasterizationModeEXT {
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
///[VkPhysicalDeviceConservativeRasterizationPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConservativeRasterizationPropertiesEXT.html) - Structure describing conservative raster properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceConservativeRasterizationPropertiesEXT`] structure
///is defined as:
///```c
///// Provided by VK_EXT_conservative_rasterization
///typedef struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    float              primitiveOverestimationSize;
///    float              maxExtraPrimitiveOverestimationSize;
///    float              extraPrimitiveOverestimationSizeGranularity;
///    VkBool32           primitiveUnderestimation;
///    VkBool32           conservativePointAndLineRasterization;
///    VkBool32           degenerateTrianglesRasterized;
///    VkBool32           degenerateLinesRasterized;
///    VkBool32           fullyCoveredFragmentShaderInputVariable;
///    VkBool32           conservativeRasterizationPostDepthCoverage;
///} VkPhysicalDeviceConservativeRasterizationPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`primitive_overestimation_size`] is the size in pixels the generating primitive is increased
///   at each of its edges during conservative rasterization overestimation mode. Even with a size
///   of 0.0, conservative rasterization overestimation rules still apply and if any part of the
///   pixel rectangle is covered by the generating primitive, fragments are generated for the entire
///   pixel. However implementations **may** make the pixel coverage area even more conservative by
///   increasing the size of the generating primitive.
/// - [`max_extra_primitive_overestimation_size`] is the maximum size in pixels of extra
///   overestimation the implementation supports in the pipeline state. A value of 0.0 means the
///   implementation does not support any additional overestimation of the generating primitive
///   during conservative rasterization. A value above 0.0 allows the application to further
///   increase the size of the generating primitive during conservative rasterization
///   overestimation.
/// - [`extra_primitive_overestimation_size_granularity`] is the granularity of extra overestimation
///   that can be specified in the pipeline state between 0.0 and
///   [`max_extra_primitive_overestimation_size`] inclusive. A value of 0.0 means the implementation
///   can use the smallest representable non-zero value in the screen space pixel fixed-point grid.
/// - [`primitive_underestimation`] is [`TRUE`] if the implementation supports the
///   `VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT` conservative rasterization mode in
///   addition to `VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT`. Otherwise the
///   implementation only supports `VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT`.
/// - [`conservative_point_and_line_rasterization`] is [`TRUE`] if the implementation supports
///   conservative rasterization of point and line primitives as well as triangle primitives.
///   Otherwise the implementation only supports triangle primitives.
/// - [`degenerate_triangles_rasterized`] is [`FALSE`] if the implementation culls primitives
///   generated from triangles that become zero area after they are quantized to the fixed-point
///   rasterization pixel grid. [`degenerate_triangles_rasterized`] is [`TRUE`] if these primitives
///   are not culled and the provoking vertex attributes and depth value are used for the fragments.
///   The primitive area calculation is done on the primitive generated from the clipped triangle if
///   applicable. Zero area primitives are backfacing and the application **can** enable backface
///   culling if desired.
/// - [`degenerate_lines_rasterized`] is [`FALSE`] if the implementation culls lines that become
///   zero length after they are quantized to the fixed-point rasterization pixel grid.
///   [`degenerate_lines_rasterized`] is [`TRUE`] if zero length lines are not culled and the
///   provoking vertex attributes and depth value are used for the fragments.
/// - [`fully_covered_fragment_shader_input_variable`] is [`TRUE`] if the implementation supports
///   the SPIR-V builtin fragment shader input variable `FullyCoveredEXT` specifying that
///   conservative rasterization is enabled and the fragment area is fully covered by the generating
///   primitive.
/// - [`conservative_rasterization_post_depth_coverage`] is [`TRUE`] if the implementation supports
///   conservative rasterization with the `PostDepthCoverage` execution mode enabled. Otherwise the
///   `PostDepthCoverage` execution mode **must** not be used when conservative rasterization is
///   enabled.
///# Description
///If the [`PhysicalDeviceConservativeRasterizationPropertiesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_conservative_rasterization`]
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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`primitive_overestimation_size`]
    ///is the size in pixels the generating primitive is increased at each of
    ///its edges during conservative rasterization overestimation mode.
    ///Even with a size of 0.0, conservative rasterization overestimation rules
    ///still apply and if any part of the pixel rectangle is covered by the
    ///generating primitive, fragments are generated for the entire pixel.
    ///However implementations **may** make the pixel coverage area even more
    ///conservative by increasing the size of the generating primitive.
    primitive_overestimation_size: f32,
    ///[`max_extra_primitive_overestimation_size`] is the maximum size in pixels
    ///of extra overestimation the implementation supports in the pipeline
    ///state.
    ///A value of 0.0 means the implementation does not support any additional
    ///overestimation of the generating primitive during conservative
    ///rasterization.
    ///A value above 0.0 allows the application to further increase the size of
    ///the generating primitive during conservative rasterization
    ///overestimation.
    max_extra_primitive_overestimation_size: f32,
    ///[`extra_primitive_overestimation_size_granularity`] is the granularity of
    ///extra overestimation that can be specified in the pipeline state between
    ///0.0 and [`max_extra_primitive_overestimation_size`] inclusive.
    ///A value of 0.0 means the implementation can use the smallest
    ///representable non-zero value in the screen space pixel fixed-point grid.
    extra_primitive_overestimation_size_granularity: f32,
    ///[`primitive_underestimation`] is
    ///[`TRUE`] if the implementation supports the
    ///`VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT` conservative
    ///rasterization mode in addition to
    ///`VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT`.
    ///Otherwise the implementation only supports
    ///`VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT`.
    primitive_underestimation: Bool32,
    ///[`conservative_point_and_line_rasterization`] is [`TRUE`] if the
    ///implementation supports conservative rasterization of point and line
    ///primitives as well as triangle primitives.
    ///Otherwise the implementation only supports triangle primitives.
    conservative_point_and_line_rasterization: Bool32,
    ///[`degenerate_triangles_rasterized`] is [`FALSE`] if the
    ///implementation culls primitives generated from triangles that become
    ///zero area after they are quantized to the fixed-point rasterization
    ///pixel grid.
    ///[`degenerate_triangles_rasterized`] is [`TRUE`] if these primitives
    ///are not culled and the provoking vertex attributes and depth value are
    ///used for the fragments.
    ///The primitive area calculation is done on the primitive generated from
    ///the clipped triangle if applicable.
    ///Zero area primitives are backfacing and the application **can** enable
    ///backface culling if desired.
    degenerate_triangles_rasterized: Bool32,
    ///[`degenerate_lines_rasterized`] is
    ///[`FALSE`] if the implementation culls lines that become zero length
    ///after they are quantized to the fixed-point rasterization pixel grid.
    ///[`degenerate_lines_rasterized`] is [`TRUE`] if zero length lines
    ///are not culled and the provoking vertex attributes and depth value are
    ///used for the fragments.
    degenerate_lines_rasterized: Bool32,
    ///[`fully_covered_fragment_shader_input_variable`] is [`TRUE`] if the
    ///implementation supports the SPIR-V builtin fragment shader input
    ///variable `FullyCoveredEXT` specifying that conservative rasterization
    ///is enabled and the fragment area is fully covered by the generating
    ///primitive.
    fully_covered_fragment_shader_input_variable: Bool32,
    ///[`conservative_rasterization_post_depth_coverage`] is [`TRUE`] if the
    ///implementation supports conservative rasterization with the
    ///`PostDepthCoverage` execution mode enabled.
    ///Otherwise the `PostDepthCoverage` execution mode **must** not be used
    ///when conservative rasterization is enabled.
    conservative_rasterization_post_depth_coverage: Bool32,
}
///[VkPipelineRasterizationConservativeStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationConservativeStateCreateInfoEXT.html) - Structure specifying conservative raster state
///# C Specifications
///Polygon rasterization **can** be made conservative by setting
///[`conservative_rasterization_mode`] to
///`VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT` or
///`VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT` in
///[`PipelineRasterizationConservativeStateCreateInfoEXT`].
///The [`PipelineRasterizationConservativeStateCreateInfoEXT`] state is set
///by adding this structure to the [`p_next`] chain of a
///[`PipelineRasterizationStateCreateInfo`] structure when creating the
///graphics pipeline.
///Enabling these modes also affects line and point rasterization if the
///implementation sets
///[`PhysicalDeviceConservativeRasterizationPropertiesEXT::
/// conservative_point_and_line_rasterization`]
///to [`TRUE`].[`PipelineRasterizationConservativeStateCreateInfoEXT`] is defined as:
///```c
///// Provided by VK_EXT_conservative_rasterization
///typedef struct VkPipelineRasterizationConservativeStateCreateInfoEXT {
///    VkStructureType                                           sType;
///    const void*                                               pNext;
///    VkPipelineRasterizationConservativeStateCreateFlagsEXT    flags;
///    VkConservativeRasterizationModeEXT                        conservativeRasterizationMode;
///    float                                                     extraPrimitiveOverestimationSize;
///} VkPipelineRasterizationConservativeStateCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`conservative_rasterization_mode`] is the conservative rasterization mode to use.
/// - [`extra_primitive_overestimation_size`] is the extra size in pixels to increase the generating
///   primitive during conservative rasterization at each of its edges in `X` and `Y` equally in
///   screen space beyond the base overestimation specified in
///   [`PhysicalDeviceConservativeRasterizationPropertiesEXT::primitive_overestimation_size`].
///# Description
///Valid Usage
/// - [`extra_primitive_overestimation_size`]**must** be in the range of `0.0` to
///   [`PhysicalDeviceConservativeRasterizationPropertiesEXT::
///   max_extra_primitive_overestimation_size`] inclusive
///Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT`
/// - [`flags`]**must** be `0`
/// - [`conservative_rasterization_mode`]**must** be a valid [`ConservativeRasterizationModeEXT`]
///   value
///# Related
/// - [`VK_EXT_conservative_rasterization`]
/// - [`ConservativeRasterizationModeEXT`]
/// - [`PipelineRasterizationConservativeStateCreateFlagsEXT`]
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
pub struct PipelineRasterizationConservativeStateCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
    ///[`conservative_rasterization_mode`] is the conservative rasterization
    ///mode to use.
    conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ///[`extra_primitive_overestimation_size`] is the extra size in pixels to
    ///increase the generating primitive during conservative rasterization at
    ///each of its edges in `X` and `Y` equally in screen space beyond the base
    ///overestimation specified in
    ///[`PhysicalDeviceConservativeRasterizationPropertiesEXT`]::`primitiveOverestimationSize`.
    extra_primitive_overestimation_size: f32,
}
