//![VK_KHR_portability_subset](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_portability_subset.html) - device extension
//!# Description
//!The `VK_KHR_portability_subset extension allows a non-conformant Vulkan
//!implementation to be built on top of another non-Vulkan graphics API, and
//!identifies differences between that implementation and a fully-conformant
//!native Vulkan implementation.This extension provides Vulkan implementations with the ability to
//! mark
//!otherwise-required capabilities as unsupported, or to establish additional
//!properties and limits that the application should adhere to in order to
//!guarantee portable behaviour and operation across platforms, including
//!platforms where Vulkan is not natively supported.The goal of this specification is to document,
//! and make queryable,
//!capabilities which are required to be supported by a fully-conformant Vulkan
//!1.0 implementation, but may be optional for an implementation of the Vulkan
//!1.0 Portability Subset.The intent is that this extension will be advertised only on
//! implementations
//!of the Vulkan 1.0 Portability Subset, and not on conformant implementations
//!of Vulkan 1.0.
//!Fully-conformant Vulkan implementations provide all the required capabilies,
//!and so will not provide this extension.
//!Therefore, the existence of this extension can be used to determine that an
//!implementation is likely not fully conformant with the Vulkan spec.If this extension is
//! supported by the Vulkan implementation, the application
//!must enable this extension.This extension defines several new structures that can be chained to
//! the
//!existing structures used by certain standard Vulkan calls, in order to query
//!for non-conformant portable behavior.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//! - **This is a *provisional* extension and  **must**  be used with caution. See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header)
//!   of provisional header files for enablement and stability details.**
//!# Contacts
//! - Bill Hollings [billhollings](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_portability_subset]
//!   @billhollings%0A<<Here describe the issue or question you have about the
//!   VK_KHR_portability_subset extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevicePortabilitySubsetFeaturesKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDevicePortabilitySubsetPropertiesKHR`]
//!# New constants
//! - [`KHR_PORTABILITY_SUBSET_EXTENSION_NAME`]
//! - [`KHR_PORTABILITY_SUBSET_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR`
//!# Known issues & F.A.Q.
//!None.
//!# Version history
//! - Revision 1, 2020-07-21 (Bill Hollings)  - Initial draft.
//!# Other information
//! * 2020-07-21
//! * No known IP claims.
//! * - Bill Hollings, The Brenwill Workshop Ltd.  - Daniel Koch, NVIDIA  - Dzmitry Malyshau,
//!   Mozilla  - Chip Davis, CodeWeavers  - Dan Ginsburg, Valve  - Mike Weiblen, LunarG  - Neil
//!   Trevett, NVIDIA  - Alexey Knyazev, Independent
//!# Related
//! - [`PhysicalDevicePortabilitySubsetFeaturesKHR`]
//! - [`PhysicalDevicePortabilitySubsetPropertiesKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PORTABILITY_SUBSET_SPEC_VERSION")]
pub const KHR_PORTABILITY_SUBSET_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PORTABILITY_SUBSET_EXTENSION_NAME")]
pub const KHR_PORTABILITY_SUBSET_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_portability_subset");
///[VkPhysicalDevicePortabilitySubsetFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePortabilitySubsetFeaturesKHR.html) - Structure describing the features that may not be supported by an implementation of the Vulkan 1.0 Portability Subset
///# C Specifications
///The [`PhysicalDevicePortabilitySubsetFeaturesKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_portability_subset
///typedef struct VkPhysicalDevicePortabilitySubsetFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           constantAlphaColorBlendFactors;
///    VkBool32           events;
///    VkBool32           imageViewFormatReinterpretation;
///    VkBool32           imageViewFormatSwizzle;
///    VkBool32           imageView2DOn3DImage;
///    VkBool32           multisampleArrayImage;
///    VkBool32           mutableComparisonSamplers;
///    VkBool32           pointPolygons;
///    VkBool32           samplerMipLodBias;
///    VkBool32           separateStencilMaskRef;
///    VkBool32           shaderSampleRateInterpolationFunctions;
///    VkBool32           tessellationIsolines;
///    VkBool32           tessellationPointMode;
///    VkBool32           triangleFans;
///    VkBool32           vertexAttributeAccessBeyondStride;
///} VkPhysicalDevicePortabilitySubsetFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`constant_alpha_color_blend_factors`] indicates whether this implementation supports constant
///   *alpha*[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blendfactors](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blendfactors)
///   used as source or destination *color*[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blending](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blending).
/// - [`events`] indicates whether this implementation supports synchronization using [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-events](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-events).
/// - [`image_view_format_reinterpretation`] indicates whether this implementation supports a
///   [`ImageView`] being created with a texel format containing a different number of components,
///   or a different number of bits in each component, than the texel format of the underlying
///   [`Image`].
/// - [`image_view_format_swizzle`] indicates whether this implementation supports remapping format
///   components using [`ImageViewCreateInfo::components`].
/// - [`image_view2_d_on3_d_image`] indicates whether this implementation supports a [`Image`] being
///   created with the `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT` flag set, permitting a 2D or 2D
///   array image view to be created on a 3D [`Image`].
/// - [`multisample_array_image`] indicates whether this implementation supports a [`Image`] being
///   created as a 2D array with multiple samples per texel.
/// - [`mutable_comparison_samplers`] indicates whether this implementation allows descriptors with comparison samplers to be [updated](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates).
/// - [`point_polygons`] indicates whether this implementation supports [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast)
///   using a *point*[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-polygonmode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-polygonmode).
/// - [`sampler_mip_lod_bias`] indicates whether this implementation supports setting a [mipmap LOD bias value](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-mipLodBias) when [creating a sampler](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers).
/// - [`separate_stencil_mask_ref`] indicates whether this implementation supports separate front and back [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-stencil](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-stencil) reference values.
/// - [`shader_sample_rate_interpolation_functions`] indicates whether this implementation supports fragment shaders which use the [`InterpolationFunction`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv-capabilities-table-InterpolationFunction) capability and the extended instructions `InterpolateAtCentroid`, `InterpolateAtOffset`, and `InterpolateAtSample` from the `GLSL.std.450` extended instruction set. This member is only meaningful if the [sampleRateShading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-sampleRateShading) feature is supported.
/// - [`tessellation_isolines`] indicates whether this implementation supports [isoline output](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation-isoline-tessellation)
///   from the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation)
///   stage of a graphics pipeline. This member is only meaningful if [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-tessellationShader)
///   are supported.
/// - [`tessellation_point_mode`] indicates whether this implementation supports [point output](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation-point-mode)
///   from the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation)
///   stage of a graphics pipeline. This member is only meaningful if [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-tessellationShader)
///   are supported.
/// - [`triangle_fans`] indicates whether this implementation supports [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-fans](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-fans)
///   primitive topology.
/// - [`vertex_attribute_access_beyond_stride`] indicates whether this implementation supports
///   accessing a vertex input attribute beyond the stride of the corresponding vertex input
///   binding.
///If the [`PhysicalDevicePortabilitySubsetFeaturesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePortabilitySubsetFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR`
///# Related
/// - [`khr_portability_subset`]
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
#[doc(alias = "VkPhysicalDevicePortabilitySubsetFeaturesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetFeaturesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`constant_alpha_color_blend_factors`] indicates whether this
    ///implementation supports constant *alpha*[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blendfactors](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blendfactors)
    ///used as source or destination *color*[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blending](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blending).
    pub constant_alpha_color_blend_factors: Bool32,
    ///[`events`] indicates whether this implementation
    ///supports synchronization using [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-events](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-events).
    pub events: Bool32,
    ///[`image_view_format_reinterpretation`] indicates whether this
    ///implementation supports a [`ImageView`] being created with a texel
    ///format containing a different number of components, or a different
    ///number of bits in each component, than the texel format of the
    ///underlying [`Image`].
    pub image_view_format_reinterpretation: Bool32,
    ///[`image_view_format_swizzle`]
    ///indicates whether this implementation supports remapping format
    ///components using [`ImageViewCreateInfo`]::`components`.
    pub image_view_format_swizzle: Bool32,
    ///[`image_view2_d_on3_d_image`] indicates
    ///whether this implementation supports a [`Image`] being created with
    ///the `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT` flag set, permitting a
    ///2D or 2D array image view to be created on a 3D [`Image`].
    pub image_view2_d_on3_d_image: Bool32,
    ///[`multisample_array_image`] indicates
    ///whether this implementation supports a [`Image`] being created as a
    ///2D array with multiple samples per texel.
    pub multisample_array_image: Bool32,
    ///[`mutable_comparison_samplers`]
    ///indicates whether this implementation allows descriptors with comparison
    ///samplers to be [updated](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates).
    pub mutable_comparison_samplers: Bool32,
    ///[`point_polygons`] indicates whether this
    ///implementation supports [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast) using a *point*[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-polygonmode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-polygonmode).
    pub point_polygons: Bool32,
    ///[`sampler_mip_lod_bias`] indicates whether
    ///this implementation supports setting a [mipmap LOD
    ///bias value](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-mipLodBias) when [creating a sampler](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers).
    pub sampler_mip_lod_bias: Bool32,
    ///[`separate_stencil_mask_ref`]
    ///indicates whether this implementation supports separate front and back
    ///[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-stencil](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-stencil) reference values.
    pub separate_stencil_mask_ref: Bool32,
    ///[`shader_sample_rate_interpolation_functions`] indicates whether this
    ///implementation supports fragment shaders which use the
    ///[`InterpolationFunction`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv-capabilities-table-InterpolationFunction) capability and the extended instructions
    ///`InterpolateAtCentroid`, `InterpolateAtOffset`, and
    ///`InterpolateAtSample` from the `GLSL.std.450` extended instruction set.
    ///This member is only meaningful if the
    ///[sampleRateShading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-sampleRateShading) feature is supported.
    pub shader_sample_rate_interpolation_functions: Bool32,
    ///[`tessellation_isolines`] indicates
    ///whether this implementation supports
    ///[isoline output](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation-isoline-tessellation) from the
    ///[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation) stage of a graphics pipeline.
    ///This member is only meaningful if
    ///[tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-tessellationShader) are supported.
    pub tessellation_isolines: Bool32,
    ///[`tessellation_point_mode`] indicates
    ///whether this implementation supports [point
    ///output](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation-point-mode) from the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation) stage of a graphics pipeline.
    ///This member is only meaningful if
    ///[tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-tessellationShader) are supported.
    pub tessellation_point_mode: Bool32,
    ///[`triangle_fans`] indicates whether this
    ///implementation supports [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-fans](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-fans) primitive topology.
    pub triangle_fans: Bool32,
    ///[`vertex_attribute_access_beyond_stride`] indicates whether this
    ///implementation supports accessing a vertex input attribute beyond the
    ///stride of the corresponding vertex input binding.
    pub vertex_attribute_access_beyond_stride: Bool32,
}
impl<'lt> Default for PhysicalDevicePortabilitySubsetFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            constant_alpha_color_blend_factors: 0,
            events: 0,
            image_view_format_reinterpretation: 0,
            image_view_format_swizzle: 0,
            image_view2_d_on3_d_image: 0,
            multisample_array_image: 0,
            mutable_comparison_samplers: 0,
            point_polygons: 0,
            sampler_mip_lod_bias: 0,
            separate_stencil_mask_ref: 0,
            shader_sample_rate_interpolation_functions: 0,
            tessellation_isolines: 0,
            tessellation_point_mode: 0,
            triangle_fans: 0,
            vertex_attribute_access_beyond_stride: 0,
        }
    }
}
impl<'lt> PhysicalDevicePortabilitySubsetFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::constant_alpha_color_blend_factors`]
    pub fn constant_alpha_color_blend_factors_raw(&self) -> Bool32 {
        self.constant_alpha_color_blend_factors
    }
    ///Gets the raw value of [`Self::events`]
    pub fn events_raw(&self) -> Bool32 {
        self.events
    }
    ///Gets the raw value of [`Self::image_view_format_reinterpretation`]
    pub fn image_view_format_reinterpretation_raw(&self) -> Bool32 {
        self.image_view_format_reinterpretation
    }
    ///Gets the raw value of [`Self::image_view_format_swizzle`]
    pub fn image_view_format_swizzle_raw(&self) -> Bool32 {
        self.image_view_format_swizzle
    }
    ///Gets the raw value of [`Self::image_view2_d_on3_d_image`]
    pub fn image_view2_d_on3_d_image_raw(&self) -> Bool32 {
        self.image_view2_d_on3_d_image
    }
    ///Gets the raw value of [`Self::multisample_array_image`]
    pub fn multisample_array_image_raw(&self) -> Bool32 {
        self.multisample_array_image
    }
    ///Gets the raw value of [`Self::mutable_comparison_samplers`]
    pub fn mutable_comparison_samplers_raw(&self) -> Bool32 {
        self.mutable_comparison_samplers
    }
    ///Gets the raw value of [`Self::point_polygons`]
    pub fn point_polygons_raw(&self) -> Bool32 {
        self.point_polygons
    }
    ///Gets the raw value of [`Self::sampler_mip_lod_bias`]
    pub fn sampler_mip_lod_bias_raw(&self) -> Bool32 {
        self.sampler_mip_lod_bias
    }
    ///Gets the raw value of [`Self::separate_stencil_mask_ref`]
    pub fn separate_stencil_mask_ref_raw(&self) -> Bool32 {
        self.separate_stencil_mask_ref
    }
    ///Gets the raw value of [`Self::shader_sample_rate_interpolation_functions`]
    pub fn shader_sample_rate_interpolation_functions_raw(&self) -> Bool32 {
        self.shader_sample_rate_interpolation_functions
    }
    ///Gets the raw value of [`Self::tessellation_isolines`]
    pub fn tessellation_isolines_raw(&self) -> Bool32 {
        self.tessellation_isolines
    }
    ///Gets the raw value of [`Self::tessellation_point_mode`]
    pub fn tessellation_point_mode_raw(&self) -> Bool32 {
        self.tessellation_point_mode
    }
    ///Gets the raw value of [`Self::triangle_fans`]
    pub fn triangle_fans_raw(&self) -> Bool32 {
        self.triangle_fans
    }
    ///Gets the raw value of [`Self::vertex_attribute_access_beyond_stride`]
    pub fn vertex_attribute_access_beyond_stride_raw(&self) -> Bool32 {
        self.vertex_attribute_access_beyond_stride
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::constant_alpha_color_blend_factors`]
    pub fn set_constant_alpha_color_blend_factors_raw(&mut self, value: Bool32) -> &mut Self {
        self.constant_alpha_color_blend_factors = value;
        self
    }
    ///Sets the raw value of [`Self::events`]
    pub fn set_events_raw(&mut self, value: Bool32) -> &mut Self {
        self.events = value;
        self
    }
    ///Sets the raw value of [`Self::image_view_format_reinterpretation`]
    pub fn set_image_view_format_reinterpretation_raw(&mut self, value: Bool32) -> &mut Self {
        self.image_view_format_reinterpretation = value;
        self
    }
    ///Sets the raw value of [`Self::image_view_format_swizzle`]
    pub fn set_image_view_format_swizzle_raw(&mut self, value: Bool32) -> &mut Self {
        self.image_view_format_swizzle = value;
        self
    }
    ///Sets the raw value of [`Self::image_view2_d_on3_d_image`]
    pub fn set_image_view2_d_on3_d_image_raw(&mut self, value: Bool32) -> &mut Self {
        self.image_view2_d_on3_d_image = value;
        self
    }
    ///Sets the raw value of [`Self::multisample_array_image`]
    pub fn set_multisample_array_image_raw(&mut self, value: Bool32) -> &mut Self {
        self.multisample_array_image = value;
        self
    }
    ///Sets the raw value of [`Self::mutable_comparison_samplers`]
    pub fn set_mutable_comparison_samplers_raw(&mut self, value: Bool32) -> &mut Self {
        self.mutable_comparison_samplers = value;
        self
    }
    ///Sets the raw value of [`Self::point_polygons`]
    pub fn set_point_polygons_raw(&mut self, value: Bool32) -> &mut Self {
        self.point_polygons = value;
        self
    }
    ///Sets the raw value of [`Self::sampler_mip_lod_bias`]
    pub fn set_sampler_mip_lod_bias_raw(&mut self, value: Bool32) -> &mut Self {
        self.sampler_mip_lod_bias = value;
        self
    }
    ///Sets the raw value of [`Self::separate_stencil_mask_ref`]
    pub fn set_separate_stencil_mask_ref_raw(&mut self, value: Bool32) -> &mut Self {
        self.separate_stencil_mask_ref = value;
        self
    }
    ///Sets the raw value of [`Self::shader_sample_rate_interpolation_functions`]
    pub fn set_shader_sample_rate_interpolation_functions_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_sample_rate_interpolation_functions = value;
        self
    }
    ///Sets the raw value of [`Self::tessellation_isolines`]
    pub fn set_tessellation_isolines_raw(&mut self, value: Bool32) -> &mut Self {
        self.tessellation_isolines = value;
        self
    }
    ///Sets the raw value of [`Self::tessellation_point_mode`]
    pub fn set_tessellation_point_mode_raw(&mut self, value: Bool32) -> &mut Self {
        self.tessellation_point_mode = value;
        self
    }
    ///Sets the raw value of [`Self::triangle_fans`]
    pub fn set_triangle_fans_raw(&mut self, value: Bool32) -> &mut Self {
        self.triangle_fans = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_attribute_access_beyond_stride`]
    pub fn set_vertex_attribute_access_beyond_stride_raw(&mut self, value: Bool32) -> &mut Self {
        self.vertex_attribute_access_beyond_stride = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::constant_alpha_color_blend_factors`]
    pub fn with_constant_alpha_color_blend_factors_raw(mut self, value: Bool32) -> Self {
        self.constant_alpha_color_blend_factors = value;
        self
    }
    ///Sets the raw value of [`Self::events`]
    pub fn with_events_raw(mut self, value: Bool32) -> Self {
        self.events = value;
        self
    }
    ///Sets the raw value of [`Self::image_view_format_reinterpretation`]
    pub fn with_image_view_format_reinterpretation_raw(mut self, value: Bool32) -> Self {
        self.image_view_format_reinterpretation = value;
        self
    }
    ///Sets the raw value of [`Self::image_view_format_swizzle`]
    pub fn with_image_view_format_swizzle_raw(mut self, value: Bool32) -> Self {
        self.image_view_format_swizzle = value;
        self
    }
    ///Sets the raw value of [`Self::image_view2_d_on3_d_image`]
    pub fn with_image_view2_d_on3_d_image_raw(mut self, value: Bool32) -> Self {
        self.image_view2_d_on3_d_image = value;
        self
    }
    ///Sets the raw value of [`Self::multisample_array_image`]
    pub fn with_multisample_array_image_raw(mut self, value: Bool32) -> Self {
        self.multisample_array_image = value;
        self
    }
    ///Sets the raw value of [`Self::mutable_comparison_samplers`]
    pub fn with_mutable_comparison_samplers_raw(mut self, value: Bool32) -> Self {
        self.mutable_comparison_samplers = value;
        self
    }
    ///Sets the raw value of [`Self::point_polygons`]
    pub fn with_point_polygons_raw(mut self, value: Bool32) -> Self {
        self.point_polygons = value;
        self
    }
    ///Sets the raw value of [`Self::sampler_mip_lod_bias`]
    pub fn with_sampler_mip_lod_bias_raw(mut self, value: Bool32) -> Self {
        self.sampler_mip_lod_bias = value;
        self
    }
    ///Sets the raw value of [`Self::separate_stencil_mask_ref`]
    pub fn with_separate_stencil_mask_ref_raw(mut self, value: Bool32) -> Self {
        self.separate_stencil_mask_ref = value;
        self
    }
    ///Sets the raw value of [`Self::shader_sample_rate_interpolation_functions`]
    pub fn with_shader_sample_rate_interpolation_functions_raw(mut self, value: Bool32) -> Self {
        self.shader_sample_rate_interpolation_functions = value;
        self
    }
    ///Sets the raw value of [`Self::tessellation_isolines`]
    pub fn with_tessellation_isolines_raw(mut self, value: Bool32) -> Self {
        self.tessellation_isolines = value;
        self
    }
    ///Sets the raw value of [`Self::tessellation_point_mode`]
    pub fn with_tessellation_point_mode_raw(mut self, value: Bool32) -> Self {
        self.tessellation_point_mode = value;
        self
    }
    ///Sets the raw value of [`Self::triangle_fans`]
    pub fn with_triangle_fans_raw(mut self, value: Bool32) -> Self {
        self.triangle_fans = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_attribute_access_beyond_stride`]
    pub fn with_vertex_attribute_access_beyond_stride_raw(mut self, value: Bool32) -> Self {
        self.vertex_attribute_access_beyond_stride = value;
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
    ///Gets the value of [`Self::constant_alpha_color_blend_factors`]
    pub fn constant_alpha_color_blend_factors(&self) -> bool {
        unsafe { std::mem::transmute(self.constant_alpha_color_blend_factors as u8) }
    }
    ///Gets the value of [`Self::events`]
    pub fn events(&self) -> bool {
        unsafe { std::mem::transmute(self.events as u8) }
    }
    ///Gets the value of [`Self::image_view_format_reinterpretation`]
    pub fn image_view_format_reinterpretation(&self) -> bool {
        unsafe { std::mem::transmute(self.image_view_format_reinterpretation as u8) }
    }
    ///Gets the value of [`Self::image_view_format_swizzle`]
    pub fn image_view_format_swizzle(&self) -> bool {
        unsafe { std::mem::transmute(self.image_view_format_swizzle as u8) }
    }
    ///Gets the value of [`Self::image_view2_d_on3_d_image`]
    pub fn image_view2_d_on3_d_image(&self) -> bool {
        unsafe { std::mem::transmute(self.image_view2_d_on3_d_image as u8) }
    }
    ///Gets the value of [`Self::multisample_array_image`]
    pub fn multisample_array_image(&self) -> bool {
        unsafe { std::mem::transmute(self.multisample_array_image as u8) }
    }
    ///Gets the value of [`Self::mutable_comparison_samplers`]
    pub fn mutable_comparison_samplers(&self) -> bool {
        unsafe { std::mem::transmute(self.mutable_comparison_samplers as u8) }
    }
    ///Gets the value of [`Self::point_polygons`]
    pub fn point_polygons(&self) -> bool {
        unsafe { std::mem::transmute(self.point_polygons as u8) }
    }
    ///Gets the value of [`Self::sampler_mip_lod_bias`]
    pub fn sampler_mip_lod_bias(&self) -> bool {
        unsafe { std::mem::transmute(self.sampler_mip_lod_bias as u8) }
    }
    ///Gets the value of [`Self::separate_stencil_mask_ref`]
    pub fn separate_stencil_mask_ref(&self) -> bool {
        unsafe { std::mem::transmute(self.separate_stencil_mask_ref as u8) }
    }
    ///Gets the value of [`Self::shader_sample_rate_interpolation_functions`]
    pub fn shader_sample_rate_interpolation_functions(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_sample_rate_interpolation_functions as u8) }
    }
    ///Gets the value of [`Self::tessellation_isolines`]
    pub fn tessellation_isolines(&self) -> bool {
        unsafe { std::mem::transmute(self.tessellation_isolines as u8) }
    }
    ///Gets the value of [`Self::tessellation_point_mode`]
    pub fn tessellation_point_mode(&self) -> bool {
        unsafe { std::mem::transmute(self.tessellation_point_mode as u8) }
    }
    ///Gets the value of [`Self::triangle_fans`]
    pub fn triangle_fans(&self) -> bool {
        unsafe { std::mem::transmute(self.triangle_fans as u8) }
    }
    ///Gets the value of [`Self::vertex_attribute_access_beyond_stride`]
    pub fn vertex_attribute_access_beyond_stride(&self) -> bool {
        unsafe { std::mem::transmute(self.vertex_attribute_access_beyond_stride as u8) }
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
    ///Gets a mutable reference to the value of [`Self::constant_alpha_color_blend_factors`]
    pub fn constant_alpha_color_blend_factors_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.constant_alpha_color_blend_factors as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.constant_alpha_color_blend_factors as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::events`]
    pub fn events_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.events as *mut Bool32).cast::<u32>().cast::<u8>().cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.events as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::image_view_format_reinterpretation`]
    pub fn image_view_format_reinterpretation_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.image_view_format_reinterpretation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.image_view_format_reinterpretation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::image_view_format_swizzle`]
    pub fn image_view_format_swizzle_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.image_view_format_swizzle as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.image_view_format_swizzle as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::image_view2_d_on3_d_image`]
    pub fn image_view2_d_on3_d_image_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.image_view2_d_on3_d_image as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.image_view2_d_on3_d_image as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::multisample_array_image`]
    pub fn multisample_array_image_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.multisample_array_image as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.multisample_array_image as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::mutable_comparison_samplers`]
    pub fn mutable_comparison_samplers_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.mutable_comparison_samplers as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.mutable_comparison_samplers as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::point_polygons`]
    pub fn point_polygons_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.point_polygons as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.point_polygons as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::sampler_mip_lod_bias`]
    pub fn sampler_mip_lod_bias_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.sampler_mip_lod_bias as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.sampler_mip_lod_bias as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::separate_stencil_mask_ref`]
    pub fn separate_stencil_mask_ref_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.separate_stencil_mask_ref as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.separate_stencil_mask_ref as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_sample_rate_interpolation_functions`]
    pub fn shader_sample_rate_interpolation_functions_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_sample_rate_interpolation_functions as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_sample_rate_interpolation_functions as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::tessellation_isolines`]
    pub fn tessellation_isolines_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.tessellation_isolines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.tessellation_isolines as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::tessellation_point_mode`]
    pub fn tessellation_point_mode_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.tessellation_point_mode as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.tessellation_point_mode as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::triangle_fans`]
    pub fn triangle_fans_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.triangle_fans as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.triangle_fans as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::vertex_attribute_access_beyond_stride`]
    pub fn vertex_attribute_access_beyond_stride_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.vertex_attribute_access_beyond_stride as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.vertex_attribute_access_beyond_stride as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the value of [`Self::constant_alpha_color_blend_factors`]
    pub fn set_constant_alpha_color_blend_factors(&mut self, value: bool) -> &mut Self {
        self.constant_alpha_color_blend_factors = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::events`]
    pub fn set_events(&mut self, value: bool) -> &mut Self {
        self.events = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::image_view_format_reinterpretation`]
    pub fn set_image_view_format_reinterpretation(&mut self, value: bool) -> &mut Self {
        self.image_view_format_reinterpretation = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::image_view_format_swizzle`]
    pub fn set_image_view_format_swizzle(&mut self, value: bool) -> &mut Self {
        self.image_view_format_swizzle = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::image_view2_d_on3_d_image`]
    pub fn set_image_view2_d_on3_d_image(&mut self, value: bool) -> &mut Self {
        self.image_view2_d_on3_d_image = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::multisample_array_image`]
    pub fn set_multisample_array_image(&mut self, value: bool) -> &mut Self {
        self.multisample_array_image = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::mutable_comparison_samplers`]
    pub fn set_mutable_comparison_samplers(&mut self, value: bool) -> &mut Self {
        self.mutable_comparison_samplers = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::point_polygons`]
    pub fn set_point_polygons(&mut self, value: bool) -> &mut Self {
        self.point_polygons = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::sampler_mip_lod_bias`]
    pub fn set_sampler_mip_lod_bias(&mut self, value: bool) -> &mut Self {
        self.sampler_mip_lod_bias = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::separate_stencil_mask_ref`]
    pub fn set_separate_stencil_mask_ref(&mut self, value: bool) -> &mut Self {
        self.separate_stencil_mask_ref = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_sample_rate_interpolation_functions`]
    pub fn set_shader_sample_rate_interpolation_functions(&mut self, value: bool) -> &mut Self {
        self.shader_sample_rate_interpolation_functions = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::tessellation_isolines`]
    pub fn set_tessellation_isolines(&mut self, value: bool) -> &mut Self {
        self.tessellation_isolines = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::tessellation_point_mode`]
    pub fn set_tessellation_point_mode(&mut self, value: bool) -> &mut Self {
        self.tessellation_point_mode = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::triangle_fans`]
    pub fn set_triangle_fans(&mut self, value: bool) -> &mut Self {
        self.triangle_fans = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::vertex_attribute_access_beyond_stride`]
    pub fn set_vertex_attribute_access_beyond_stride(&mut self, value: bool) -> &mut Self {
        self.vertex_attribute_access_beyond_stride = value as u8 as u32;
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
    ///Sets the value of [`Self::constant_alpha_color_blend_factors`]
    pub fn with_constant_alpha_color_blend_factors(mut self, value: bool) -> Self {
        self.constant_alpha_color_blend_factors = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::events`]
    pub fn with_events(mut self, value: bool) -> Self {
        self.events = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::image_view_format_reinterpretation`]
    pub fn with_image_view_format_reinterpretation(mut self, value: bool) -> Self {
        self.image_view_format_reinterpretation = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::image_view_format_swizzle`]
    pub fn with_image_view_format_swizzle(mut self, value: bool) -> Self {
        self.image_view_format_swizzle = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::image_view2_d_on3_d_image`]
    pub fn with_image_view2_d_on3_d_image(mut self, value: bool) -> Self {
        self.image_view2_d_on3_d_image = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::multisample_array_image`]
    pub fn with_multisample_array_image(mut self, value: bool) -> Self {
        self.multisample_array_image = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::mutable_comparison_samplers`]
    pub fn with_mutable_comparison_samplers(mut self, value: bool) -> Self {
        self.mutable_comparison_samplers = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::point_polygons`]
    pub fn with_point_polygons(mut self, value: bool) -> Self {
        self.point_polygons = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::sampler_mip_lod_bias`]
    pub fn with_sampler_mip_lod_bias(mut self, value: bool) -> Self {
        self.sampler_mip_lod_bias = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::separate_stencil_mask_ref`]
    pub fn with_separate_stencil_mask_ref(mut self, value: bool) -> Self {
        self.separate_stencil_mask_ref = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_sample_rate_interpolation_functions`]
    pub fn with_shader_sample_rate_interpolation_functions(mut self, value: bool) -> Self {
        self.shader_sample_rate_interpolation_functions = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::tessellation_isolines`]
    pub fn with_tessellation_isolines(mut self, value: bool) -> Self {
        self.tessellation_isolines = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::tessellation_point_mode`]
    pub fn with_tessellation_point_mode(mut self, value: bool) -> Self {
        self.tessellation_point_mode = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::triangle_fans`]
    pub fn with_triangle_fans(mut self, value: bool) -> Self {
        self.triangle_fans = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::vertex_attribute_access_beyond_stride`]
    pub fn with_vertex_attribute_access_beyond_stride(mut self, value: bool) -> Self {
        self.vertex_attribute_access_beyond_stride = value as u8 as u32;
        self
    }
}
///[VkPhysicalDevicePortabilitySubsetPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePortabilitySubsetPropertiesKHR.html) - Structure describing additional properties supported by a portable implementation
///# C Specifications
///The [`PhysicalDevicePortabilitySubsetPropertiesKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_portability_subset
///typedef struct VkPhysicalDevicePortabilitySubsetPropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           minVertexInputBindingStrideAlignment;
///} VkPhysicalDevicePortabilitySubsetPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`min_vertex_input_binding_stride_alignment`] indicates the minimum alignment for vertex input
///   strides. [`VertexInputBindingDescription::stride`] **must**  be a multiple of, and at least as
///   large as, this value. The value  **must**  be a power of two.
///# Description
///If the [`PhysicalDevicePortabilitySubsetPropertiesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR`
///# Related
/// - [`khr_portability_subset`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDevicePortabilitySubsetPropertiesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetPropertiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`min_vertex_input_binding_stride_alignment`] indicates the minimum
    ///alignment for vertex input strides.
    ///[`VertexInputBindingDescription`]::`stride` **must**  be a multiple
    ///of, and at least as large as, this value.
    ///The value  **must**  be a power of two.
    pub min_vertex_input_binding_stride_alignment: u32,
}
impl<'lt> Default for PhysicalDevicePortabilitySubsetPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            min_vertex_input_binding_stride_alignment: 0,
        }
    }
}
impl<'lt> PhysicalDevicePortabilitySubsetPropertiesKHR<'lt> {
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
    ///Gets the value of [`Self::min_vertex_input_binding_stride_alignment`]
    pub fn min_vertex_input_binding_stride_alignment(&self) -> u32 {
        self.min_vertex_input_binding_stride_alignment
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
    ///Gets a mutable reference to the value of [`Self::min_vertex_input_binding_stride_alignment`]
    pub fn min_vertex_input_binding_stride_alignment_mut(&mut self) -> &mut u32 {
        &mut self.min_vertex_input_binding_stride_alignment
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
    ///Sets the value of [`Self::min_vertex_input_binding_stride_alignment`]
    pub fn set_min_vertex_input_binding_stride_alignment(&mut self, value: u32) -> &mut Self {
        self.min_vertex_input_binding_stride_alignment = value;
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
    ///Sets the value of [`Self::min_vertex_input_binding_stride_alignment`]
    pub fn with_min_vertex_input_binding_stride_alignment(mut self, value: u32) -> Self {
        self.min_vertex_input_binding_stride_alignment = value;
        self
    }
}
