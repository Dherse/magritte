pub use crate::common::vulkan1_2::{
    ConformanceVersion, DescriptorBindingFlagBits, DescriptorBindingFlags, DriverId, ResolveModeFlagBits,
    ResolveModeFlags, SamplerReductionMode, SemaphoreType, SemaphoreWaitFlagBits, SemaphoreWaitFlags,
    ShaderFloatControlsIndependence,
};
#[cfg(feature = "VK_EXT_fragment_density_map")]
use crate::extensions::ext_fragment_density_map::RenderPassFragmentDensityMapCreateInfoEXT;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::extensions::khr_fragment_shading_rate::FragmentShadingRateAttachmentInfoKHR;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::MemoryBarrier2;
use crate::{
    context::Context,
    vulkan1_0::{
        AccessFlags, AttachmentDescriptionFlags, AttachmentLoadOp, AttachmentStoreOp, Buffer, DependencyFlags,
        DeviceMemory, DeviceSize, Format, ImageAspectFlags, ImageCreateFlags, ImageLayout, ImageUsageFlags, ImageView,
        PipelineBindPoint, PipelineStageFlags, RenderPassCreateFlags, SampleCountFlagBits, SampleCountFlags, Semaphore,
        ShaderStageFlags, StructureType, SubpassContents, SubpassDescriptionFlags, LUID_SIZE, MAX_DRIVER_INFO_SIZE,
        MAX_DRIVER_NAME_SIZE,
    },
    vulkan1_1::{PointClippingBehavior, SubgroupFeatureFlags},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
impl ConformanceVersion {
    ///Get a reference to the `major` field.
    pub fn major(&self) -> u8 {
        self.major
    }
    ///Get a reference to the `minor` field.
    pub fn minor(&self) -> u8 {
        self.minor
    }
    ///Get a reference to the `subminor` field.
    pub fn subminor(&self) -> u8 {
        self.subminor
    }
    ///Get a reference to the `patch` field.
    pub fn patch(&self) -> u8 {
        self.patch
    }
    ///Get a mutable reference to the `major` field.
    pub fn major_mut(&mut self) -> &mut u8 {
        &mut self.major
    }
    ///Get a mutable reference to the `minor` field.
    pub fn minor_mut(&mut self) -> &mut u8 {
        &mut self.minor
    }
    ///Get a mutable reference to the `subminor` field.
    pub fn subminor_mut(&mut self) -> &mut u8 {
        &mut self.subminor
    }
    ///Get a mutable reference to the `patch` field.
    pub fn patch_mut(&mut self) -> &mut u8 {
        &mut self.patch
    }
    ///Sets the `major` field.
    pub fn set_major(&mut self, major: u8) -> &mut Self {
        self.major = major;
        self
    }
    ///Sets the `minor` field.
    pub fn set_minor(&mut self, minor: u8) -> &mut Self {
        self.minor = minor;
        self
    }
    ///Sets the `subminor` field.
    pub fn set_subminor(&mut self, subminor: u8) -> &mut Self {
        self.subminor = subminor;
        self
    }
    ///Sets the `patch` field.
    pub fn set_patch(&mut self, patch: u8) -> &mut Self {
        self.patch = patch;
        self
    }
    ///Sets the `major` field in a builder way.
    pub fn with_major(mut self, major: u8) -> Self {
        self.major = major;
        self
    }
    ///Sets the `minor` field in a builder way.
    pub fn with_minor(mut self, minor: u8) -> Self {
        self.minor = minor;
        self
    }
    ///Sets the `subminor` field in a builder way.
    pub fn with_subminor(mut self, subminor: u8) -> Self {
        self.subminor = subminor;
        self
    }
    ///Sets the `patch` field in a builder way.
    pub fn with_patch(mut self, patch: u8) -> Self {
        self.patch = patch;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ConformanceVersion {
    type LowLevel = crate::native::vulkan1_2::ConformanceVersion;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::ConformanceVersion {
            major: self.major.into_low_level(context, bump),
            minor: self.minor.into_low_level(context, bump),
            subminor: self.subminor.into_low_level(context, bump),
            patch: self.patch.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ConformanceVersion {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            major: crate::conv::FromLowLevel::from_low_level(context, value.major),
            minor: crate::conv::FromLowLevel::from_low_level(context, value.minor),
            subminor: crate::conv::FromLowLevel::from_low_level(context, value.subminor),
            patch: crate::conv::FromLowLevel::from_low_level(context, value.patch),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceDriverProperties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDriverProperties {
    #[doc(alias = "driverID")]
    pub driver_id: DriverId,
    #[doc(alias = "driverName")]
    pub driver_name: String,
    #[doc(alias = "driverInfo")]
    pub driver_info: String,
    #[doc(alias = "conformanceVersion")]
    pub conformance_version: ConformanceVersion,
}
impl PhysicalDeviceDriverProperties {
    ///Get a reference to the `driver_id` field.
    pub fn driver_id(&self) -> DriverId {
        self.driver_id
    }
    ///Get a reference to the `driver_name` field.
    pub fn driver_name(&self) -> &String {
        &self.driver_name
    }
    ///Get a reference to the `driver_info` field.
    pub fn driver_info(&self) -> &String {
        &self.driver_info
    }
    ///Get a reference to the `conformance_version` field.
    pub fn conformance_version(&self) -> ConformanceVersion {
        self.conformance_version
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDriverProperties {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceDriverProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let driver_name_bytes = self.driver_name.as_bytes();
        debug_assert!(
            memchr::memchr(0, driver_name_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            driver_name_bytes.len() <= MAX_DRIVER_NAME_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut driver_name: [std::ffi::c_char; MAX_DRIVER_NAME_SIZE as usize] = [0; MAX_DRIVER_NAME_SIZE as usize];
        driver_name[0..driver_name_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            driver_name_bytes.as_ptr() as *const std::ffi::c_char,
            driver_name_bytes.len(),
        ));
        let driver_info_bytes = self.driver_info.as_bytes();
        debug_assert!(
            memchr::memchr(0, driver_info_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            driver_info_bytes.len() <= MAX_DRIVER_INFO_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut driver_info: [std::ffi::c_char; MAX_DRIVER_INFO_SIZE as usize] = [0; MAX_DRIVER_INFO_SIZE as usize];
        driver_info[0..driver_info_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            driver_info_bytes.as_ptr() as *const std::ffi::c_char,
            driver_info_bytes.len(),
        ));
        crate::native::vulkan1_2::PhysicalDeviceDriverProperties {
            s_type: StructureType::PhysicalDeviceDriverProperties,
            p_next: std::ptr::null_mut(),
            driver_id: self.driver_id.into_low_level(context, bump),
            driver_name: driver_name,
            driver_info: driver_info,
            conformance_version: self.conformance_version.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDriverProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let driver_name_cstr = std::ffi::CStr::from_ptr(value.driver_name.as_ptr());
        let driver_name = driver_name_cstr.to_string_lossy().into_owned();
        let driver_info_cstr = std::ffi::CStr::from_ptr(value.driver_info.as_ptr());
        let driver_info = driver_info_cstr.to_string_lossy().into_owned();
        Self {
            driver_id: crate::conv::FromLowLevel::from_low_level(context, value.driver_id),
            driver_name: driver_name,
            driver_info: driver_info,
            conformance_version: crate::conv::FromLowLevel::from_low_level(context, value.conformance_version),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    #[doc(alias = "shaderSubgroupExtendedTypes")]
    pub shader_subgroup_extended_types: bool,
}
impl PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    ///Get a reference to the `shader_subgroup_extended_types` field.
    pub fn shader_subgroup_extended_types(&self) -> &bool {
        &self.shader_subgroup_extended_types
    }
    ///Get a mutable reference to the `shader_subgroup_extended_types` field.
    pub fn shader_subgroup_extended_types_mut(&mut self) -> &mut bool {
        &mut self.shader_subgroup_extended_types
    }
    ///Sets the `shader_subgroup_extended_types` field.
    pub fn set_shader_subgroup_extended_types(&mut self, shader_subgroup_extended_types: bool) -> &mut Self {
        self.shader_subgroup_extended_types = shader_subgroup_extended_types;
        self
    }
    ///Sets the `shader_subgroup_extended_types` field in a builder way.
    pub fn with_shader_subgroup_extended_types(mut self, shader_subgroup_extended_types: bool) -> Self {
        self.shader_subgroup_extended_types = shader_subgroup_extended_types;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
            s_type: StructureType::PhysicalDeviceShaderSubgroupExtendedTypesFeatures,
            p_next: std::ptr::null_mut(),
            shader_subgroup_extended_types: self.shader_subgroup_extended_types.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_subgroup_extended_types: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_subgroup_extended_types,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSamplerFilterMinmaxProperties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceSamplerFilterMinmaxProperties {
    #[doc(alias = "filterMinmaxSingleComponentFormats")]
    pub filter_minmax_single_component_formats: bool,
    #[doc(alias = "filterMinmaxImageComponentMapping")]
    pub filter_minmax_image_component_mapping: bool,
}
impl PhysicalDeviceSamplerFilterMinmaxProperties {
    ///Get a reference to the `filter_minmax_single_component_formats` field.
    pub fn filter_minmax_single_component_formats(&self) -> &bool {
        &self.filter_minmax_single_component_formats
    }
    ///Get a reference to the `filter_minmax_image_component_mapping` field.
    pub fn filter_minmax_image_component_mapping(&self) -> &bool {
        &self.filter_minmax_image_component_mapping
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSamplerFilterMinmaxProperties {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceSamplerFilterMinmaxProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceSamplerFilterMinmaxProperties {
            s_type: StructureType::PhysicalDeviceSamplerFilterMinmaxProperties,
            p_next: std::ptr::null_mut(),
            filter_minmax_single_component_formats: self
                .filter_minmax_single_component_formats
                .into_low_level(context, bump),
            filter_minmax_image_component_mapping: self
                .filter_minmax_image_component_mapping
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSamplerFilterMinmaxProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            filter_minmax_single_component_formats: crate::conv::FromLowLevel::from_low_level(
                context,
                value.filter_minmax_single_component_formats,
            ),
            filter_minmax_image_component_mapping: crate::conv::FromLowLevel::from_low_level(
                context,
                value.filter_minmax_image_component_mapping,
            ),
        }
    }
}
#[doc(alias = "VkSamplerReductionModeCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SamplerReductionModeCreateInfo {
    #[doc(alias = "reductionMode")]
    pub reduction_mode: SamplerReductionMode,
}
impl SamplerReductionModeCreateInfo {
    ///Get a reference to the `reduction_mode` field.
    pub fn reduction_mode(&self) -> SamplerReductionMode {
        self.reduction_mode
    }
    ///Get a mutable reference to the `reduction_mode` field.
    pub fn reduction_mode_mut(&mut self) -> &mut SamplerReductionMode {
        &mut self.reduction_mode
    }
    ///Sets the `reduction_mode` field.
    pub fn set_reduction_mode(&mut self, reduction_mode: SamplerReductionMode) -> &mut Self {
        self.reduction_mode = reduction_mode;
        self
    }
    ///Sets the `reduction_mode` field in a builder way.
    pub fn with_reduction_mode(mut self, reduction_mode: SamplerReductionMode) -> Self {
        self.reduction_mode = reduction_mode;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SamplerReductionModeCreateInfo {
    type LowLevel = crate::native::vulkan1_2::SamplerReductionModeCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::SamplerReductionModeCreateInfo {
            s_type: StructureType::SamplerReductionModeCreateInfo,
            p_next: std::ptr::null(),
            reduction_mode: self.reduction_mode.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SamplerReductionModeCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            reduction_mode: crate::conv::FromLowLevel::from_low_level(context, value.reduction_mode),
        }
    }
}
#[doc(alias = "VkImageFormatListCreateInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageFormatListCreateInfo {
    #[doc(alias = "pViewFormats")]
    pub view_formats: SmallVec<[Format; 8]>,
}
impl ImageFormatListCreateInfo {
    ///Get a reference to the `view_formats` field.
    pub fn view_formats(&self) -> &SmallVec<[Format; 8]> {
        &self.view_formats
    }
    ///Get a mutable reference to the `view_formats` field.
    pub fn view_formats_mut(&mut self) -> &mut SmallVec<[Format; 8]> {
        &mut self.view_formats
    }
    ///Sets the `view_formats` field.
    pub fn set_view_formats(&mut self, view_formats: SmallVec<[Format; 8]>) -> &mut Self {
        self.view_formats = view_formats;
        self
    }
    ///Sets the `view_formats` field in a builder way.
    pub fn with_view_formats(mut self, view_formats: SmallVec<[Format; 8]>) -> Self {
        self.view_formats = view_formats;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageFormatListCreateInfo {
    type LowLevel = crate::native::vulkan1_2::ImageFormatListCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_view_formats = self.view_formats.len() as u32;
        let view_formats = bump
            .alloc_slice_fill_iter(self.view_formats.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_2::ImageFormatListCreateInfo {
            s_type: StructureType::ImageFormatListCreateInfo,
            p_next: std::ptr::null(),
            view_format_count: len_view_formats,
            view_formats: view_formats,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageFormatListCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let view_formats_len = value.view_format_count;
        let mut view_formats = SmallVec::with_capacity(view_formats_len as usize);
        for i in 0..view_formats_len {
            view_formats.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.view_formats.add(i as usize).read(),
            ));
        }
        Self {
            view_formats: view_formats,
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderFloat16Int8Features")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderFloat16Int8Features {
    #[doc(alias = "shaderFloat16")]
    pub shader_float16: bool,
    #[doc(alias = "shaderInt8")]
    pub shader_int8: bool,
}
impl PhysicalDeviceShaderFloat16Int8Features {
    ///Get a reference to the `shader_float16` field.
    pub fn shader_float16(&self) -> &bool {
        &self.shader_float16
    }
    ///Get a reference to the `shader_int8` field.
    pub fn shader_int8(&self) -> &bool {
        &self.shader_int8
    }
    ///Get a mutable reference to the `shader_float16` field.
    pub fn shader_float16_mut(&mut self) -> &mut bool {
        &mut self.shader_float16
    }
    ///Get a mutable reference to the `shader_int8` field.
    pub fn shader_int8_mut(&mut self) -> &mut bool {
        &mut self.shader_int8
    }
    ///Sets the `shader_float16` field.
    pub fn set_shader_float16(&mut self, shader_float16: bool) -> &mut Self {
        self.shader_float16 = shader_float16;
        self
    }
    ///Sets the `shader_int8` field.
    pub fn set_shader_int8(&mut self, shader_int8: bool) -> &mut Self {
        self.shader_int8 = shader_int8;
        self
    }
    ///Sets the `shader_float16` field in a builder way.
    pub fn with_shader_float16(mut self, shader_float16: bool) -> Self {
        self.shader_float16 = shader_float16;
        self
    }
    ///Sets the `shader_int8` field in a builder way.
    pub fn with_shader_int8(mut self, shader_int8: bool) -> Self {
        self.shader_int8 = shader_int8;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderFloat16Int8Features {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceShaderFloat16Int8Features;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceShaderFloat16Int8Features {
            s_type: StructureType::PhysicalDeviceShaderFloat16Int8Features,
            p_next: std::ptr::null_mut(),
            shader_float16: self.shader_float16.into_low_level(context, bump),
            shader_int8: self.shader_int8.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderFloat16Int8Features {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_float16: crate::conv::FromLowLevel::from_low_level(context, value.shader_float16),
            shader_int8: crate::conv::FromLowLevel::from_low_level(context, value.shader_int8),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFloatControlsProperties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFloatControlsProperties {
    #[doc(alias = "denormBehaviorIndependence")]
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    #[doc(alias = "roundingModeIndependence")]
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat16")]
    pub shader_signed_zero_inf_nan_preserve_float16: bool,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat32")]
    pub shader_signed_zero_inf_nan_preserve_float32: bool,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat64")]
    pub shader_signed_zero_inf_nan_preserve_float64: bool,
    #[doc(alias = "shaderDenormPreserveFloat16")]
    pub shader_denorm_preserve_float16: bool,
    #[doc(alias = "shaderDenormPreserveFloat32")]
    pub shader_denorm_preserve_float32: bool,
    #[doc(alias = "shaderDenormPreserveFloat64")]
    pub shader_denorm_preserve_float64: bool,
    #[doc(alias = "shaderDenormFlushToZeroFloat16")]
    pub shader_denorm_flush_to_zero_float16: bool,
    #[doc(alias = "shaderDenormFlushToZeroFloat32")]
    pub shader_denorm_flush_to_zero_float32: bool,
    #[doc(alias = "shaderDenormFlushToZeroFloat64")]
    pub shader_denorm_flush_to_zero_float64: bool,
    #[doc(alias = "shaderRoundingModeRTEFloat16")]
    pub shader_rounding_mode_rte_float16: bool,
    #[doc(alias = "shaderRoundingModeRTEFloat32")]
    pub shader_rounding_mode_rte_float32: bool,
    #[doc(alias = "shaderRoundingModeRTEFloat64")]
    pub shader_rounding_mode_rte_float64: bool,
    #[doc(alias = "shaderRoundingModeRTZFloat16")]
    pub shader_rounding_mode_rtz_float16: bool,
    #[doc(alias = "shaderRoundingModeRTZFloat32")]
    pub shader_rounding_mode_rtz_float32: bool,
    #[doc(alias = "shaderRoundingModeRTZFloat64")]
    pub shader_rounding_mode_rtz_float64: bool,
}
impl PhysicalDeviceFloatControlsProperties {
    ///Get a reference to the `denorm_behavior_independence` field.
    pub fn denorm_behavior_independence(&self) -> ShaderFloatControlsIndependence {
        self.denorm_behavior_independence
    }
    ///Get a reference to the `rounding_mode_independence` field.
    pub fn rounding_mode_independence(&self) -> ShaderFloatControlsIndependence {
        self.rounding_mode_independence
    }
    ///Get a reference to the `shader_signed_zero_inf_nan_preserve_float16` field.
    pub fn shader_signed_zero_inf_nan_preserve_float16(&self) -> &bool {
        &self.shader_signed_zero_inf_nan_preserve_float16
    }
    ///Get a reference to the `shader_signed_zero_inf_nan_preserve_float32` field.
    pub fn shader_signed_zero_inf_nan_preserve_float32(&self) -> &bool {
        &self.shader_signed_zero_inf_nan_preserve_float32
    }
    ///Get a reference to the `shader_signed_zero_inf_nan_preserve_float64` field.
    pub fn shader_signed_zero_inf_nan_preserve_float64(&self) -> &bool {
        &self.shader_signed_zero_inf_nan_preserve_float64
    }
    ///Get a reference to the `shader_denorm_preserve_float16` field.
    pub fn shader_denorm_preserve_float16(&self) -> &bool {
        &self.shader_denorm_preserve_float16
    }
    ///Get a reference to the `shader_denorm_preserve_float32` field.
    pub fn shader_denorm_preserve_float32(&self) -> &bool {
        &self.shader_denorm_preserve_float32
    }
    ///Get a reference to the `shader_denorm_preserve_float64` field.
    pub fn shader_denorm_preserve_float64(&self) -> &bool {
        &self.shader_denorm_preserve_float64
    }
    ///Get a reference to the `shader_denorm_flush_to_zero_float16` field.
    pub fn shader_denorm_flush_to_zero_float16(&self) -> &bool {
        &self.shader_denorm_flush_to_zero_float16
    }
    ///Get a reference to the `shader_denorm_flush_to_zero_float32` field.
    pub fn shader_denorm_flush_to_zero_float32(&self) -> &bool {
        &self.shader_denorm_flush_to_zero_float32
    }
    ///Get a reference to the `shader_denorm_flush_to_zero_float64` field.
    pub fn shader_denorm_flush_to_zero_float64(&self) -> &bool {
        &self.shader_denorm_flush_to_zero_float64
    }
    ///Get a reference to the `shader_rounding_mode_rte_float16` field.
    pub fn shader_rounding_mode_rte_float16(&self) -> &bool {
        &self.shader_rounding_mode_rte_float16
    }
    ///Get a reference to the `shader_rounding_mode_rte_float32` field.
    pub fn shader_rounding_mode_rte_float32(&self) -> &bool {
        &self.shader_rounding_mode_rte_float32
    }
    ///Get a reference to the `shader_rounding_mode_rte_float64` field.
    pub fn shader_rounding_mode_rte_float64(&self) -> &bool {
        &self.shader_rounding_mode_rte_float64
    }
    ///Get a reference to the `shader_rounding_mode_rtz_float16` field.
    pub fn shader_rounding_mode_rtz_float16(&self) -> &bool {
        &self.shader_rounding_mode_rtz_float16
    }
    ///Get a reference to the `shader_rounding_mode_rtz_float32` field.
    pub fn shader_rounding_mode_rtz_float32(&self) -> &bool {
        &self.shader_rounding_mode_rtz_float32
    }
    ///Get a reference to the `shader_rounding_mode_rtz_float64` field.
    pub fn shader_rounding_mode_rtz_float64(&self) -> &bool {
        &self.shader_rounding_mode_rtz_float64
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFloatControlsProperties {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceFloatControlsProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceFloatControlsProperties {
            s_type: StructureType::PhysicalDeviceFloatControlsProperties,
            p_next: std::ptr::null_mut(),
            denorm_behavior_independence: self.denorm_behavior_independence.into_low_level(context, bump),
            rounding_mode_independence: self.rounding_mode_independence.into_low_level(context, bump),
            shader_signed_zero_inf_nan_preserve_float16: self
                .shader_signed_zero_inf_nan_preserve_float16
                .into_low_level(context, bump),
            shader_signed_zero_inf_nan_preserve_float32: self
                .shader_signed_zero_inf_nan_preserve_float32
                .into_low_level(context, bump),
            shader_signed_zero_inf_nan_preserve_float64: self
                .shader_signed_zero_inf_nan_preserve_float64
                .into_low_level(context, bump),
            shader_denorm_preserve_float16: self.shader_denorm_preserve_float16.into_low_level(context, bump),
            shader_denorm_preserve_float32: self.shader_denorm_preserve_float32.into_low_level(context, bump),
            shader_denorm_preserve_float64: self.shader_denorm_preserve_float64.into_low_level(context, bump),
            shader_denorm_flush_to_zero_float16: self.shader_denorm_flush_to_zero_float16.into_low_level(context, bump),
            shader_denorm_flush_to_zero_float32: self.shader_denorm_flush_to_zero_float32.into_low_level(context, bump),
            shader_denorm_flush_to_zero_float64: self.shader_denorm_flush_to_zero_float64.into_low_level(context, bump),
            shader_rounding_mode_rte_float16: self.shader_rounding_mode_rte_float16.into_low_level(context, bump),
            shader_rounding_mode_rte_float32: self.shader_rounding_mode_rte_float32.into_low_level(context, bump),
            shader_rounding_mode_rte_float64: self.shader_rounding_mode_rte_float64.into_low_level(context, bump),
            shader_rounding_mode_rtz_float16: self.shader_rounding_mode_rtz_float16.into_low_level(context, bump),
            shader_rounding_mode_rtz_float32: self.shader_rounding_mode_rtz_float32.into_low_level(context, bump),
            shader_rounding_mode_rtz_float64: self.shader_rounding_mode_rtz_float64.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFloatControlsProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            denorm_behavior_independence: crate::conv::FromLowLevel::from_low_level(
                context,
                value.denorm_behavior_independence,
            ),
            rounding_mode_independence: crate::conv::FromLowLevel::from_low_level(
                context,
                value.rounding_mode_independence,
            ),
            shader_signed_zero_inf_nan_preserve_float16: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_signed_zero_inf_nan_preserve_float16,
            ),
            shader_signed_zero_inf_nan_preserve_float32: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_signed_zero_inf_nan_preserve_float32,
            ),
            shader_signed_zero_inf_nan_preserve_float64: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_signed_zero_inf_nan_preserve_float64,
            ),
            shader_denorm_preserve_float16: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_denorm_preserve_float16,
            ),
            shader_denorm_preserve_float32: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_denorm_preserve_float32,
            ),
            shader_denorm_preserve_float64: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_denorm_preserve_float64,
            ),
            shader_denorm_flush_to_zero_float16: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_denorm_flush_to_zero_float16,
            ),
            shader_denorm_flush_to_zero_float32: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_denorm_flush_to_zero_float32,
            ),
            shader_denorm_flush_to_zero_float64: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_denorm_flush_to_zero_float64,
            ),
            shader_rounding_mode_rte_float16: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_rounding_mode_rte_float16,
            ),
            shader_rounding_mode_rte_float32: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_rounding_mode_rte_float32,
            ),
            shader_rounding_mode_rte_float64: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_rounding_mode_rte_float64,
            ),
            shader_rounding_mode_rtz_float16: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_rounding_mode_rtz_float16,
            ),
            shader_rounding_mode_rtz_float32: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_rounding_mode_rtz_float32,
            ),
            shader_rounding_mode_rtz_float64: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_rounding_mode_rtz_float64,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceHostQueryResetFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceHostQueryResetFeatures {
    #[doc(alias = "hostQueryReset")]
    pub host_query_reset: bool,
}
impl PhysicalDeviceHostQueryResetFeatures {
    ///Get a reference to the `host_query_reset` field.
    pub fn host_query_reset(&self) -> &bool {
        &self.host_query_reset
    }
    ///Get a mutable reference to the `host_query_reset` field.
    pub fn host_query_reset_mut(&mut self) -> &mut bool {
        &mut self.host_query_reset
    }
    ///Sets the `host_query_reset` field.
    pub fn set_host_query_reset(&mut self, host_query_reset: bool) -> &mut Self {
        self.host_query_reset = host_query_reset;
        self
    }
    ///Sets the `host_query_reset` field in a builder way.
    pub fn with_host_query_reset(mut self, host_query_reset: bool) -> Self {
        self.host_query_reset = host_query_reset;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceHostQueryResetFeatures {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceHostQueryResetFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceHostQueryResetFeatures {
            s_type: StructureType::PhysicalDeviceHostQueryResetFeatures,
            p_next: std::ptr::null_mut(),
            host_query_reset: self.host_query_reset.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceHostQueryResetFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            host_query_reset: crate::conv::FromLowLevel::from_low_level(context, value.host_query_reset),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDescriptorIndexingFeatures {
    #[doc(alias = "shaderInputAttachmentArrayDynamicIndexing")]
    pub shader_input_attachment_array_dynamic_indexing: bool,
    #[doc(alias = "shaderUniformTexelBufferArrayDynamicIndexing")]
    pub shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    #[doc(alias = "shaderStorageTexelBufferArrayDynamicIndexing")]
    pub shader_storage_texel_buffer_array_dynamic_indexing: bool,
    #[doc(alias = "shaderUniformBufferArrayNonUniformIndexing")]
    pub shader_uniform_buffer_array_non_uniform_indexing: bool,
    #[doc(alias = "shaderSampledImageArrayNonUniformIndexing")]
    pub shader_sampled_image_array_non_uniform_indexing: bool,
    #[doc(alias = "shaderStorageBufferArrayNonUniformIndexing")]
    pub shader_storage_buffer_array_non_uniform_indexing: bool,
    #[doc(alias = "shaderStorageImageArrayNonUniformIndexing")]
    pub shader_storage_image_array_non_uniform_indexing: bool,
    #[doc(alias = "shaderInputAttachmentArrayNonUniformIndexing")]
    pub shader_input_attachment_array_non_uniform_indexing: bool,
    #[doc(alias = "shaderUniformTexelBufferArrayNonUniformIndexing")]
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    #[doc(alias = "shaderStorageTexelBufferArrayNonUniformIndexing")]
    pub shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    #[doc(alias = "descriptorBindingUniformBufferUpdateAfterBind")]
    pub descriptor_binding_uniform_buffer_update_after_bind: bool,
    #[doc(alias = "descriptorBindingSampledImageUpdateAfterBind")]
    pub descriptor_binding_sampled_image_update_after_bind: bool,
    #[doc(alias = "descriptorBindingStorageImageUpdateAfterBind")]
    pub descriptor_binding_storage_image_update_after_bind: bool,
    #[doc(alias = "descriptorBindingStorageBufferUpdateAfterBind")]
    pub descriptor_binding_storage_buffer_update_after_bind: bool,
    #[doc(alias = "descriptorBindingUniformTexelBufferUpdateAfterBind")]
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    #[doc(alias = "descriptorBindingStorageTexelBufferUpdateAfterBind")]
    pub descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    #[doc(alias = "descriptorBindingUpdateUnusedWhilePending")]
    pub descriptor_binding_update_unused_while_pending: bool,
    #[doc(alias = "descriptorBindingPartiallyBound")]
    pub descriptor_binding_partially_bound: bool,
    #[doc(alias = "descriptorBindingVariableDescriptorCount")]
    pub descriptor_binding_variable_descriptor_count: bool,
    #[doc(alias = "runtimeDescriptorArray")]
    pub runtime_descriptor_array: bool,
}
impl PhysicalDeviceDescriptorIndexingFeatures {
    ///Get a reference to the `shader_input_attachment_array_dynamic_indexing` field.
    pub fn shader_input_attachment_array_dynamic_indexing(&self) -> &bool {
        &self.shader_input_attachment_array_dynamic_indexing
    }
    ///Get a reference to the `shader_uniform_texel_buffer_array_dynamic_indexing` field.
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(&self) -> &bool {
        &self.shader_uniform_texel_buffer_array_dynamic_indexing
    }
    ///Get a reference to the `shader_storage_texel_buffer_array_dynamic_indexing` field.
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(&self) -> &bool {
        &self.shader_storage_texel_buffer_array_dynamic_indexing
    }
    ///Get a reference to the `shader_uniform_buffer_array_non_uniform_indexing` field.
    pub fn shader_uniform_buffer_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_uniform_buffer_array_non_uniform_indexing
    }
    ///Get a reference to the `shader_sampled_image_array_non_uniform_indexing` field.
    pub fn shader_sampled_image_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_sampled_image_array_non_uniform_indexing
    }
    ///Get a reference to the `shader_storage_buffer_array_non_uniform_indexing` field.
    pub fn shader_storage_buffer_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_storage_buffer_array_non_uniform_indexing
    }
    ///Get a reference to the `shader_storage_image_array_non_uniform_indexing` field.
    pub fn shader_storage_image_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_storage_image_array_non_uniform_indexing
    }
    ///Get a reference to the `shader_input_attachment_array_non_uniform_indexing` field.
    pub fn shader_input_attachment_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_input_attachment_array_non_uniform_indexing
    }
    ///Get a reference to the `shader_uniform_texel_buffer_array_non_uniform_indexing` field.
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_uniform_texel_buffer_array_non_uniform_indexing
    }
    ///Get a reference to the `shader_storage_texel_buffer_array_non_uniform_indexing` field.
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_storage_texel_buffer_array_non_uniform_indexing
    }
    ///Get a reference to the `descriptor_binding_uniform_buffer_update_after_bind` field.
    pub fn descriptor_binding_uniform_buffer_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_uniform_buffer_update_after_bind
    }
    ///Get a reference to the `descriptor_binding_sampled_image_update_after_bind` field.
    pub fn descriptor_binding_sampled_image_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_sampled_image_update_after_bind
    }
    ///Get a reference to the `descriptor_binding_storage_image_update_after_bind` field.
    pub fn descriptor_binding_storage_image_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_storage_image_update_after_bind
    }
    ///Get a reference to the `descriptor_binding_storage_buffer_update_after_bind` field.
    pub fn descriptor_binding_storage_buffer_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_storage_buffer_update_after_bind
    }
    ///Get a reference to the `descriptor_binding_uniform_texel_buffer_update_after_bind` field.
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_uniform_texel_buffer_update_after_bind
    }
    ///Get a reference to the `descriptor_binding_storage_texel_buffer_update_after_bind` field.
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_storage_texel_buffer_update_after_bind
    }
    ///Get a reference to the `descriptor_binding_update_unused_while_pending` field.
    pub fn descriptor_binding_update_unused_while_pending(&self) -> &bool {
        &self.descriptor_binding_update_unused_while_pending
    }
    ///Get a reference to the `descriptor_binding_partially_bound` field.
    pub fn descriptor_binding_partially_bound(&self) -> &bool {
        &self.descriptor_binding_partially_bound
    }
    ///Get a reference to the `descriptor_binding_variable_descriptor_count` field.
    pub fn descriptor_binding_variable_descriptor_count(&self) -> &bool {
        &self.descriptor_binding_variable_descriptor_count
    }
    ///Get a reference to the `runtime_descriptor_array` field.
    pub fn runtime_descriptor_array(&self) -> &bool {
        &self.runtime_descriptor_array
    }
    ///Get a mutable reference to the `shader_input_attachment_array_dynamic_indexing` field.
    pub fn shader_input_attachment_array_dynamic_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_input_attachment_array_dynamic_indexing
    }
    ///Get a mutable reference to the `shader_uniform_texel_buffer_array_dynamic_indexing` field.
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_uniform_texel_buffer_array_dynamic_indexing
    }
    ///Get a mutable reference to the `shader_storage_texel_buffer_array_dynamic_indexing` field.
    pub fn shader_storage_texel_buffer_array_dynamic_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_storage_texel_buffer_array_dynamic_indexing
    }
    ///Get a mutable reference to the `shader_uniform_buffer_array_non_uniform_indexing` field.
    pub fn shader_uniform_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_uniform_buffer_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `shader_sampled_image_array_non_uniform_indexing` field.
    pub fn shader_sampled_image_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_sampled_image_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `shader_storage_buffer_array_non_uniform_indexing` field.
    pub fn shader_storage_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_storage_buffer_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `shader_storage_image_array_non_uniform_indexing` field.
    pub fn shader_storage_image_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_storage_image_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `shader_input_attachment_array_non_uniform_indexing` field.
    pub fn shader_input_attachment_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_input_attachment_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `shader_uniform_texel_buffer_array_non_uniform_indexing`
    /// field.
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_uniform_texel_buffer_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `shader_storage_texel_buffer_array_non_uniform_indexing`
    /// field.
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_storage_texel_buffer_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `descriptor_binding_uniform_buffer_update_after_bind` field.
    pub fn descriptor_binding_uniform_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_uniform_buffer_update_after_bind
    }
    ///Get a mutable reference to the `descriptor_binding_sampled_image_update_after_bind` field.
    pub fn descriptor_binding_sampled_image_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_sampled_image_update_after_bind
    }
    ///Get a mutable reference to the `descriptor_binding_storage_image_update_after_bind` field.
    pub fn descriptor_binding_storage_image_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_storage_image_update_after_bind
    }
    ///Get a mutable reference to the `descriptor_binding_storage_buffer_update_after_bind` field.
    pub fn descriptor_binding_storage_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_storage_buffer_update_after_bind
    }
    ///Get a mutable reference to the `descriptor_binding_uniform_texel_buffer_update_after_bind`
    /// field.
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_uniform_texel_buffer_update_after_bind
    }
    ///Get a mutable reference to the `descriptor_binding_storage_texel_buffer_update_after_bind`
    /// field.
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_storage_texel_buffer_update_after_bind
    }
    ///Get a mutable reference to the `descriptor_binding_update_unused_while_pending` field.
    pub fn descriptor_binding_update_unused_while_pending_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_update_unused_while_pending
    }
    ///Get a mutable reference to the `descriptor_binding_partially_bound` field.
    pub fn descriptor_binding_partially_bound_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_partially_bound
    }
    ///Get a mutable reference to the `descriptor_binding_variable_descriptor_count` field.
    pub fn descriptor_binding_variable_descriptor_count_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_variable_descriptor_count
    }
    ///Get a mutable reference to the `runtime_descriptor_array` field.
    pub fn runtime_descriptor_array_mut(&mut self) -> &mut bool {
        &mut self.runtime_descriptor_array
    }
    ///Sets the `shader_input_attachment_array_dynamic_indexing` field.
    pub fn set_shader_input_attachment_array_dynamic_indexing(
        &mut self,
        shader_input_attachment_array_dynamic_indexing: bool,
    ) -> &mut Self {
        self.shader_input_attachment_array_dynamic_indexing = shader_input_attachment_array_dynamic_indexing;
        self
    }
    ///Sets the `shader_uniform_texel_buffer_array_dynamic_indexing` field.
    pub fn set_shader_uniform_texel_buffer_array_dynamic_indexing(
        &mut self,
        shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    ) -> &mut Self {
        self.shader_uniform_texel_buffer_array_dynamic_indexing = shader_uniform_texel_buffer_array_dynamic_indexing;
        self
    }
    ///Sets the `shader_storage_texel_buffer_array_dynamic_indexing` field.
    pub fn set_shader_storage_texel_buffer_array_dynamic_indexing(
        &mut self,
        shader_storage_texel_buffer_array_dynamic_indexing: bool,
    ) -> &mut Self {
        self.shader_storage_texel_buffer_array_dynamic_indexing = shader_storage_texel_buffer_array_dynamic_indexing;
        self
    }
    ///Sets the `shader_uniform_buffer_array_non_uniform_indexing` field.
    pub fn set_shader_uniform_buffer_array_non_uniform_indexing(
        &mut self,
        shader_uniform_buffer_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_uniform_buffer_array_non_uniform_indexing = shader_uniform_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_sampled_image_array_non_uniform_indexing` field.
    pub fn set_shader_sampled_image_array_non_uniform_indexing(
        &mut self,
        shader_sampled_image_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_sampled_image_array_non_uniform_indexing = shader_sampled_image_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_storage_buffer_array_non_uniform_indexing` field.
    pub fn set_shader_storage_buffer_array_non_uniform_indexing(
        &mut self,
        shader_storage_buffer_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_storage_buffer_array_non_uniform_indexing = shader_storage_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_storage_image_array_non_uniform_indexing` field.
    pub fn set_shader_storage_image_array_non_uniform_indexing(
        &mut self,
        shader_storage_image_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_storage_image_array_non_uniform_indexing = shader_storage_image_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_input_attachment_array_non_uniform_indexing` field.
    pub fn set_shader_input_attachment_array_non_uniform_indexing(
        &mut self,
        shader_input_attachment_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_input_attachment_array_non_uniform_indexing = shader_input_attachment_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_uniform_texel_buffer_array_non_uniform_indexing` field.
    pub fn set_shader_uniform_texel_buffer_array_non_uniform_indexing(
        &mut self,
        shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_uniform_texel_buffer_array_non_uniform_indexing =
            shader_uniform_texel_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_storage_texel_buffer_array_non_uniform_indexing` field.
    pub fn set_shader_storage_texel_buffer_array_non_uniform_indexing(
        &mut self,
        shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_storage_texel_buffer_array_non_uniform_indexing =
            shader_storage_texel_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `descriptor_binding_uniform_buffer_update_after_bind` field.
    pub fn set_descriptor_binding_uniform_buffer_update_after_bind(
        &mut self,
        descriptor_binding_uniform_buffer_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_uniform_buffer_update_after_bind = descriptor_binding_uniform_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_sampled_image_update_after_bind` field.
    pub fn set_descriptor_binding_sampled_image_update_after_bind(
        &mut self,
        descriptor_binding_sampled_image_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_sampled_image_update_after_bind = descriptor_binding_sampled_image_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_storage_image_update_after_bind` field.
    pub fn set_descriptor_binding_storage_image_update_after_bind(
        &mut self,
        descriptor_binding_storage_image_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_storage_image_update_after_bind = descriptor_binding_storage_image_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_storage_buffer_update_after_bind` field.
    pub fn set_descriptor_binding_storage_buffer_update_after_bind(
        &mut self,
        descriptor_binding_storage_buffer_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_storage_buffer_update_after_bind = descriptor_binding_storage_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_uniform_texel_buffer_update_after_bind` field.
    pub fn set_descriptor_binding_uniform_texel_buffer_update_after_bind(
        &mut self,
        descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_uniform_texel_buffer_update_after_bind =
            descriptor_binding_uniform_texel_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_storage_texel_buffer_update_after_bind` field.
    pub fn set_descriptor_binding_storage_texel_buffer_update_after_bind(
        &mut self,
        descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_storage_texel_buffer_update_after_bind =
            descriptor_binding_storage_texel_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_update_unused_while_pending` field.
    pub fn set_descriptor_binding_update_unused_while_pending(
        &mut self,
        descriptor_binding_update_unused_while_pending: bool,
    ) -> &mut Self {
        self.descriptor_binding_update_unused_while_pending = descriptor_binding_update_unused_while_pending;
        self
    }
    ///Sets the `descriptor_binding_partially_bound` field.
    pub fn set_descriptor_binding_partially_bound(&mut self, descriptor_binding_partially_bound: bool) -> &mut Self {
        self.descriptor_binding_partially_bound = descriptor_binding_partially_bound;
        self
    }
    ///Sets the `descriptor_binding_variable_descriptor_count` field.
    pub fn set_descriptor_binding_variable_descriptor_count(
        &mut self,
        descriptor_binding_variable_descriptor_count: bool,
    ) -> &mut Self {
        self.descriptor_binding_variable_descriptor_count = descriptor_binding_variable_descriptor_count;
        self
    }
    ///Sets the `runtime_descriptor_array` field.
    pub fn set_runtime_descriptor_array(&mut self, runtime_descriptor_array: bool) -> &mut Self {
        self.runtime_descriptor_array = runtime_descriptor_array;
        self
    }
    ///Sets the `shader_input_attachment_array_dynamic_indexing` field in a builder way.
    pub fn with_shader_input_attachment_array_dynamic_indexing(
        mut self,
        shader_input_attachment_array_dynamic_indexing: bool,
    ) -> Self {
        self.shader_input_attachment_array_dynamic_indexing = shader_input_attachment_array_dynamic_indexing;
        self
    }
    ///Sets the `shader_uniform_texel_buffer_array_dynamic_indexing` field in a builder way.
    pub fn with_shader_uniform_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.shader_uniform_texel_buffer_array_dynamic_indexing = shader_uniform_texel_buffer_array_dynamic_indexing;
        self
    }
    ///Sets the `shader_storage_texel_buffer_array_dynamic_indexing` field in a builder way.
    pub fn with_shader_storage_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_storage_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.shader_storage_texel_buffer_array_dynamic_indexing = shader_storage_texel_buffer_array_dynamic_indexing;
        self
    }
    ///Sets the `shader_uniform_buffer_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_uniform_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_uniform_buffer_array_non_uniform_indexing = shader_uniform_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_sampled_image_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_sampled_image_array_non_uniform_indexing(
        mut self,
        shader_sampled_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_sampled_image_array_non_uniform_indexing = shader_sampled_image_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_storage_buffer_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_storage_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_storage_buffer_array_non_uniform_indexing = shader_storage_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_storage_image_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_storage_image_array_non_uniform_indexing(
        mut self,
        shader_storage_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_storage_image_array_non_uniform_indexing = shader_storage_image_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_input_attachment_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_input_attachment_array_non_uniform_indexing(
        mut self,
        shader_input_attachment_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_input_attachment_array_non_uniform_indexing = shader_input_attachment_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_uniform_texel_buffer_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_uniform_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_uniform_texel_buffer_array_non_uniform_indexing =
            shader_uniform_texel_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_storage_texel_buffer_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_storage_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_storage_texel_buffer_array_non_uniform_indexing =
            shader_storage_texel_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `descriptor_binding_uniform_buffer_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_uniform_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_buffer_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_uniform_buffer_update_after_bind = descriptor_binding_uniform_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_sampled_image_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_sampled_image_update_after_bind(
        mut self,
        descriptor_binding_sampled_image_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_sampled_image_update_after_bind = descriptor_binding_sampled_image_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_storage_image_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_storage_image_update_after_bind(
        mut self,
        descriptor_binding_storage_image_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_storage_image_update_after_bind = descriptor_binding_storage_image_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_storage_buffer_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_storage_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_buffer_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_storage_buffer_update_after_bind = descriptor_binding_storage_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_uniform_texel_buffer_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_uniform_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_uniform_texel_buffer_update_after_bind =
            descriptor_binding_uniform_texel_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_storage_texel_buffer_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_storage_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_storage_texel_buffer_update_after_bind =
            descriptor_binding_storage_texel_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_update_unused_while_pending` field in a builder way.
    pub fn with_descriptor_binding_update_unused_while_pending(
        mut self,
        descriptor_binding_update_unused_while_pending: bool,
    ) -> Self {
        self.descriptor_binding_update_unused_while_pending = descriptor_binding_update_unused_while_pending;
        self
    }
    ///Sets the `descriptor_binding_partially_bound` field in a builder way.
    pub fn with_descriptor_binding_partially_bound(mut self, descriptor_binding_partially_bound: bool) -> Self {
        self.descriptor_binding_partially_bound = descriptor_binding_partially_bound;
        self
    }
    ///Sets the `descriptor_binding_variable_descriptor_count` field in a builder way.
    pub fn with_descriptor_binding_variable_descriptor_count(
        mut self,
        descriptor_binding_variable_descriptor_count: bool,
    ) -> Self {
        self.descriptor_binding_variable_descriptor_count = descriptor_binding_variable_descriptor_count;
        self
    }
    ///Sets the `runtime_descriptor_array` field in a builder way.
    pub fn with_runtime_descriptor_array(mut self, runtime_descriptor_array: bool) -> Self {
        self.runtime_descriptor_array = runtime_descriptor_array;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDescriptorIndexingFeatures {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceDescriptorIndexingFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceDescriptorIndexingFeatures {
            s_type: StructureType::PhysicalDeviceDescriptorIndexingFeatures,
            p_next: std::ptr::null_mut(),
            shader_input_attachment_array_dynamic_indexing: self
                .shader_input_attachment_array_dynamic_indexing
                .into_low_level(context, bump),
            shader_uniform_texel_buffer_array_dynamic_indexing: self
                .shader_uniform_texel_buffer_array_dynamic_indexing
                .into_low_level(context, bump),
            shader_storage_texel_buffer_array_dynamic_indexing: self
                .shader_storage_texel_buffer_array_dynamic_indexing
                .into_low_level(context, bump),
            shader_uniform_buffer_array_non_uniform_indexing: self
                .shader_uniform_buffer_array_non_uniform_indexing
                .into_low_level(context, bump),
            shader_sampled_image_array_non_uniform_indexing: self
                .shader_sampled_image_array_non_uniform_indexing
                .into_low_level(context, bump),
            shader_storage_buffer_array_non_uniform_indexing: self
                .shader_storage_buffer_array_non_uniform_indexing
                .into_low_level(context, bump),
            shader_storage_image_array_non_uniform_indexing: self
                .shader_storage_image_array_non_uniform_indexing
                .into_low_level(context, bump),
            shader_input_attachment_array_non_uniform_indexing: self
                .shader_input_attachment_array_non_uniform_indexing
                .into_low_level(context, bump),
            shader_uniform_texel_buffer_array_non_uniform_indexing: self
                .shader_uniform_texel_buffer_array_non_uniform_indexing
                .into_low_level(context, bump),
            shader_storage_texel_buffer_array_non_uniform_indexing: self
                .shader_storage_texel_buffer_array_non_uniform_indexing
                .into_low_level(context, bump),
            descriptor_binding_uniform_buffer_update_after_bind: self
                .descriptor_binding_uniform_buffer_update_after_bind
                .into_low_level(context, bump),
            descriptor_binding_sampled_image_update_after_bind: self
                .descriptor_binding_sampled_image_update_after_bind
                .into_low_level(context, bump),
            descriptor_binding_storage_image_update_after_bind: self
                .descriptor_binding_storage_image_update_after_bind
                .into_low_level(context, bump),
            descriptor_binding_storage_buffer_update_after_bind: self
                .descriptor_binding_storage_buffer_update_after_bind
                .into_low_level(context, bump),
            descriptor_binding_uniform_texel_buffer_update_after_bind: self
                .descriptor_binding_uniform_texel_buffer_update_after_bind
                .into_low_level(context, bump),
            descriptor_binding_storage_texel_buffer_update_after_bind: self
                .descriptor_binding_storage_texel_buffer_update_after_bind
                .into_low_level(context, bump),
            descriptor_binding_update_unused_while_pending: self
                .descriptor_binding_update_unused_while_pending
                .into_low_level(context, bump),
            descriptor_binding_partially_bound: self.descriptor_binding_partially_bound.into_low_level(context, bump),
            descriptor_binding_variable_descriptor_count: self
                .descriptor_binding_variable_descriptor_count
                .into_low_level(context, bump),
            runtime_descriptor_array: self.runtime_descriptor_array.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDescriptorIndexingFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_input_attachment_array_dynamic_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_input_attachment_array_dynamic_indexing,
            ),
            shader_uniform_texel_buffer_array_dynamic_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_uniform_texel_buffer_array_dynamic_indexing,
            ),
            shader_storage_texel_buffer_array_dynamic_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_storage_texel_buffer_array_dynamic_indexing,
            ),
            shader_uniform_buffer_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_uniform_buffer_array_non_uniform_indexing,
            ),
            shader_sampled_image_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_sampled_image_array_non_uniform_indexing,
            ),
            shader_storage_buffer_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_storage_buffer_array_non_uniform_indexing,
            ),
            shader_storage_image_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_storage_image_array_non_uniform_indexing,
            ),
            shader_input_attachment_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_input_attachment_array_non_uniform_indexing,
            ),
            shader_uniform_texel_buffer_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_uniform_texel_buffer_array_non_uniform_indexing,
            ),
            shader_storage_texel_buffer_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_storage_texel_buffer_array_non_uniform_indexing,
            ),
            descriptor_binding_uniform_buffer_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_uniform_buffer_update_after_bind,
            ),
            descriptor_binding_sampled_image_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_sampled_image_update_after_bind,
            ),
            descriptor_binding_storage_image_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_storage_image_update_after_bind,
            ),
            descriptor_binding_storage_buffer_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_storage_buffer_update_after_bind,
            ),
            descriptor_binding_uniform_texel_buffer_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_uniform_texel_buffer_update_after_bind,
            ),
            descriptor_binding_storage_texel_buffer_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_storage_texel_buffer_update_after_bind,
            ),
            descriptor_binding_update_unused_while_pending: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_update_unused_while_pending,
            ),
            descriptor_binding_partially_bound: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_partially_bound,
            ),
            descriptor_binding_variable_descriptor_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_variable_descriptor_count,
            ),
            runtime_descriptor_array: crate::conv::FromLowLevel::from_low_level(
                context,
                value.runtime_descriptor_array,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingProperties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDescriptorIndexingProperties {
    #[doc(alias = "maxUpdateAfterBindDescriptorsInAllPools")]
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    #[doc(alias = "shaderUniformBufferArrayNonUniformIndexingNative")]
    pub shader_uniform_buffer_array_non_uniform_indexing_native: bool,
    #[doc(alias = "shaderSampledImageArrayNonUniformIndexingNative")]
    pub shader_sampled_image_array_non_uniform_indexing_native: bool,
    #[doc(alias = "shaderStorageBufferArrayNonUniformIndexingNative")]
    pub shader_storage_buffer_array_non_uniform_indexing_native: bool,
    #[doc(alias = "shaderStorageImageArrayNonUniformIndexingNative")]
    pub shader_storage_image_array_non_uniform_indexing_native: bool,
    #[doc(alias = "shaderInputAttachmentArrayNonUniformIndexingNative")]
    pub shader_input_attachment_array_non_uniform_indexing_native: bool,
    #[doc(alias = "robustBufferAccessUpdateAfterBind")]
    pub robust_buffer_access_update_after_bind: bool,
    #[doc(alias = "quadDivergentImplicitLod")]
    pub quad_divergent_implicit_lod: bool,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindSamplers")]
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindUniformBuffers")]
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindStorageBuffers")]
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindSampledImages")]
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindStorageImages")]
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindInputAttachments")]
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    #[doc(alias = "maxPerStageUpdateAfterBindResources")]
    pub max_per_stage_update_after_bind_resources: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindSamplers")]
    pub max_descriptor_set_update_after_bind_samplers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindUniformBuffers")]
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindUniformBuffersDynamic")]
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageBuffers")]
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageBuffersDynamic")]
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindSampledImages")]
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageImages")]
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindInputAttachments")]
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
}
impl PhysicalDeviceDescriptorIndexingProperties {
    ///Get a reference to the `max_update_after_bind_descriptors_in_all_pools` field.
    pub fn max_update_after_bind_descriptors_in_all_pools(&self) -> u32 {
        self.max_update_after_bind_descriptors_in_all_pools
    }
    ///Get a reference to the `shader_uniform_buffer_array_non_uniform_indexing_native` field.
    pub fn shader_uniform_buffer_array_non_uniform_indexing_native(&self) -> &bool {
        &self.shader_uniform_buffer_array_non_uniform_indexing_native
    }
    ///Get a reference to the `shader_sampled_image_array_non_uniform_indexing_native` field.
    pub fn shader_sampled_image_array_non_uniform_indexing_native(&self) -> &bool {
        &self.shader_sampled_image_array_non_uniform_indexing_native
    }
    ///Get a reference to the `shader_storage_buffer_array_non_uniform_indexing_native` field.
    pub fn shader_storage_buffer_array_non_uniform_indexing_native(&self) -> &bool {
        &self.shader_storage_buffer_array_non_uniform_indexing_native
    }
    ///Get a reference to the `shader_storage_image_array_non_uniform_indexing_native` field.
    pub fn shader_storage_image_array_non_uniform_indexing_native(&self) -> &bool {
        &self.shader_storage_image_array_non_uniform_indexing_native
    }
    ///Get a reference to the `shader_input_attachment_array_non_uniform_indexing_native` field.
    pub fn shader_input_attachment_array_non_uniform_indexing_native(&self) -> &bool {
        &self.shader_input_attachment_array_non_uniform_indexing_native
    }
    ///Get a reference to the `robust_buffer_access_update_after_bind` field.
    pub fn robust_buffer_access_update_after_bind(&self) -> &bool {
        &self.robust_buffer_access_update_after_bind
    }
    ///Get a reference to the `quad_divergent_implicit_lod` field.
    pub fn quad_divergent_implicit_lod(&self) -> &bool {
        &self.quad_divergent_implicit_lod
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_samplers` field.
    pub fn max_per_stage_descriptor_update_after_bind_samplers(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_samplers
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_uniform_buffers` field.
    pub fn max_per_stage_descriptor_update_after_bind_uniform_buffers(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_uniform_buffers
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_storage_buffers` field.
    pub fn max_per_stage_descriptor_update_after_bind_storage_buffers(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_storage_buffers
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_sampled_images` field.
    pub fn max_per_stage_descriptor_update_after_bind_sampled_images(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_sampled_images
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_storage_images` field.
    pub fn max_per_stage_descriptor_update_after_bind_storage_images(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_storage_images
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_input_attachments` field.
    pub fn max_per_stage_descriptor_update_after_bind_input_attachments(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_input_attachments
    }
    ///Get a reference to the `max_per_stage_update_after_bind_resources` field.
    pub fn max_per_stage_update_after_bind_resources(&self) -> u32 {
        self.max_per_stage_update_after_bind_resources
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_samplers` field.
    pub fn max_descriptor_set_update_after_bind_samplers(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_samplers
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_uniform_buffers` field.
    pub fn max_descriptor_set_update_after_bind_uniform_buffers(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_uniform_buffers
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_uniform_buffers_dynamic` field.
    pub fn max_descriptor_set_update_after_bind_uniform_buffers_dynamic(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_storage_buffers` field.
    pub fn max_descriptor_set_update_after_bind_storage_buffers(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_storage_buffers
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_storage_buffers_dynamic` field.
    pub fn max_descriptor_set_update_after_bind_storage_buffers_dynamic(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_storage_buffers_dynamic
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_sampled_images` field.
    pub fn max_descriptor_set_update_after_bind_sampled_images(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_sampled_images
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_storage_images` field.
    pub fn max_descriptor_set_update_after_bind_storage_images(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_storage_images
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_input_attachments` field.
    pub fn max_descriptor_set_update_after_bind_input_attachments(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_input_attachments
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDescriptorIndexingProperties {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceDescriptorIndexingProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceDescriptorIndexingProperties {
            s_type: StructureType::PhysicalDeviceDescriptorIndexingProperties,
            p_next: std::ptr::null_mut(),
            max_update_after_bind_descriptors_in_all_pools: self
                .max_update_after_bind_descriptors_in_all_pools
                .into_low_level(context, bump),
            shader_uniform_buffer_array_non_uniform_indexing_native: self
                .shader_uniform_buffer_array_non_uniform_indexing_native
                .into_low_level(context, bump),
            shader_sampled_image_array_non_uniform_indexing_native: self
                .shader_sampled_image_array_non_uniform_indexing_native
                .into_low_level(context, bump),
            shader_storage_buffer_array_non_uniform_indexing_native: self
                .shader_storage_buffer_array_non_uniform_indexing_native
                .into_low_level(context, bump),
            shader_storage_image_array_non_uniform_indexing_native: self
                .shader_storage_image_array_non_uniform_indexing_native
                .into_low_level(context, bump),
            shader_input_attachment_array_non_uniform_indexing_native: self
                .shader_input_attachment_array_non_uniform_indexing_native
                .into_low_level(context, bump),
            robust_buffer_access_update_after_bind: self
                .robust_buffer_access_update_after_bind
                .into_low_level(context, bump),
            quad_divergent_implicit_lod: self.quad_divergent_implicit_lod.into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_samplers: self
                .max_per_stage_descriptor_update_after_bind_samplers
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_uniform_buffers: self
                .max_per_stage_descriptor_update_after_bind_uniform_buffers
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_storage_buffers: self
                .max_per_stage_descriptor_update_after_bind_storage_buffers
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_sampled_images: self
                .max_per_stage_descriptor_update_after_bind_sampled_images
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_storage_images: self
                .max_per_stage_descriptor_update_after_bind_storage_images
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_input_attachments: self
                .max_per_stage_descriptor_update_after_bind_input_attachments
                .into_low_level(context, bump),
            max_per_stage_update_after_bind_resources: self
                .max_per_stage_update_after_bind_resources
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_samplers: self
                .max_descriptor_set_update_after_bind_samplers
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_uniform_buffers: self
                .max_descriptor_set_update_after_bind_uniform_buffers
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: self
                .max_descriptor_set_update_after_bind_uniform_buffers_dynamic
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_storage_buffers: self
                .max_descriptor_set_update_after_bind_storage_buffers
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: self
                .max_descriptor_set_update_after_bind_storage_buffers_dynamic
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_sampled_images: self
                .max_descriptor_set_update_after_bind_sampled_images
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_storage_images: self
                .max_descriptor_set_update_after_bind_storage_images
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_input_attachments: self
                .max_descriptor_set_update_after_bind_input_attachments
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDescriptorIndexingProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_update_after_bind_descriptors_in_all_pools: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_update_after_bind_descriptors_in_all_pools,
            ),
            shader_uniform_buffer_array_non_uniform_indexing_native: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_uniform_buffer_array_non_uniform_indexing_native,
            ),
            shader_sampled_image_array_non_uniform_indexing_native: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_sampled_image_array_non_uniform_indexing_native,
            ),
            shader_storage_buffer_array_non_uniform_indexing_native: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_storage_buffer_array_non_uniform_indexing_native,
            ),
            shader_storage_image_array_non_uniform_indexing_native: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_storage_image_array_non_uniform_indexing_native,
            ),
            shader_input_attachment_array_non_uniform_indexing_native: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_input_attachment_array_non_uniform_indexing_native,
            ),
            robust_buffer_access_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.robust_buffer_access_update_after_bind,
            ),
            quad_divergent_implicit_lod: crate::conv::FromLowLevel::from_low_level(
                context,
                value.quad_divergent_implicit_lod,
            ),
            max_per_stage_descriptor_update_after_bind_samplers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_samplers,
            ),
            max_per_stage_descriptor_update_after_bind_uniform_buffers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_uniform_buffers,
            ),
            max_per_stage_descriptor_update_after_bind_storage_buffers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_storage_buffers,
            ),
            max_per_stage_descriptor_update_after_bind_sampled_images: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_sampled_images,
            ),
            max_per_stage_descriptor_update_after_bind_storage_images: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_storage_images,
            ),
            max_per_stage_descriptor_update_after_bind_input_attachments: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_input_attachments,
            ),
            max_per_stage_update_after_bind_resources: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_update_after_bind_resources,
            ),
            max_descriptor_set_update_after_bind_samplers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_samplers,
            ),
            max_descriptor_set_update_after_bind_uniform_buffers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_uniform_buffers,
            ),
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_uniform_buffers_dynamic,
            ),
            max_descriptor_set_update_after_bind_storage_buffers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_storage_buffers,
            ),
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_storage_buffers_dynamic,
            ),
            max_descriptor_set_update_after_bind_sampled_images: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_sampled_images,
            ),
            max_descriptor_set_update_after_bind_storage_images: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_storage_images,
            ),
            max_descriptor_set_update_after_bind_input_attachments: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_input_attachments,
            ),
        }
    }
}
#[doc(alias = "VkDescriptorSetLayoutBindingFlagsCreateInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DescriptorSetLayoutBindingFlagsCreateInfo {
    #[doc(alias = "pBindingFlags")]
    pub binding_flags: SmallVec<[DescriptorBindingFlags; 8]>,
}
impl DescriptorSetLayoutBindingFlagsCreateInfo {
    ///Get a reference to the `binding_flags` field.
    pub fn binding_flags(&self) -> &SmallVec<[DescriptorBindingFlags; 8]> {
        &self.binding_flags
    }
    ///Get a mutable reference to the `binding_flags` field.
    pub fn binding_flags_mut(&mut self) -> &mut SmallVec<[DescriptorBindingFlags; 8]> {
        &mut self.binding_flags
    }
    ///Sets the `binding_flags` field.
    pub fn set_binding_flags(&mut self, binding_flags: SmallVec<[DescriptorBindingFlags; 8]>) -> &mut Self {
        self.binding_flags = binding_flags;
        self
    }
    ///Sets the `binding_flags` field in a builder way.
    pub fn with_binding_flags(mut self, binding_flags: SmallVec<[DescriptorBindingFlags; 8]>) -> Self {
        self.binding_flags = binding_flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DescriptorSetLayoutBindingFlagsCreateInfo {
    type LowLevel = crate::native::vulkan1_2::DescriptorSetLayoutBindingFlagsCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_binding_flags = self.binding_flags.len() as u32;
        let binding_flags = bump
            .alloc_slice_fill_iter(self.binding_flags.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_2::DescriptorSetLayoutBindingFlagsCreateInfo {
            s_type: StructureType::DescriptorSetLayoutBindingFlagsCreateInfo,
            p_next: std::ptr::null(),
            binding_count: len_binding_flags,
            binding_flags: binding_flags,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DescriptorSetLayoutBindingFlagsCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let binding_flags_len = value.binding_count;
        let mut binding_flags = SmallVec::with_capacity(binding_flags_len as usize);
        for i in 0..binding_flags_len {
            binding_flags.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.binding_flags.add(i as usize).read(),
            ));
        }
        Self {
            binding_flags: binding_flags,
        }
    }
}
#[doc(alias = "VkDescriptorSetVariableDescriptorCountAllocateInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DescriptorSetVariableDescriptorCountAllocateInfo {
    #[doc(alias = "pDescriptorCounts")]
    pub descriptor_counts: SmallVec<[u32; 8]>,
}
impl DescriptorSetVariableDescriptorCountAllocateInfo {
    ///Get a reference to the `descriptor_counts` field.
    pub fn descriptor_counts(&self) -> &SmallVec<[u32; 8]> {
        &self.descriptor_counts
    }
    ///Get a mutable reference to the `descriptor_counts` field.
    pub fn descriptor_counts_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.descriptor_counts
    }
    ///Sets the `descriptor_counts` field.
    pub fn set_descriptor_counts(&mut self, descriptor_counts: SmallVec<[u32; 8]>) -> &mut Self {
        self.descriptor_counts = descriptor_counts;
        self
    }
    ///Sets the `descriptor_counts` field in a builder way.
    pub fn with_descriptor_counts(mut self, descriptor_counts: SmallVec<[u32; 8]>) -> Self {
        self.descriptor_counts = descriptor_counts;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DescriptorSetVariableDescriptorCountAllocateInfo {
    type LowLevel = crate::native::vulkan1_2::DescriptorSetVariableDescriptorCountAllocateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_descriptor_counts = self.descriptor_counts.len() as u32;
        let descriptor_counts = bump
            .alloc_slice_fill_iter(self.descriptor_counts.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_2::DescriptorSetVariableDescriptorCountAllocateInfo {
            s_type: StructureType::DescriptorSetVariableDescriptorCountAllocateInfo,
            p_next: std::ptr::null(),
            descriptor_set_count: len_descriptor_counts,
            descriptor_counts: descriptor_counts,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DescriptorSetVariableDescriptorCountAllocateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let descriptor_counts_len = value.descriptor_set_count;
        let mut descriptor_counts = SmallVec::with_capacity(descriptor_counts_len as usize);
        for i in 0..descriptor_counts_len {
            descriptor_counts.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_counts.add(i as usize).read(),
            ));
        }
        Self {
            descriptor_counts: descriptor_counts,
        }
    }
}
#[doc(alias = "VkDescriptorSetVariableDescriptorCountLayoutSupport")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DescriptorSetVariableDescriptorCountLayoutSupport {
    #[doc(alias = "maxVariableDescriptorCount")]
    pub max_variable_descriptor_count: u32,
}
impl DescriptorSetVariableDescriptorCountLayoutSupport {
    ///Get a reference to the `max_variable_descriptor_count` field.
    pub fn max_variable_descriptor_count(&self) -> u32 {
        self.max_variable_descriptor_count
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DescriptorSetVariableDescriptorCountLayoutSupport {
    type LowLevel = crate::native::vulkan1_2::DescriptorSetVariableDescriptorCountLayoutSupport;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::DescriptorSetVariableDescriptorCountLayoutSupport {
            s_type: StructureType::DescriptorSetVariableDescriptorCountLayoutSupport,
            p_next: std::ptr::null_mut(),
            max_variable_descriptor_count: self.max_variable_descriptor_count.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DescriptorSetVariableDescriptorCountLayoutSupport {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_variable_descriptor_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_variable_descriptor_count,
            ),
        }
    }
}
#[doc(alias = "VkAttachmentDescription2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttachmentDescription2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[AttachmentDescription2Extension; 1]>,
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlagBits,
    #[doc(alias = "loadOp")]
    pub load_op: AttachmentLoadOp,
    #[doc(alias = "storeOp")]
    pub store_op: AttachmentStoreOp,
    #[doc(alias = "stencilLoadOp")]
    pub stencil_load_op: AttachmentLoadOp,
    #[doc(alias = "stencilStoreOp")]
    pub stencil_store_op: AttachmentStoreOp,
    #[doc(alias = "initialLayout")]
    pub initial_layout: ImageLayout,
    #[doc(alias = "finalLayout")]
    pub final_layout: ImageLayout,
}
impl AttachmentDescription2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<AttachmentDescription2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[AttachmentDescription2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> AttachmentDescriptionFlags {
        self.flags
    }
    ///Get a reference to the `format` field.
    pub fn format(&self) -> Format {
        self.format
    }
    ///Get a reference to the `samples` field.
    pub fn samples(&self) -> SampleCountFlagBits {
        self.samples
    }
    ///Get a reference to the `load_op` field.
    pub fn load_op(&self) -> AttachmentLoadOp {
        self.load_op
    }
    ///Get a reference to the `store_op` field.
    pub fn store_op(&self) -> AttachmentStoreOp {
        self.store_op
    }
    ///Get a reference to the `stencil_load_op` field.
    pub fn stencil_load_op(&self) -> AttachmentLoadOp {
        self.stencil_load_op
    }
    ///Get a reference to the `stencil_store_op` field.
    pub fn stencil_store_op(&self) -> AttachmentStoreOp {
        self.stencil_store_op
    }
    ///Get a reference to the `initial_layout` field.
    pub fn initial_layout(&self) -> ImageLayout {
        self.initial_layout
    }
    ///Get a reference to the `final_layout` field.
    pub fn final_layout(&self) -> ImageLayout {
        self.final_layout
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[AttachmentDescription2Extension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut AttachmentDescriptionFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `format` field.
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Get a mutable reference to the `samples` field.
    pub fn samples_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.samples
    }
    ///Get a mutable reference to the `load_op` field.
    pub fn load_op_mut(&mut self) -> &mut AttachmentLoadOp {
        &mut self.load_op
    }
    ///Get a mutable reference to the `store_op` field.
    pub fn store_op_mut(&mut self) -> &mut AttachmentStoreOp {
        &mut self.store_op
    }
    ///Get a mutable reference to the `stencil_load_op` field.
    pub fn stencil_load_op_mut(&mut self) -> &mut AttachmentLoadOp {
        &mut self.stencil_load_op
    }
    ///Get a mutable reference to the `stencil_store_op` field.
    pub fn stencil_store_op_mut(&mut self) -> &mut AttachmentStoreOp {
        &mut self.stencil_store_op
    }
    ///Get a mutable reference to the `initial_layout` field.
    pub fn initial_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.initial_layout
    }
    ///Get a mutable reference to the `final_layout` field.
    pub fn final_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.final_layout
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[AttachmentDescription2Extension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: AttachmentDescriptionFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `format` field.
    pub fn set_format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }
    ///Sets the `samples` field.
    pub fn set_samples(&mut self, samples: SampleCountFlagBits) -> &mut Self {
        self.samples = samples;
        self
    }
    ///Sets the `load_op` field.
    pub fn set_load_op(&mut self, load_op: AttachmentLoadOp) -> &mut Self {
        self.load_op = load_op;
        self
    }
    ///Sets the `store_op` field.
    pub fn set_store_op(&mut self, store_op: AttachmentStoreOp) -> &mut Self {
        self.store_op = store_op;
        self
    }
    ///Sets the `stencil_load_op` field.
    pub fn set_stencil_load_op(&mut self, stencil_load_op: AttachmentLoadOp) -> &mut Self {
        self.stencil_load_op = stencil_load_op;
        self
    }
    ///Sets the `stencil_store_op` field.
    pub fn set_stencil_store_op(&mut self, stencil_store_op: AttachmentStoreOp) -> &mut Self {
        self.stencil_store_op = stencil_store_op;
        self
    }
    ///Sets the `initial_layout` field.
    pub fn set_initial_layout(&mut self, initial_layout: ImageLayout) -> &mut Self {
        self.initial_layout = initial_layout;
        self
    }
    ///Sets the `final_layout` field.
    pub fn set_final_layout(&mut self, final_layout: ImageLayout) -> &mut Self {
        self.final_layout = final_layout;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[AttachmentDescription2Extension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: AttachmentDescriptionFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `format` field in a builder way.
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }
    ///Sets the `samples` field in a builder way.
    pub fn with_samples(mut self, samples: SampleCountFlagBits) -> Self {
        self.samples = samples;
        self
    }
    ///Sets the `load_op` field in a builder way.
    pub fn with_load_op(mut self, load_op: AttachmentLoadOp) -> Self {
        self.load_op = load_op;
        self
    }
    ///Sets the `store_op` field in a builder way.
    pub fn with_store_op(mut self, store_op: AttachmentStoreOp) -> Self {
        self.store_op = store_op;
        self
    }
    ///Sets the `stencil_load_op` field in a builder way.
    pub fn with_stencil_load_op(mut self, stencil_load_op: AttachmentLoadOp) -> Self {
        self.stencil_load_op = stencil_load_op;
        self
    }
    ///Sets the `stencil_store_op` field in a builder way.
    pub fn with_stencil_store_op(mut self, stencil_store_op: AttachmentStoreOp) -> Self {
        self.stencil_store_op = stencil_store_op;
        self
    }
    ///Sets the `initial_layout` field in a builder way.
    pub fn with_initial_layout(mut self, initial_layout: ImageLayout) -> Self {
        self.initial_layout = initial_layout;
        self
    }
    ///Sets the `final_layout` field in a builder way.
    pub fn with_final_layout(mut self, final_layout: ImageLayout) -> Self {
        self.final_layout = final_layout;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AttachmentDescription2 {
    type LowLevel = crate::native::vulkan1_2::AttachmentDescription2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_2::AttachmentDescription2 {
            s_type: StructureType::AttachmentDescription2,
            p_next: next,
            flags: self.flags.into_low_level(context, bump),
            format: self.format.into_low_level(context, bump),
            samples: self.samples.into_low_level(context, bump),
            load_op: self.load_op.into_low_level(context, bump),
            store_op: self.store_op.into_low_level(context, bump),
            stencil_load_op: self.stencil_load_op.into_low_level(context, bump),
            stencil_store_op: self.stencil_store_op.into_low_level(context, bump),
            initial_layout: self.initial_layout.into_low_level(context, bump),
            final_layout: self.final_layout.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AttachmentDescription2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            format: crate::conv::FromLowLevel::from_low_level(context, value.format),
            samples: crate::conv::FromLowLevel::from_low_level(context, value.samples),
            load_op: crate::conv::FromLowLevel::from_low_level(context, value.load_op),
            store_op: crate::conv::FromLowLevel::from_low_level(context, value.store_op),
            stencil_load_op: crate::conv::FromLowLevel::from_low_level(context, value.stencil_load_op),
            stencil_store_op: crate::conv::FromLowLevel::from_low_level(context, value.stencil_store_op),
            initial_layout: crate::conv::FromLowLevel::from_low_level(context, value.initial_layout),
            final_layout: crate::conv::FromLowLevel::from_low_level(context, value.final_layout),
        }
    }
}
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`AttachmentDescription2`]
pub enum AttachmentDescription2Extension {
    ///Contains a type [`AttachmentDescriptionStencilLayout`] for extending
    /// [`AttachmentDescription2`]
    AttachmentDescriptionStencilLayout(AttachmentDescriptionStencilLayout),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AttachmentDescription2Extension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::AttachmentDescriptionStencilLayout(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_2::AttachmentDescriptionStencilLayout)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AttachmentDescription2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            crate::native::vulkan1_0::StructureType::AttachmentDescriptionStencilLayout => {
                Self::AttachmentDescriptionStencilLayout(AttachmentDescriptionStencilLayout::from_low_level(
                    context,
                    std::ptr::read(value.cast::<crate::native::vulkan1_2::AttachmentDescriptionStencilLayout>()),
                ))
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(AttachmentDescription2)
            ),
        }
    }
}
impl From<AttachmentDescriptionStencilLayout> for AttachmentDescription2Extension {
    fn from(ext: AttachmentDescriptionStencilLayout) -> Self {
        Self::AttachmentDescriptionStencilLayout(ext)
    }
}
impl TryInto<AttachmentDescriptionStencilLayout> for AttachmentDescription2Extension {
    type Error = AttachmentDescription2Extension;
    fn try_into(self) -> Result<AttachmentDescriptionStencilLayout, Self::Error> {
        match self {
            Self::AttachmentDescriptionStencilLayout(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkAttachmentReference2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttachmentReference2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[AttachmentReference2Extension; 1]>,
    pub attachment: u32,
    pub layout: ImageLayout,
    #[doc(alias = "aspectMask")]
    pub aspect_mask: ImageAspectFlags,
}
impl AttachmentReference2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<AttachmentReference2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[AttachmentReference2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `attachment` field.
    pub fn attachment(&self) -> u32 {
        self.attachment
    }
    ///Get a reference to the `layout` field.
    pub fn layout(&self) -> ImageLayout {
        self.layout
    }
    ///Get a reference to the `aspect_mask` field.
    pub fn aspect_mask(&self) -> ImageAspectFlags {
        self.aspect_mask
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[AttachmentReference2Extension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `attachment` field.
    pub fn attachment_mut(&mut self) -> &mut u32 {
        &mut self.attachment
    }
    ///Get a mutable reference to the `layout` field.
    pub fn layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.layout
    }
    ///Get a mutable reference to the `aspect_mask` field.
    pub fn aspect_mask_mut(&mut self) -> &mut ImageAspectFlags {
        &mut self.aspect_mask
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[AttachmentReference2Extension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `attachment` field.
    pub fn set_attachment(&mut self, attachment: u32) -> &mut Self {
        self.attachment = attachment;
        self
    }
    ///Sets the `layout` field.
    pub fn set_layout(&mut self, layout: ImageLayout) -> &mut Self {
        self.layout = layout;
        self
    }
    ///Sets the `aspect_mask` field.
    pub fn set_aspect_mask(&mut self, aspect_mask: ImageAspectFlags) -> &mut Self {
        self.aspect_mask = aspect_mask;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[AttachmentReference2Extension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `attachment` field in a builder way.
    pub fn with_attachment(mut self, attachment: u32) -> Self {
        self.attachment = attachment;
        self
    }
    ///Sets the `layout` field in a builder way.
    pub fn with_layout(mut self, layout: ImageLayout) -> Self {
        self.layout = layout;
        self
    }
    ///Sets the `aspect_mask` field in a builder way.
    pub fn with_aspect_mask(mut self, aspect_mask: ImageAspectFlags) -> Self {
        self.aspect_mask = aspect_mask;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AttachmentReference2 {
    type LowLevel = crate::native::vulkan1_2::AttachmentReference2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_2::AttachmentReference2 {
            s_type: StructureType::AttachmentReference2,
            p_next: next,
            attachment: self.attachment.into_low_level(context, bump),
            layout: self.layout.into_low_level(context, bump),
            aspect_mask: self.aspect_mask.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AttachmentReference2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            attachment: crate::conv::FromLowLevel::from_low_level(context, value.attachment),
            layout: crate::conv::FromLowLevel::from_low_level(context, value.layout),
            aspect_mask: crate::conv::FromLowLevel::from_low_level(context, value.aspect_mask),
        }
    }
}
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`AttachmentReference2`]
pub enum AttachmentReference2Extension {
    ///Contains a type [`AttachmentReferenceStencilLayout`] for extending [`AttachmentReference2`]
    AttachmentReferenceStencilLayout(AttachmentReferenceStencilLayout),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AttachmentReference2Extension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::AttachmentReferenceStencilLayout(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_2::AttachmentReferenceStencilLayout)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AttachmentReference2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            crate::native::vulkan1_0::StructureType::AttachmentReferenceStencilLayout => {
                Self::AttachmentReferenceStencilLayout(AttachmentReferenceStencilLayout::from_low_level(
                    context,
                    std::ptr::read(value.cast::<crate::native::vulkan1_2::AttachmentReferenceStencilLayout>()),
                ))
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(AttachmentReference2)
            ),
        }
    }
}
impl From<AttachmentReferenceStencilLayout> for AttachmentReference2Extension {
    fn from(ext: AttachmentReferenceStencilLayout) -> Self {
        Self::AttachmentReferenceStencilLayout(ext)
    }
}
impl TryInto<AttachmentReferenceStencilLayout> for AttachmentReference2Extension {
    type Error = AttachmentReference2Extension;
    fn try_into(self) -> Result<AttachmentReferenceStencilLayout, Self::Error> {
        match self {
            Self::AttachmentReferenceStencilLayout(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkSubpassDescription2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubpassDescription2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[SubpassDescription2Extension; 1]>,
    pub flags: SubpassDescriptionFlags,
    #[doc(alias = "pipelineBindPoint")]
    pub pipeline_bind_point: PipelineBindPoint,
    #[doc(alias = "viewMask")]
    pub view_mask: u32,
    #[doc(alias = "pInputAttachments")]
    pub input_attachments: SmallVec<[AttachmentReference2; 8]>,
    #[doc(alias = "pColorAttachments")]
    pub color_attachments: SmallVec<[AttachmentReference2; 8]>,
    #[doc(alias = "pResolveAttachments")]
    pub resolve_attachments: SmallVec<[AttachmentReference2; 8]>,
    #[doc(alias = "pDepthStencilAttachment")]
    pub depth_stencil_attachment: Option<AttachmentReference2>,
    #[doc(alias = "pPreserveAttachments")]
    pub preserve_attachments: SmallVec<[u32; 8]>,
}
impl SubpassDescription2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<SubpassDescription2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[SubpassDescription2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> SubpassDescriptionFlags {
        self.flags
    }
    ///Get a reference to the `pipeline_bind_point` field.
    pub fn pipeline_bind_point(&self) -> PipelineBindPoint {
        self.pipeline_bind_point
    }
    ///Get a reference to the `view_mask` field.
    pub fn view_mask(&self) -> u32 {
        self.view_mask
    }
    ///Get a reference to the `input_attachments` field.
    pub fn input_attachments(&self) -> &SmallVec<[AttachmentReference2; 8]> {
        &self.input_attachments
    }
    ///Get a reference to the `color_attachments` field.
    pub fn color_attachments(&self) -> &SmallVec<[AttachmentReference2; 8]> {
        &self.color_attachments
    }
    ///Get a reference to the `resolve_attachments` field.
    pub fn resolve_attachments(&self) -> &SmallVec<[AttachmentReference2; 8]> {
        &self.resolve_attachments
    }
    ///Get a reference to the `depth_stencil_attachment` field.
    pub fn depth_stencil_attachment(&self) -> &Option<AttachmentReference2> {
        &self.depth_stencil_attachment
    }
    ///Get a reference to the `preserve_attachments` field.
    pub fn preserve_attachments(&self) -> &SmallVec<[u32; 8]> {
        &self.preserve_attachments
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[SubpassDescription2Extension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut SubpassDescriptionFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `pipeline_bind_point` field.
    pub fn pipeline_bind_point_mut(&mut self) -> &mut PipelineBindPoint {
        &mut self.pipeline_bind_point
    }
    ///Get a mutable reference to the `view_mask` field.
    pub fn view_mask_mut(&mut self) -> &mut u32 {
        &mut self.view_mask
    }
    ///Get a mutable reference to the `input_attachments` field.
    pub fn input_attachments_mut(&mut self) -> &mut SmallVec<[AttachmentReference2; 8]> {
        &mut self.input_attachments
    }
    ///Get a mutable reference to the `color_attachments` field.
    pub fn color_attachments_mut(&mut self) -> &mut SmallVec<[AttachmentReference2; 8]> {
        &mut self.color_attachments
    }
    ///Get a mutable reference to the `resolve_attachments` field.
    pub fn resolve_attachments_mut(&mut self) -> &mut SmallVec<[AttachmentReference2; 8]> {
        &mut self.resolve_attachments
    }
    ///Get a mutable reference to the `depth_stencil_attachment` field.
    pub fn depth_stencil_attachment_mut(&mut self) -> &mut Option<AttachmentReference2> {
        &mut self.depth_stencil_attachment
    }
    ///Get a mutable reference to the `preserve_attachments` field.
    pub fn preserve_attachments_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.preserve_attachments
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[SubpassDescription2Extension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: SubpassDescriptionFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `pipeline_bind_point` field.
    pub fn set_pipeline_bind_point(&mut self, pipeline_bind_point: PipelineBindPoint) -> &mut Self {
        self.pipeline_bind_point = pipeline_bind_point;
        self
    }
    ///Sets the `view_mask` field.
    pub fn set_view_mask(&mut self, view_mask: u32) -> &mut Self {
        self.view_mask = view_mask;
        self
    }
    ///Sets the `input_attachments` field.
    pub fn set_input_attachments(&mut self, input_attachments: SmallVec<[AttachmentReference2; 8]>) -> &mut Self {
        self.input_attachments = input_attachments;
        self
    }
    ///Sets the `color_attachments` field.
    pub fn set_color_attachments(&mut self, color_attachments: SmallVec<[AttachmentReference2; 8]>) -> &mut Self {
        self.color_attachments = color_attachments;
        self
    }
    ///Sets the `resolve_attachments` field.
    pub fn set_resolve_attachments(&mut self, resolve_attachments: SmallVec<[AttachmentReference2; 8]>) -> &mut Self {
        self.resolve_attachments = resolve_attachments;
        self
    }
    ///Sets the `depth_stencil_attachment` field.
    pub fn set_depth_stencil_attachment(
        &mut self,
        depth_stencil_attachment: Option<AttachmentReference2>,
    ) -> &mut Self {
        self.depth_stencil_attachment = depth_stencil_attachment;
        self
    }
    ///Sets the `preserve_attachments` field.
    pub fn set_preserve_attachments(&mut self, preserve_attachments: SmallVec<[u32; 8]>) -> &mut Self {
        self.preserve_attachments = preserve_attachments;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[SubpassDescription2Extension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: SubpassDescriptionFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `pipeline_bind_point` field in a builder way.
    pub fn with_pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
        self.pipeline_bind_point = pipeline_bind_point;
        self
    }
    ///Sets the `view_mask` field in a builder way.
    pub fn with_view_mask(mut self, view_mask: u32) -> Self {
        self.view_mask = view_mask;
        self
    }
    ///Sets the `input_attachments` field in a builder way.
    pub fn with_input_attachments(mut self, input_attachments: SmallVec<[AttachmentReference2; 8]>) -> Self {
        self.input_attachments = input_attachments;
        self
    }
    ///Sets the `color_attachments` field in a builder way.
    pub fn with_color_attachments(mut self, color_attachments: SmallVec<[AttachmentReference2; 8]>) -> Self {
        self.color_attachments = color_attachments;
        self
    }
    ///Sets the `resolve_attachments` field in a builder way.
    pub fn with_resolve_attachments(mut self, resolve_attachments: SmallVec<[AttachmentReference2; 8]>) -> Self {
        self.resolve_attachments = resolve_attachments;
        self
    }
    ///Sets the `depth_stencil_attachment` field in a builder way.
    pub fn with_depth_stencil_attachment(mut self, depth_stencil_attachment: Option<AttachmentReference2>) -> Self {
        self.depth_stencil_attachment = depth_stencil_attachment;
        self
    }
    ///Sets the `preserve_attachments` field in a builder way.
    pub fn with_preserve_attachments(mut self, preserve_attachments: SmallVec<[u32; 8]>) -> Self {
        self.preserve_attachments = preserve_attachments;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubpassDescription2 {
    type LowLevel = crate::native::vulkan1_2::SubpassDescription2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        let len_input_attachments = self.input_attachments.len() as u32;
        let input_attachments = bump
            .alloc_slice_fill_iter(self.input_attachments.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_color_attachments = self.color_attachments.len() as u32;
        let color_attachments = bump
            .alloc_slice_fill_iter(self.color_attachments.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let resolve_attachments = bump
            .alloc_slice_fill_iter(self.resolve_attachments.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_preserve_attachments = self.preserve_attachments.len() as u32;
        let preserve_attachments = bump
            .alloc_slice_fill_iter(
                self.preserve_attachments
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::vulkan1_2::SubpassDescription2 {
            s_type: StructureType::SubpassDescription2,
            p_next: next,
            flags: self.flags.into_low_level(context, bump),
            pipeline_bind_point: self.pipeline_bind_point.into_low_level(context, bump),
            view_mask: self.view_mask.into_low_level(context, bump),
            input_attachment_count: len_input_attachments,
            input_attachments: input_attachments,
            color_attachment_count: len_color_attachments,
            color_attachments: color_attachments,
            resolve_attachments: resolve_attachments,
            depth_stencil_attachment: self
                .depth_stencil_attachment
                .as_ref()
                .map(|v| bump.alloc(v.into_low_level(context, bump)) as *const _)
                .unwrap_or_else(std::ptr::null),
            preserve_attachment_count: len_preserve_attachments,
            preserve_attachments: preserve_attachments,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SubpassDescription2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        let input_attachments_len = value.input_attachment_count;
        let mut input_attachments = SmallVec::with_capacity(input_attachments_len as usize);
        for i in 0..input_attachments_len {
            input_attachments.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.input_attachments.add(i as usize).read(),
            ));
        }
        let color_attachments_len = value.color_attachment_count;
        let mut color_attachments = SmallVec::with_capacity(color_attachments_len as usize);
        for i in 0..color_attachments_len {
            color_attachments.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.color_attachments.add(i as usize).read(),
            ));
        }
        let resolve_attachments_len = value.color_attachment_count;
        let mut resolve_attachments = SmallVec::with_capacity(resolve_attachments_len as usize);
        for i in 0..resolve_attachments_len {
            resolve_attachments.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.resolve_attachments.add(i as usize).read(),
            ));
        }
        let preserve_attachments_len = value.preserve_attachment_count;
        let mut preserve_attachments = SmallVec::with_capacity(preserve_attachments_len as usize);
        for i in 0..preserve_attachments_len {
            preserve_attachments.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.preserve_attachments.add(i as usize).read(),
            ));
        }
        Self {
            extensions: extensions,
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            pipeline_bind_point: crate::conv::FromLowLevel::from_low_level(context, value.pipeline_bind_point),
            view_mask: crate::conv::FromLowLevel::from_low_level(context, value.view_mask),
            input_attachments: input_attachments,
            color_attachments: color_attachments,
            resolve_attachments: resolve_attachments,
            depth_stencil_attachment: crate::conv::FromLowLevel::from_low_level(
                context,
                *value.depth_stencil_attachment,
            ),
            preserve_attachments: preserve_attachments,
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`SubpassDescription2`]
pub enum SubpassDescription2Extension {
    ///Contains a type [`SubpassDescriptionDepthStencilResolve`] for extending
    /// [`SubpassDescription2`]
    SubpassDescriptionDepthStencilResolve(SubpassDescriptionDepthStencilResolve),
    #[cfg(feature = "VK_KHR_fragment_shading_rate")]
    ///Contains a type [`FragmentShadingRateAttachmentInfoKHR`] for extending
    /// [`SubpassDescription2`]
    FragmentShadingRateAttachmentInfoKHR(FragmentShadingRateAttachmentInfoKHR),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubpassDescription2Extension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::SubpassDescriptionDepthStencilResolve(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_2::SubpassDescriptionDepthStencilResolve)
                .cast(),
            #[cfg(feature = "VK_KHR_fragment_shading_rate")]
            Self::FragmentShadingRateAttachmentInfoKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_fragment_shading_rate::FragmentShadingRateAttachmentInfoKHR)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SubpassDescription2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { crate :: native :: vulkan1_0 :: StructureType :: SubpassDescriptionDepthStencilResolve => Self :: SubpassDescriptionDepthStencilResolve (SubpassDescriptionDepthStencilResolve :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: SubpassDescriptionDepthStencilResolve > ()))) , # [cfg (feature = "VK_KHR_fragment_shading_rate")] crate :: native :: vulkan1_0 :: StructureType :: FragmentShadingRateAttachmentInfoKhr => Self :: FragmentShadingRateAttachmentInfoKHR (FragmentShadingRateAttachmentInfoKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_fragment_shading_rate :: FragmentShadingRateAttachmentInfoKHR > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (SubpassDescription2)) }
    }
}
impl From<SubpassDescriptionDepthStencilResolve> for SubpassDescription2Extension {
    fn from(ext: SubpassDescriptionDepthStencilResolve) -> Self {
        Self::SubpassDescriptionDepthStencilResolve(ext)
    }
}
impl TryInto<SubpassDescriptionDepthStencilResolve> for SubpassDescription2Extension {
    type Error = SubpassDescription2Extension;
    fn try_into(self) -> Result<SubpassDescriptionDepthStencilResolve, Self::Error> {
        match self {
            Self::SubpassDescriptionDepthStencilResolve(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
impl From<FragmentShadingRateAttachmentInfoKHR> for SubpassDescription2Extension {
    fn from(ext: FragmentShadingRateAttachmentInfoKHR) -> Self {
        Self::FragmentShadingRateAttachmentInfoKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
impl TryInto<FragmentShadingRateAttachmentInfoKHR> for SubpassDescription2Extension {
    type Error = SubpassDescription2Extension;
    fn try_into(self) -> Result<FragmentShadingRateAttachmentInfoKHR, Self::Error> {
        match self {
            Self::FragmentShadingRateAttachmentInfoKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkSubpassDependency2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubpassDependency2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[SubpassDependency2Extension; 1]>,
    #[doc(alias = "srcSubpass")]
    pub src_subpass: u32,
    #[doc(alias = "dstSubpass")]
    pub dst_subpass: u32,
    #[doc(alias = "srcStageMask")]
    pub src_stage_mask: PipelineStageFlags,
    #[doc(alias = "dstStageMask")]
    pub dst_stage_mask: PipelineStageFlags,
    #[doc(alias = "srcAccessMask")]
    pub src_access_mask: AccessFlags,
    #[doc(alias = "dstAccessMask")]
    pub dst_access_mask: AccessFlags,
    #[doc(alias = "dependencyFlags")]
    pub dependency_flags: DependencyFlags,
    #[doc(alias = "viewOffset")]
    pub view_offset: i32,
}
impl SubpassDependency2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<SubpassDependency2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[SubpassDependency2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `src_subpass` field.
    pub fn src_subpass(&self) -> u32 {
        self.src_subpass
    }
    ///Get a reference to the `dst_subpass` field.
    pub fn dst_subpass(&self) -> u32 {
        self.dst_subpass
    }
    ///Get a reference to the `src_stage_mask` field.
    pub fn src_stage_mask(&self) -> PipelineStageFlags {
        self.src_stage_mask
    }
    ///Get a reference to the `dst_stage_mask` field.
    pub fn dst_stage_mask(&self) -> PipelineStageFlags {
        self.dst_stage_mask
    }
    ///Get a reference to the `src_access_mask` field.
    pub fn src_access_mask(&self) -> AccessFlags {
        self.src_access_mask
    }
    ///Get a reference to the `dst_access_mask` field.
    pub fn dst_access_mask(&self) -> AccessFlags {
        self.dst_access_mask
    }
    ///Get a reference to the `dependency_flags` field.
    pub fn dependency_flags(&self) -> DependencyFlags {
        self.dependency_flags
    }
    ///Get a reference to the `view_offset` field.
    pub fn view_offset(&self) -> i32 {
        self.view_offset
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[SubpassDependency2Extension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `src_subpass` field.
    pub fn src_subpass_mut(&mut self) -> &mut u32 {
        &mut self.src_subpass
    }
    ///Get a mutable reference to the `dst_subpass` field.
    pub fn dst_subpass_mut(&mut self) -> &mut u32 {
        &mut self.dst_subpass
    }
    ///Get a mutable reference to the `src_stage_mask` field.
    pub fn src_stage_mask_mut(&mut self) -> &mut PipelineStageFlags {
        &mut self.src_stage_mask
    }
    ///Get a mutable reference to the `dst_stage_mask` field.
    pub fn dst_stage_mask_mut(&mut self) -> &mut PipelineStageFlags {
        &mut self.dst_stage_mask
    }
    ///Get a mutable reference to the `src_access_mask` field.
    pub fn src_access_mask_mut(&mut self) -> &mut AccessFlags {
        &mut self.src_access_mask
    }
    ///Get a mutable reference to the `dst_access_mask` field.
    pub fn dst_access_mask_mut(&mut self) -> &mut AccessFlags {
        &mut self.dst_access_mask
    }
    ///Get a mutable reference to the `dependency_flags` field.
    pub fn dependency_flags_mut(&mut self) -> &mut DependencyFlags {
        &mut self.dependency_flags
    }
    ///Get a mutable reference to the `view_offset` field.
    pub fn view_offset_mut(&mut self) -> &mut i32 {
        &mut self.view_offset
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[SubpassDependency2Extension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `src_subpass` field.
    pub fn set_src_subpass(&mut self, src_subpass: u32) -> &mut Self {
        self.src_subpass = src_subpass;
        self
    }
    ///Sets the `dst_subpass` field.
    pub fn set_dst_subpass(&mut self, dst_subpass: u32) -> &mut Self {
        self.dst_subpass = dst_subpass;
        self
    }
    ///Sets the `src_stage_mask` field.
    pub fn set_src_stage_mask(&mut self, src_stage_mask: PipelineStageFlags) -> &mut Self {
        self.src_stage_mask = src_stage_mask;
        self
    }
    ///Sets the `dst_stage_mask` field.
    pub fn set_dst_stage_mask(&mut self, dst_stage_mask: PipelineStageFlags) -> &mut Self {
        self.dst_stage_mask = dst_stage_mask;
        self
    }
    ///Sets the `src_access_mask` field.
    pub fn set_src_access_mask(&mut self, src_access_mask: AccessFlags) -> &mut Self {
        self.src_access_mask = src_access_mask;
        self
    }
    ///Sets the `dst_access_mask` field.
    pub fn set_dst_access_mask(&mut self, dst_access_mask: AccessFlags) -> &mut Self {
        self.dst_access_mask = dst_access_mask;
        self
    }
    ///Sets the `dependency_flags` field.
    pub fn set_dependency_flags(&mut self, dependency_flags: DependencyFlags) -> &mut Self {
        self.dependency_flags = dependency_flags;
        self
    }
    ///Sets the `view_offset` field.
    pub fn set_view_offset(&mut self, view_offset: i32) -> &mut Self {
        self.view_offset = view_offset;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[SubpassDependency2Extension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `src_subpass` field in a builder way.
    pub fn with_src_subpass(mut self, src_subpass: u32) -> Self {
        self.src_subpass = src_subpass;
        self
    }
    ///Sets the `dst_subpass` field in a builder way.
    pub fn with_dst_subpass(mut self, dst_subpass: u32) -> Self {
        self.dst_subpass = dst_subpass;
        self
    }
    ///Sets the `src_stage_mask` field in a builder way.
    pub fn with_src_stage_mask(mut self, src_stage_mask: PipelineStageFlags) -> Self {
        self.src_stage_mask = src_stage_mask;
        self
    }
    ///Sets the `dst_stage_mask` field in a builder way.
    pub fn with_dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags) -> Self {
        self.dst_stage_mask = dst_stage_mask;
        self
    }
    ///Sets the `src_access_mask` field in a builder way.
    pub fn with_src_access_mask(mut self, src_access_mask: AccessFlags) -> Self {
        self.src_access_mask = src_access_mask;
        self
    }
    ///Sets the `dst_access_mask` field in a builder way.
    pub fn with_dst_access_mask(mut self, dst_access_mask: AccessFlags) -> Self {
        self.dst_access_mask = dst_access_mask;
        self
    }
    ///Sets the `dependency_flags` field in a builder way.
    pub fn with_dependency_flags(mut self, dependency_flags: DependencyFlags) -> Self {
        self.dependency_flags = dependency_flags;
        self
    }
    ///Sets the `view_offset` field in a builder way.
    pub fn with_view_offset(mut self, view_offset: i32) -> Self {
        self.view_offset = view_offset;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubpassDependency2 {
    type LowLevel = crate::native::vulkan1_2::SubpassDependency2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_2::SubpassDependency2 {
            s_type: StructureType::SubpassDependency2,
            p_next: next,
            src_subpass: self.src_subpass.into_low_level(context, bump),
            dst_subpass: self.dst_subpass.into_low_level(context, bump),
            src_stage_mask: self.src_stage_mask.into_low_level(context, bump),
            dst_stage_mask: self.dst_stage_mask.into_low_level(context, bump),
            src_access_mask: self.src_access_mask.into_low_level(context, bump),
            dst_access_mask: self.dst_access_mask.into_low_level(context, bump),
            dependency_flags: self.dependency_flags.into_low_level(context, bump),
            view_offset: self.view_offset.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SubpassDependency2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            src_subpass: crate::conv::FromLowLevel::from_low_level(context, value.src_subpass),
            dst_subpass: crate::conv::FromLowLevel::from_low_level(context, value.dst_subpass),
            src_stage_mask: crate::conv::FromLowLevel::from_low_level(context, value.src_stage_mask),
            dst_stage_mask: crate::conv::FromLowLevel::from_low_level(context, value.dst_stage_mask),
            src_access_mask: crate::conv::FromLowLevel::from_low_level(context, value.src_access_mask),
            dst_access_mask: crate::conv::FromLowLevel::from_low_level(context, value.dst_access_mask),
            dependency_flags: crate::conv::FromLowLevel::from_low_level(context, value.dependency_flags),
            view_offset: crate::conv::FromLowLevel::from_low_level(context, value.view_offset),
        }
    }
}
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`SubpassDependency2`]
pub enum SubpassDependency2Extension {
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`MemoryBarrier2`] for extending [`SubpassDependency2`]
    MemoryBarrier2(MemoryBarrier2),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubpassDependency2Extension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VULKAN_1_3")]
            Self::MemoryBarrier2(ext) => {
                (bump.alloc(ext.into_low_level(context, bump)) as *mut crate::native::vulkan1_3::MemoryBarrier2).cast()
            },
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SubpassDependency2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            #[cfg(feature = "VULKAN_1_3")]
            crate::native::vulkan1_0::StructureType::MemoryBarrier2 => {
                Self::MemoryBarrier2(MemoryBarrier2::from_low_level(
                    context,
                    std::ptr::read(value.cast::<crate::native::vulkan1_3::MemoryBarrier2>()),
                ))
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(SubpassDependency2)
            ),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<MemoryBarrier2> for SubpassDependency2Extension {
    fn from(ext: MemoryBarrier2) -> Self {
        Self::MemoryBarrier2(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<MemoryBarrier2> for SubpassDependency2Extension {
    type Error = SubpassDependency2Extension;
    fn try_into(self) -> Result<MemoryBarrier2, Self::Error> {
        match self {
            Self::MemoryBarrier2(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkRenderPassCreateInfo2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderPassCreateInfo2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[RenderPassCreateInfo2Extension; 1]>,
    pub flags: RenderPassCreateFlags,
    #[doc(alias = "pAttachments")]
    pub attachments: SmallVec<[AttachmentDescription2; 8]>,
    #[doc(alias = "pSubpasses")]
    pub subpasses: SmallVec<[SubpassDescription2; 8]>,
    #[doc(alias = "pDependencies")]
    pub dependencies: SmallVec<[SubpassDependency2; 8]>,
    #[doc(alias = "pCorrelatedViewMasks")]
    pub correlated_view_masks: SmallVec<[u32; 8]>,
}
impl RenderPassCreateInfo2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<RenderPassCreateInfo2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[RenderPassCreateInfo2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> RenderPassCreateFlags {
        self.flags
    }
    ///Get a reference to the `attachments` field.
    pub fn attachments(&self) -> &SmallVec<[AttachmentDescription2; 8]> {
        &self.attachments
    }
    ///Get a reference to the `subpasses` field.
    pub fn subpasses(&self) -> &SmallVec<[SubpassDescription2; 8]> {
        &self.subpasses
    }
    ///Get a reference to the `dependencies` field.
    pub fn dependencies(&self) -> &SmallVec<[SubpassDependency2; 8]> {
        &self.dependencies
    }
    ///Get a reference to the `correlated_view_masks` field.
    pub fn correlated_view_masks(&self) -> &SmallVec<[u32; 8]> {
        &self.correlated_view_masks
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[RenderPassCreateInfo2Extension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut RenderPassCreateFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `attachments` field.
    pub fn attachments_mut(&mut self) -> &mut SmallVec<[AttachmentDescription2; 8]> {
        &mut self.attachments
    }
    ///Get a mutable reference to the `subpasses` field.
    pub fn subpasses_mut(&mut self) -> &mut SmallVec<[SubpassDescription2; 8]> {
        &mut self.subpasses
    }
    ///Get a mutable reference to the `dependencies` field.
    pub fn dependencies_mut(&mut self) -> &mut SmallVec<[SubpassDependency2; 8]> {
        &mut self.dependencies
    }
    ///Get a mutable reference to the `correlated_view_masks` field.
    pub fn correlated_view_masks_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.correlated_view_masks
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[RenderPassCreateInfo2Extension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: RenderPassCreateFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `attachments` field.
    pub fn set_attachments(&mut self, attachments: SmallVec<[AttachmentDescription2; 8]>) -> &mut Self {
        self.attachments = attachments;
        self
    }
    ///Sets the `subpasses` field.
    pub fn set_subpasses(&mut self, subpasses: SmallVec<[SubpassDescription2; 8]>) -> &mut Self {
        self.subpasses = subpasses;
        self
    }
    ///Sets the `dependencies` field.
    pub fn set_dependencies(&mut self, dependencies: SmallVec<[SubpassDependency2; 8]>) -> &mut Self {
        self.dependencies = dependencies;
        self
    }
    ///Sets the `correlated_view_masks` field.
    pub fn set_correlated_view_masks(&mut self, correlated_view_masks: SmallVec<[u32; 8]>) -> &mut Self {
        self.correlated_view_masks = correlated_view_masks;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[RenderPassCreateInfo2Extension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: RenderPassCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `attachments` field in a builder way.
    pub fn with_attachments(mut self, attachments: SmallVec<[AttachmentDescription2; 8]>) -> Self {
        self.attachments = attachments;
        self
    }
    ///Sets the `subpasses` field in a builder way.
    pub fn with_subpasses(mut self, subpasses: SmallVec<[SubpassDescription2; 8]>) -> Self {
        self.subpasses = subpasses;
        self
    }
    ///Sets the `dependencies` field in a builder way.
    pub fn with_dependencies(mut self, dependencies: SmallVec<[SubpassDependency2; 8]>) -> Self {
        self.dependencies = dependencies;
        self
    }
    ///Sets the `correlated_view_masks` field in a builder way.
    pub fn with_correlated_view_masks(mut self, correlated_view_masks: SmallVec<[u32; 8]>) -> Self {
        self.correlated_view_masks = correlated_view_masks;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderPassCreateInfo2 {
    type LowLevel = crate::native::vulkan1_2::RenderPassCreateInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        let len_attachments = self.attachments.len() as u32;
        let attachments = bump
            .alloc_slice_fill_iter(self.attachments.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_subpasses = self.subpasses.len() as u32;
        let subpasses = bump
            .alloc_slice_fill_iter(self.subpasses.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_dependencies = self.dependencies.len() as u32;
        let dependencies = bump
            .alloc_slice_fill_iter(self.dependencies.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_correlated_view_masks = self.correlated_view_masks.len() as u32;
        let correlated_view_masks = bump
            .alloc_slice_fill_iter(
                self.correlated_view_masks
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::vulkan1_2::RenderPassCreateInfo2 {
            s_type: StructureType::RenderPassCreateInfo2,
            p_next: next,
            flags: self.flags.into_low_level(context, bump),
            attachment_count: len_attachments,
            attachments: attachments,
            subpass_count: len_subpasses,
            subpasses: subpasses,
            dependency_count: len_dependencies,
            dependencies: dependencies,
            correlated_view_mask_count: len_correlated_view_masks,
            correlated_view_masks: correlated_view_masks,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RenderPassCreateInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        let attachments_len = value.attachment_count;
        let mut attachments = SmallVec::with_capacity(attachments_len as usize);
        for i in 0..attachments_len {
            attachments.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.attachments.add(i as usize).read(),
            ));
        }
        let subpasses_len = value.subpass_count;
        let mut subpasses = SmallVec::with_capacity(subpasses_len as usize);
        for i in 0..subpasses_len {
            subpasses.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.subpasses.add(i as usize).read(),
            ));
        }
        let dependencies_len = value.dependency_count;
        let mut dependencies = SmallVec::with_capacity(dependencies_len as usize);
        for i in 0..dependencies_len {
            dependencies.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.dependencies.add(i as usize).read(),
            ));
        }
        let correlated_view_masks_len = value.correlated_view_mask_count;
        let mut correlated_view_masks = SmallVec::with_capacity(correlated_view_masks_len as usize);
        for i in 0..correlated_view_masks_len {
            correlated_view_masks.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.correlated_view_masks.add(i as usize).read(),
            ));
        }
        Self {
            extensions: extensions,
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            attachments: attachments,
            subpasses: subpasses,
            dependencies: dependencies,
            correlated_view_masks: correlated_view_masks,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`RenderPassCreateInfo2`]
pub enum RenderPassCreateInfo2Extension {
    #[cfg(feature = "VK_EXT_fragment_density_map")]
    ///Contains a type [`RenderPassFragmentDensityMapCreateInfoEXT`] for extending
    /// [`RenderPassCreateInfo2`]
    RenderPassFragmentDensityMapCreateInfoEXT(RenderPassFragmentDensityMapCreateInfoEXT),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderPassCreateInfo2Extension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VK_EXT_fragment_density_map")]
            Self::RenderPassFragmentDensityMapCreateInfoEXT(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::ext_fragment_density_map::RenderPassFragmentDensityMapCreateInfoEXT)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RenderPassCreateInfo2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_EXT_fragment_density_map")] crate :: native :: vulkan1_0 :: StructureType :: RenderPassFragmentDensityMapCreateInfoExt => Self :: RenderPassFragmentDensityMapCreateInfoEXT (RenderPassFragmentDensityMapCreateInfoEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_fragment_density_map :: RenderPassFragmentDensityMapCreateInfoEXT > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (RenderPassCreateInfo2)) }
    }
}
#[cfg(feature = "VK_EXT_fragment_density_map")]
impl From<RenderPassFragmentDensityMapCreateInfoEXT> for RenderPassCreateInfo2Extension {
    fn from(ext: RenderPassFragmentDensityMapCreateInfoEXT) -> Self {
        Self::RenderPassFragmentDensityMapCreateInfoEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_fragment_density_map")]
impl TryInto<RenderPassFragmentDensityMapCreateInfoEXT> for RenderPassCreateInfo2Extension {
    type Error = RenderPassCreateInfo2Extension;
    fn try_into(self) -> Result<RenderPassFragmentDensityMapCreateInfoEXT, Self::Error> {
        match self {
            Self::RenderPassFragmentDensityMapCreateInfoEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkSubpassBeginInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubpassBeginInfo {
    pub contents: SubpassContents,
}
impl SubpassBeginInfo {
    ///Get a reference to the `contents` field.
    pub fn contents(&self) -> SubpassContents {
        self.contents
    }
    ///Get a mutable reference to the `contents` field.
    pub fn contents_mut(&mut self) -> &mut SubpassContents {
        &mut self.contents
    }
    ///Sets the `contents` field.
    pub fn set_contents(&mut self, contents: SubpassContents) -> &mut Self {
        self.contents = contents;
        self
    }
    ///Sets the `contents` field in a builder way.
    pub fn with_contents(mut self, contents: SubpassContents) -> Self {
        self.contents = contents;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubpassBeginInfo {
    type LowLevel = crate::native::vulkan1_2::SubpassBeginInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::SubpassBeginInfo {
            s_type: StructureType::SubpassBeginInfo,
            p_next: std::ptr::null(),
            contents: self.contents.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SubpassBeginInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            contents: crate::conv::FromLowLevel::from_low_level(context, value.contents),
        }
    }
}
#[doc(alias = "VkSubpassEndInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubpassEndInfo {}
impl SubpassEndInfo {}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubpassEndInfo {
    type LowLevel = crate::native::vulkan1_2::SubpassEndInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::SubpassEndInfo {
            s_type: StructureType::SubpassEndInfo,
            p_next: std::ptr::null(),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SubpassEndInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {}
    }
}
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceTimelineSemaphoreFeatures {
    #[doc(alias = "timelineSemaphore")]
    pub timeline_semaphore: bool,
}
impl PhysicalDeviceTimelineSemaphoreFeatures {
    ///Get a reference to the `timeline_semaphore` field.
    pub fn timeline_semaphore(&self) -> &bool {
        &self.timeline_semaphore
    }
    ///Get a mutable reference to the `timeline_semaphore` field.
    pub fn timeline_semaphore_mut(&mut self) -> &mut bool {
        &mut self.timeline_semaphore
    }
    ///Sets the `timeline_semaphore` field.
    pub fn set_timeline_semaphore(&mut self, timeline_semaphore: bool) -> &mut Self {
        self.timeline_semaphore = timeline_semaphore;
        self
    }
    ///Sets the `timeline_semaphore` field in a builder way.
    pub fn with_timeline_semaphore(mut self, timeline_semaphore: bool) -> Self {
        self.timeline_semaphore = timeline_semaphore;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceTimelineSemaphoreFeatures {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceTimelineSemaphoreFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceTimelineSemaphoreFeatures {
            s_type: StructureType::PhysicalDeviceTimelineSemaphoreFeatures,
            p_next: std::ptr::null_mut(),
            timeline_semaphore: self.timeline_semaphore.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceTimelineSemaphoreFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            timeline_semaphore: crate::conv::FromLowLevel::from_low_level(context, value.timeline_semaphore),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreProperties")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceTimelineSemaphoreProperties {
    #[doc(alias = "maxTimelineSemaphoreValueDifference")]
    pub max_timeline_semaphore_value_difference: u64,
}
impl PhysicalDeviceTimelineSemaphoreProperties {
    ///Get a reference to the `max_timeline_semaphore_value_difference` field.
    pub fn max_timeline_semaphore_value_difference(&self) -> u64 {
        self.max_timeline_semaphore_value_difference
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceTimelineSemaphoreProperties {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceTimelineSemaphoreProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceTimelineSemaphoreProperties {
            s_type: StructureType::PhysicalDeviceTimelineSemaphoreProperties,
            p_next: std::ptr::null_mut(),
            max_timeline_semaphore_value_difference: self
                .max_timeline_semaphore_value_difference
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceTimelineSemaphoreProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_timeline_semaphore_value_difference: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_timeline_semaphore_value_difference,
            ),
        }
    }
}
#[doc(alias = "VkSemaphoreTypeCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemaphoreTypeCreateInfo {
    #[doc(alias = "semaphoreType")]
    pub semaphore_type: SemaphoreType,
    #[doc(alias = "initialValue")]
    pub initial_value: u64,
}
impl SemaphoreTypeCreateInfo {
    ///Get a reference to the `semaphore_type` field.
    pub fn semaphore_type(&self) -> SemaphoreType {
        self.semaphore_type
    }
    ///Get a reference to the `initial_value` field.
    pub fn initial_value(&self) -> u64 {
        self.initial_value
    }
    ///Get a mutable reference to the `semaphore_type` field.
    pub fn semaphore_type_mut(&mut self) -> &mut SemaphoreType {
        &mut self.semaphore_type
    }
    ///Get a mutable reference to the `initial_value` field.
    pub fn initial_value_mut(&mut self) -> &mut u64 {
        &mut self.initial_value
    }
    ///Sets the `semaphore_type` field.
    pub fn set_semaphore_type(&mut self, semaphore_type: SemaphoreType) -> &mut Self {
        self.semaphore_type = semaphore_type;
        self
    }
    ///Sets the `initial_value` field.
    pub fn set_initial_value(&mut self, initial_value: u64) -> &mut Self {
        self.initial_value = initial_value;
        self
    }
    ///Sets the `semaphore_type` field in a builder way.
    pub fn with_semaphore_type(mut self, semaphore_type: SemaphoreType) -> Self {
        self.semaphore_type = semaphore_type;
        self
    }
    ///Sets the `initial_value` field in a builder way.
    pub fn with_initial_value(mut self, initial_value: u64) -> Self {
        self.initial_value = initial_value;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SemaphoreTypeCreateInfo {
    type LowLevel = crate::native::vulkan1_2::SemaphoreTypeCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::SemaphoreTypeCreateInfo {
            s_type: StructureType::SemaphoreTypeCreateInfo,
            p_next: std::ptr::null(),
            semaphore_type: self.semaphore_type.into_low_level(context, bump),
            initial_value: self.initial_value.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SemaphoreTypeCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            semaphore_type: crate::conv::FromLowLevel::from_low_level(context, value.semaphore_type),
            initial_value: crate::conv::FromLowLevel::from_low_level(context, value.initial_value),
        }
    }
}
#[doc(alias = "VkTimelineSemaphoreSubmitInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TimelineSemaphoreSubmitInfo {
    #[doc(alias = "pWaitSemaphoreValues")]
    pub wait_semaphore_values: SmallVec<[u64; 8]>,
    #[doc(alias = "pSignalSemaphoreValues")]
    pub signal_semaphore_values: SmallVec<[u64; 8]>,
}
impl TimelineSemaphoreSubmitInfo {
    ///Get a reference to the `wait_semaphore_values` field.
    pub fn wait_semaphore_values(&self) -> &SmallVec<[u64; 8]> {
        &self.wait_semaphore_values
    }
    ///Get a reference to the `signal_semaphore_values` field.
    pub fn signal_semaphore_values(&self) -> &SmallVec<[u64; 8]> {
        &self.signal_semaphore_values
    }
    ///Get a mutable reference to the `wait_semaphore_values` field.
    pub fn wait_semaphore_values_mut(&mut self) -> &mut SmallVec<[u64; 8]> {
        &mut self.wait_semaphore_values
    }
    ///Get a mutable reference to the `signal_semaphore_values` field.
    pub fn signal_semaphore_values_mut(&mut self) -> &mut SmallVec<[u64; 8]> {
        &mut self.signal_semaphore_values
    }
    ///Sets the `wait_semaphore_values` field.
    pub fn set_wait_semaphore_values(&mut self, wait_semaphore_values: SmallVec<[u64; 8]>) -> &mut Self {
        self.wait_semaphore_values = wait_semaphore_values;
        self
    }
    ///Sets the `signal_semaphore_values` field.
    pub fn set_signal_semaphore_values(&mut self, signal_semaphore_values: SmallVec<[u64; 8]>) -> &mut Self {
        self.signal_semaphore_values = signal_semaphore_values;
        self
    }
    ///Sets the `wait_semaphore_values` field in a builder way.
    pub fn with_wait_semaphore_values(mut self, wait_semaphore_values: SmallVec<[u64; 8]>) -> Self {
        self.wait_semaphore_values = wait_semaphore_values;
        self
    }
    ///Sets the `signal_semaphore_values` field in a builder way.
    pub fn with_signal_semaphore_values(mut self, signal_semaphore_values: SmallVec<[u64; 8]>) -> Self {
        self.signal_semaphore_values = signal_semaphore_values;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for TimelineSemaphoreSubmitInfo {
    type LowLevel = crate::native::vulkan1_2::TimelineSemaphoreSubmitInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_wait_semaphore_values = self.wait_semaphore_values.len() as u32;
        let wait_semaphore_values = bump
            .alloc_slice_fill_iter(
                self.wait_semaphore_values
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        let len_signal_semaphore_values = self.signal_semaphore_values.len() as u32;
        let signal_semaphore_values = bump
            .alloc_slice_fill_iter(
                self.signal_semaphore_values
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::vulkan1_2::TimelineSemaphoreSubmitInfo {
            s_type: StructureType::TimelineSemaphoreSubmitInfo,
            p_next: std::ptr::null(),
            wait_semaphore_value_count: len_wait_semaphore_values,
            wait_semaphore_values: wait_semaphore_values,
            signal_semaphore_value_count: len_signal_semaphore_values,
            signal_semaphore_values: signal_semaphore_values,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for TimelineSemaphoreSubmitInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let wait_semaphore_values_len = value.wait_semaphore_value_count;
        let mut wait_semaphore_values = SmallVec::with_capacity(wait_semaphore_values_len as usize);
        for i in 0..wait_semaphore_values_len {
            wait_semaphore_values.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.wait_semaphore_values.add(i as usize).read(),
            ));
        }
        let signal_semaphore_values_len = value.signal_semaphore_value_count;
        let mut signal_semaphore_values = SmallVec::with_capacity(signal_semaphore_values_len as usize);
        for i in 0..signal_semaphore_values_len {
            signal_semaphore_values.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.signal_semaphore_values.add(i as usize).read(),
            ));
        }
        Self {
            wait_semaphore_values: wait_semaphore_values,
            signal_semaphore_values: signal_semaphore_values,
        }
    }
}
#[doc(alias = "VkSemaphoreWaitInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemaphoreWaitInfo {
    pub flags: SemaphoreWaitFlags,
    #[doc(alias = "pSemaphores")]
    pub semaphores: SmallVec<[Semaphore; 8]>,
    #[doc(alias = "pValues")]
    pub values: SmallVec<[u64; 8]>,
}
impl SemaphoreWaitInfo {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> SemaphoreWaitFlags {
        self.flags
    }
    ///Get a reference to the `semaphores` field.
    pub fn semaphores(&self) -> &SmallVec<[Semaphore; 8]> {
        &self.semaphores
    }
    ///Get a reference to the `values` field.
    pub fn values(&self) -> &SmallVec<[u64; 8]> {
        &self.values
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut SemaphoreWaitFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `semaphores` field.
    pub fn semaphores_mut(&mut self) -> &mut SmallVec<[Semaphore; 8]> {
        &mut self.semaphores
    }
    ///Get a mutable reference to the `values` field.
    pub fn values_mut(&mut self) -> &mut SmallVec<[u64; 8]> {
        &mut self.values
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: SemaphoreWaitFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `semaphores` field.
    pub fn set_semaphores(&mut self, semaphores: SmallVec<[Semaphore; 8]>) -> &mut Self {
        self.semaphores = semaphores;
        self
    }
    ///Sets the `values` field.
    pub fn set_values(&mut self, values: SmallVec<[u64; 8]>) -> &mut Self {
        self.values = values;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: SemaphoreWaitFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `semaphores` field in a builder way.
    pub fn with_semaphores(mut self, semaphores: SmallVec<[Semaphore; 8]>) -> Self {
        self.semaphores = semaphores;
        self
    }
    ///Sets the `values` field in a builder way.
    pub fn with_values(mut self, values: SmallVec<[u64; 8]>) -> Self {
        self.values = values;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SemaphoreWaitInfo {
    type LowLevel = crate::native::vulkan1_2::SemaphoreWaitInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_semaphores = self.semaphores.len() as u32;
        let semaphores = bump
            .alloc_slice_fill_iter(self.semaphores.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let values = bump
            .alloc_slice_fill_iter(self.values.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_2::SemaphoreWaitInfo {
            s_type: StructureType::SemaphoreWaitInfo,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            semaphore_count: len_semaphores,
            semaphores: semaphores,
            values: values,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SemaphoreWaitInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let semaphores_len = value.semaphore_count;
        let mut semaphores = SmallVec::with_capacity(semaphores_len as usize);
        for i in 0..semaphores_len {
            semaphores.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.semaphores.add(i as usize).read(),
            ));
        }
        let values_len = value.semaphore_count;
        let mut values = SmallVec::with_capacity(values_len as usize);
        for i in 0..values_len {
            values.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.values.add(i as usize).read(),
            ));
        }
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            semaphores: semaphores,
            values: values,
        }
    }
}
#[doc(alias = "VkSemaphoreSignalInfo")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemaphoreSignalInfo {
    pub semaphore: Semaphore,
    pub value: u64,
}
impl SemaphoreSignalInfo {
    ///Get a reference to the `semaphore` field.
    pub fn semaphore(&self) -> &Semaphore {
        &self.semaphore
    }
    ///Get a reference to the `value` field.
    pub fn value(&self) -> u64 {
        self.value
    }
    ///Get a mutable reference to the `semaphore` field.
    pub fn semaphore_mut(&mut self) -> &mut Semaphore {
        &mut self.semaphore
    }
    ///Get a mutable reference to the `value` field.
    pub fn value_mut(&mut self) -> &mut u64 {
        &mut self.value
    }
    ///Sets the `semaphore` field.
    pub fn set_semaphore(&mut self, semaphore: Semaphore) -> &mut Self {
        self.semaphore = semaphore;
        self
    }
    ///Sets the `value` field.
    pub fn set_value(&mut self, value: u64) -> &mut Self {
        self.value = value;
        self
    }
    ///Sets the `semaphore` field in a builder way.
    pub fn with_semaphore(mut self, semaphore: Semaphore) -> Self {
        self.semaphore = semaphore;
        self
    }
    ///Sets the `value` field in a builder way.
    pub fn with_value(mut self, value: u64) -> Self {
        self.value = value;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SemaphoreSignalInfo {
    type LowLevel = crate::native::vulkan1_2::SemaphoreSignalInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::SemaphoreSignalInfo {
            s_type: StructureType::SemaphoreSignalInfo,
            p_next: std::ptr::null(),
            semaphore: self.semaphore.into_low_level(context, bump),
            value: self.value.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SemaphoreSignalInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            semaphore: crate::conv::FromLowLevel::from_low_level(context, value.semaphore),
            value: crate::conv::FromLowLevel::from_low_level(context, value.value),
        }
    }
}
#[doc(alias = "VkPhysicalDevice8BitStorageFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevice8BitStorageFeatures {
    #[doc(alias = "storageBuffer8BitAccess")]
    pub storage_buffer8_bit_access: bool,
    #[doc(alias = "uniformAndStorageBuffer8BitAccess")]
    pub uniform_and_storage_buffer8_bit_access: bool,
    #[doc(alias = "storagePushConstant8")]
    pub storage_push_constant8: bool,
}
impl PhysicalDevice8BitStorageFeatures {
    ///Get a reference to the `storage_buffer8_bit_access` field.
    pub fn storage_buffer8_bit_access(&self) -> &bool {
        &self.storage_buffer8_bit_access
    }
    ///Get a reference to the `uniform_and_storage_buffer8_bit_access` field.
    pub fn uniform_and_storage_buffer8_bit_access(&self) -> &bool {
        &self.uniform_and_storage_buffer8_bit_access
    }
    ///Get a reference to the `storage_push_constant8` field.
    pub fn storage_push_constant8(&self) -> &bool {
        &self.storage_push_constant8
    }
    ///Get a mutable reference to the `storage_buffer8_bit_access` field.
    pub fn storage_buffer8_bit_access_mut(&mut self) -> &mut bool {
        &mut self.storage_buffer8_bit_access
    }
    ///Get a mutable reference to the `uniform_and_storage_buffer8_bit_access` field.
    pub fn uniform_and_storage_buffer8_bit_access_mut(&mut self) -> &mut bool {
        &mut self.uniform_and_storage_buffer8_bit_access
    }
    ///Get a mutable reference to the `storage_push_constant8` field.
    pub fn storage_push_constant8_mut(&mut self) -> &mut bool {
        &mut self.storage_push_constant8
    }
    ///Sets the `storage_buffer8_bit_access` field.
    pub fn set_storage_buffer8_bit_access(&mut self, storage_buffer8_bit_access: bool) -> &mut Self {
        self.storage_buffer8_bit_access = storage_buffer8_bit_access;
        self
    }
    ///Sets the `uniform_and_storage_buffer8_bit_access` field.
    pub fn set_uniform_and_storage_buffer8_bit_access(
        &mut self,
        uniform_and_storage_buffer8_bit_access: bool,
    ) -> &mut Self {
        self.uniform_and_storage_buffer8_bit_access = uniform_and_storage_buffer8_bit_access;
        self
    }
    ///Sets the `storage_push_constant8` field.
    pub fn set_storage_push_constant8(&mut self, storage_push_constant8: bool) -> &mut Self {
        self.storage_push_constant8 = storage_push_constant8;
        self
    }
    ///Sets the `storage_buffer8_bit_access` field in a builder way.
    pub fn with_storage_buffer8_bit_access(mut self, storage_buffer8_bit_access: bool) -> Self {
        self.storage_buffer8_bit_access = storage_buffer8_bit_access;
        self
    }
    ///Sets the `uniform_and_storage_buffer8_bit_access` field in a builder way.
    pub fn with_uniform_and_storage_buffer8_bit_access(mut self, uniform_and_storage_buffer8_bit_access: bool) -> Self {
        self.uniform_and_storage_buffer8_bit_access = uniform_and_storage_buffer8_bit_access;
        self
    }
    ///Sets the `storage_push_constant8` field in a builder way.
    pub fn with_storage_push_constant8(mut self, storage_push_constant8: bool) -> Self {
        self.storage_push_constant8 = storage_push_constant8;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevice8BitStorageFeatures {
    type LowLevel = crate::native::vulkan1_2::PhysicalDevice8BitStorageFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDevice8BitStorageFeatures {
            s_type: StructureType::PhysicalDevice8bitStorageFeatures,
            p_next: std::ptr::null_mut(),
            storage_buffer8_bit_access: self.storage_buffer8_bit_access.into_low_level(context, bump),
            uniform_and_storage_buffer8_bit_access: self
                .uniform_and_storage_buffer8_bit_access
                .into_low_level(context, bump),
            storage_push_constant8: self.storage_push_constant8.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevice8BitStorageFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            storage_buffer8_bit_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.storage_buffer8_bit_access,
            ),
            uniform_and_storage_buffer8_bit_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.uniform_and_storage_buffer8_bit_access,
            ),
            storage_push_constant8: crate::conv::FromLowLevel::from_low_level(context, value.storage_push_constant8),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkanMemoryModelFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceVulkanMemoryModelFeatures {
    #[doc(alias = "vulkanMemoryModel")]
    pub vulkan_memory_model: bool,
    #[doc(alias = "vulkanMemoryModelDeviceScope")]
    pub vulkan_memory_model_device_scope: bool,
    #[doc(alias = "vulkanMemoryModelAvailabilityVisibilityChains")]
    pub vulkan_memory_model_availability_visibility_chains: bool,
}
impl PhysicalDeviceVulkanMemoryModelFeatures {
    ///Get a reference to the `vulkan_memory_model` field.
    pub fn vulkan_memory_model(&self) -> &bool {
        &self.vulkan_memory_model
    }
    ///Get a reference to the `vulkan_memory_model_device_scope` field.
    pub fn vulkan_memory_model_device_scope(&self) -> &bool {
        &self.vulkan_memory_model_device_scope
    }
    ///Get a reference to the `vulkan_memory_model_availability_visibility_chains` field.
    pub fn vulkan_memory_model_availability_visibility_chains(&self) -> &bool {
        &self.vulkan_memory_model_availability_visibility_chains
    }
    ///Get a mutable reference to the `vulkan_memory_model` field.
    pub fn vulkan_memory_model_mut(&mut self) -> &mut bool {
        &mut self.vulkan_memory_model
    }
    ///Get a mutable reference to the `vulkan_memory_model_device_scope` field.
    pub fn vulkan_memory_model_device_scope_mut(&mut self) -> &mut bool {
        &mut self.vulkan_memory_model_device_scope
    }
    ///Get a mutable reference to the `vulkan_memory_model_availability_visibility_chains` field.
    pub fn vulkan_memory_model_availability_visibility_chains_mut(&mut self) -> &mut bool {
        &mut self.vulkan_memory_model_availability_visibility_chains
    }
    ///Sets the `vulkan_memory_model` field.
    pub fn set_vulkan_memory_model(&mut self, vulkan_memory_model: bool) -> &mut Self {
        self.vulkan_memory_model = vulkan_memory_model;
        self
    }
    ///Sets the `vulkan_memory_model_device_scope` field.
    pub fn set_vulkan_memory_model_device_scope(&mut self, vulkan_memory_model_device_scope: bool) -> &mut Self {
        self.vulkan_memory_model_device_scope = vulkan_memory_model_device_scope;
        self
    }
    ///Sets the `vulkan_memory_model_availability_visibility_chains` field.
    pub fn set_vulkan_memory_model_availability_visibility_chains(
        &mut self,
        vulkan_memory_model_availability_visibility_chains: bool,
    ) -> &mut Self {
        self.vulkan_memory_model_availability_visibility_chains = vulkan_memory_model_availability_visibility_chains;
        self
    }
    ///Sets the `vulkan_memory_model` field in a builder way.
    pub fn with_vulkan_memory_model(mut self, vulkan_memory_model: bool) -> Self {
        self.vulkan_memory_model = vulkan_memory_model;
        self
    }
    ///Sets the `vulkan_memory_model_device_scope` field in a builder way.
    pub fn with_vulkan_memory_model_device_scope(mut self, vulkan_memory_model_device_scope: bool) -> Self {
        self.vulkan_memory_model_device_scope = vulkan_memory_model_device_scope;
        self
    }
    ///Sets the `vulkan_memory_model_availability_visibility_chains` field in a builder way.
    pub fn with_vulkan_memory_model_availability_visibility_chains(
        mut self,
        vulkan_memory_model_availability_visibility_chains: bool,
    ) -> Self {
        self.vulkan_memory_model_availability_visibility_chains = vulkan_memory_model_availability_visibility_chains;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceVulkanMemoryModelFeatures {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceVulkanMemoryModelFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceVulkanMemoryModelFeatures {
            s_type: StructureType::PhysicalDeviceVulkanMemoryModelFeatures,
            p_next: std::ptr::null_mut(),
            vulkan_memory_model: self.vulkan_memory_model.into_low_level(context, bump),
            vulkan_memory_model_device_scope: self.vulkan_memory_model_device_scope.into_low_level(context, bump),
            vulkan_memory_model_availability_visibility_chains: self
                .vulkan_memory_model_availability_visibility_chains
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceVulkanMemoryModelFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            vulkan_memory_model: crate::conv::FromLowLevel::from_low_level(context, value.vulkan_memory_model),
            vulkan_memory_model_device_scope: crate::conv::FromLowLevel::from_low_level(
                context,
                value.vulkan_memory_model_device_scope,
            ),
            vulkan_memory_model_availability_visibility_chains: crate::conv::FromLowLevel::from_low_level(
                context,
                value.vulkan_memory_model_availability_visibility_chains,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderAtomicInt64Features")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderAtomicInt64Features {
    #[doc(alias = "shaderBufferInt64Atomics")]
    pub shader_buffer_int64_atomics: bool,
    #[doc(alias = "shaderSharedInt64Atomics")]
    pub shader_shared_int64_atomics: bool,
}
impl PhysicalDeviceShaderAtomicInt64Features {
    ///Get a reference to the `shader_buffer_int64_atomics` field.
    pub fn shader_buffer_int64_atomics(&self) -> &bool {
        &self.shader_buffer_int64_atomics
    }
    ///Get a reference to the `shader_shared_int64_atomics` field.
    pub fn shader_shared_int64_atomics(&self) -> &bool {
        &self.shader_shared_int64_atomics
    }
    ///Get a mutable reference to the `shader_buffer_int64_atomics` field.
    pub fn shader_buffer_int64_atomics_mut(&mut self) -> &mut bool {
        &mut self.shader_buffer_int64_atomics
    }
    ///Get a mutable reference to the `shader_shared_int64_atomics` field.
    pub fn shader_shared_int64_atomics_mut(&mut self) -> &mut bool {
        &mut self.shader_shared_int64_atomics
    }
    ///Sets the `shader_buffer_int64_atomics` field.
    pub fn set_shader_buffer_int64_atomics(&mut self, shader_buffer_int64_atomics: bool) -> &mut Self {
        self.shader_buffer_int64_atomics = shader_buffer_int64_atomics;
        self
    }
    ///Sets the `shader_shared_int64_atomics` field.
    pub fn set_shader_shared_int64_atomics(&mut self, shader_shared_int64_atomics: bool) -> &mut Self {
        self.shader_shared_int64_atomics = shader_shared_int64_atomics;
        self
    }
    ///Sets the `shader_buffer_int64_atomics` field in a builder way.
    pub fn with_shader_buffer_int64_atomics(mut self, shader_buffer_int64_atomics: bool) -> Self {
        self.shader_buffer_int64_atomics = shader_buffer_int64_atomics;
        self
    }
    ///Sets the `shader_shared_int64_atomics` field in a builder way.
    pub fn with_shader_shared_int64_atomics(mut self, shader_shared_int64_atomics: bool) -> Self {
        self.shader_shared_int64_atomics = shader_shared_int64_atomics;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderAtomicInt64Features {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceShaderAtomicInt64Features;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceShaderAtomicInt64Features {
            s_type: StructureType::PhysicalDeviceShaderAtomicInt64Features,
            p_next: std::ptr::null_mut(),
            shader_buffer_int64_atomics: self.shader_buffer_int64_atomics.into_low_level(context, bump),
            shader_shared_int64_atomics: self.shader_shared_int64_atomics.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderAtomicInt64Features {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_buffer_int64_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_buffer_int64_atomics,
            ),
            shader_shared_int64_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_shared_int64_atomics,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceDepthStencilResolveProperties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDepthStencilResolveProperties {
    #[doc(alias = "supportedDepthResolveModes")]
    pub supported_depth_resolve_modes: ResolveModeFlags,
    #[doc(alias = "supportedStencilResolveModes")]
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    #[doc(alias = "independentResolveNone")]
    pub independent_resolve_none: bool,
    #[doc(alias = "independentResolve")]
    pub independent_resolve: bool,
}
impl PhysicalDeviceDepthStencilResolveProperties {
    ///Get a reference to the `supported_depth_resolve_modes` field.
    pub fn supported_depth_resolve_modes(&self) -> ResolveModeFlags {
        self.supported_depth_resolve_modes
    }
    ///Get a reference to the `supported_stencil_resolve_modes` field.
    pub fn supported_stencil_resolve_modes(&self) -> ResolveModeFlags {
        self.supported_stencil_resolve_modes
    }
    ///Get a reference to the `independent_resolve_none` field.
    pub fn independent_resolve_none(&self) -> &bool {
        &self.independent_resolve_none
    }
    ///Get a reference to the `independent_resolve` field.
    pub fn independent_resolve(&self) -> &bool {
        &self.independent_resolve
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDepthStencilResolveProperties {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceDepthStencilResolveProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceDepthStencilResolveProperties {
            s_type: StructureType::PhysicalDeviceDepthStencilResolveProperties,
            p_next: std::ptr::null_mut(),
            supported_depth_resolve_modes: self.supported_depth_resolve_modes.into_low_level(context, bump),
            supported_stencil_resolve_modes: self.supported_stencil_resolve_modes.into_low_level(context, bump),
            independent_resolve_none: self.independent_resolve_none.into_low_level(context, bump),
            independent_resolve: self.independent_resolve.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDepthStencilResolveProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            supported_depth_resolve_modes: crate::conv::FromLowLevel::from_low_level(
                context,
                value.supported_depth_resolve_modes,
            ),
            supported_stencil_resolve_modes: crate::conv::FromLowLevel::from_low_level(
                context,
                value.supported_stencil_resolve_modes,
            ),
            independent_resolve_none: crate::conv::FromLowLevel::from_low_level(
                context,
                value.independent_resolve_none,
            ),
            independent_resolve: crate::conv::FromLowLevel::from_low_level(context, value.independent_resolve),
        }
    }
}
#[doc(alias = "VkSubpassDescriptionDepthStencilResolve")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubpassDescriptionDepthStencilResolve {
    #[doc(alias = "depthResolveMode")]
    pub depth_resolve_mode: ResolveModeFlagBits,
    #[doc(alias = "stencilResolveMode")]
    pub stencil_resolve_mode: ResolveModeFlagBits,
    #[doc(alias = "pDepthStencilResolveAttachment")]
    pub depth_stencil_resolve_attachment: Option<AttachmentReference2>,
}
impl SubpassDescriptionDepthStencilResolve {
    ///Get a reference to the `depth_resolve_mode` field.
    pub fn depth_resolve_mode(&self) -> ResolveModeFlagBits {
        self.depth_resolve_mode
    }
    ///Get a reference to the `stencil_resolve_mode` field.
    pub fn stencil_resolve_mode(&self) -> ResolveModeFlagBits {
        self.stencil_resolve_mode
    }
    ///Get a reference to the `depth_stencil_resolve_attachment` field.
    pub fn depth_stencil_resolve_attachment(&self) -> &Option<AttachmentReference2> {
        &self.depth_stencil_resolve_attachment
    }
    ///Get a mutable reference to the `depth_resolve_mode` field.
    pub fn depth_resolve_mode_mut(&mut self) -> &mut ResolveModeFlagBits {
        &mut self.depth_resolve_mode
    }
    ///Get a mutable reference to the `stencil_resolve_mode` field.
    pub fn stencil_resolve_mode_mut(&mut self) -> &mut ResolveModeFlagBits {
        &mut self.stencil_resolve_mode
    }
    ///Get a mutable reference to the `depth_stencil_resolve_attachment` field.
    pub fn depth_stencil_resolve_attachment_mut(&mut self) -> &mut Option<AttachmentReference2> {
        &mut self.depth_stencil_resolve_attachment
    }
    ///Sets the `depth_resolve_mode` field.
    pub fn set_depth_resolve_mode(&mut self, depth_resolve_mode: ResolveModeFlagBits) -> &mut Self {
        self.depth_resolve_mode = depth_resolve_mode;
        self
    }
    ///Sets the `stencil_resolve_mode` field.
    pub fn set_stencil_resolve_mode(&mut self, stencil_resolve_mode: ResolveModeFlagBits) -> &mut Self {
        self.stencil_resolve_mode = stencil_resolve_mode;
        self
    }
    ///Sets the `depth_stencil_resolve_attachment` field.
    pub fn set_depth_stencil_resolve_attachment(
        &mut self,
        depth_stencil_resolve_attachment: Option<AttachmentReference2>,
    ) -> &mut Self {
        self.depth_stencil_resolve_attachment = depth_stencil_resolve_attachment;
        self
    }
    ///Sets the `depth_resolve_mode` field in a builder way.
    pub fn with_depth_resolve_mode(mut self, depth_resolve_mode: ResolveModeFlagBits) -> Self {
        self.depth_resolve_mode = depth_resolve_mode;
        self
    }
    ///Sets the `stencil_resolve_mode` field in a builder way.
    pub fn with_stencil_resolve_mode(mut self, stencil_resolve_mode: ResolveModeFlagBits) -> Self {
        self.stencil_resolve_mode = stencil_resolve_mode;
        self
    }
    ///Sets the `depth_stencil_resolve_attachment` field in a builder way.
    pub fn with_depth_stencil_resolve_attachment(
        mut self,
        depth_stencil_resolve_attachment: Option<AttachmentReference2>,
    ) -> Self {
        self.depth_stencil_resolve_attachment = depth_stencil_resolve_attachment;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubpassDescriptionDepthStencilResolve {
    type LowLevel = crate::native::vulkan1_2::SubpassDescriptionDepthStencilResolve;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::SubpassDescriptionDepthStencilResolve {
            s_type: StructureType::SubpassDescriptionDepthStencilResolve,
            p_next: std::ptr::null(),
            depth_resolve_mode: self.depth_resolve_mode.into_low_level(context, bump),
            stencil_resolve_mode: self.stencil_resolve_mode.into_low_level(context, bump),
            depth_stencil_resolve_attachment: self
                .depth_stencil_resolve_attachment
                .as_ref()
                .map(|v| bump.alloc(v.into_low_level(context, bump)) as *const _)
                .unwrap_or_else(std::ptr::null),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SubpassDescriptionDepthStencilResolve {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            depth_resolve_mode: crate::conv::FromLowLevel::from_low_level(context, value.depth_resolve_mode),
            stencil_resolve_mode: crate::conv::FromLowLevel::from_low_level(context, value.stencil_resolve_mode),
            depth_stencil_resolve_attachment: crate::conv::FromLowLevel::from_low_level(
                context,
                *value.depth_stencil_resolve_attachment,
            ),
        }
    }
}
#[doc(alias = "VkImageStencilUsageCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageStencilUsageCreateInfo {
    #[doc(alias = "stencilUsage")]
    pub stencil_usage: ImageUsageFlags,
}
impl ImageStencilUsageCreateInfo {
    ///Get a reference to the `stencil_usage` field.
    pub fn stencil_usage(&self) -> ImageUsageFlags {
        self.stencil_usage
    }
    ///Get a mutable reference to the `stencil_usage` field.
    pub fn stencil_usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.stencil_usage
    }
    ///Sets the `stencil_usage` field.
    pub fn set_stencil_usage(&mut self, stencil_usage: ImageUsageFlags) -> &mut Self {
        self.stencil_usage = stencil_usage;
        self
    }
    ///Sets the `stencil_usage` field in a builder way.
    pub fn with_stencil_usage(mut self, stencil_usage: ImageUsageFlags) -> Self {
        self.stencil_usage = stencil_usage;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageStencilUsageCreateInfo {
    type LowLevel = crate::native::vulkan1_2::ImageStencilUsageCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::ImageStencilUsageCreateInfo {
            s_type: StructureType::ImageStencilUsageCreateInfo,
            p_next: std::ptr::null(),
            stencil_usage: self.stencil_usage.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageStencilUsageCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            stencil_usage: crate::conv::FromLowLevel::from_low_level(context, value.stencil_usage),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceScalarBlockLayoutFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceScalarBlockLayoutFeatures {
    #[doc(alias = "scalarBlockLayout")]
    pub scalar_block_layout: bool,
}
impl PhysicalDeviceScalarBlockLayoutFeatures {
    ///Get a reference to the `scalar_block_layout` field.
    pub fn scalar_block_layout(&self) -> &bool {
        &self.scalar_block_layout
    }
    ///Get a mutable reference to the `scalar_block_layout` field.
    pub fn scalar_block_layout_mut(&mut self) -> &mut bool {
        &mut self.scalar_block_layout
    }
    ///Sets the `scalar_block_layout` field.
    pub fn set_scalar_block_layout(&mut self, scalar_block_layout: bool) -> &mut Self {
        self.scalar_block_layout = scalar_block_layout;
        self
    }
    ///Sets the `scalar_block_layout` field in a builder way.
    pub fn with_scalar_block_layout(mut self, scalar_block_layout: bool) -> Self {
        self.scalar_block_layout = scalar_block_layout;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceScalarBlockLayoutFeatures {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceScalarBlockLayoutFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceScalarBlockLayoutFeatures {
            s_type: StructureType::PhysicalDeviceScalarBlockLayoutFeatures,
            p_next: std::ptr::null_mut(),
            scalar_block_layout: self.scalar_block_layout.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceScalarBlockLayoutFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            scalar_block_layout: crate::conv::FromLowLevel::from_low_level(context, value.scalar_block_layout),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceUniformBufferStandardLayoutFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures {
    #[doc(alias = "uniformBufferStandardLayout")]
    pub uniform_buffer_standard_layout: bool,
}
impl PhysicalDeviceUniformBufferStandardLayoutFeatures {
    ///Get a reference to the `uniform_buffer_standard_layout` field.
    pub fn uniform_buffer_standard_layout(&self) -> &bool {
        &self.uniform_buffer_standard_layout
    }
    ///Get a mutable reference to the `uniform_buffer_standard_layout` field.
    pub fn uniform_buffer_standard_layout_mut(&mut self) -> &mut bool {
        &mut self.uniform_buffer_standard_layout
    }
    ///Sets the `uniform_buffer_standard_layout` field.
    pub fn set_uniform_buffer_standard_layout(&mut self, uniform_buffer_standard_layout: bool) -> &mut Self {
        self.uniform_buffer_standard_layout = uniform_buffer_standard_layout;
        self
    }
    ///Sets the `uniform_buffer_standard_layout` field in a builder way.
    pub fn with_uniform_buffer_standard_layout(mut self, uniform_buffer_standard_layout: bool) -> Self {
        self.uniform_buffer_standard_layout = uniform_buffer_standard_layout;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceUniformBufferStandardLayoutFeatures {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceUniformBufferStandardLayoutFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceUniformBufferStandardLayoutFeatures {
            s_type: StructureType::PhysicalDeviceUniformBufferStandardLayoutFeatures,
            p_next: std::ptr::null_mut(),
            uniform_buffer_standard_layout: self.uniform_buffer_standard_layout.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceUniformBufferStandardLayoutFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            uniform_buffer_standard_layout: crate::conv::FromLowLevel::from_low_level(
                context,
                value.uniform_buffer_standard_layout,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceBufferDeviceAddressFeatures {
    #[doc(alias = "bufferDeviceAddress")]
    pub buffer_device_address: bool,
    #[doc(alias = "bufferDeviceAddressCaptureReplay")]
    pub buffer_device_address_capture_replay: bool,
    #[doc(alias = "bufferDeviceAddressMultiDevice")]
    pub buffer_device_address_multi_device: bool,
}
impl PhysicalDeviceBufferDeviceAddressFeatures {
    ///Get a reference to the `buffer_device_address` field.
    pub fn buffer_device_address(&self) -> &bool {
        &self.buffer_device_address
    }
    ///Get a reference to the `buffer_device_address_capture_replay` field.
    pub fn buffer_device_address_capture_replay(&self) -> &bool {
        &self.buffer_device_address_capture_replay
    }
    ///Get a reference to the `buffer_device_address_multi_device` field.
    pub fn buffer_device_address_multi_device(&self) -> &bool {
        &self.buffer_device_address_multi_device
    }
    ///Get a mutable reference to the `buffer_device_address` field.
    pub fn buffer_device_address_mut(&mut self) -> &mut bool {
        &mut self.buffer_device_address
    }
    ///Get a mutable reference to the `buffer_device_address_capture_replay` field.
    pub fn buffer_device_address_capture_replay_mut(&mut self) -> &mut bool {
        &mut self.buffer_device_address_capture_replay
    }
    ///Get a mutable reference to the `buffer_device_address_multi_device` field.
    pub fn buffer_device_address_multi_device_mut(&mut self) -> &mut bool {
        &mut self.buffer_device_address_multi_device
    }
    ///Sets the `buffer_device_address` field.
    pub fn set_buffer_device_address(&mut self, buffer_device_address: bool) -> &mut Self {
        self.buffer_device_address = buffer_device_address;
        self
    }
    ///Sets the `buffer_device_address_capture_replay` field.
    pub fn set_buffer_device_address_capture_replay(
        &mut self,
        buffer_device_address_capture_replay: bool,
    ) -> &mut Self {
        self.buffer_device_address_capture_replay = buffer_device_address_capture_replay;
        self
    }
    ///Sets the `buffer_device_address_multi_device` field.
    pub fn set_buffer_device_address_multi_device(&mut self, buffer_device_address_multi_device: bool) -> &mut Self {
        self.buffer_device_address_multi_device = buffer_device_address_multi_device;
        self
    }
    ///Sets the `buffer_device_address` field in a builder way.
    pub fn with_buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.buffer_device_address = buffer_device_address;
        self
    }
    ///Sets the `buffer_device_address_capture_replay` field in a builder way.
    pub fn with_buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.buffer_device_address_capture_replay = buffer_device_address_capture_replay;
        self
    }
    ///Sets the `buffer_device_address_multi_device` field in a builder way.
    pub fn with_buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.buffer_device_address_multi_device = buffer_device_address_multi_device;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceBufferDeviceAddressFeatures {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceBufferDeviceAddressFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceBufferDeviceAddressFeatures {
            s_type: StructureType::PhysicalDeviceBufferDeviceAddressFeatures,
            p_next: std::ptr::null_mut(),
            buffer_device_address: self.buffer_device_address.into_low_level(context, bump),
            buffer_device_address_capture_replay: self
                .buffer_device_address_capture_replay
                .into_low_level(context, bump),
            buffer_device_address_multi_device: self.buffer_device_address_multi_device.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceBufferDeviceAddressFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            buffer_device_address: crate::conv::FromLowLevel::from_low_level(context, value.buffer_device_address),
            buffer_device_address_capture_replay: crate::conv::FromLowLevel::from_low_level(
                context,
                value.buffer_device_address_capture_replay,
            ),
            buffer_device_address_multi_device: crate::conv::FromLowLevel::from_low_level(
                context,
                value.buffer_device_address_multi_device,
            ),
        }
    }
}
#[doc(alias = "VkBufferDeviceAddressInfo")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferDeviceAddressInfo {
    pub buffer: Buffer,
}
impl BufferDeviceAddressInfo {
    ///Get a reference to the `buffer` field.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    ///Get a mutable reference to the `buffer` field.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Sets the `buffer` field.
    pub fn set_buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `buffer` field in a builder way.
    pub fn with_buffer(mut self, buffer: Buffer) -> Self {
        self.buffer = buffer;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferDeviceAddressInfo {
    type LowLevel = crate::native::vulkan1_2::BufferDeviceAddressInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::BufferDeviceAddressInfo {
            s_type: StructureType::BufferDeviceAddressInfo,
            p_next: std::ptr::null(),
            buffer: self.buffer.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferDeviceAddressInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            buffer: crate::conv::FromLowLevel::from_low_level(context, value.buffer),
        }
    }
}
#[doc(alias = "VkBufferOpaqueCaptureAddressCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferOpaqueCaptureAddressCreateInfo {
    #[doc(alias = "opaqueCaptureAddress")]
    pub opaque_capture_address: u64,
}
impl BufferOpaqueCaptureAddressCreateInfo {
    ///Get a reference to the `opaque_capture_address` field.
    pub fn opaque_capture_address(&self) -> u64 {
        self.opaque_capture_address
    }
    ///Get a mutable reference to the `opaque_capture_address` field.
    pub fn opaque_capture_address_mut(&mut self) -> &mut u64 {
        &mut self.opaque_capture_address
    }
    ///Sets the `opaque_capture_address` field.
    pub fn set_opaque_capture_address(&mut self, opaque_capture_address: u64) -> &mut Self {
        self.opaque_capture_address = opaque_capture_address;
        self
    }
    ///Sets the `opaque_capture_address` field in a builder way.
    pub fn with_opaque_capture_address(mut self, opaque_capture_address: u64) -> Self {
        self.opaque_capture_address = opaque_capture_address;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferOpaqueCaptureAddressCreateInfo {
    type LowLevel = crate::native::vulkan1_2::BufferOpaqueCaptureAddressCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::BufferOpaqueCaptureAddressCreateInfo {
            s_type: StructureType::BufferOpaqueCaptureAddressCreateInfo,
            p_next: std::ptr::null(),
            opaque_capture_address: self.opaque_capture_address.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferOpaqueCaptureAddressCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            opaque_capture_address: crate::conv::FromLowLevel::from_low_level(context, value.opaque_capture_address),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceImagelessFramebufferFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceImagelessFramebufferFeatures {
    #[doc(alias = "imagelessFramebuffer")]
    pub imageless_framebuffer: bool,
}
impl PhysicalDeviceImagelessFramebufferFeatures {
    ///Get a reference to the `imageless_framebuffer` field.
    pub fn imageless_framebuffer(&self) -> &bool {
        &self.imageless_framebuffer
    }
    ///Get a mutable reference to the `imageless_framebuffer` field.
    pub fn imageless_framebuffer_mut(&mut self) -> &mut bool {
        &mut self.imageless_framebuffer
    }
    ///Sets the `imageless_framebuffer` field.
    pub fn set_imageless_framebuffer(&mut self, imageless_framebuffer: bool) -> &mut Self {
        self.imageless_framebuffer = imageless_framebuffer;
        self
    }
    ///Sets the `imageless_framebuffer` field in a builder way.
    pub fn with_imageless_framebuffer(mut self, imageless_framebuffer: bool) -> Self {
        self.imageless_framebuffer = imageless_framebuffer;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceImagelessFramebufferFeatures {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceImagelessFramebufferFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceImagelessFramebufferFeatures {
            s_type: StructureType::PhysicalDeviceImagelessFramebufferFeatures,
            p_next: std::ptr::null_mut(),
            imageless_framebuffer: self.imageless_framebuffer.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceImagelessFramebufferFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            imageless_framebuffer: crate::conv::FromLowLevel::from_low_level(context, value.imageless_framebuffer),
        }
    }
}
#[doc(alias = "VkFramebufferAttachmentsCreateInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FramebufferAttachmentsCreateInfo {
    #[doc(alias = "pAttachmentImageInfos")]
    pub attachment_image_infos: SmallVec<[FramebufferAttachmentImageInfo; 8]>,
}
impl FramebufferAttachmentsCreateInfo {
    ///Get a reference to the `attachment_image_infos` field.
    pub fn attachment_image_infos(&self) -> &SmallVec<[FramebufferAttachmentImageInfo; 8]> {
        &self.attachment_image_infos
    }
    ///Get a mutable reference to the `attachment_image_infos` field.
    pub fn attachment_image_infos_mut(&mut self) -> &mut SmallVec<[FramebufferAttachmentImageInfo; 8]> {
        &mut self.attachment_image_infos
    }
    ///Sets the `attachment_image_infos` field.
    pub fn set_attachment_image_infos(
        &mut self,
        attachment_image_infos: SmallVec<[FramebufferAttachmentImageInfo; 8]>,
    ) -> &mut Self {
        self.attachment_image_infos = attachment_image_infos;
        self
    }
    ///Sets the `attachment_image_infos` field in a builder way.
    pub fn with_attachment_image_infos(
        mut self,
        attachment_image_infos: SmallVec<[FramebufferAttachmentImageInfo; 8]>,
    ) -> Self {
        self.attachment_image_infos = attachment_image_infos;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for FramebufferAttachmentsCreateInfo {
    type LowLevel = crate::native::vulkan1_2::FramebufferAttachmentsCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_attachment_image_infos = self.attachment_image_infos.len() as u32;
        let attachment_image_infos = bump
            .alloc_slice_fill_iter(
                self.attachment_image_infos
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::vulkan1_2::FramebufferAttachmentsCreateInfo {
            s_type: StructureType::FramebufferAttachmentsCreateInfo,
            p_next: std::ptr::null(),
            attachment_image_info_count: len_attachment_image_infos,
            attachment_image_infos: attachment_image_infos,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for FramebufferAttachmentsCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let attachment_image_infos_len = value.attachment_image_info_count;
        let mut attachment_image_infos = SmallVec::with_capacity(attachment_image_infos_len as usize);
        for i in 0..attachment_image_infos_len {
            attachment_image_infos.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.attachment_image_infos.add(i as usize).read(),
            ));
        }
        Self {
            attachment_image_infos: attachment_image_infos,
        }
    }
}
#[doc(alias = "VkFramebufferAttachmentImageInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FramebufferAttachmentImageInfo {
    pub flags: ImageCreateFlags,
    pub usage: ImageUsageFlags,
    pub width: u32,
    pub height: u32,
    #[doc(alias = "layerCount")]
    pub layer_count: u32,
    #[doc(alias = "pViewFormats")]
    pub view_formats: SmallVec<[Format; 8]>,
}
impl FramebufferAttachmentImageInfo {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> ImageCreateFlags {
        self.flags
    }
    ///Get a reference to the `usage` field.
    pub fn usage(&self) -> ImageUsageFlags {
        self.usage
    }
    ///Get a reference to the `width` field.
    pub fn width(&self) -> u32 {
        self.width
    }
    ///Get a reference to the `height` field.
    pub fn height(&self) -> u32 {
        self.height
    }
    ///Get a reference to the `layer_count` field.
    pub fn layer_count(&self) -> u32 {
        self.layer_count
    }
    ///Get a reference to the `view_formats` field.
    pub fn view_formats(&self) -> &SmallVec<[Format; 8]> {
        &self.view_formats
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut ImageCreateFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `usage` field.
    pub fn usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.usage
    }
    ///Get a mutable reference to the `width` field.
    pub fn width_mut(&mut self) -> &mut u32 {
        &mut self.width
    }
    ///Get a mutable reference to the `height` field.
    pub fn height_mut(&mut self) -> &mut u32 {
        &mut self.height
    }
    ///Get a mutable reference to the `layer_count` field.
    pub fn layer_count_mut(&mut self) -> &mut u32 {
        &mut self.layer_count
    }
    ///Get a mutable reference to the `view_formats` field.
    pub fn view_formats_mut(&mut self) -> &mut SmallVec<[Format; 8]> {
        &mut self.view_formats
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: ImageCreateFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `usage` field.
    pub fn set_usage(&mut self, usage: ImageUsageFlags) -> &mut Self {
        self.usage = usage;
        self
    }
    ///Sets the `width` field.
    pub fn set_width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }
    ///Sets the `height` field.
    pub fn set_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }
    ///Sets the `layer_count` field.
    pub fn set_layer_count(&mut self, layer_count: u32) -> &mut Self {
        self.layer_count = layer_count;
        self
    }
    ///Sets the `view_formats` field.
    pub fn set_view_formats(&mut self, view_formats: SmallVec<[Format; 8]>) -> &mut Self {
        self.view_formats = view_formats;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: ImageCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `usage` field in a builder way.
    pub fn with_usage(mut self, usage: ImageUsageFlags) -> Self {
        self.usage = usage;
        self
    }
    ///Sets the `width` field in a builder way.
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    ///Sets the `height` field in a builder way.
    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    ///Sets the `layer_count` field in a builder way.
    pub fn with_layer_count(mut self, layer_count: u32) -> Self {
        self.layer_count = layer_count;
        self
    }
    ///Sets the `view_formats` field in a builder way.
    pub fn with_view_formats(mut self, view_formats: SmallVec<[Format; 8]>) -> Self {
        self.view_formats = view_formats;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for FramebufferAttachmentImageInfo {
    type LowLevel = crate::native::vulkan1_2::FramebufferAttachmentImageInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_view_formats = self.view_formats.len() as u32;
        let view_formats = bump
            .alloc_slice_fill_iter(self.view_formats.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_2::FramebufferAttachmentImageInfo {
            s_type: StructureType::FramebufferAttachmentImageInfo,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            usage: self.usage.into_low_level(context, bump),
            width: self.width.into_low_level(context, bump),
            height: self.height.into_low_level(context, bump),
            layer_count: self.layer_count.into_low_level(context, bump),
            view_format_count: len_view_formats,
            view_formats: view_formats,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for FramebufferAttachmentImageInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let view_formats_len = value.view_format_count;
        let mut view_formats = SmallVec::with_capacity(view_formats_len as usize);
        for i in 0..view_formats_len {
            view_formats.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.view_formats.add(i as usize).read(),
            ));
        }
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            usage: crate::conv::FromLowLevel::from_low_level(context, value.usage),
            width: crate::conv::FromLowLevel::from_low_level(context, value.width),
            height: crate::conv::FromLowLevel::from_low_level(context, value.height),
            layer_count: crate::conv::FromLowLevel::from_low_level(context, value.layer_count),
            view_formats: view_formats,
        }
    }
}
#[doc(alias = "VkRenderPassAttachmentBeginInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderPassAttachmentBeginInfo {
    #[doc(alias = "pAttachments")]
    pub attachments: SmallVec<[ImageView; 8]>,
}
impl RenderPassAttachmentBeginInfo {
    ///Get a reference to the `attachments` field.
    pub fn attachments(&self) -> &SmallVec<[ImageView; 8]> {
        &self.attachments
    }
    ///Get a mutable reference to the `attachments` field.
    pub fn attachments_mut(&mut self) -> &mut SmallVec<[ImageView; 8]> {
        &mut self.attachments
    }
    ///Sets the `attachments` field.
    pub fn set_attachments(&mut self, attachments: SmallVec<[ImageView; 8]>) -> &mut Self {
        self.attachments = attachments;
        self
    }
    ///Sets the `attachments` field in a builder way.
    pub fn with_attachments(mut self, attachments: SmallVec<[ImageView; 8]>) -> Self {
        self.attachments = attachments;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderPassAttachmentBeginInfo {
    type LowLevel = crate::native::vulkan1_2::RenderPassAttachmentBeginInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_attachments = self.attachments.len() as u32;
        let attachments = bump
            .alloc_slice_fill_iter(self.attachments.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_2::RenderPassAttachmentBeginInfo {
            s_type: StructureType::RenderPassAttachmentBeginInfo,
            p_next: std::ptr::null(),
            attachment_count: len_attachments,
            attachments: attachments,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RenderPassAttachmentBeginInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let attachments_len = value.attachment_count;
        let mut attachments = SmallVec::with_capacity(attachments_len as usize);
        for i in 0..attachments_len {
            attachments.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.attachments.add(i as usize).read(),
            ));
        }
        Self {
            attachments: attachments,
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    #[doc(alias = "separateDepthStencilLayouts")]
    pub separate_depth_stencil_layouts: bool,
}
impl PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    ///Get a reference to the `separate_depth_stencil_layouts` field.
    pub fn separate_depth_stencil_layouts(&self) -> &bool {
        &self.separate_depth_stencil_layouts
    }
    ///Get a mutable reference to the `separate_depth_stencil_layouts` field.
    pub fn separate_depth_stencil_layouts_mut(&mut self) -> &mut bool {
        &mut self.separate_depth_stencil_layouts
    }
    ///Sets the `separate_depth_stencil_layouts` field.
    pub fn set_separate_depth_stencil_layouts(&mut self, separate_depth_stencil_layouts: bool) -> &mut Self {
        self.separate_depth_stencil_layouts = separate_depth_stencil_layouts;
        self
    }
    ///Sets the `separate_depth_stencil_layouts` field in a builder way.
    pub fn with_separate_depth_stencil_layouts(mut self, separate_depth_stencil_layouts: bool) -> Self {
        self.separate_depth_stencil_layouts = separate_depth_stencil_layouts;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
            s_type: StructureType::PhysicalDeviceSeparateDepthStencilLayoutsFeatures,
            p_next: std::ptr::null_mut(),
            separate_depth_stencil_layouts: self.separate_depth_stencil_layouts.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            separate_depth_stencil_layouts: crate::conv::FromLowLevel::from_low_level(
                context,
                value.separate_depth_stencil_layouts,
            ),
        }
    }
}
#[doc(alias = "VkAttachmentReferenceStencilLayout")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttachmentReferenceStencilLayout {
    #[doc(alias = "stencilLayout")]
    pub stencil_layout: ImageLayout,
}
impl AttachmentReferenceStencilLayout {
    ///Get a reference to the `stencil_layout` field.
    pub fn stencil_layout(&self) -> ImageLayout {
        self.stencil_layout
    }
    ///Get a mutable reference to the `stencil_layout` field.
    pub fn stencil_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.stencil_layout
    }
    ///Sets the `stencil_layout` field.
    pub fn set_stencil_layout(&mut self, stencil_layout: ImageLayout) -> &mut Self {
        self.stencil_layout = stencil_layout;
        self
    }
    ///Sets the `stencil_layout` field in a builder way.
    pub fn with_stencil_layout(mut self, stencil_layout: ImageLayout) -> Self {
        self.stencil_layout = stencil_layout;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AttachmentReferenceStencilLayout {
    type LowLevel = crate::native::vulkan1_2::AttachmentReferenceStencilLayout;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::AttachmentReferenceStencilLayout {
            s_type: StructureType::AttachmentReferenceStencilLayout,
            p_next: std::ptr::null_mut(),
            stencil_layout: self.stencil_layout.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AttachmentReferenceStencilLayout {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            stencil_layout: crate::conv::FromLowLevel::from_low_level(context, value.stencil_layout),
        }
    }
}
#[doc(alias = "VkAttachmentDescriptionStencilLayout")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttachmentDescriptionStencilLayout {
    #[doc(alias = "stencilInitialLayout")]
    pub stencil_initial_layout: ImageLayout,
    #[doc(alias = "stencilFinalLayout")]
    pub stencil_final_layout: ImageLayout,
}
impl AttachmentDescriptionStencilLayout {
    ///Get a reference to the `stencil_initial_layout` field.
    pub fn stencil_initial_layout(&self) -> ImageLayout {
        self.stencil_initial_layout
    }
    ///Get a reference to the `stencil_final_layout` field.
    pub fn stencil_final_layout(&self) -> ImageLayout {
        self.stencil_final_layout
    }
    ///Get a mutable reference to the `stencil_initial_layout` field.
    pub fn stencil_initial_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.stencil_initial_layout
    }
    ///Get a mutable reference to the `stencil_final_layout` field.
    pub fn stencil_final_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.stencil_final_layout
    }
    ///Sets the `stencil_initial_layout` field.
    pub fn set_stencil_initial_layout(&mut self, stencil_initial_layout: ImageLayout) -> &mut Self {
        self.stencil_initial_layout = stencil_initial_layout;
        self
    }
    ///Sets the `stencil_final_layout` field.
    pub fn set_stencil_final_layout(&mut self, stencil_final_layout: ImageLayout) -> &mut Self {
        self.stencil_final_layout = stencil_final_layout;
        self
    }
    ///Sets the `stencil_initial_layout` field in a builder way.
    pub fn with_stencil_initial_layout(mut self, stencil_initial_layout: ImageLayout) -> Self {
        self.stencil_initial_layout = stencil_initial_layout;
        self
    }
    ///Sets the `stencil_final_layout` field in a builder way.
    pub fn with_stencil_final_layout(mut self, stencil_final_layout: ImageLayout) -> Self {
        self.stencil_final_layout = stencil_final_layout;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AttachmentDescriptionStencilLayout {
    type LowLevel = crate::native::vulkan1_2::AttachmentDescriptionStencilLayout;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::AttachmentDescriptionStencilLayout {
            s_type: StructureType::AttachmentDescriptionStencilLayout,
            p_next: std::ptr::null_mut(),
            stencil_initial_layout: self.stencil_initial_layout.into_low_level(context, bump),
            stencil_final_layout: self.stencil_final_layout.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AttachmentDescriptionStencilLayout {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            stencil_initial_layout: crate::conv::FromLowLevel::from_low_level(context, value.stencil_initial_layout),
            stencil_final_layout: crate::conv::FromLowLevel::from_low_level(context, value.stencil_final_layout),
        }
    }
}
#[doc(alias = "VkMemoryOpaqueCaptureAddressAllocateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryOpaqueCaptureAddressAllocateInfo {
    #[doc(alias = "opaqueCaptureAddress")]
    pub opaque_capture_address: u64,
}
impl MemoryOpaqueCaptureAddressAllocateInfo {
    ///Get a reference to the `opaque_capture_address` field.
    pub fn opaque_capture_address(&self) -> u64 {
        self.opaque_capture_address
    }
    ///Get a mutable reference to the `opaque_capture_address` field.
    pub fn opaque_capture_address_mut(&mut self) -> &mut u64 {
        &mut self.opaque_capture_address
    }
    ///Sets the `opaque_capture_address` field.
    pub fn set_opaque_capture_address(&mut self, opaque_capture_address: u64) -> &mut Self {
        self.opaque_capture_address = opaque_capture_address;
        self
    }
    ///Sets the `opaque_capture_address` field in a builder way.
    pub fn with_opaque_capture_address(mut self, opaque_capture_address: u64) -> Self {
        self.opaque_capture_address = opaque_capture_address;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryOpaqueCaptureAddressAllocateInfo {
    type LowLevel = crate::native::vulkan1_2::MemoryOpaqueCaptureAddressAllocateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::MemoryOpaqueCaptureAddressAllocateInfo {
            s_type: StructureType::MemoryOpaqueCaptureAddressAllocateInfo,
            p_next: std::ptr::null(),
            opaque_capture_address: self.opaque_capture_address.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryOpaqueCaptureAddressAllocateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            opaque_capture_address: crate::conv::FromLowLevel::from_low_level(context, value.opaque_capture_address),
        }
    }
}
#[doc(alias = "VkDeviceMemoryOpaqueCaptureAddressInfo")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceMemoryOpaqueCaptureAddressInfo {
    pub memory: DeviceMemory,
}
impl DeviceMemoryOpaqueCaptureAddressInfo {
    ///Get a reference to the `memory` field.
    pub fn memory(&self) -> &DeviceMemory {
        &self.memory
    }
    ///Get a mutable reference to the `memory` field.
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Sets the `memory` field.
    pub fn set_memory(&mut self, memory: DeviceMemory) -> &mut Self {
        self.memory = memory;
        self
    }
    ///Sets the `memory` field in a builder way.
    pub fn with_memory(mut self, memory: DeviceMemory) -> Self {
        self.memory = memory;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceMemoryOpaqueCaptureAddressInfo {
    type LowLevel = crate::native::vulkan1_2::DeviceMemoryOpaqueCaptureAddressInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::DeviceMemoryOpaqueCaptureAddressInfo {
            s_type: StructureType::DeviceMemoryOpaqueCaptureAddressInfo,
            p_next: std::ptr::null(),
            memory: self.memory.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceMemoryOpaqueCaptureAddressInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            memory: crate::conv::FromLowLevel::from_low_level(context, value.memory),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkan11Features")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceVulkan11Features {
    #[doc(alias = "storageBuffer16BitAccess")]
    pub storage_buffer16_bit_access: bool,
    #[doc(alias = "uniformAndStorageBuffer16BitAccess")]
    pub uniform_and_storage_buffer16_bit_access: bool,
    #[doc(alias = "storagePushConstant16")]
    pub storage_push_constant16: bool,
    #[doc(alias = "storageInputOutput16")]
    pub storage_input_output16: bool,
    pub multiview: bool,
    #[doc(alias = "multiviewGeometryShader")]
    pub multiview_geometry_shader: bool,
    #[doc(alias = "multiviewTessellationShader")]
    pub multiview_tessellation_shader: bool,
    #[doc(alias = "variablePointersStorageBuffer")]
    pub variable_pointers_storage_buffer: bool,
    #[doc(alias = "variablePointers")]
    pub variable_pointers: bool,
    #[doc(alias = "protectedMemory")]
    pub protected_memory: bool,
    #[doc(alias = "samplerYcbcrConversion")]
    pub sampler_ycbcr_conversion: bool,
    #[doc(alias = "shaderDrawParameters")]
    pub shader_draw_parameters: bool,
}
impl PhysicalDeviceVulkan11Features {
    ///Get a reference to the `storage_buffer16_bit_access` field.
    pub fn storage_buffer16_bit_access(&self) -> &bool {
        &self.storage_buffer16_bit_access
    }
    ///Get a reference to the `uniform_and_storage_buffer16_bit_access` field.
    pub fn uniform_and_storage_buffer16_bit_access(&self) -> &bool {
        &self.uniform_and_storage_buffer16_bit_access
    }
    ///Get a reference to the `storage_push_constant16` field.
    pub fn storage_push_constant16(&self) -> &bool {
        &self.storage_push_constant16
    }
    ///Get a reference to the `storage_input_output16` field.
    pub fn storage_input_output16(&self) -> &bool {
        &self.storage_input_output16
    }
    ///Get a reference to the `multiview` field.
    pub fn multiview(&self) -> &bool {
        &self.multiview
    }
    ///Get a reference to the `multiview_geometry_shader` field.
    pub fn multiview_geometry_shader(&self) -> &bool {
        &self.multiview_geometry_shader
    }
    ///Get a reference to the `multiview_tessellation_shader` field.
    pub fn multiview_tessellation_shader(&self) -> &bool {
        &self.multiview_tessellation_shader
    }
    ///Get a reference to the `variable_pointers_storage_buffer` field.
    pub fn variable_pointers_storage_buffer(&self) -> &bool {
        &self.variable_pointers_storage_buffer
    }
    ///Get a reference to the `variable_pointers` field.
    pub fn variable_pointers(&self) -> &bool {
        &self.variable_pointers
    }
    ///Get a reference to the `protected_memory` field.
    pub fn protected_memory(&self) -> &bool {
        &self.protected_memory
    }
    ///Get a reference to the `sampler_ycbcr_conversion` field.
    pub fn sampler_ycbcr_conversion(&self) -> &bool {
        &self.sampler_ycbcr_conversion
    }
    ///Get a reference to the `shader_draw_parameters` field.
    pub fn shader_draw_parameters(&self) -> &bool {
        &self.shader_draw_parameters
    }
    ///Get a mutable reference to the `storage_buffer16_bit_access` field.
    pub fn storage_buffer16_bit_access_mut(&mut self) -> &mut bool {
        &mut self.storage_buffer16_bit_access
    }
    ///Get a mutable reference to the `uniform_and_storage_buffer16_bit_access` field.
    pub fn uniform_and_storage_buffer16_bit_access_mut(&mut self) -> &mut bool {
        &mut self.uniform_and_storage_buffer16_bit_access
    }
    ///Get a mutable reference to the `storage_push_constant16` field.
    pub fn storage_push_constant16_mut(&mut self) -> &mut bool {
        &mut self.storage_push_constant16
    }
    ///Get a mutable reference to the `storage_input_output16` field.
    pub fn storage_input_output16_mut(&mut self) -> &mut bool {
        &mut self.storage_input_output16
    }
    ///Get a mutable reference to the `multiview` field.
    pub fn multiview_mut(&mut self) -> &mut bool {
        &mut self.multiview
    }
    ///Get a mutable reference to the `multiview_geometry_shader` field.
    pub fn multiview_geometry_shader_mut(&mut self) -> &mut bool {
        &mut self.multiview_geometry_shader
    }
    ///Get a mutable reference to the `multiview_tessellation_shader` field.
    pub fn multiview_tessellation_shader_mut(&mut self) -> &mut bool {
        &mut self.multiview_tessellation_shader
    }
    ///Get a mutable reference to the `variable_pointers_storage_buffer` field.
    pub fn variable_pointers_storage_buffer_mut(&mut self) -> &mut bool {
        &mut self.variable_pointers_storage_buffer
    }
    ///Get a mutable reference to the `variable_pointers` field.
    pub fn variable_pointers_mut(&mut self) -> &mut bool {
        &mut self.variable_pointers
    }
    ///Get a mutable reference to the `protected_memory` field.
    pub fn protected_memory_mut(&mut self) -> &mut bool {
        &mut self.protected_memory
    }
    ///Get a mutable reference to the `sampler_ycbcr_conversion` field.
    pub fn sampler_ycbcr_conversion_mut(&mut self) -> &mut bool {
        &mut self.sampler_ycbcr_conversion
    }
    ///Get a mutable reference to the `shader_draw_parameters` field.
    pub fn shader_draw_parameters_mut(&mut self) -> &mut bool {
        &mut self.shader_draw_parameters
    }
    ///Sets the `storage_buffer16_bit_access` field.
    pub fn set_storage_buffer16_bit_access(&mut self, storage_buffer16_bit_access: bool) -> &mut Self {
        self.storage_buffer16_bit_access = storage_buffer16_bit_access;
        self
    }
    ///Sets the `uniform_and_storage_buffer16_bit_access` field.
    pub fn set_uniform_and_storage_buffer16_bit_access(
        &mut self,
        uniform_and_storage_buffer16_bit_access: bool,
    ) -> &mut Self {
        self.uniform_and_storage_buffer16_bit_access = uniform_and_storage_buffer16_bit_access;
        self
    }
    ///Sets the `storage_push_constant16` field.
    pub fn set_storage_push_constant16(&mut self, storage_push_constant16: bool) -> &mut Self {
        self.storage_push_constant16 = storage_push_constant16;
        self
    }
    ///Sets the `storage_input_output16` field.
    pub fn set_storage_input_output16(&mut self, storage_input_output16: bool) -> &mut Self {
        self.storage_input_output16 = storage_input_output16;
        self
    }
    ///Sets the `multiview` field.
    pub fn set_multiview(&mut self, multiview: bool) -> &mut Self {
        self.multiview = multiview;
        self
    }
    ///Sets the `multiview_geometry_shader` field.
    pub fn set_multiview_geometry_shader(&mut self, multiview_geometry_shader: bool) -> &mut Self {
        self.multiview_geometry_shader = multiview_geometry_shader;
        self
    }
    ///Sets the `multiview_tessellation_shader` field.
    pub fn set_multiview_tessellation_shader(&mut self, multiview_tessellation_shader: bool) -> &mut Self {
        self.multiview_tessellation_shader = multiview_tessellation_shader;
        self
    }
    ///Sets the `variable_pointers_storage_buffer` field.
    pub fn set_variable_pointers_storage_buffer(&mut self, variable_pointers_storage_buffer: bool) -> &mut Self {
        self.variable_pointers_storage_buffer = variable_pointers_storage_buffer;
        self
    }
    ///Sets the `variable_pointers` field.
    pub fn set_variable_pointers(&mut self, variable_pointers: bool) -> &mut Self {
        self.variable_pointers = variable_pointers;
        self
    }
    ///Sets the `protected_memory` field.
    pub fn set_protected_memory(&mut self, protected_memory: bool) -> &mut Self {
        self.protected_memory = protected_memory;
        self
    }
    ///Sets the `sampler_ycbcr_conversion` field.
    pub fn set_sampler_ycbcr_conversion(&mut self, sampler_ycbcr_conversion: bool) -> &mut Self {
        self.sampler_ycbcr_conversion = sampler_ycbcr_conversion;
        self
    }
    ///Sets the `shader_draw_parameters` field.
    pub fn set_shader_draw_parameters(&mut self, shader_draw_parameters: bool) -> &mut Self {
        self.shader_draw_parameters = shader_draw_parameters;
        self
    }
    ///Sets the `storage_buffer16_bit_access` field in a builder way.
    pub fn with_storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: bool) -> Self {
        self.storage_buffer16_bit_access = storage_buffer16_bit_access;
        self
    }
    ///Sets the `uniform_and_storage_buffer16_bit_access` field in a builder way.
    pub fn with_uniform_and_storage_buffer16_bit_access(
        mut self,
        uniform_and_storage_buffer16_bit_access: bool,
    ) -> Self {
        self.uniform_and_storage_buffer16_bit_access = uniform_and_storage_buffer16_bit_access;
        self
    }
    ///Sets the `storage_push_constant16` field in a builder way.
    pub fn with_storage_push_constant16(mut self, storage_push_constant16: bool) -> Self {
        self.storage_push_constant16 = storage_push_constant16;
        self
    }
    ///Sets the `storage_input_output16` field in a builder way.
    pub fn with_storage_input_output16(mut self, storage_input_output16: bool) -> Self {
        self.storage_input_output16 = storage_input_output16;
        self
    }
    ///Sets the `multiview` field in a builder way.
    pub fn with_multiview(mut self, multiview: bool) -> Self {
        self.multiview = multiview;
        self
    }
    ///Sets the `multiview_geometry_shader` field in a builder way.
    pub fn with_multiview_geometry_shader(mut self, multiview_geometry_shader: bool) -> Self {
        self.multiview_geometry_shader = multiview_geometry_shader;
        self
    }
    ///Sets the `multiview_tessellation_shader` field in a builder way.
    pub fn with_multiview_tessellation_shader(mut self, multiview_tessellation_shader: bool) -> Self {
        self.multiview_tessellation_shader = multiview_tessellation_shader;
        self
    }
    ///Sets the `variable_pointers_storage_buffer` field in a builder way.
    pub fn with_variable_pointers_storage_buffer(mut self, variable_pointers_storage_buffer: bool) -> Self {
        self.variable_pointers_storage_buffer = variable_pointers_storage_buffer;
        self
    }
    ///Sets the `variable_pointers` field in a builder way.
    pub fn with_variable_pointers(mut self, variable_pointers: bool) -> Self {
        self.variable_pointers = variable_pointers;
        self
    }
    ///Sets the `protected_memory` field in a builder way.
    pub fn with_protected_memory(mut self, protected_memory: bool) -> Self {
        self.protected_memory = protected_memory;
        self
    }
    ///Sets the `sampler_ycbcr_conversion` field in a builder way.
    pub fn with_sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: bool) -> Self {
        self.sampler_ycbcr_conversion = sampler_ycbcr_conversion;
        self
    }
    ///Sets the `shader_draw_parameters` field in a builder way.
    pub fn with_shader_draw_parameters(mut self, shader_draw_parameters: bool) -> Self {
        self.shader_draw_parameters = shader_draw_parameters;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceVulkan11Features {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceVulkan11Features;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceVulkan11Features {
            s_type: StructureType::PhysicalDeviceVulkan11Features,
            p_next: std::ptr::null_mut(),
            storage_buffer16_bit_access: self.storage_buffer16_bit_access.into_low_level(context, bump),
            uniform_and_storage_buffer16_bit_access: self
                .uniform_and_storage_buffer16_bit_access
                .into_low_level(context, bump),
            storage_push_constant16: self.storage_push_constant16.into_low_level(context, bump),
            storage_input_output16: self.storage_input_output16.into_low_level(context, bump),
            multiview: self.multiview.into_low_level(context, bump),
            multiview_geometry_shader: self.multiview_geometry_shader.into_low_level(context, bump),
            multiview_tessellation_shader: self.multiview_tessellation_shader.into_low_level(context, bump),
            variable_pointers_storage_buffer: self.variable_pointers_storage_buffer.into_low_level(context, bump),
            variable_pointers: self.variable_pointers.into_low_level(context, bump),
            protected_memory: self.protected_memory.into_low_level(context, bump),
            sampler_ycbcr_conversion: self.sampler_ycbcr_conversion.into_low_level(context, bump),
            shader_draw_parameters: self.shader_draw_parameters.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceVulkan11Features {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            storage_buffer16_bit_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.storage_buffer16_bit_access,
            ),
            uniform_and_storage_buffer16_bit_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.uniform_and_storage_buffer16_bit_access,
            ),
            storage_push_constant16: crate::conv::FromLowLevel::from_low_level(context, value.storage_push_constant16),
            storage_input_output16: crate::conv::FromLowLevel::from_low_level(context, value.storage_input_output16),
            multiview: crate::conv::FromLowLevel::from_low_level(context, value.multiview),
            multiview_geometry_shader: crate::conv::FromLowLevel::from_low_level(
                context,
                value.multiview_geometry_shader,
            ),
            multiview_tessellation_shader: crate::conv::FromLowLevel::from_low_level(
                context,
                value.multiview_tessellation_shader,
            ),
            variable_pointers_storage_buffer: crate::conv::FromLowLevel::from_low_level(
                context,
                value.variable_pointers_storage_buffer,
            ),
            variable_pointers: crate::conv::FromLowLevel::from_low_level(context, value.variable_pointers),
            protected_memory: crate::conv::FromLowLevel::from_low_level(context, value.protected_memory),
            sampler_ycbcr_conversion: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sampler_ycbcr_conversion,
            ),
            shader_draw_parameters: crate::conv::FromLowLevel::from_low_level(context, value.shader_draw_parameters),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkan11Properties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceVulkan11Properties {
    #[doc(alias = "deviceUUID")]
    pub device_uuid: uuid::Uuid,
    #[doc(alias = "driverUUID")]
    pub driver_uuid: uuid::Uuid,
    #[doc(alias = "deviceLUID")]
    pub device_luid: [u8; LUID_SIZE as usize],
    #[doc(alias = "deviceNodeMask")]
    pub device_node_mask: u32,
    #[doc(alias = "deviceLUIDValid")]
    pub device_luid_valid: bool,
    #[doc(alias = "subgroupSize")]
    pub subgroup_size: u32,
    #[doc(alias = "subgroupSupportedStages")]
    pub subgroup_supported_stages: ShaderStageFlags,
    #[doc(alias = "subgroupSupportedOperations")]
    pub subgroup_supported_operations: SubgroupFeatureFlags,
    #[doc(alias = "subgroupQuadOperationsInAllStages")]
    pub subgroup_quad_operations_in_all_stages: bool,
    #[doc(alias = "pointClippingBehavior")]
    pub point_clipping_behavior: PointClippingBehavior,
    #[doc(alias = "maxMultiviewViewCount")]
    pub max_multiview_view_count: u32,
    #[doc(alias = "maxMultiviewInstanceIndex")]
    pub max_multiview_instance_index: u32,
    #[doc(alias = "protectedNoFault")]
    pub protected_no_fault: bool,
    #[doc(alias = "maxPerSetDescriptors")]
    pub max_per_set_descriptors: u32,
    #[doc(alias = "maxMemoryAllocationSize")]
    pub max_memory_allocation_size: DeviceSize,
}
impl PhysicalDeviceVulkan11Properties {
    ///Get a reference to the `device_uuid` field.
    pub fn device_uuid(&self) -> uuid::Uuid {
        self.device_uuid
    }
    ///Get a reference to the `driver_uuid` field.
    pub fn driver_uuid(&self) -> uuid::Uuid {
        self.driver_uuid
    }
    ///Get a reference to the `device_luid` field.
    pub fn device_luid(&self) -> [u8; LUID_SIZE as usize] {
        self.device_luid
    }
    ///Get a reference to the `device_node_mask` field.
    pub fn device_node_mask(&self) -> u32 {
        self.device_node_mask
    }
    ///Get a reference to the `device_luid_valid` field.
    pub fn device_luid_valid(&self) -> &bool {
        &self.device_luid_valid
    }
    ///Get a reference to the `subgroup_size` field.
    pub fn subgroup_size(&self) -> u32 {
        self.subgroup_size
    }
    ///Get a reference to the `subgroup_supported_stages` field.
    pub fn subgroup_supported_stages(&self) -> ShaderStageFlags {
        self.subgroup_supported_stages
    }
    ///Get a reference to the `subgroup_supported_operations` field.
    pub fn subgroup_supported_operations(&self) -> SubgroupFeatureFlags {
        self.subgroup_supported_operations
    }
    ///Get a reference to the `subgroup_quad_operations_in_all_stages` field.
    pub fn subgroup_quad_operations_in_all_stages(&self) -> &bool {
        &self.subgroup_quad_operations_in_all_stages
    }
    ///Get a reference to the `point_clipping_behavior` field.
    pub fn point_clipping_behavior(&self) -> PointClippingBehavior {
        self.point_clipping_behavior
    }
    ///Get a reference to the `max_multiview_view_count` field.
    pub fn max_multiview_view_count(&self) -> u32 {
        self.max_multiview_view_count
    }
    ///Get a reference to the `max_multiview_instance_index` field.
    pub fn max_multiview_instance_index(&self) -> u32 {
        self.max_multiview_instance_index
    }
    ///Get a reference to the `protected_no_fault` field.
    pub fn protected_no_fault(&self) -> &bool {
        &self.protected_no_fault
    }
    ///Get a reference to the `max_per_set_descriptors` field.
    pub fn max_per_set_descriptors(&self) -> u32 {
        self.max_per_set_descriptors
    }
    ///Get a reference to the `max_memory_allocation_size` field.
    pub fn max_memory_allocation_size(&self) -> &DeviceSize {
        &self.max_memory_allocation_size
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceVulkan11Properties {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceVulkan11Properties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceVulkan11Properties {
            s_type: StructureType::PhysicalDeviceVulkan11Properties,
            p_next: std::ptr::null_mut(),
            device_uuid: self.device_uuid.into_low_level(context, bump),
            driver_uuid: self.driver_uuid.into_low_level(context, bump),
            device_luid: self.device_luid.into_low_level(context, bump),
            device_node_mask: self.device_node_mask.into_low_level(context, bump),
            device_luid_valid: self.device_luid_valid.into_low_level(context, bump),
            subgroup_size: self.subgroup_size.into_low_level(context, bump),
            subgroup_supported_stages: self.subgroup_supported_stages.into_low_level(context, bump),
            subgroup_supported_operations: self.subgroup_supported_operations.into_low_level(context, bump),
            subgroup_quad_operations_in_all_stages: self
                .subgroup_quad_operations_in_all_stages
                .into_low_level(context, bump),
            point_clipping_behavior: self.point_clipping_behavior.into_low_level(context, bump),
            max_multiview_view_count: self.max_multiview_view_count.into_low_level(context, bump),
            max_multiview_instance_index: self.max_multiview_instance_index.into_low_level(context, bump),
            protected_no_fault: self.protected_no_fault.into_low_level(context, bump),
            max_per_set_descriptors: self.max_per_set_descriptors.into_low_level(context, bump),
            max_memory_allocation_size: self.max_memory_allocation_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceVulkan11Properties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            device_uuid: crate::conv::FromLowLevel::from_low_level(context, value.device_uuid),
            driver_uuid: crate::conv::FromLowLevel::from_low_level(context, value.driver_uuid),
            device_luid: crate::conv::FromLowLevel::from_low_level(context, value.device_luid),
            device_node_mask: crate::conv::FromLowLevel::from_low_level(context, value.device_node_mask),
            device_luid_valid: crate::conv::FromLowLevel::from_low_level(context, value.device_luid_valid),
            subgroup_size: crate::conv::FromLowLevel::from_low_level(context, value.subgroup_size),
            subgroup_supported_stages: crate::conv::FromLowLevel::from_low_level(
                context,
                value.subgroup_supported_stages,
            ),
            subgroup_supported_operations: crate::conv::FromLowLevel::from_low_level(
                context,
                value.subgroup_supported_operations,
            ),
            subgroup_quad_operations_in_all_stages: crate::conv::FromLowLevel::from_low_level(
                context,
                value.subgroup_quad_operations_in_all_stages,
            ),
            point_clipping_behavior: crate::conv::FromLowLevel::from_low_level(context, value.point_clipping_behavior),
            max_multiview_view_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_multiview_view_count,
            ),
            max_multiview_instance_index: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_multiview_instance_index,
            ),
            protected_no_fault: crate::conv::FromLowLevel::from_low_level(context, value.protected_no_fault),
            max_per_set_descriptors: crate::conv::FromLowLevel::from_low_level(context, value.max_per_set_descriptors),
            max_memory_allocation_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_memory_allocation_size,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkan12Features")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceVulkan12Features {
    #[doc(alias = "samplerMirrorClampToEdge")]
    pub sampler_mirror_clamp_to_edge: bool,
    #[doc(alias = "drawIndirectCount")]
    pub draw_indirect_count: bool,
    #[doc(alias = "storageBuffer8BitAccess")]
    pub storage_buffer8_bit_access: bool,
    #[doc(alias = "uniformAndStorageBuffer8BitAccess")]
    pub uniform_and_storage_buffer8_bit_access: bool,
    #[doc(alias = "storagePushConstant8")]
    pub storage_push_constant8: bool,
    #[doc(alias = "shaderBufferInt64Atomics")]
    pub shader_buffer_int64_atomics: bool,
    #[doc(alias = "shaderSharedInt64Atomics")]
    pub shader_shared_int64_atomics: bool,
    #[doc(alias = "shaderFloat16")]
    pub shader_float16: bool,
    #[doc(alias = "shaderInt8")]
    pub shader_int8: bool,
    #[doc(alias = "descriptorIndexing")]
    pub descriptor_indexing: bool,
    #[doc(alias = "shaderInputAttachmentArrayDynamicIndexing")]
    pub shader_input_attachment_array_dynamic_indexing: bool,
    #[doc(alias = "shaderUniformTexelBufferArrayDynamicIndexing")]
    pub shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    #[doc(alias = "shaderStorageTexelBufferArrayDynamicIndexing")]
    pub shader_storage_texel_buffer_array_dynamic_indexing: bool,
    #[doc(alias = "shaderUniformBufferArrayNonUniformIndexing")]
    pub shader_uniform_buffer_array_non_uniform_indexing: bool,
    #[doc(alias = "shaderSampledImageArrayNonUniformIndexing")]
    pub shader_sampled_image_array_non_uniform_indexing: bool,
    #[doc(alias = "shaderStorageBufferArrayNonUniformIndexing")]
    pub shader_storage_buffer_array_non_uniform_indexing: bool,
    #[doc(alias = "shaderStorageImageArrayNonUniformIndexing")]
    pub shader_storage_image_array_non_uniform_indexing: bool,
    #[doc(alias = "shaderInputAttachmentArrayNonUniformIndexing")]
    pub shader_input_attachment_array_non_uniform_indexing: bool,
    #[doc(alias = "shaderUniformTexelBufferArrayNonUniformIndexing")]
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    #[doc(alias = "shaderStorageTexelBufferArrayNonUniformIndexing")]
    pub shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    #[doc(alias = "descriptorBindingUniformBufferUpdateAfterBind")]
    pub descriptor_binding_uniform_buffer_update_after_bind: bool,
    #[doc(alias = "descriptorBindingSampledImageUpdateAfterBind")]
    pub descriptor_binding_sampled_image_update_after_bind: bool,
    #[doc(alias = "descriptorBindingStorageImageUpdateAfterBind")]
    pub descriptor_binding_storage_image_update_after_bind: bool,
    #[doc(alias = "descriptorBindingStorageBufferUpdateAfterBind")]
    pub descriptor_binding_storage_buffer_update_after_bind: bool,
    #[doc(alias = "descriptorBindingUniformTexelBufferUpdateAfterBind")]
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    #[doc(alias = "descriptorBindingStorageTexelBufferUpdateAfterBind")]
    pub descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    #[doc(alias = "descriptorBindingUpdateUnusedWhilePending")]
    pub descriptor_binding_update_unused_while_pending: bool,
    #[doc(alias = "descriptorBindingPartiallyBound")]
    pub descriptor_binding_partially_bound: bool,
    #[doc(alias = "descriptorBindingVariableDescriptorCount")]
    pub descriptor_binding_variable_descriptor_count: bool,
    #[doc(alias = "runtimeDescriptorArray")]
    pub runtime_descriptor_array: bool,
    #[doc(alias = "samplerFilterMinmax")]
    pub sampler_filter_minmax: bool,
    #[doc(alias = "scalarBlockLayout")]
    pub scalar_block_layout: bool,
    #[doc(alias = "imagelessFramebuffer")]
    pub imageless_framebuffer: bool,
    #[doc(alias = "uniformBufferStandardLayout")]
    pub uniform_buffer_standard_layout: bool,
    #[doc(alias = "shaderSubgroupExtendedTypes")]
    pub shader_subgroup_extended_types: bool,
    #[doc(alias = "separateDepthStencilLayouts")]
    pub separate_depth_stencil_layouts: bool,
    #[doc(alias = "hostQueryReset")]
    pub host_query_reset: bool,
    #[doc(alias = "timelineSemaphore")]
    pub timeline_semaphore: bool,
    #[doc(alias = "bufferDeviceAddress")]
    pub buffer_device_address: bool,
    #[doc(alias = "bufferDeviceAddressCaptureReplay")]
    pub buffer_device_address_capture_replay: bool,
    #[doc(alias = "bufferDeviceAddressMultiDevice")]
    pub buffer_device_address_multi_device: bool,
    #[doc(alias = "vulkanMemoryModel")]
    pub vulkan_memory_model: bool,
    #[doc(alias = "vulkanMemoryModelDeviceScope")]
    pub vulkan_memory_model_device_scope: bool,
    #[doc(alias = "vulkanMemoryModelAvailabilityVisibilityChains")]
    pub vulkan_memory_model_availability_visibility_chains: bool,
    #[doc(alias = "shaderOutputViewportIndex")]
    pub shader_output_viewport_index: bool,
    #[doc(alias = "shaderOutputLayer")]
    pub shader_output_layer: bool,
    #[doc(alias = "subgroupBroadcastDynamicId")]
    pub subgroup_broadcast_dynamic_id: bool,
}
impl PhysicalDeviceVulkan12Features {
    ///Get a reference to the `sampler_mirror_clamp_to_edge` field.
    pub fn sampler_mirror_clamp_to_edge(&self) -> &bool {
        &self.sampler_mirror_clamp_to_edge
    }
    ///Get a reference to the `draw_indirect_count` field.
    pub fn draw_indirect_count(&self) -> &bool {
        &self.draw_indirect_count
    }
    ///Get a reference to the `storage_buffer8_bit_access` field.
    pub fn storage_buffer8_bit_access(&self) -> &bool {
        &self.storage_buffer8_bit_access
    }
    ///Get a reference to the `uniform_and_storage_buffer8_bit_access` field.
    pub fn uniform_and_storage_buffer8_bit_access(&self) -> &bool {
        &self.uniform_and_storage_buffer8_bit_access
    }
    ///Get a reference to the `storage_push_constant8` field.
    pub fn storage_push_constant8(&self) -> &bool {
        &self.storage_push_constant8
    }
    ///Get a reference to the `shader_buffer_int64_atomics` field.
    pub fn shader_buffer_int64_atomics(&self) -> &bool {
        &self.shader_buffer_int64_atomics
    }
    ///Get a reference to the `shader_shared_int64_atomics` field.
    pub fn shader_shared_int64_atomics(&self) -> &bool {
        &self.shader_shared_int64_atomics
    }
    ///Get a reference to the `shader_float16` field.
    pub fn shader_float16(&self) -> &bool {
        &self.shader_float16
    }
    ///Get a reference to the `shader_int8` field.
    pub fn shader_int8(&self) -> &bool {
        &self.shader_int8
    }
    ///Get a reference to the `descriptor_indexing` field.
    pub fn descriptor_indexing(&self) -> &bool {
        &self.descriptor_indexing
    }
    ///Get a reference to the `shader_input_attachment_array_dynamic_indexing` field.
    pub fn shader_input_attachment_array_dynamic_indexing(&self) -> &bool {
        &self.shader_input_attachment_array_dynamic_indexing
    }
    ///Get a reference to the `shader_uniform_texel_buffer_array_dynamic_indexing` field.
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(&self) -> &bool {
        &self.shader_uniform_texel_buffer_array_dynamic_indexing
    }
    ///Get a reference to the `shader_storage_texel_buffer_array_dynamic_indexing` field.
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(&self) -> &bool {
        &self.shader_storage_texel_buffer_array_dynamic_indexing
    }
    ///Get a reference to the `shader_uniform_buffer_array_non_uniform_indexing` field.
    pub fn shader_uniform_buffer_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_uniform_buffer_array_non_uniform_indexing
    }
    ///Get a reference to the `shader_sampled_image_array_non_uniform_indexing` field.
    pub fn shader_sampled_image_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_sampled_image_array_non_uniform_indexing
    }
    ///Get a reference to the `shader_storage_buffer_array_non_uniform_indexing` field.
    pub fn shader_storage_buffer_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_storage_buffer_array_non_uniform_indexing
    }
    ///Get a reference to the `shader_storage_image_array_non_uniform_indexing` field.
    pub fn shader_storage_image_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_storage_image_array_non_uniform_indexing
    }
    ///Get a reference to the `shader_input_attachment_array_non_uniform_indexing` field.
    pub fn shader_input_attachment_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_input_attachment_array_non_uniform_indexing
    }
    ///Get a reference to the `shader_uniform_texel_buffer_array_non_uniform_indexing` field.
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_uniform_texel_buffer_array_non_uniform_indexing
    }
    ///Get a reference to the `shader_storage_texel_buffer_array_non_uniform_indexing` field.
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(&self) -> &bool {
        &self.shader_storage_texel_buffer_array_non_uniform_indexing
    }
    ///Get a reference to the `descriptor_binding_uniform_buffer_update_after_bind` field.
    pub fn descriptor_binding_uniform_buffer_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_uniform_buffer_update_after_bind
    }
    ///Get a reference to the `descriptor_binding_sampled_image_update_after_bind` field.
    pub fn descriptor_binding_sampled_image_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_sampled_image_update_after_bind
    }
    ///Get a reference to the `descriptor_binding_storage_image_update_after_bind` field.
    pub fn descriptor_binding_storage_image_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_storage_image_update_after_bind
    }
    ///Get a reference to the `descriptor_binding_storage_buffer_update_after_bind` field.
    pub fn descriptor_binding_storage_buffer_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_storage_buffer_update_after_bind
    }
    ///Get a reference to the `descriptor_binding_uniform_texel_buffer_update_after_bind` field.
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_uniform_texel_buffer_update_after_bind
    }
    ///Get a reference to the `descriptor_binding_storage_texel_buffer_update_after_bind` field.
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_storage_texel_buffer_update_after_bind
    }
    ///Get a reference to the `descriptor_binding_update_unused_while_pending` field.
    pub fn descriptor_binding_update_unused_while_pending(&self) -> &bool {
        &self.descriptor_binding_update_unused_while_pending
    }
    ///Get a reference to the `descriptor_binding_partially_bound` field.
    pub fn descriptor_binding_partially_bound(&self) -> &bool {
        &self.descriptor_binding_partially_bound
    }
    ///Get a reference to the `descriptor_binding_variable_descriptor_count` field.
    pub fn descriptor_binding_variable_descriptor_count(&self) -> &bool {
        &self.descriptor_binding_variable_descriptor_count
    }
    ///Get a reference to the `runtime_descriptor_array` field.
    pub fn runtime_descriptor_array(&self) -> &bool {
        &self.runtime_descriptor_array
    }
    ///Get a reference to the `sampler_filter_minmax` field.
    pub fn sampler_filter_minmax(&self) -> &bool {
        &self.sampler_filter_minmax
    }
    ///Get a reference to the `scalar_block_layout` field.
    pub fn scalar_block_layout(&self) -> &bool {
        &self.scalar_block_layout
    }
    ///Get a reference to the `imageless_framebuffer` field.
    pub fn imageless_framebuffer(&self) -> &bool {
        &self.imageless_framebuffer
    }
    ///Get a reference to the `uniform_buffer_standard_layout` field.
    pub fn uniform_buffer_standard_layout(&self) -> &bool {
        &self.uniform_buffer_standard_layout
    }
    ///Get a reference to the `shader_subgroup_extended_types` field.
    pub fn shader_subgroup_extended_types(&self) -> &bool {
        &self.shader_subgroup_extended_types
    }
    ///Get a reference to the `separate_depth_stencil_layouts` field.
    pub fn separate_depth_stencil_layouts(&self) -> &bool {
        &self.separate_depth_stencil_layouts
    }
    ///Get a reference to the `host_query_reset` field.
    pub fn host_query_reset(&self) -> &bool {
        &self.host_query_reset
    }
    ///Get a reference to the `timeline_semaphore` field.
    pub fn timeline_semaphore(&self) -> &bool {
        &self.timeline_semaphore
    }
    ///Get a reference to the `buffer_device_address` field.
    pub fn buffer_device_address(&self) -> &bool {
        &self.buffer_device_address
    }
    ///Get a reference to the `buffer_device_address_capture_replay` field.
    pub fn buffer_device_address_capture_replay(&self) -> &bool {
        &self.buffer_device_address_capture_replay
    }
    ///Get a reference to the `buffer_device_address_multi_device` field.
    pub fn buffer_device_address_multi_device(&self) -> &bool {
        &self.buffer_device_address_multi_device
    }
    ///Get a reference to the `vulkan_memory_model` field.
    pub fn vulkan_memory_model(&self) -> &bool {
        &self.vulkan_memory_model
    }
    ///Get a reference to the `vulkan_memory_model_device_scope` field.
    pub fn vulkan_memory_model_device_scope(&self) -> &bool {
        &self.vulkan_memory_model_device_scope
    }
    ///Get a reference to the `vulkan_memory_model_availability_visibility_chains` field.
    pub fn vulkan_memory_model_availability_visibility_chains(&self) -> &bool {
        &self.vulkan_memory_model_availability_visibility_chains
    }
    ///Get a reference to the `shader_output_viewport_index` field.
    pub fn shader_output_viewport_index(&self) -> &bool {
        &self.shader_output_viewport_index
    }
    ///Get a reference to the `shader_output_layer` field.
    pub fn shader_output_layer(&self) -> &bool {
        &self.shader_output_layer
    }
    ///Get a reference to the `subgroup_broadcast_dynamic_id` field.
    pub fn subgroup_broadcast_dynamic_id(&self) -> &bool {
        &self.subgroup_broadcast_dynamic_id
    }
    ///Get a mutable reference to the `sampler_mirror_clamp_to_edge` field.
    pub fn sampler_mirror_clamp_to_edge_mut(&mut self) -> &mut bool {
        &mut self.sampler_mirror_clamp_to_edge
    }
    ///Get a mutable reference to the `draw_indirect_count` field.
    pub fn draw_indirect_count_mut(&mut self) -> &mut bool {
        &mut self.draw_indirect_count
    }
    ///Get a mutable reference to the `storage_buffer8_bit_access` field.
    pub fn storage_buffer8_bit_access_mut(&mut self) -> &mut bool {
        &mut self.storage_buffer8_bit_access
    }
    ///Get a mutable reference to the `uniform_and_storage_buffer8_bit_access` field.
    pub fn uniform_and_storage_buffer8_bit_access_mut(&mut self) -> &mut bool {
        &mut self.uniform_and_storage_buffer8_bit_access
    }
    ///Get a mutable reference to the `storage_push_constant8` field.
    pub fn storage_push_constant8_mut(&mut self) -> &mut bool {
        &mut self.storage_push_constant8
    }
    ///Get a mutable reference to the `shader_buffer_int64_atomics` field.
    pub fn shader_buffer_int64_atomics_mut(&mut self) -> &mut bool {
        &mut self.shader_buffer_int64_atomics
    }
    ///Get a mutable reference to the `shader_shared_int64_atomics` field.
    pub fn shader_shared_int64_atomics_mut(&mut self) -> &mut bool {
        &mut self.shader_shared_int64_atomics
    }
    ///Get a mutable reference to the `shader_float16` field.
    pub fn shader_float16_mut(&mut self) -> &mut bool {
        &mut self.shader_float16
    }
    ///Get a mutable reference to the `shader_int8` field.
    pub fn shader_int8_mut(&mut self) -> &mut bool {
        &mut self.shader_int8
    }
    ///Get a mutable reference to the `descriptor_indexing` field.
    pub fn descriptor_indexing_mut(&mut self) -> &mut bool {
        &mut self.descriptor_indexing
    }
    ///Get a mutable reference to the `shader_input_attachment_array_dynamic_indexing` field.
    pub fn shader_input_attachment_array_dynamic_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_input_attachment_array_dynamic_indexing
    }
    ///Get a mutable reference to the `shader_uniform_texel_buffer_array_dynamic_indexing` field.
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_uniform_texel_buffer_array_dynamic_indexing
    }
    ///Get a mutable reference to the `shader_storage_texel_buffer_array_dynamic_indexing` field.
    pub fn shader_storage_texel_buffer_array_dynamic_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_storage_texel_buffer_array_dynamic_indexing
    }
    ///Get a mutable reference to the `shader_uniform_buffer_array_non_uniform_indexing` field.
    pub fn shader_uniform_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_uniform_buffer_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `shader_sampled_image_array_non_uniform_indexing` field.
    pub fn shader_sampled_image_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_sampled_image_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `shader_storage_buffer_array_non_uniform_indexing` field.
    pub fn shader_storage_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_storage_buffer_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `shader_storage_image_array_non_uniform_indexing` field.
    pub fn shader_storage_image_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_storage_image_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `shader_input_attachment_array_non_uniform_indexing` field.
    pub fn shader_input_attachment_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_input_attachment_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `shader_uniform_texel_buffer_array_non_uniform_indexing`
    /// field.
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_uniform_texel_buffer_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `shader_storage_texel_buffer_array_non_uniform_indexing`
    /// field.
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        &mut self.shader_storage_texel_buffer_array_non_uniform_indexing
    }
    ///Get a mutable reference to the `descriptor_binding_uniform_buffer_update_after_bind` field.
    pub fn descriptor_binding_uniform_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_uniform_buffer_update_after_bind
    }
    ///Get a mutable reference to the `descriptor_binding_sampled_image_update_after_bind` field.
    pub fn descriptor_binding_sampled_image_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_sampled_image_update_after_bind
    }
    ///Get a mutable reference to the `descriptor_binding_storage_image_update_after_bind` field.
    pub fn descriptor_binding_storage_image_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_storage_image_update_after_bind
    }
    ///Get a mutable reference to the `descriptor_binding_storage_buffer_update_after_bind` field.
    pub fn descriptor_binding_storage_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_storage_buffer_update_after_bind
    }
    ///Get a mutable reference to the `descriptor_binding_uniform_texel_buffer_update_after_bind`
    /// field.
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_uniform_texel_buffer_update_after_bind
    }
    ///Get a mutable reference to the `descriptor_binding_storage_texel_buffer_update_after_bind`
    /// field.
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_storage_texel_buffer_update_after_bind
    }
    ///Get a mutable reference to the `descriptor_binding_update_unused_while_pending` field.
    pub fn descriptor_binding_update_unused_while_pending_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_update_unused_while_pending
    }
    ///Get a mutable reference to the `descriptor_binding_partially_bound` field.
    pub fn descriptor_binding_partially_bound_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_partially_bound
    }
    ///Get a mutable reference to the `descriptor_binding_variable_descriptor_count` field.
    pub fn descriptor_binding_variable_descriptor_count_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_variable_descriptor_count
    }
    ///Get a mutable reference to the `runtime_descriptor_array` field.
    pub fn runtime_descriptor_array_mut(&mut self) -> &mut bool {
        &mut self.runtime_descriptor_array
    }
    ///Get a mutable reference to the `sampler_filter_minmax` field.
    pub fn sampler_filter_minmax_mut(&mut self) -> &mut bool {
        &mut self.sampler_filter_minmax
    }
    ///Get a mutable reference to the `scalar_block_layout` field.
    pub fn scalar_block_layout_mut(&mut self) -> &mut bool {
        &mut self.scalar_block_layout
    }
    ///Get a mutable reference to the `imageless_framebuffer` field.
    pub fn imageless_framebuffer_mut(&mut self) -> &mut bool {
        &mut self.imageless_framebuffer
    }
    ///Get a mutable reference to the `uniform_buffer_standard_layout` field.
    pub fn uniform_buffer_standard_layout_mut(&mut self) -> &mut bool {
        &mut self.uniform_buffer_standard_layout
    }
    ///Get a mutable reference to the `shader_subgroup_extended_types` field.
    pub fn shader_subgroup_extended_types_mut(&mut self) -> &mut bool {
        &mut self.shader_subgroup_extended_types
    }
    ///Get a mutable reference to the `separate_depth_stencil_layouts` field.
    pub fn separate_depth_stencil_layouts_mut(&mut self) -> &mut bool {
        &mut self.separate_depth_stencil_layouts
    }
    ///Get a mutable reference to the `host_query_reset` field.
    pub fn host_query_reset_mut(&mut self) -> &mut bool {
        &mut self.host_query_reset
    }
    ///Get a mutable reference to the `timeline_semaphore` field.
    pub fn timeline_semaphore_mut(&mut self) -> &mut bool {
        &mut self.timeline_semaphore
    }
    ///Get a mutable reference to the `buffer_device_address` field.
    pub fn buffer_device_address_mut(&mut self) -> &mut bool {
        &mut self.buffer_device_address
    }
    ///Get a mutable reference to the `buffer_device_address_capture_replay` field.
    pub fn buffer_device_address_capture_replay_mut(&mut self) -> &mut bool {
        &mut self.buffer_device_address_capture_replay
    }
    ///Get a mutable reference to the `buffer_device_address_multi_device` field.
    pub fn buffer_device_address_multi_device_mut(&mut self) -> &mut bool {
        &mut self.buffer_device_address_multi_device
    }
    ///Get a mutable reference to the `vulkan_memory_model` field.
    pub fn vulkan_memory_model_mut(&mut self) -> &mut bool {
        &mut self.vulkan_memory_model
    }
    ///Get a mutable reference to the `vulkan_memory_model_device_scope` field.
    pub fn vulkan_memory_model_device_scope_mut(&mut self) -> &mut bool {
        &mut self.vulkan_memory_model_device_scope
    }
    ///Get a mutable reference to the `vulkan_memory_model_availability_visibility_chains` field.
    pub fn vulkan_memory_model_availability_visibility_chains_mut(&mut self) -> &mut bool {
        &mut self.vulkan_memory_model_availability_visibility_chains
    }
    ///Get a mutable reference to the `shader_output_viewport_index` field.
    pub fn shader_output_viewport_index_mut(&mut self) -> &mut bool {
        &mut self.shader_output_viewport_index
    }
    ///Get a mutable reference to the `shader_output_layer` field.
    pub fn shader_output_layer_mut(&mut self) -> &mut bool {
        &mut self.shader_output_layer
    }
    ///Get a mutable reference to the `subgroup_broadcast_dynamic_id` field.
    pub fn subgroup_broadcast_dynamic_id_mut(&mut self) -> &mut bool {
        &mut self.subgroup_broadcast_dynamic_id
    }
    ///Sets the `sampler_mirror_clamp_to_edge` field.
    pub fn set_sampler_mirror_clamp_to_edge(&mut self, sampler_mirror_clamp_to_edge: bool) -> &mut Self {
        self.sampler_mirror_clamp_to_edge = sampler_mirror_clamp_to_edge;
        self
    }
    ///Sets the `draw_indirect_count` field.
    pub fn set_draw_indirect_count(&mut self, draw_indirect_count: bool) -> &mut Self {
        self.draw_indirect_count = draw_indirect_count;
        self
    }
    ///Sets the `storage_buffer8_bit_access` field.
    pub fn set_storage_buffer8_bit_access(&mut self, storage_buffer8_bit_access: bool) -> &mut Self {
        self.storage_buffer8_bit_access = storage_buffer8_bit_access;
        self
    }
    ///Sets the `uniform_and_storage_buffer8_bit_access` field.
    pub fn set_uniform_and_storage_buffer8_bit_access(
        &mut self,
        uniform_and_storage_buffer8_bit_access: bool,
    ) -> &mut Self {
        self.uniform_and_storage_buffer8_bit_access = uniform_and_storage_buffer8_bit_access;
        self
    }
    ///Sets the `storage_push_constant8` field.
    pub fn set_storage_push_constant8(&mut self, storage_push_constant8: bool) -> &mut Self {
        self.storage_push_constant8 = storage_push_constant8;
        self
    }
    ///Sets the `shader_buffer_int64_atomics` field.
    pub fn set_shader_buffer_int64_atomics(&mut self, shader_buffer_int64_atomics: bool) -> &mut Self {
        self.shader_buffer_int64_atomics = shader_buffer_int64_atomics;
        self
    }
    ///Sets the `shader_shared_int64_atomics` field.
    pub fn set_shader_shared_int64_atomics(&mut self, shader_shared_int64_atomics: bool) -> &mut Self {
        self.shader_shared_int64_atomics = shader_shared_int64_atomics;
        self
    }
    ///Sets the `shader_float16` field.
    pub fn set_shader_float16(&mut self, shader_float16: bool) -> &mut Self {
        self.shader_float16 = shader_float16;
        self
    }
    ///Sets the `shader_int8` field.
    pub fn set_shader_int8(&mut self, shader_int8: bool) -> &mut Self {
        self.shader_int8 = shader_int8;
        self
    }
    ///Sets the `descriptor_indexing` field.
    pub fn set_descriptor_indexing(&mut self, descriptor_indexing: bool) -> &mut Self {
        self.descriptor_indexing = descriptor_indexing;
        self
    }
    ///Sets the `shader_input_attachment_array_dynamic_indexing` field.
    pub fn set_shader_input_attachment_array_dynamic_indexing(
        &mut self,
        shader_input_attachment_array_dynamic_indexing: bool,
    ) -> &mut Self {
        self.shader_input_attachment_array_dynamic_indexing = shader_input_attachment_array_dynamic_indexing;
        self
    }
    ///Sets the `shader_uniform_texel_buffer_array_dynamic_indexing` field.
    pub fn set_shader_uniform_texel_buffer_array_dynamic_indexing(
        &mut self,
        shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    ) -> &mut Self {
        self.shader_uniform_texel_buffer_array_dynamic_indexing = shader_uniform_texel_buffer_array_dynamic_indexing;
        self
    }
    ///Sets the `shader_storage_texel_buffer_array_dynamic_indexing` field.
    pub fn set_shader_storage_texel_buffer_array_dynamic_indexing(
        &mut self,
        shader_storage_texel_buffer_array_dynamic_indexing: bool,
    ) -> &mut Self {
        self.shader_storage_texel_buffer_array_dynamic_indexing = shader_storage_texel_buffer_array_dynamic_indexing;
        self
    }
    ///Sets the `shader_uniform_buffer_array_non_uniform_indexing` field.
    pub fn set_shader_uniform_buffer_array_non_uniform_indexing(
        &mut self,
        shader_uniform_buffer_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_uniform_buffer_array_non_uniform_indexing = shader_uniform_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_sampled_image_array_non_uniform_indexing` field.
    pub fn set_shader_sampled_image_array_non_uniform_indexing(
        &mut self,
        shader_sampled_image_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_sampled_image_array_non_uniform_indexing = shader_sampled_image_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_storage_buffer_array_non_uniform_indexing` field.
    pub fn set_shader_storage_buffer_array_non_uniform_indexing(
        &mut self,
        shader_storage_buffer_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_storage_buffer_array_non_uniform_indexing = shader_storage_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_storage_image_array_non_uniform_indexing` field.
    pub fn set_shader_storage_image_array_non_uniform_indexing(
        &mut self,
        shader_storage_image_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_storage_image_array_non_uniform_indexing = shader_storage_image_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_input_attachment_array_non_uniform_indexing` field.
    pub fn set_shader_input_attachment_array_non_uniform_indexing(
        &mut self,
        shader_input_attachment_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_input_attachment_array_non_uniform_indexing = shader_input_attachment_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_uniform_texel_buffer_array_non_uniform_indexing` field.
    pub fn set_shader_uniform_texel_buffer_array_non_uniform_indexing(
        &mut self,
        shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_uniform_texel_buffer_array_non_uniform_indexing =
            shader_uniform_texel_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_storage_texel_buffer_array_non_uniform_indexing` field.
    pub fn set_shader_storage_texel_buffer_array_non_uniform_indexing(
        &mut self,
        shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    ) -> &mut Self {
        self.shader_storage_texel_buffer_array_non_uniform_indexing =
            shader_storage_texel_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `descriptor_binding_uniform_buffer_update_after_bind` field.
    pub fn set_descriptor_binding_uniform_buffer_update_after_bind(
        &mut self,
        descriptor_binding_uniform_buffer_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_uniform_buffer_update_after_bind = descriptor_binding_uniform_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_sampled_image_update_after_bind` field.
    pub fn set_descriptor_binding_sampled_image_update_after_bind(
        &mut self,
        descriptor_binding_sampled_image_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_sampled_image_update_after_bind = descriptor_binding_sampled_image_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_storage_image_update_after_bind` field.
    pub fn set_descriptor_binding_storage_image_update_after_bind(
        &mut self,
        descriptor_binding_storage_image_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_storage_image_update_after_bind = descriptor_binding_storage_image_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_storage_buffer_update_after_bind` field.
    pub fn set_descriptor_binding_storage_buffer_update_after_bind(
        &mut self,
        descriptor_binding_storage_buffer_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_storage_buffer_update_after_bind = descriptor_binding_storage_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_uniform_texel_buffer_update_after_bind` field.
    pub fn set_descriptor_binding_uniform_texel_buffer_update_after_bind(
        &mut self,
        descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_uniform_texel_buffer_update_after_bind =
            descriptor_binding_uniform_texel_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_storage_texel_buffer_update_after_bind` field.
    pub fn set_descriptor_binding_storage_texel_buffer_update_after_bind(
        &mut self,
        descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_storage_texel_buffer_update_after_bind =
            descriptor_binding_storage_texel_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_update_unused_while_pending` field.
    pub fn set_descriptor_binding_update_unused_while_pending(
        &mut self,
        descriptor_binding_update_unused_while_pending: bool,
    ) -> &mut Self {
        self.descriptor_binding_update_unused_while_pending = descriptor_binding_update_unused_while_pending;
        self
    }
    ///Sets the `descriptor_binding_partially_bound` field.
    pub fn set_descriptor_binding_partially_bound(&mut self, descriptor_binding_partially_bound: bool) -> &mut Self {
        self.descriptor_binding_partially_bound = descriptor_binding_partially_bound;
        self
    }
    ///Sets the `descriptor_binding_variable_descriptor_count` field.
    pub fn set_descriptor_binding_variable_descriptor_count(
        &mut self,
        descriptor_binding_variable_descriptor_count: bool,
    ) -> &mut Self {
        self.descriptor_binding_variable_descriptor_count = descriptor_binding_variable_descriptor_count;
        self
    }
    ///Sets the `runtime_descriptor_array` field.
    pub fn set_runtime_descriptor_array(&mut self, runtime_descriptor_array: bool) -> &mut Self {
        self.runtime_descriptor_array = runtime_descriptor_array;
        self
    }
    ///Sets the `sampler_filter_minmax` field.
    pub fn set_sampler_filter_minmax(&mut self, sampler_filter_minmax: bool) -> &mut Self {
        self.sampler_filter_minmax = sampler_filter_minmax;
        self
    }
    ///Sets the `scalar_block_layout` field.
    pub fn set_scalar_block_layout(&mut self, scalar_block_layout: bool) -> &mut Self {
        self.scalar_block_layout = scalar_block_layout;
        self
    }
    ///Sets the `imageless_framebuffer` field.
    pub fn set_imageless_framebuffer(&mut self, imageless_framebuffer: bool) -> &mut Self {
        self.imageless_framebuffer = imageless_framebuffer;
        self
    }
    ///Sets the `uniform_buffer_standard_layout` field.
    pub fn set_uniform_buffer_standard_layout(&mut self, uniform_buffer_standard_layout: bool) -> &mut Self {
        self.uniform_buffer_standard_layout = uniform_buffer_standard_layout;
        self
    }
    ///Sets the `shader_subgroup_extended_types` field.
    pub fn set_shader_subgroup_extended_types(&mut self, shader_subgroup_extended_types: bool) -> &mut Self {
        self.shader_subgroup_extended_types = shader_subgroup_extended_types;
        self
    }
    ///Sets the `separate_depth_stencil_layouts` field.
    pub fn set_separate_depth_stencil_layouts(&mut self, separate_depth_stencil_layouts: bool) -> &mut Self {
        self.separate_depth_stencil_layouts = separate_depth_stencil_layouts;
        self
    }
    ///Sets the `host_query_reset` field.
    pub fn set_host_query_reset(&mut self, host_query_reset: bool) -> &mut Self {
        self.host_query_reset = host_query_reset;
        self
    }
    ///Sets the `timeline_semaphore` field.
    pub fn set_timeline_semaphore(&mut self, timeline_semaphore: bool) -> &mut Self {
        self.timeline_semaphore = timeline_semaphore;
        self
    }
    ///Sets the `buffer_device_address` field.
    pub fn set_buffer_device_address(&mut self, buffer_device_address: bool) -> &mut Self {
        self.buffer_device_address = buffer_device_address;
        self
    }
    ///Sets the `buffer_device_address_capture_replay` field.
    pub fn set_buffer_device_address_capture_replay(
        &mut self,
        buffer_device_address_capture_replay: bool,
    ) -> &mut Self {
        self.buffer_device_address_capture_replay = buffer_device_address_capture_replay;
        self
    }
    ///Sets the `buffer_device_address_multi_device` field.
    pub fn set_buffer_device_address_multi_device(&mut self, buffer_device_address_multi_device: bool) -> &mut Self {
        self.buffer_device_address_multi_device = buffer_device_address_multi_device;
        self
    }
    ///Sets the `vulkan_memory_model` field.
    pub fn set_vulkan_memory_model(&mut self, vulkan_memory_model: bool) -> &mut Self {
        self.vulkan_memory_model = vulkan_memory_model;
        self
    }
    ///Sets the `vulkan_memory_model_device_scope` field.
    pub fn set_vulkan_memory_model_device_scope(&mut self, vulkan_memory_model_device_scope: bool) -> &mut Self {
        self.vulkan_memory_model_device_scope = vulkan_memory_model_device_scope;
        self
    }
    ///Sets the `vulkan_memory_model_availability_visibility_chains` field.
    pub fn set_vulkan_memory_model_availability_visibility_chains(
        &mut self,
        vulkan_memory_model_availability_visibility_chains: bool,
    ) -> &mut Self {
        self.vulkan_memory_model_availability_visibility_chains = vulkan_memory_model_availability_visibility_chains;
        self
    }
    ///Sets the `shader_output_viewport_index` field.
    pub fn set_shader_output_viewport_index(&mut self, shader_output_viewport_index: bool) -> &mut Self {
        self.shader_output_viewport_index = shader_output_viewport_index;
        self
    }
    ///Sets the `shader_output_layer` field.
    pub fn set_shader_output_layer(&mut self, shader_output_layer: bool) -> &mut Self {
        self.shader_output_layer = shader_output_layer;
        self
    }
    ///Sets the `subgroup_broadcast_dynamic_id` field.
    pub fn set_subgroup_broadcast_dynamic_id(&mut self, subgroup_broadcast_dynamic_id: bool) -> &mut Self {
        self.subgroup_broadcast_dynamic_id = subgroup_broadcast_dynamic_id;
        self
    }
    ///Sets the `sampler_mirror_clamp_to_edge` field in a builder way.
    pub fn with_sampler_mirror_clamp_to_edge(mut self, sampler_mirror_clamp_to_edge: bool) -> Self {
        self.sampler_mirror_clamp_to_edge = sampler_mirror_clamp_to_edge;
        self
    }
    ///Sets the `draw_indirect_count` field in a builder way.
    pub fn with_draw_indirect_count(mut self, draw_indirect_count: bool) -> Self {
        self.draw_indirect_count = draw_indirect_count;
        self
    }
    ///Sets the `storage_buffer8_bit_access` field in a builder way.
    pub fn with_storage_buffer8_bit_access(mut self, storage_buffer8_bit_access: bool) -> Self {
        self.storage_buffer8_bit_access = storage_buffer8_bit_access;
        self
    }
    ///Sets the `uniform_and_storage_buffer8_bit_access` field in a builder way.
    pub fn with_uniform_and_storage_buffer8_bit_access(mut self, uniform_and_storage_buffer8_bit_access: bool) -> Self {
        self.uniform_and_storage_buffer8_bit_access = uniform_and_storage_buffer8_bit_access;
        self
    }
    ///Sets the `storage_push_constant8` field in a builder way.
    pub fn with_storage_push_constant8(mut self, storage_push_constant8: bool) -> Self {
        self.storage_push_constant8 = storage_push_constant8;
        self
    }
    ///Sets the `shader_buffer_int64_atomics` field in a builder way.
    pub fn with_shader_buffer_int64_atomics(mut self, shader_buffer_int64_atomics: bool) -> Self {
        self.shader_buffer_int64_atomics = shader_buffer_int64_atomics;
        self
    }
    ///Sets the `shader_shared_int64_atomics` field in a builder way.
    pub fn with_shader_shared_int64_atomics(mut self, shader_shared_int64_atomics: bool) -> Self {
        self.shader_shared_int64_atomics = shader_shared_int64_atomics;
        self
    }
    ///Sets the `shader_float16` field in a builder way.
    pub fn with_shader_float16(mut self, shader_float16: bool) -> Self {
        self.shader_float16 = shader_float16;
        self
    }
    ///Sets the `shader_int8` field in a builder way.
    pub fn with_shader_int8(mut self, shader_int8: bool) -> Self {
        self.shader_int8 = shader_int8;
        self
    }
    ///Sets the `descriptor_indexing` field in a builder way.
    pub fn with_descriptor_indexing(mut self, descriptor_indexing: bool) -> Self {
        self.descriptor_indexing = descriptor_indexing;
        self
    }
    ///Sets the `shader_input_attachment_array_dynamic_indexing` field in a builder way.
    pub fn with_shader_input_attachment_array_dynamic_indexing(
        mut self,
        shader_input_attachment_array_dynamic_indexing: bool,
    ) -> Self {
        self.shader_input_attachment_array_dynamic_indexing = shader_input_attachment_array_dynamic_indexing;
        self
    }
    ///Sets the `shader_uniform_texel_buffer_array_dynamic_indexing` field in a builder way.
    pub fn with_shader_uniform_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.shader_uniform_texel_buffer_array_dynamic_indexing = shader_uniform_texel_buffer_array_dynamic_indexing;
        self
    }
    ///Sets the `shader_storage_texel_buffer_array_dynamic_indexing` field in a builder way.
    pub fn with_shader_storage_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_storage_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.shader_storage_texel_buffer_array_dynamic_indexing = shader_storage_texel_buffer_array_dynamic_indexing;
        self
    }
    ///Sets the `shader_uniform_buffer_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_uniform_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_uniform_buffer_array_non_uniform_indexing = shader_uniform_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_sampled_image_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_sampled_image_array_non_uniform_indexing(
        mut self,
        shader_sampled_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_sampled_image_array_non_uniform_indexing = shader_sampled_image_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_storage_buffer_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_storage_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_storage_buffer_array_non_uniform_indexing = shader_storage_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_storage_image_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_storage_image_array_non_uniform_indexing(
        mut self,
        shader_storage_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_storage_image_array_non_uniform_indexing = shader_storage_image_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_input_attachment_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_input_attachment_array_non_uniform_indexing(
        mut self,
        shader_input_attachment_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_input_attachment_array_non_uniform_indexing = shader_input_attachment_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_uniform_texel_buffer_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_uniform_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_uniform_texel_buffer_array_non_uniform_indexing =
            shader_uniform_texel_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `shader_storage_texel_buffer_array_non_uniform_indexing` field in a builder way.
    pub fn with_shader_storage_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.shader_storage_texel_buffer_array_non_uniform_indexing =
            shader_storage_texel_buffer_array_non_uniform_indexing;
        self
    }
    ///Sets the `descriptor_binding_uniform_buffer_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_uniform_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_buffer_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_uniform_buffer_update_after_bind = descriptor_binding_uniform_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_sampled_image_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_sampled_image_update_after_bind(
        mut self,
        descriptor_binding_sampled_image_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_sampled_image_update_after_bind = descriptor_binding_sampled_image_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_storage_image_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_storage_image_update_after_bind(
        mut self,
        descriptor_binding_storage_image_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_storage_image_update_after_bind = descriptor_binding_storage_image_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_storage_buffer_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_storage_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_buffer_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_storage_buffer_update_after_bind = descriptor_binding_storage_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_uniform_texel_buffer_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_uniform_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_uniform_texel_buffer_update_after_bind =
            descriptor_binding_uniform_texel_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_storage_texel_buffer_update_after_bind` field in a builder way.
    pub fn with_descriptor_binding_storage_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_storage_texel_buffer_update_after_bind =
            descriptor_binding_storage_texel_buffer_update_after_bind;
        self
    }
    ///Sets the `descriptor_binding_update_unused_while_pending` field in a builder way.
    pub fn with_descriptor_binding_update_unused_while_pending(
        mut self,
        descriptor_binding_update_unused_while_pending: bool,
    ) -> Self {
        self.descriptor_binding_update_unused_while_pending = descriptor_binding_update_unused_while_pending;
        self
    }
    ///Sets the `descriptor_binding_partially_bound` field in a builder way.
    pub fn with_descriptor_binding_partially_bound(mut self, descriptor_binding_partially_bound: bool) -> Self {
        self.descriptor_binding_partially_bound = descriptor_binding_partially_bound;
        self
    }
    ///Sets the `descriptor_binding_variable_descriptor_count` field in a builder way.
    pub fn with_descriptor_binding_variable_descriptor_count(
        mut self,
        descriptor_binding_variable_descriptor_count: bool,
    ) -> Self {
        self.descriptor_binding_variable_descriptor_count = descriptor_binding_variable_descriptor_count;
        self
    }
    ///Sets the `runtime_descriptor_array` field in a builder way.
    pub fn with_runtime_descriptor_array(mut self, runtime_descriptor_array: bool) -> Self {
        self.runtime_descriptor_array = runtime_descriptor_array;
        self
    }
    ///Sets the `sampler_filter_minmax` field in a builder way.
    pub fn with_sampler_filter_minmax(mut self, sampler_filter_minmax: bool) -> Self {
        self.sampler_filter_minmax = sampler_filter_minmax;
        self
    }
    ///Sets the `scalar_block_layout` field in a builder way.
    pub fn with_scalar_block_layout(mut self, scalar_block_layout: bool) -> Self {
        self.scalar_block_layout = scalar_block_layout;
        self
    }
    ///Sets the `imageless_framebuffer` field in a builder way.
    pub fn with_imageless_framebuffer(mut self, imageless_framebuffer: bool) -> Self {
        self.imageless_framebuffer = imageless_framebuffer;
        self
    }
    ///Sets the `uniform_buffer_standard_layout` field in a builder way.
    pub fn with_uniform_buffer_standard_layout(mut self, uniform_buffer_standard_layout: bool) -> Self {
        self.uniform_buffer_standard_layout = uniform_buffer_standard_layout;
        self
    }
    ///Sets the `shader_subgroup_extended_types` field in a builder way.
    pub fn with_shader_subgroup_extended_types(mut self, shader_subgroup_extended_types: bool) -> Self {
        self.shader_subgroup_extended_types = shader_subgroup_extended_types;
        self
    }
    ///Sets the `separate_depth_stencil_layouts` field in a builder way.
    pub fn with_separate_depth_stencil_layouts(mut self, separate_depth_stencil_layouts: bool) -> Self {
        self.separate_depth_stencil_layouts = separate_depth_stencil_layouts;
        self
    }
    ///Sets the `host_query_reset` field in a builder way.
    pub fn with_host_query_reset(mut self, host_query_reset: bool) -> Self {
        self.host_query_reset = host_query_reset;
        self
    }
    ///Sets the `timeline_semaphore` field in a builder way.
    pub fn with_timeline_semaphore(mut self, timeline_semaphore: bool) -> Self {
        self.timeline_semaphore = timeline_semaphore;
        self
    }
    ///Sets the `buffer_device_address` field in a builder way.
    pub fn with_buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.buffer_device_address = buffer_device_address;
        self
    }
    ///Sets the `buffer_device_address_capture_replay` field in a builder way.
    pub fn with_buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.buffer_device_address_capture_replay = buffer_device_address_capture_replay;
        self
    }
    ///Sets the `buffer_device_address_multi_device` field in a builder way.
    pub fn with_buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.buffer_device_address_multi_device = buffer_device_address_multi_device;
        self
    }
    ///Sets the `vulkan_memory_model` field in a builder way.
    pub fn with_vulkan_memory_model(mut self, vulkan_memory_model: bool) -> Self {
        self.vulkan_memory_model = vulkan_memory_model;
        self
    }
    ///Sets the `vulkan_memory_model_device_scope` field in a builder way.
    pub fn with_vulkan_memory_model_device_scope(mut self, vulkan_memory_model_device_scope: bool) -> Self {
        self.vulkan_memory_model_device_scope = vulkan_memory_model_device_scope;
        self
    }
    ///Sets the `vulkan_memory_model_availability_visibility_chains` field in a builder way.
    pub fn with_vulkan_memory_model_availability_visibility_chains(
        mut self,
        vulkan_memory_model_availability_visibility_chains: bool,
    ) -> Self {
        self.vulkan_memory_model_availability_visibility_chains = vulkan_memory_model_availability_visibility_chains;
        self
    }
    ///Sets the `shader_output_viewport_index` field in a builder way.
    pub fn with_shader_output_viewport_index(mut self, shader_output_viewport_index: bool) -> Self {
        self.shader_output_viewport_index = shader_output_viewport_index;
        self
    }
    ///Sets the `shader_output_layer` field in a builder way.
    pub fn with_shader_output_layer(mut self, shader_output_layer: bool) -> Self {
        self.shader_output_layer = shader_output_layer;
        self
    }
    ///Sets the `subgroup_broadcast_dynamic_id` field in a builder way.
    pub fn with_subgroup_broadcast_dynamic_id(mut self, subgroup_broadcast_dynamic_id: bool) -> Self {
        self.subgroup_broadcast_dynamic_id = subgroup_broadcast_dynamic_id;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceVulkan12Features {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceVulkan12Features;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_2::PhysicalDeviceVulkan12Features {
            s_type: StructureType::PhysicalDeviceVulkan12Features,
            p_next: std::ptr::null_mut(),
            sampler_mirror_clamp_to_edge: self.sampler_mirror_clamp_to_edge.into_low_level(context, bump),
            draw_indirect_count: self.draw_indirect_count.into_low_level(context, bump),
            storage_buffer8_bit_access: self.storage_buffer8_bit_access.into_low_level(context, bump),
            uniform_and_storage_buffer8_bit_access: self
                .uniform_and_storage_buffer8_bit_access
                .into_low_level(context, bump),
            storage_push_constant8: self.storage_push_constant8.into_low_level(context, bump),
            shader_buffer_int64_atomics: self.shader_buffer_int64_atomics.into_low_level(context, bump),
            shader_shared_int64_atomics: self.shader_shared_int64_atomics.into_low_level(context, bump),
            shader_float16: self.shader_float16.into_low_level(context, bump),
            shader_int8: self.shader_int8.into_low_level(context, bump),
            descriptor_indexing: self.descriptor_indexing.into_low_level(context, bump),
            shader_input_attachment_array_dynamic_indexing: self
                .shader_input_attachment_array_dynamic_indexing
                .into_low_level(context, bump),
            shader_uniform_texel_buffer_array_dynamic_indexing: self
                .shader_uniform_texel_buffer_array_dynamic_indexing
                .into_low_level(context, bump),
            shader_storage_texel_buffer_array_dynamic_indexing: self
                .shader_storage_texel_buffer_array_dynamic_indexing
                .into_low_level(context, bump),
            shader_uniform_buffer_array_non_uniform_indexing: self
                .shader_uniform_buffer_array_non_uniform_indexing
                .into_low_level(context, bump),
            shader_sampled_image_array_non_uniform_indexing: self
                .shader_sampled_image_array_non_uniform_indexing
                .into_low_level(context, bump),
            shader_storage_buffer_array_non_uniform_indexing: self
                .shader_storage_buffer_array_non_uniform_indexing
                .into_low_level(context, bump),
            shader_storage_image_array_non_uniform_indexing: self
                .shader_storage_image_array_non_uniform_indexing
                .into_low_level(context, bump),
            shader_input_attachment_array_non_uniform_indexing: self
                .shader_input_attachment_array_non_uniform_indexing
                .into_low_level(context, bump),
            shader_uniform_texel_buffer_array_non_uniform_indexing: self
                .shader_uniform_texel_buffer_array_non_uniform_indexing
                .into_low_level(context, bump),
            shader_storage_texel_buffer_array_non_uniform_indexing: self
                .shader_storage_texel_buffer_array_non_uniform_indexing
                .into_low_level(context, bump),
            descriptor_binding_uniform_buffer_update_after_bind: self
                .descriptor_binding_uniform_buffer_update_after_bind
                .into_low_level(context, bump),
            descriptor_binding_sampled_image_update_after_bind: self
                .descriptor_binding_sampled_image_update_after_bind
                .into_low_level(context, bump),
            descriptor_binding_storage_image_update_after_bind: self
                .descriptor_binding_storage_image_update_after_bind
                .into_low_level(context, bump),
            descriptor_binding_storage_buffer_update_after_bind: self
                .descriptor_binding_storage_buffer_update_after_bind
                .into_low_level(context, bump),
            descriptor_binding_uniform_texel_buffer_update_after_bind: self
                .descriptor_binding_uniform_texel_buffer_update_after_bind
                .into_low_level(context, bump),
            descriptor_binding_storage_texel_buffer_update_after_bind: self
                .descriptor_binding_storage_texel_buffer_update_after_bind
                .into_low_level(context, bump),
            descriptor_binding_update_unused_while_pending: self
                .descriptor_binding_update_unused_while_pending
                .into_low_level(context, bump),
            descriptor_binding_partially_bound: self.descriptor_binding_partially_bound.into_low_level(context, bump),
            descriptor_binding_variable_descriptor_count: self
                .descriptor_binding_variable_descriptor_count
                .into_low_level(context, bump),
            runtime_descriptor_array: self.runtime_descriptor_array.into_low_level(context, bump),
            sampler_filter_minmax: self.sampler_filter_minmax.into_low_level(context, bump),
            scalar_block_layout: self.scalar_block_layout.into_low_level(context, bump),
            imageless_framebuffer: self.imageless_framebuffer.into_low_level(context, bump),
            uniform_buffer_standard_layout: self.uniform_buffer_standard_layout.into_low_level(context, bump),
            shader_subgroup_extended_types: self.shader_subgroup_extended_types.into_low_level(context, bump),
            separate_depth_stencil_layouts: self.separate_depth_stencil_layouts.into_low_level(context, bump),
            host_query_reset: self.host_query_reset.into_low_level(context, bump),
            timeline_semaphore: self.timeline_semaphore.into_low_level(context, bump),
            buffer_device_address: self.buffer_device_address.into_low_level(context, bump),
            buffer_device_address_capture_replay: self
                .buffer_device_address_capture_replay
                .into_low_level(context, bump),
            buffer_device_address_multi_device: self.buffer_device_address_multi_device.into_low_level(context, bump),
            vulkan_memory_model: self.vulkan_memory_model.into_low_level(context, bump),
            vulkan_memory_model_device_scope: self.vulkan_memory_model_device_scope.into_low_level(context, bump),
            vulkan_memory_model_availability_visibility_chains: self
                .vulkan_memory_model_availability_visibility_chains
                .into_low_level(context, bump),
            shader_output_viewport_index: self.shader_output_viewport_index.into_low_level(context, bump),
            shader_output_layer: self.shader_output_layer.into_low_level(context, bump),
            subgroup_broadcast_dynamic_id: self.subgroup_broadcast_dynamic_id.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceVulkan12Features {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            sampler_mirror_clamp_to_edge: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sampler_mirror_clamp_to_edge,
            ),
            draw_indirect_count: crate::conv::FromLowLevel::from_low_level(context, value.draw_indirect_count),
            storage_buffer8_bit_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.storage_buffer8_bit_access,
            ),
            uniform_and_storage_buffer8_bit_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.uniform_and_storage_buffer8_bit_access,
            ),
            storage_push_constant8: crate::conv::FromLowLevel::from_low_level(context, value.storage_push_constant8),
            shader_buffer_int64_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_buffer_int64_atomics,
            ),
            shader_shared_int64_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_shared_int64_atomics,
            ),
            shader_float16: crate::conv::FromLowLevel::from_low_level(context, value.shader_float16),
            shader_int8: crate::conv::FromLowLevel::from_low_level(context, value.shader_int8),
            descriptor_indexing: crate::conv::FromLowLevel::from_low_level(context, value.descriptor_indexing),
            shader_input_attachment_array_dynamic_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_input_attachment_array_dynamic_indexing,
            ),
            shader_uniform_texel_buffer_array_dynamic_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_uniform_texel_buffer_array_dynamic_indexing,
            ),
            shader_storage_texel_buffer_array_dynamic_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_storage_texel_buffer_array_dynamic_indexing,
            ),
            shader_uniform_buffer_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_uniform_buffer_array_non_uniform_indexing,
            ),
            shader_sampled_image_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_sampled_image_array_non_uniform_indexing,
            ),
            shader_storage_buffer_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_storage_buffer_array_non_uniform_indexing,
            ),
            shader_storage_image_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_storage_image_array_non_uniform_indexing,
            ),
            shader_input_attachment_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_input_attachment_array_non_uniform_indexing,
            ),
            shader_uniform_texel_buffer_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_uniform_texel_buffer_array_non_uniform_indexing,
            ),
            shader_storage_texel_buffer_array_non_uniform_indexing: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_storage_texel_buffer_array_non_uniform_indexing,
            ),
            descriptor_binding_uniform_buffer_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_uniform_buffer_update_after_bind,
            ),
            descriptor_binding_sampled_image_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_sampled_image_update_after_bind,
            ),
            descriptor_binding_storage_image_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_storage_image_update_after_bind,
            ),
            descriptor_binding_storage_buffer_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_storage_buffer_update_after_bind,
            ),
            descriptor_binding_uniform_texel_buffer_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_uniform_texel_buffer_update_after_bind,
            ),
            descriptor_binding_storage_texel_buffer_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_storage_texel_buffer_update_after_bind,
            ),
            descriptor_binding_update_unused_while_pending: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_update_unused_while_pending,
            ),
            descriptor_binding_partially_bound: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_partially_bound,
            ),
            descriptor_binding_variable_descriptor_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_variable_descriptor_count,
            ),
            runtime_descriptor_array: crate::conv::FromLowLevel::from_low_level(
                context,
                value.runtime_descriptor_array,
            ),
            sampler_filter_minmax: crate::conv::FromLowLevel::from_low_level(context, value.sampler_filter_minmax),
            scalar_block_layout: crate::conv::FromLowLevel::from_low_level(context, value.scalar_block_layout),
            imageless_framebuffer: crate::conv::FromLowLevel::from_low_level(context, value.imageless_framebuffer),
            uniform_buffer_standard_layout: crate::conv::FromLowLevel::from_low_level(
                context,
                value.uniform_buffer_standard_layout,
            ),
            shader_subgroup_extended_types: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_subgroup_extended_types,
            ),
            separate_depth_stencil_layouts: crate::conv::FromLowLevel::from_low_level(
                context,
                value.separate_depth_stencil_layouts,
            ),
            host_query_reset: crate::conv::FromLowLevel::from_low_level(context, value.host_query_reset),
            timeline_semaphore: crate::conv::FromLowLevel::from_low_level(context, value.timeline_semaphore),
            buffer_device_address: crate::conv::FromLowLevel::from_low_level(context, value.buffer_device_address),
            buffer_device_address_capture_replay: crate::conv::FromLowLevel::from_low_level(
                context,
                value.buffer_device_address_capture_replay,
            ),
            buffer_device_address_multi_device: crate::conv::FromLowLevel::from_low_level(
                context,
                value.buffer_device_address_multi_device,
            ),
            vulkan_memory_model: crate::conv::FromLowLevel::from_low_level(context, value.vulkan_memory_model),
            vulkan_memory_model_device_scope: crate::conv::FromLowLevel::from_low_level(
                context,
                value.vulkan_memory_model_device_scope,
            ),
            vulkan_memory_model_availability_visibility_chains: crate::conv::FromLowLevel::from_low_level(
                context,
                value.vulkan_memory_model_availability_visibility_chains,
            ),
            shader_output_viewport_index: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_output_viewport_index,
            ),
            shader_output_layer: crate::conv::FromLowLevel::from_low_level(context, value.shader_output_layer),
            subgroup_broadcast_dynamic_id: crate::conv::FromLowLevel::from_low_level(
                context,
                value.subgroup_broadcast_dynamic_id,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkan12Properties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceVulkan12Properties {
    #[doc(alias = "driverID")]
    pub driver_id: DriverId,
    #[doc(alias = "driverName")]
    pub driver_name: String,
    #[doc(alias = "driverInfo")]
    pub driver_info: String,
    #[doc(alias = "conformanceVersion")]
    pub conformance_version: ConformanceVersion,
    #[doc(alias = "denormBehaviorIndependence")]
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    #[doc(alias = "roundingModeIndependence")]
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat16")]
    pub shader_signed_zero_inf_nan_preserve_float16: bool,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat32")]
    pub shader_signed_zero_inf_nan_preserve_float32: bool,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat64")]
    pub shader_signed_zero_inf_nan_preserve_float64: bool,
    #[doc(alias = "shaderDenormPreserveFloat16")]
    pub shader_denorm_preserve_float16: bool,
    #[doc(alias = "shaderDenormPreserveFloat32")]
    pub shader_denorm_preserve_float32: bool,
    #[doc(alias = "shaderDenormPreserveFloat64")]
    pub shader_denorm_preserve_float64: bool,
    #[doc(alias = "shaderDenormFlushToZeroFloat16")]
    pub shader_denorm_flush_to_zero_float16: bool,
    #[doc(alias = "shaderDenormFlushToZeroFloat32")]
    pub shader_denorm_flush_to_zero_float32: bool,
    #[doc(alias = "shaderDenormFlushToZeroFloat64")]
    pub shader_denorm_flush_to_zero_float64: bool,
    #[doc(alias = "shaderRoundingModeRTEFloat16")]
    pub shader_rounding_mode_rte_float16: bool,
    #[doc(alias = "shaderRoundingModeRTEFloat32")]
    pub shader_rounding_mode_rte_float32: bool,
    #[doc(alias = "shaderRoundingModeRTEFloat64")]
    pub shader_rounding_mode_rte_float64: bool,
    #[doc(alias = "shaderRoundingModeRTZFloat16")]
    pub shader_rounding_mode_rtz_float16: bool,
    #[doc(alias = "shaderRoundingModeRTZFloat32")]
    pub shader_rounding_mode_rtz_float32: bool,
    #[doc(alias = "shaderRoundingModeRTZFloat64")]
    pub shader_rounding_mode_rtz_float64: bool,
    #[doc(alias = "maxUpdateAfterBindDescriptorsInAllPools")]
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    #[doc(alias = "shaderUniformBufferArrayNonUniformIndexingNative")]
    pub shader_uniform_buffer_array_non_uniform_indexing_native: bool,
    #[doc(alias = "shaderSampledImageArrayNonUniformIndexingNative")]
    pub shader_sampled_image_array_non_uniform_indexing_native: bool,
    #[doc(alias = "shaderStorageBufferArrayNonUniformIndexingNative")]
    pub shader_storage_buffer_array_non_uniform_indexing_native: bool,
    #[doc(alias = "shaderStorageImageArrayNonUniformIndexingNative")]
    pub shader_storage_image_array_non_uniform_indexing_native: bool,
    #[doc(alias = "shaderInputAttachmentArrayNonUniformIndexingNative")]
    pub shader_input_attachment_array_non_uniform_indexing_native: bool,
    #[doc(alias = "robustBufferAccessUpdateAfterBind")]
    pub robust_buffer_access_update_after_bind: bool,
    #[doc(alias = "quadDivergentImplicitLod")]
    pub quad_divergent_implicit_lod: bool,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindSamplers")]
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindUniformBuffers")]
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindStorageBuffers")]
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindSampledImages")]
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindStorageImages")]
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindInputAttachments")]
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    #[doc(alias = "maxPerStageUpdateAfterBindResources")]
    pub max_per_stage_update_after_bind_resources: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindSamplers")]
    pub max_descriptor_set_update_after_bind_samplers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindUniformBuffers")]
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindUniformBuffersDynamic")]
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageBuffers")]
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageBuffersDynamic")]
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindSampledImages")]
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageImages")]
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindInputAttachments")]
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
    #[doc(alias = "supportedDepthResolveModes")]
    pub supported_depth_resolve_modes: ResolveModeFlags,
    #[doc(alias = "supportedStencilResolveModes")]
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    #[doc(alias = "independentResolveNone")]
    pub independent_resolve_none: bool,
    #[doc(alias = "independentResolve")]
    pub independent_resolve: bool,
    #[doc(alias = "filterMinmaxSingleComponentFormats")]
    pub filter_minmax_single_component_formats: bool,
    #[doc(alias = "filterMinmaxImageComponentMapping")]
    pub filter_minmax_image_component_mapping: bool,
    #[doc(alias = "maxTimelineSemaphoreValueDifference")]
    pub max_timeline_semaphore_value_difference: u64,
    #[doc(alias = "framebufferIntegerColorSampleCounts")]
    pub framebuffer_integer_color_sample_counts: SampleCountFlags,
}
impl PhysicalDeviceVulkan12Properties {
    ///Get a reference to the `driver_id` field.
    pub fn driver_id(&self) -> DriverId {
        self.driver_id
    }
    ///Get a reference to the `driver_name` field.
    pub fn driver_name(&self) -> &String {
        &self.driver_name
    }
    ///Get a reference to the `driver_info` field.
    pub fn driver_info(&self) -> &String {
        &self.driver_info
    }
    ///Get a reference to the `conformance_version` field.
    pub fn conformance_version(&self) -> ConformanceVersion {
        self.conformance_version
    }
    ///Get a reference to the `denorm_behavior_independence` field.
    pub fn denorm_behavior_independence(&self) -> ShaderFloatControlsIndependence {
        self.denorm_behavior_independence
    }
    ///Get a reference to the `rounding_mode_independence` field.
    pub fn rounding_mode_independence(&self) -> ShaderFloatControlsIndependence {
        self.rounding_mode_independence
    }
    ///Get a reference to the `shader_signed_zero_inf_nan_preserve_float16` field.
    pub fn shader_signed_zero_inf_nan_preserve_float16(&self) -> &bool {
        &self.shader_signed_zero_inf_nan_preserve_float16
    }
    ///Get a reference to the `shader_signed_zero_inf_nan_preserve_float32` field.
    pub fn shader_signed_zero_inf_nan_preserve_float32(&self) -> &bool {
        &self.shader_signed_zero_inf_nan_preserve_float32
    }
    ///Get a reference to the `shader_signed_zero_inf_nan_preserve_float64` field.
    pub fn shader_signed_zero_inf_nan_preserve_float64(&self) -> &bool {
        &self.shader_signed_zero_inf_nan_preserve_float64
    }
    ///Get a reference to the `shader_denorm_preserve_float16` field.
    pub fn shader_denorm_preserve_float16(&self) -> &bool {
        &self.shader_denorm_preserve_float16
    }
    ///Get a reference to the `shader_denorm_preserve_float32` field.
    pub fn shader_denorm_preserve_float32(&self) -> &bool {
        &self.shader_denorm_preserve_float32
    }
    ///Get a reference to the `shader_denorm_preserve_float64` field.
    pub fn shader_denorm_preserve_float64(&self) -> &bool {
        &self.shader_denorm_preserve_float64
    }
    ///Get a reference to the `shader_denorm_flush_to_zero_float16` field.
    pub fn shader_denorm_flush_to_zero_float16(&self) -> &bool {
        &self.shader_denorm_flush_to_zero_float16
    }
    ///Get a reference to the `shader_denorm_flush_to_zero_float32` field.
    pub fn shader_denorm_flush_to_zero_float32(&self) -> &bool {
        &self.shader_denorm_flush_to_zero_float32
    }
    ///Get a reference to the `shader_denorm_flush_to_zero_float64` field.
    pub fn shader_denorm_flush_to_zero_float64(&self) -> &bool {
        &self.shader_denorm_flush_to_zero_float64
    }
    ///Get a reference to the `shader_rounding_mode_rte_float16` field.
    pub fn shader_rounding_mode_rte_float16(&self) -> &bool {
        &self.shader_rounding_mode_rte_float16
    }
    ///Get a reference to the `shader_rounding_mode_rte_float32` field.
    pub fn shader_rounding_mode_rte_float32(&self) -> &bool {
        &self.shader_rounding_mode_rte_float32
    }
    ///Get a reference to the `shader_rounding_mode_rte_float64` field.
    pub fn shader_rounding_mode_rte_float64(&self) -> &bool {
        &self.shader_rounding_mode_rte_float64
    }
    ///Get a reference to the `shader_rounding_mode_rtz_float16` field.
    pub fn shader_rounding_mode_rtz_float16(&self) -> &bool {
        &self.shader_rounding_mode_rtz_float16
    }
    ///Get a reference to the `shader_rounding_mode_rtz_float32` field.
    pub fn shader_rounding_mode_rtz_float32(&self) -> &bool {
        &self.shader_rounding_mode_rtz_float32
    }
    ///Get a reference to the `shader_rounding_mode_rtz_float64` field.
    pub fn shader_rounding_mode_rtz_float64(&self) -> &bool {
        &self.shader_rounding_mode_rtz_float64
    }
    ///Get a reference to the `max_update_after_bind_descriptors_in_all_pools` field.
    pub fn max_update_after_bind_descriptors_in_all_pools(&self) -> u32 {
        self.max_update_after_bind_descriptors_in_all_pools
    }
    ///Get a reference to the `shader_uniform_buffer_array_non_uniform_indexing_native` field.
    pub fn shader_uniform_buffer_array_non_uniform_indexing_native(&self) -> &bool {
        &self.shader_uniform_buffer_array_non_uniform_indexing_native
    }
    ///Get a reference to the `shader_sampled_image_array_non_uniform_indexing_native` field.
    pub fn shader_sampled_image_array_non_uniform_indexing_native(&self) -> &bool {
        &self.shader_sampled_image_array_non_uniform_indexing_native
    }
    ///Get a reference to the `shader_storage_buffer_array_non_uniform_indexing_native` field.
    pub fn shader_storage_buffer_array_non_uniform_indexing_native(&self) -> &bool {
        &self.shader_storage_buffer_array_non_uniform_indexing_native
    }
    ///Get a reference to the `shader_storage_image_array_non_uniform_indexing_native` field.
    pub fn shader_storage_image_array_non_uniform_indexing_native(&self) -> &bool {
        &self.shader_storage_image_array_non_uniform_indexing_native
    }
    ///Get a reference to the `shader_input_attachment_array_non_uniform_indexing_native` field.
    pub fn shader_input_attachment_array_non_uniform_indexing_native(&self) -> &bool {
        &self.shader_input_attachment_array_non_uniform_indexing_native
    }
    ///Get a reference to the `robust_buffer_access_update_after_bind` field.
    pub fn robust_buffer_access_update_after_bind(&self) -> &bool {
        &self.robust_buffer_access_update_after_bind
    }
    ///Get a reference to the `quad_divergent_implicit_lod` field.
    pub fn quad_divergent_implicit_lod(&self) -> &bool {
        &self.quad_divergent_implicit_lod
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_samplers` field.
    pub fn max_per_stage_descriptor_update_after_bind_samplers(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_samplers
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_uniform_buffers` field.
    pub fn max_per_stage_descriptor_update_after_bind_uniform_buffers(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_uniform_buffers
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_storage_buffers` field.
    pub fn max_per_stage_descriptor_update_after_bind_storage_buffers(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_storage_buffers
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_sampled_images` field.
    pub fn max_per_stage_descriptor_update_after_bind_sampled_images(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_sampled_images
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_storage_images` field.
    pub fn max_per_stage_descriptor_update_after_bind_storage_images(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_storage_images
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_input_attachments` field.
    pub fn max_per_stage_descriptor_update_after_bind_input_attachments(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_input_attachments
    }
    ///Get a reference to the `max_per_stage_update_after_bind_resources` field.
    pub fn max_per_stage_update_after_bind_resources(&self) -> u32 {
        self.max_per_stage_update_after_bind_resources
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_samplers` field.
    pub fn max_descriptor_set_update_after_bind_samplers(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_samplers
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_uniform_buffers` field.
    pub fn max_descriptor_set_update_after_bind_uniform_buffers(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_uniform_buffers
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_uniform_buffers_dynamic` field.
    pub fn max_descriptor_set_update_after_bind_uniform_buffers_dynamic(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_storage_buffers` field.
    pub fn max_descriptor_set_update_after_bind_storage_buffers(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_storage_buffers
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_storage_buffers_dynamic` field.
    pub fn max_descriptor_set_update_after_bind_storage_buffers_dynamic(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_storage_buffers_dynamic
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_sampled_images` field.
    pub fn max_descriptor_set_update_after_bind_sampled_images(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_sampled_images
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_storage_images` field.
    pub fn max_descriptor_set_update_after_bind_storage_images(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_storage_images
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_input_attachments` field.
    pub fn max_descriptor_set_update_after_bind_input_attachments(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_input_attachments
    }
    ///Get a reference to the `supported_depth_resolve_modes` field.
    pub fn supported_depth_resolve_modes(&self) -> ResolveModeFlags {
        self.supported_depth_resolve_modes
    }
    ///Get a reference to the `supported_stencil_resolve_modes` field.
    pub fn supported_stencil_resolve_modes(&self) -> ResolveModeFlags {
        self.supported_stencil_resolve_modes
    }
    ///Get a reference to the `independent_resolve_none` field.
    pub fn independent_resolve_none(&self) -> &bool {
        &self.independent_resolve_none
    }
    ///Get a reference to the `independent_resolve` field.
    pub fn independent_resolve(&self) -> &bool {
        &self.independent_resolve
    }
    ///Get a reference to the `filter_minmax_single_component_formats` field.
    pub fn filter_minmax_single_component_formats(&self) -> &bool {
        &self.filter_minmax_single_component_formats
    }
    ///Get a reference to the `filter_minmax_image_component_mapping` field.
    pub fn filter_minmax_image_component_mapping(&self) -> &bool {
        &self.filter_minmax_image_component_mapping
    }
    ///Get a reference to the `max_timeline_semaphore_value_difference` field.
    pub fn max_timeline_semaphore_value_difference(&self) -> u64 {
        self.max_timeline_semaphore_value_difference
    }
    ///Get a reference to the `framebuffer_integer_color_sample_counts` field.
    pub fn framebuffer_integer_color_sample_counts(&self) -> SampleCountFlags {
        self.framebuffer_integer_color_sample_counts
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceVulkan12Properties {
    type LowLevel = crate::native::vulkan1_2::PhysicalDeviceVulkan12Properties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let driver_name_bytes = self.driver_name.as_bytes();
        debug_assert!(
            memchr::memchr(0, driver_name_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            driver_name_bytes.len() <= MAX_DRIVER_NAME_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut driver_name: [std::ffi::c_char; MAX_DRIVER_NAME_SIZE as usize] = [0; MAX_DRIVER_NAME_SIZE as usize];
        driver_name[0..driver_name_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            driver_name_bytes.as_ptr() as *const std::ffi::c_char,
            driver_name_bytes.len(),
        ));
        let driver_info_bytes = self.driver_info.as_bytes();
        debug_assert!(
            memchr::memchr(0, driver_info_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            driver_info_bytes.len() <= MAX_DRIVER_INFO_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut driver_info: [std::ffi::c_char; MAX_DRIVER_INFO_SIZE as usize] = [0; MAX_DRIVER_INFO_SIZE as usize];
        driver_info[0..driver_info_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            driver_info_bytes.as_ptr() as *const std::ffi::c_char,
            driver_info_bytes.len(),
        ));
        crate::native::vulkan1_2::PhysicalDeviceVulkan12Properties {
            s_type: StructureType::PhysicalDeviceVulkan12Properties,
            p_next: std::ptr::null_mut(),
            driver_id: self.driver_id.into_low_level(context, bump),
            driver_name: driver_name,
            driver_info: driver_info,
            conformance_version: self.conformance_version.into_low_level(context, bump),
            denorm_behavior_independence: self.denorm_behavior_independence.into_low_level(context, bump),
            rounding_mode_independence: self.rounding_mode_independence.into_low_level(context, bump),
            shader_signed_zero_inf_nan_preserve_float16: self
                .shader_signed_zero_inf_nan_preserve_float16
                .into_low_level(context, bump),
            shader_signed_zero_inf_nan_preserve_float32: self
                .shader_signed_zero_inf_nan_preserve_float32
                .into_low_level(context, bump),
            shader_signed_zero_inf_nan_preserve_float64: self
                .shader_signed_zero_inf_nan_preserve_float64
                .into_low_level(context, bump),
            shader_denorm_preserve_float16: self.shader_denorm_preserve_float16.into_low_level(context, bump),
            shader_denorm_preserve_float32: self.shader_denorm_preserve_float32.into_low_level(context, bump),
            shader_denorm_preserve_float64: self.shader_denorm_preserve_float64.into_low_level(context, bump),
            shader_denorm_flush_to_zero_float16: self.shader_denorm_flush_to_zero_float16.into_low_level(context, bump),
            shader_denorm_flush_to_zero_float32: self.shader_denorm_flush_to_zero_float32.into_low_level(context, bump),
            shader_denorm_flush_to_zero_float64: self.shader_denorm_flush_to_zero_float64.into_low_level(context, bump),
            shader_rounding_mode_rte_float16: self.shader_rounding_mode_rte_float16.into_low_level(context, bump),
            shader_rounding_mode_rte_float32: self.shader_rounding_mode_rte_float32.into_low_level(context, bump),
            shader_rounding_mode_rte_float64: self.shader_rounding_mode_rte_float64.into_low_level(context, bump),
            shader_rounding_mode_rtz_float16: self.shader_rounding_mode_rtz_float16.into_low_level(context, bump),
            shader_rounding_mode_rtz_float32: self.shader_rounding_mode_rtz_float32.into_low_level(context, bump),
            shader_rounding_mode_rtz_float64: self.shader_rounding_mode_rtz_float64.into_low_level(context, bump),
            max_update_after_bind_descriptors_in_all_pools: self
                .max_update_after_bind_descriptors_in_all_pools
                .into_low_level(context, bump),
            shader_uniform_buffer_array_non_uniform_indexing_native: self
                .shader_uniform_buffer_array_non_uniform_indexing_native
                .into_low_level(context, bump),
            shader_sampled_image_array_non_uniform_indexing_native: self
                .shader_sampled_image_array_non_uniform_indexing_native
                .into_low_level(context, bump),
            shader_storage_buffer_array_non_uniform_indexing_native: self
                .shader_storage_buffer_array_non_uniform_indexing_native
                .into_low_level(context, bump),
            shader_storage_image_array_non_uniform_indexing_native: self
                .shader_storage_image_array_non_uniform_indexing_native
                .into_low_level(context, bump),
            shader_input_attachment_array_non_uniform_indexing_native: self
                .shader_input_attachment_array_non_uniform_indexing_native
                .into_low_level(context, bump),
            robust_buffer_access_update_after_bind: self
                .robust_buffer_access_update_after_bind
                .into_low_level(context, bump),
            quad_divergent_implicit_lod: self.quad_divergent_implicit_lod.into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_samplers: self
                .max_per_stage_descriptor_update_after_bind_samplers
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_uniform_buffers: self
                .max_per_stage_descriptor_update_after_bind_uniform_buffers
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_storage_buffers: self
                .max_per_stage_descriptor_update_after_bind_storage_buffers
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_sampled_images: self
                .max_per_stage_descriptor_update_after_bind_sampled_images
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_storage_images: self
                .max_per_stage_descriptor_update_after_bind_storage_images
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_input_attachments: self
                .max_per_stage_descriptor_update_after_bind_input_attachments
                .into_low_level(context, bump),
            max_per_stage_update_after_bind_resources: self
                .max_per_stage_update_after_bind_resources
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_samplers: self
                .max_descriptor_set_update_after_bind_samplers
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_uniform_buffers: self
                .max_descriptor_set_update_after_bind_uniform_buffers
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: self
                .max_descriptor_set_update_after_bind_uniform_buffers_dynamic
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_storage_buffers: self
                .max_descriptor_set_update_after_bind_storage_buffers
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: self
                .max_descriptor_set_update_after_bind_storage_buffers_dynamic
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_sampled_images: self
                .max_descriptor_set_update_after_bind_sampled_images
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_storage_images: self
                .max_descriptor_set_update_after_bind_storage_images
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_input_attachments: self
                .max_descriptor_set_update_after_bind_input_attachments
                .into_low_level(context, bump),
            supported_depth_resolve_modes: self.supported_depth_resolve_modes.into_low_level(context, bump),
            supported_stencil_resolve_modes: self.supported_stencil_resolve_modes.into_low_level(context, bump),
            independent_resolve_none: self.independent_resolve_none.into_low_level(context, bump),
            independent_resolve: self.independent_resolve.into_low_level(context, bump),
            filter_minmax_single_component_formats: self
                .filter_minmax_single_component_formats
                .into_low_level(context, bump),
            filter_minmax_image_component_mapping: self
                .filter_minmax_image_component_mapping
                .into_low_level(context, bump),
            max_timeline_semaphore_value_difference: self
                .max_timeline_semaphore_value_difference
                .into_low_level(context, bump),
            framebuffer_integer_color_sample_counts: self
                .framebuffer_integer_color_sample_counts
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceVulkan12Properties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let driver_name_cstr = std::ffi::CStr::from_ptr(value.driver_name.as_ptr());
        let driver_name = driver_name_cstr.to_string_lossy().into_owned();
        let driver_info_cstr = std::ffi::CStr::from_ptr(value.driver_info.as_ptr());
        let driver_info = driver_info_cstr.to_string_lossy().into_owned();
        Self {
            driver_id: crate::conv::FromLowLevel::from_low_level(context, value.driver_id),
            driver_name: driver_name,
            driver_info: driver_info,
            conformance_version: crate::conv::FromLowLevel::from_low_level(context, value.conformance_version),
            denorm_behavior_independence: crate::conv::FromLowLevel::from_low_level(
                context,
                value.denorm_behavior_independence,
            ),
            rounding_mode_independence: crate::conv::FromLowLevel::from_low_level(
                context,
                value.rounding_mode_independence,
            ),
            shader_signed_zero_inf_nan_preserve_float16: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_signed_zero_inf_nan_preserve_float16,
            ),
            shader_signed_zero_inf_nan_preserve_float32: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_signed_zero_inf_nan_preserve_float32,
            ),
            shader_signed_zero_inf_nan_preserve_float64: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_signed_zero_inf_nan_preserve_float64,
            ),
            shader_denorm_preserve_float16: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_denorm_preserve_float16,
            ),
            shader_denorm_preserve_float32: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_denorm_preserve_float32,
            ),
            shader_denorm_preserve_float64: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_denorm_preserve_float64,
            ),
            shader_denorm_flush_to_zero_float16: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_denorm_flush_to_zero_float16,
            ),
            shader_denorm_flush_to_zero_float32: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_denorm_flush_to_zero_float32,
            ),
            shader_denorm_flush_to_zero_float64: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_denorm_flush_to_zero_float64,
            ),
            shader_rounding_mode_rte_float16: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_rounding_mode_rte_float16,
            ),
            shader_rounding_mode_rte_float32: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_rounding_mode_rte_float32,
            ),
            shader_rounding_mode_rte_float64: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_rounding_mode_rte_float64,
            ),
            shader_rounding_mode_rtz_float16: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_rounding_mode_rtz_float16,
            ),
            shader_rounding_mode_rtz_float32: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_rounding_mode_rtz_float32,
            ),
            shader_rounding_mode_rtz_float64: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_rounding_mode_rtz_float64,
            ),
            max_update_after_bind_descriptors_in_all_pools: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_update_after_bind_descriptors_in_all_pools,
            ),
            shader_uniform_buffer_array_non_uniform_indexing_native: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_uniform_buffer_array_non_uniform_indexing_native,
            ),
            shader_sampled_image_array_non_uniform_indexing_native: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_sampled_image_array_non_uniform_indexing_native,
            ),
            shader_storage_buffer_array_non_uniform_indexing_native: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_storage_buffer_array_non_uniform_indexing_native,
            ),
            shader_storage_image_array_non_uniform_indexing_native: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_storage_image_array_non_uniform_indexing_native,
            ),
            shader_input_attachment_array_non_uniform_indexing_native: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_input_attachment_array_non_uniform_indexing_native,
            ),
            robust_buffer_access_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.robust_buffer_access_update_after_bind,
            ),
            quad_divergent_implicit_lod: crate::conv::FromLowLevel::from_low_level(
                context,
                value.quad_divergent_implicit_lod,
            ),
            max_per_stage_descriptor_update_after_bind_samplers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_samplers,
            ),
            max_per_stage_descriptor_update_after_bind_uniform_buffers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_uniform_buffers,
            ),
            max_per_stage_descriptor_update_after_bind_storage_buffers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_storage_buffers,
            ),
            max_per_stage_descriptor_update_after_bind_sampled_images: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_sampled_images,
            ),
            max_per_stage_descriptor_update_after_bind_storage_images: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_storage_images,
            ),
            max_per_stage_descriptor_update_after_bind_input_attachments: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_update_after_bind_input_attachments,
            ),
            max_per_stage_update_after_bind_resources: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_update_after_bind_resources,
            ),
            max_descriptor_set_update_after_bind_samplers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_samplers,
            ),
            max_descriptor_set_update_after_bind_uniform_buffers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_uniform_buffers,
            ),
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_uniform_buffers_dynamic,
            ),
            max_descriptor_set_update_after_bind_storage_buffers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_storage_buffers,
            ),
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_storage_buffers_dynamic,
            ),
            max_descriptor_set_update_after_bind_sampled_images: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_sampled_images,
            ),
            max_descriptor_set_update_after_bind_storage_images: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_storage_images,
            ),
            max_descriptor_set_update_after_bind_input_attachments: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_input_attachments,
            ),
            supported_depth_resolve_modes: crate::conv::FromLowLevel::from_low_level(
                context,
                value.supported_depth_resolve_modes,
            ),
            supported_stencil_resolve_modes: crate::conv::FromLowLevel::from_low_level(
                context,
                value.supported_stencil_resolve_modes,
            ),
            independent_resolve_none: crate::conv::FromLowLevel::from_low_level(
                context,
                value.independent_resolve_none,
            ),
            independent_resolve: crate::conv::FromLowLevel::from_low_level(context, value.independent_resolve),
            filter_minmax_single_component_formats: crate::conv::FromLowLevel::from_low_level(
                context,
                value.filter_minmax_single_component_formats,
            ),
            filter_minmax_image_component_mapping: crate::conv::FromLowLevel::from_low_level(
                context,
                value.filter_minmax_image_component_mapping,
            ),
            max_timeline_semaphore_value_difference: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_timeline_semaphore_value_difference,
            ),
            framebuffer_integer_color_sample_counts: crate::conv::FromLowLevel::from_low_level(
                context,
                value.framebuffer_integer_color_sample_counts,
            ),
        }
    }
}
