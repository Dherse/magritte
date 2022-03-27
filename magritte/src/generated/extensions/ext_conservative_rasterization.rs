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
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
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
impl<'lt> Default for PhysicalDeviceConservativeRasterizationPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            primitive_overestimation_size: 0.0,
            max_extra_primitive_overestimation_size: 0.0,
            extra_primitive_overestimation_size_granularity: 0.0,
            primitive_underestimation: 0,
            conservative_point_and_line_rasterization: 0,
            degenerate_triangles_rasterized: 0,
            degenerate_lines_rasterized: 0,
            fully_covered_fragment_shader_input_variable: 0,
            conservative_rasterization_post_depth_coverage: 0,
        }
    }
}
impl<'lt> PhysicalDeviceConservativeRasterizationPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::primitive_overestimation_size`]
    pub fn primitive_overestimation_size_raw(&self) -> f32 {
        self.primitive_overestimation_size
    }
    ///Gets the raw value of [`Self::max_extra_primitive_overestimation_size`]
    pub fn max_extra_primitive_overestimation_size_raw(&self) -> f32 {
        self.max_extra_primitive_overestimation_size
    }
    ///Gets the raw value of [`Self::extra_primitive_overestimation_size_granularity`]
    pub fn extra_primitive_overestimation_size_granularity_raw(&self) -> f32 {
        self.extra_primitive_overestimation_size_granularity
    }
    ///Gets the raw value of [`Self::primitive_underestimation`]
    pub fn primitive_underestimation_raw(&self) -> Bool32 {
        self.primitive_underestimation
    }
    ///Gets the raw value of [`Self::conservative_point_and_line_rasterization`]
    pub fn conservative_point_and_line_rasterization_raw(&self) -> Bool32 {
        self.conservative_point_and_line_rasterization
    }
    ///Gets the raw value of [`Self::degenerate_triangles_rasterized`]
    pub fn degenerate_triangles_rasterized_raw(&self) -> Bool32 {
        self.degenerate_triangles_rasterized
    }
    ///Gets the raw value of [`Self::degenerate_lines_rasterized`]
    pub fn degenerate_lines_rasterized_raw(&self) -> Bool32 {
        self.degenerate_lines_rasterized
    }
    ///Gets the raw value of [`Self::fully_covered_fragment_shader_input_variable`]
    pub fn fully_covered_fragment_shader_input_variable_raw(&self) -> Bool32 {
        self.fully_covered_fragment_shader_input_variable
    }
    ///Gets the raw value of [`Self::conservative_rasterization_post_depth_coverage`]
    pub fn conservative_rasterization_post_depth_coverage_raw(&self) -> Bool32 {
        self.conservative_rasterization_post_depth_coverage
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::primitive_overestimation_size`]
    pub fn set_primitive_overestimation_size_raw(&mut self, value: f32) -> &mut Self {
        self.primitive_overestimation_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_extra_primitive_overestimation_size`]
    pub fn set_max_extra_primitive_overestimation_size_raw(&mut self, value: f32) -> &mut Self {
        self.max_extra_primitive_overestimation_size = value;
        self
    }
    ///Sets the raw value of [`Self::extra_primitive_overestimation_size_granularity`]
    pub fn set_extra_primitive_overestimation_size_granularity_raw(&mut self, value: f32) -> &mut Self {
        self.extra_primitive_overestimation_size_granularity = value;
        self
    }
    ///Sets the raw value of [`Self::primitive_underestimation`]
    pub fn set_primitive_underestimation_raw(&mut self, value: Bool32) -> &mut Self {
        self.primitive_underestimation = value;
        self
    }
    ///Sets the raw value of [`Self::conservative_point_and_line_rasterization`]
    pub fn set_conservative_point_and_line_rasterization_raw(&mut self, value: Bool32) -> &mut Self {
        self.conservative_point_and_line_rasterization = value;
        self
    }
    ///Sets the raw value of [`Self::degenerate_triangles_rasterized`]
    pub fn set_degenerate_triangles_rasterized_raw(&mut self, value: Bool32) -> &mut Self {
        self.degenerate_triangles_rasterized = value;
        self
    }
    ///Sets the raw value of [`Self::degenerate_lines_rasterized`]
    pub fn set_degenerate_lines_rasterized_raw(&mut self, value: Bool32) -> &mut Self {
        self.degenerate_lines_rasterized = value;
        self
    }
    ///Sets the raw value of [`Self::fully_covered_fragment_shader_input_variable`]
    pub fn set_fully_covered_fragment_shader_input_variable_raw(&mut self, value: Bool32) -> &mut Self {
        self.fully_covered_fragment_shader_input_variable = value;
        self
    }
    ///Sets the raw value of [`Self::conservative_rasterization_post_depth_coverage`]
    pub fn set_conservative_rasterization_post_depth_coverage_raw(&mut self, value: Bool32) -> &mut Self {
        self.conservative_rasterization_post_depth_coverage = value;
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
    ///Gets the value of [`Self::primitive_overestimation_size`]
    pub fn primitive_overestimation_size(&self) -> f32 {
        self.primitive_overestimation_size
    }
    ///Gets the value of [`Self::max_extra_primitive_overestimation_size`]
    pub fn max_extra_primitive_overestimation_size(&self) -> f32 {
        self.max_extra_primitive_overestimation_size
    }
    ///Gets the value of [`Self::extra_primitive_overestimation_size_granularity`]
    pub fn extra_primitive_overestimation_size_granularity(&self) -> f32 {
        self.extra_primitive_overestimation_size_granularity
    }
    ///Gets the value of [`Self::primitive_underestimation`]
    pub fn primitive_underestimation(&self) -> bool {
        unsafe { std::mem::transmute(self.primitive_underestimation as u8) }
    }
    ///Gets the value of [`Self::conservative_point_and_line_rasterization`]
    pub fn conservative_point_and_line_rasterization(&self) -> bool {
        unsafe { std::mem::transmute(self.conservative_point_and_line_rasterization as u8) }
    }
    ///Gets the value of [`Self::degenerate_triangles_rasterized`]
    pub fn degenerate_triangles_rasterized(&self) -> bool {
        unsafe { std::mem::transmute(self.degenerate_triangles_rasterized as u8) }
    }
    ///Gets the value of [`Self::degenerate_lines_rasterized`]
    pub fn degenerate_lines_rasterized(&self) -> bool {
        unsafe { std::mem::transmute(self.degenerate_lines_rasterized as u8) }
    }
    ///Gets the value of [`Self::fully_covered_fragment_shader_input_variable`]
    pub fn fully_covered_fragment_shader_input_variable(&self) -> bool {
        unsafe { std::mem::transmute(self.fully_covered_fragment_shader_input_variable as u8) }
    }
    ///Gets the value of [`Self::conservative_rasterization_post_depth_coverage`]
    pub fn conservative_rasterization_post_depth_coverage(&self) -> bool {
        unsafe { std::mem::transmute(self.conservative_rasterization_post_depth_coverage as u8) }
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
    ///Gets a mutable reference to the value of [`Self::primitive_overestimation_size`]
    pub fn primitive_overestimation_size_mut(&mut self) -> &mut f32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_extra_primitive_overestimation_size`]
    pub fn max_extra_primitive_overestimation_size_mut(&mut self) -> &mut f32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::extra_primitive_overestimation_size_granularity`]
    pub fn extra_primitive_overestimation_size_granularity_mut(&mut self) -> &mut f32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::primitive_underestimation`]
    pub fn primitive_underestimation_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.primitive_underestimation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.primitive_underestimation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::conservative_point_and_line_rasterization`]
    pub fn conservative_point_and_line_rasterization_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.conservative_point_and_line_rasterization as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.conservative_point_and_line_rasterization as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::degenerate_triangles_rasterized`]
    pub fn degenerate_triangles_rasterized_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.degenerate_triangles_rasterized as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.degenerate_triangles_rasterized as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::degenerate_lines_rasterized`]
    pub fn degenerate_lines_rasterized_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.degenerate_lines_rasterized as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.degenerate_lines_rasterized as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::fully_covered_fragment_shader_input_variable`]
    pub fn fully_covered_fragment_shader_input_variable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fully_covered_fragment_shader_input_variable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fully_covered_fragment_shader_input_variable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::conservative_rasterization_post_depth_coverage`]
    pub fn conservative_rasterization_post_depth_coverage_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.conservative_rasterization_post_depth_coverage as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.conservative_rasterization_post_depth_coverage as *mut Bool32)
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
    ///Sets the raw value of [`Self::primitive_overestimation_size`]
    pub fn set_primitive_overestimation_size(&mut self, value: f32) -> &mut Self {
        self.primitive_overestimation_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_extra_primitive_overestimation_size`]
    pub fn set_max_extra_primitive_overestimation_size(&mut self, value: f32) -> &mut Self {
        self.max_extra_primitive_overestimation_size = value;
        self
    }
    ///Sets the raw value of [`Self::extra_primitive_overestimation_size_granularity`]
    pub fn set_extra_primitive_overestimation_size_granularity(&mut self, value: f32) -> &mut Self {
        self.extra_primitive_overestimation_size_granularity = value;
        self
    }
    ///Sets the raw value of [`Self::primitive_underestimation`]
    pub fn set_primitive_underestimation(&mut self, value: bool) -> &mut Self {
        self.primitive_underestimation = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::conservative_point_and_line_rasterization`]
    pub fn set_conservative_point_and_line_rasterization(&mut self, value: bool) -> &mut Self {
        self.conservative_point_and_line_rasterization = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::degenerate_triangles_rasterized`]
    pub fn set_degenerate_triangles_rasterized(&mut self, value: bool) -> &mut Self {
        self.degenerate_triangles_rasterized = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::degenerate_lines_rasterized`]
    pub fn set_degenerate_lines_rasterized(&mut self, value: bool) -> &mut Self {
        self.degenerate_lines_rasterized = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::fully_covered_fragment_shader_input_variable`]
    pub fn set_fully_covered_fragment_shader_input_variable(&mut self, value: bool) -> &mut Self {
        self.fully_covered_fragment_shader_input_variable = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::conservative_rasterization_post_depth_coverage`]
    pub fn set_conservative_rasterization_post_depth_coverage(&mut self, value: bool) -> &mut Self {
        self.conservative_rasterization_post_depth_coverage = value as u8 as u32;
        self
    }
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
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
impl<'lt> Default for PipelineRasterizationConservativeStateCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            conservative_rasterization_mode: Default::default(),
            extra_primitive_overestimation_size: 0.0,
        }
    }
}
impl<'lt> PipelineRasterizationConservativeStateCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::extra_primitive_overestimation_size`]
    pub fn extra_primitive_overestimation_size_raw(&self) -> f32 {
        self.extra_primitive_overestimation_size
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::extra_primitive_overestimation_size`]
    pub fn set_extra_primitive_overestimation_size_raw(&mut self, value: f32) -> &mut Self {
        self.extra_primitive_overestimation_size = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> PipelineRasterizationConservativeStateCreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::conservative_rasterization_mode`]
    pub fn conservative_rasterization_mode(&self) -> ConservativeRasterizationModeEXT {
        self.conservative_rasterization_mode
    }
    ///Gets the value of [`Self::extra_primitive_overestimation_size`]
    pub fn extra_primitive_overestimation_size(&self) -> f32 {
        self.extra_primitive_overestimation_size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PipelineRasterizationConservativeStateCreateFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::conservative_rasterization_mode`]
    pub fn conservative_rasterization_mode_mut(&mut self) -> &mut ConservativeRasterizationModeEXT {
        &mut self.conservative_rasterization_mode
    }
    ///Gets a mutable reference to the value of [`Self::extra_primitive_overestimation_size`]
    pub fn extra_primitive_overestimation_size_mut(&mut self) -> &mut f32 {
        &mut getter
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
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ext_conservative_rasterization::PipelineRasterizationConservativeStateCreateFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::conservative_rasterization_mode`]
    pub fn set_conservative_rasterization_mode(
        &mut self,
        value: crate::extensions::ext_conservative_rasterization::ConservativeRasterizationModeEXT,
    ) -> &mut Self {
        self.conservative_rasterization_mode = value;
        self
    }
    ///Sets the raw value of [`Self::extra_primitive_overestimation_size`]
    pub fn set_extra_primitive_overestimation_size(&mut self, value: f32) -> &mut Self {
        self.extra_primitive_overestimation_size = value;
        self
    }
}
