pub use crate::common::extensions::android_external_memory_android_hardware_buffer::{
    AHardwareBuffer, ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME,
    ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{ComponentMapping, DeviceMemory, DeviceSize, Format, FormatFeatureFlags, StructureType},
    vulkan1_1::{ChromaLocation, SamplerYcbcrModelConversion, SamplerYcbcrRange},
    vulkan1_3::FormatFeatureFlags2,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkAndroidHardwareBufferUsageANDROID")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AndroidHardwareBufferUsageANDROID {
    #[doc(alias = "androidHardwareBufferUsage")]
    pub android_hardware_buffer_usage: u64,
}
impl AndroidHardwareBufferUsageANDROID {
    ///Get a reference to the `android_hardware_buffer_usage` field.
    pub fn android_hardware_buffer_usage(&self) -> u64 {
        self.android_hardware_buffer_usage
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AndroidHardwareBufferUsageANDROID {
    type LowLevel =
        crate::native::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferUsageANDROID;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferUsageANDROID {
            s_type: StructureType::AndroidHardwareBufferUsageAndroid,
            p_next: std::ptr::null_mut(),
            android_hardware_buffer_usage: self.android_hardware_buffer_usage.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AndroidHardwareBufferUsageANDROID {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            android_hardware_buffer_usage: crate::conv::FromLowLevel::from_low_level(
                context,
                value.android_hardware_buffer_usage,
            ),
        }
    }
}
#[doc(alias = "VkAndroidHardwareBufferPropertiesANDROID")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AndroidHardwareBufferPropertiesANDROID {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[AndroidHardwareBufferPropertiesANDROIDExtension; 1]>,
    #[doc(alias = "allocationSize")]
    pub allocation_size: DeviceSize,
    #[doc(alias = "memoryTypeBits")]
    pub memory_type_bits: u32,
}
impl AndroidHardwareBufferPropertiesANDROID {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<AndroidHardwareBufferPropertiesANDROIDExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[AndroidHardwareBufferPropertiesANDROIDExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `allocation_size` field.
    pub fn allocation_size(&self) -> &DeviceSize {
        &self.allocation_size
    }
    ///Get a reference to the `memory_type_bits` field.
    pub fn memory_type_bits(&self) -> u32 {
        self.memory_type_bits
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AndroidHardwareBufferPropertiesANDROID {
    type LowLevel = crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferPropertiesANDROID ;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null_mut();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferPropertiesANDROID { s_type : StructureType :: AndroidHardwareBufferPropertiesAndroid , p_next : next , allocation_size : self . allocation_size . into_low_level (context , bump) , memory_type_bits : self . memory_type_bits . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AndroidHardwareBufferPropertiesANDROID {
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
            allocation_size: crate::conv::FromLowLevel::from_low_level(context, value.allocation_size),
            memory_type_bits: crate::conv::FromLowLevel::from_low_level(context, value.memory_type_bits),
        }
    }
}
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`AndroidHardwareBufferPropertiesANDROID`]
pub enum AndroidHardwareBufferPropertiesANDROIDExtension {
    ///Contains a type [`AndroidHardwareBufferFormatPropertiesANDROID`] for extending
    /// [`AndroidHardwareBufferPropertiesANDROID`]
    AndroidHardwareBufferFormatPropertiesANDROID(AndroidHardwareBufferFormatPropertiesANDROID),
    ///Contains a type [`AndroidHardwareBufferFormatProperties2ANDROID`] for extending
    /// [`AndroidHardwareBufferPropertiesANDROID`]
    AndroidHardwareBufferFormatProperties2ANDROID(AndroidHardwareBufferFormatProperties2ANDROID),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AndroidHardwareBufferPropertiesANDROIDExtension {
    type LowLevel = *mut crate::native::vulkan1_0::BaseOutStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self { Self :: AndroidHardwareBufferFormatPropertiesANDROID (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferFormatPropertiesANDROID) . cast () , Self :: AndroidHardwareBufferFormatProperties2ANDROID (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferFormatProperties2ANDROID) . cast () , other => unreachable ! ("unexpected variant {:?}" , other) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AndroidHardwareBufferPropertiesANDROIDExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { crate :: native :: vulkan1_0 :: StructureType :: AndroidHardwareBufferFormatPropertiesAndroid => Self :: AndroidHardwareBufferFormatPropertiesANDROID (AndroidHardwareBufferFormatPropertiesANDROID :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferFormatPropertiesANDROID > ()))) , crate :: native :: vulkan1_0 :: StructureType :: AndroidHardwareBufferFormatProperties2Android => Self :: AndroidHardwareBufferFormatProperties2ANDROID (AndroidHardwareBufferFormatProperties2ANDROID :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferFormatProperties2ANDROID > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (AndroidHardwareBufferPropertiesANDROID)) }
    }
}
impl From<AndroidHardwareBufferFormatPropertiesANDROID> for AndroidHardwareBufferPropertiesANDROIDExtension {
    fn from(ext: AndroidHardwareBufferFormatPropertiesANDROID) -> Self {
        Self::AndroidHardwareBufferFormatPropertiesANDROID(ext)
    }
}
impl TryInto<AndroidHardwareBufferFormatPropertiesANDROID> for AndroidHardwareBufferPropertiesANDROIDExtension {
    type Error = AndroidHardwareBufferPropertiesANDROIDExtension;
    fn try_into(self) -> Result<AndroidHardwareBufferFormatPropertiesANDROID, Self::Error> {
        match self {
            Self::AndroidHardwareBufferFormatPropertiesANDROID(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<AndroidHardwareBufferFormatProperties2ANDROID> for AndroidHardwareBufferPropertiesANDROIDExtension {
    fn from(ext: AndroidHardwareBufferFormatProperties2ANDROID) -> Self {
        Self::AndroidHardwareBufferFormatProperties2ANDROID(ext)
    }
}
impl TryInto<AndroidHardwareBufferFormatProperties2ANDROID> for AndroidHardwareBufferPropertiesANDROIDExtension {
    type Error = AndroidHardwareBufferPropertiesANDROIDExtension;
    fn try_into(self) -> Result<AndroidHardwareBufferFormatProperties2ANDROID, Self::Error> {
        match self {
            Self::AndroidHardwareBufferFormatProperties2ANDROID(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkMemoryGetAndroidHardwareBufferInfoANDROID")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID {
    pub memory: DeviceMemory,
}
impl MemoryGetAndroidHardwareBufferInfoANDROID {
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
unsafe impl crate::conv::IntoLowLevel for MemoryGetAndroidHardwareBufferInfoANDROID {
    type LowLevel = crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: MemoryGetAndroidHardwareBufferInfoANDROID ;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: MemoryGetAndroidHardwareBufferInfoANDROID { s_type : StructureType :: MemoryGetAndroidHardwareBufferInfoAndroid , p_next : std :: ptr :: null () , memory : self . memory . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryGetAndroidHardwareBufferInfoANDROID {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            memory: crate::conv::FromLowLevel::from_low_level(context, value.memory),
        }
    }
}
#[doc(alias = "VkAndroidHardwareBufferFormatPropertiesANDROID")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AndroidHardwareBufferFormatPropertiesANDROID {
    pub format: Format,
    #[doc(alias = "externalFormat")]
    pub external_format: u64,
    #[doc(alias = "formatFeatures")]
    pub format_features: FormatFeatureFlags,
    #[doc(alias = "samplerYcbcrConversionComponents")]
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    #[doc(alias = "suggestedYcbcrModel")]
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    #[doc(alias = "suggestedYcbcrRange")]
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    #[doc(alias = "suggestedXChromaOffset")]
    pub suggested_x_chroma_offset: ChromaLocation,
    #[doc(alias = "suggestedYChromaOffset")]
    pub suggested_y_chroma_offset: ChromaLocation,
}
impl AndroidHardwareBufferFormatPropertiesANDROID {
    ///Get a reference to the `format` field.
    pub fn format(&self) -> Format {
        self.format
    }
    ///Get a reference to the `external_format` field.
    pub fn external_format(&self) -> u64 {
        self.external_format
    }
    ///Get a reference to the `format_features` field.
    pub fn format_features(&self) -> FormatFeatureFlags {
        self.format_features
    }
    ///Get a reference to the `sampler_ycbcr_conversion_components` field.
    pub fn sampler_ycbcr_conversion_components(&self) -> ComponentMapping {
        self.sampler_ycbcr_conversion_components
    }
    ///Get a reference to the `suggested_ycbcr_model` field.
    pub fn suggested_ycbcr_model(&self) -> SamplerYcbcrModelConversion {
        self.suggested_ycbcr_model
    }
    ///Get a reference to the `suggested_ycbcr_range` field.
    pub fn suggested_ycbcr_range(&self) -> SamplerYcbcrRange {
        self.suggested_ycbcr_range
    }
    ///Get a reference to the `suggested_x_chroma_offset` field.
    pub fn suggested_x_chroma_offset(&self) -> ChromaLocation {
        self.suggested_x_chroma_offset
    }
    ///Get a reference to the `suggested_y_chroma_offset` field.
    pub fn suggested_y_chroma_offset(&self) -> ChromaLocation {
        self.suggested_y_chroma_offset
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AndroidHardwareBufferFormatPropertiesANDROID {
    type LowLevel = crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferFormatPropertiesANDROID ;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferFormatPropertiesANDROID { s_type : StructureType :: AndroidHardwareBufferFormatPropertiesAndroid , p_next : std :: ptr :: null_mut () , format : self . format . into_low_level (context , bump) , external_format : self . external_format . into_low_level (context , bump) , format_features : self . format_features . into_low_level (context , bump) , sampler_ycbcr_conversion_components : self . sampler_ycbcr_conversion_components . into_low_level (context , bump) , suggested_ycbcr_model : self . suggested_ycbcr_model . into_low_level (context , bump) , suggested_ycbcr_range : self . suggested_ycbcr_range . into_low_level (context , bump) , suggested_x_chroma_offset : self . suggested_x_chroma_offset . into_low_level (context , bump) , suggested_y_chroma_offset : self . suggested_y_chroma_offset . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AndroidHardwareBufferFormatPropertiesANDROID {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            format: crate::conv::FromLowLevel::from_low_level(context, value.format),
            external_format: crate::conv::FromLowLevel::from_low_level(context, value.external_format),
            format_features: crate::conv::FromLowLevel::from_low_level(context, value.format_features),
            sampler_ycbcr_conversion_components: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sampler_ycbcr_conversion_components,
            ),
            suggested_ycbcr_model: crate::conv::FromLowLevel::from_low_level(context, value.suggested_ycbcr_model),
            suggested_ycbcr_range: crate::conv::FromLowLevel::from_low_level(context, value.suggested_ycbcr_range),
            suggested_x_chroma_offset: crate::conv::FromLowLevel::from_low_level(
                context,
                value.suggested_x_chroma_offset,
            ),
            suggested_y_chroma_offset: crate::conv::FromLowLevel::from_low_level(
                context,
                value.suggested_y_chroma_offset,
            ),
        }
    }
}
#[doc(alias = "VkExternalFormatANDROID")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalFormatANDROID {
    #[doc(alias = "externalFormat")]
    pub external_format: u64,
}
impl ExternalFormatANDROID {
    ///Get a reference to the `external_format` field.
    pub fn external_format(&self) -> u64 {
        self.external_format
    }
    ///Get a mutable reference to the `external_format` field.
    pub fn external_format_mut(&mut self) -> &mut u64 {
        &mut self.external_format
    }
    ///Sets the `external_format` field.
    pub fn set_external_format(&mut self, external_format: u64) -> &mut Self {
        self.external_format = external_format;
        self
    }
    ///Sets the `external_format` field in a builder way.
    pub fn with_external_format(mut self, external_format: u64) -> Self {
        self.external_format = external_format;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalFormatANDROID {
    type LowLevel = crate::native::extensions::android_external_memory_android_hardware_buffer::ExternalFormatANDROID;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::android_external_memory_android_hardware_buffer::ExternalFormatANDROID {
            s_type: StructureType::ExternalFormatAndroid,
            p_next: std::ptr::null_mut(),
            external_format: self.external_format.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExternalFormatANDROID {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            external_format: crate::conv::FromLowLevel::from_low_level(context, value.external_format),
        }
    }
}
#[doc(alias = "VkAndroidHardwareBufferFormatProperties2ANDROID")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AndroidHardwareBufferFormatProperties2ANDROID {
    pub format: Format,
    #[doc(alias = "externalFormat")]
    pub external_format: u64,
    #[doc(alias = "formatFeatures")]
    pub format_features: FormatFeatureFlags2,
    #[doc(alias = "samplerYcbcrConversionComponents")]
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    #[doc(alias = "suggestedYcbcrModel")]
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    #[doc(alias = "suggestedYcbcrRange")]
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    #[doc(alias = "suggestedXChromaOffset")]
    pub suggested_x_chroma_offset: ChromaLocation,
    #[doc(alias = "suggestedYChromaOffset")]
    pub suggested_y_chroma_offset: ChromaLocation,
}
impl AndroidHardwareBufferFormatProperties2ANDROID {
    ///Get a reference to the `format` field.
    pub fn format(&self) -> Format {
        self.format
    }
    ///Get a reference to the `external_format` field.
    pub fn external_format(&self) -> u64 {
        self.external_format
    }
    ///Get a reference to the `format_features` field.
    pub fn format_features(&self) -> FormatFeatureFlags2 {
        self.format_features
    }
    ///Get a reference to the `sampler_ycbcr_conversion_components` field.
    pub fn sampler_ycbcr_conversion_components(&self) -> ComponentMapping {
        self.sampler_ycbcr_conversion_components
    }
    ///Get a reference to the `suggested_ycbcr_model` field.
    pub fn suggested_ycbcr_model(&self) -> SamplerYcbcrModelConversion {
        self.suggested_ycbcr_model
    }
    ///Get a reference to the `suggested_ycbcr_range` field.
    pub fn suggested_ycbcr_range(&self) -> SamplerYcbcrRange {
        self.suggested_ycbcr_range
    }
    ///Get a reference to the `suggested_x_chroma_offset` field.
    pub fn suggested_x_chroma_offset(&self) -> ChromaLocation {
        self.suggested_x_chroma_offset
    }
    ///Get a reference to the `suggested_y_chroma_offset` field.
    pub fn suggested_y_chroma_offset(&self) -> ChromaLocation {
        self.suggested_y_chroma_offset
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AndroidHardwareBufferFormatProperties2ANDROID {
    type LowLevel = crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferFormatProperties2ANDROID ;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferFormatProperties2ANDROID { s_type : StructureType :: AndroidHardwareBufferFormatProperties2Android , p_next : std :: ptr :: null_mut () , format : self . format . into_low_level (context , bump) , external_format : self . external_format . into_low_level (context , bump) , format_features : self . format_features . into_low_level (context , bump) , sampler_ycbcr_conversion_components : self . sampler_ycbcr_conversion_components . into_low_level (context , bump) , suggested_ycbcr_model : self . suggested_ycbcr_model . into_low_level (context , bump) , suggested_ycbcr_range : self . suggested_ycbcr_range . into_low_level (context , bump) , suggested_x_chroma_offset : self . suggested_x_chroma_offset . into_low_level (context , bump) , suggested_y_chroma_offset : self . suggested_y_chroma_offset . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AndroidHardwareBufferFormatProperties2ANDROID {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            format: crate::conv::FromLowLevel::from_low_level(context, value.format),
            external_format: crate::conv::FromLowLevel::from_low_level(context, value.external_format),
            format_features: crate::conv::FromLowLevel::from_low_level(context, value.format_features),
            sampler_ycbcr_conversion_components: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sampler_ycbcr_conversion_components,
            ),
            suggested_ycbcr_model: crate::conv::FromLowLevel::from_low_level(context, value.suggested_ycbcr_model),
            suggested_ycbcr_range: crate::conv::FromLowLevel::from_low_level(context, value.suggested_ycbcr_range),
            suggested_x_chroma_offset: crate::conv::FromLowLevel::from_low_level(
                context,
                value.suggested_x_chroma_offset,
            ),
            suggested_y_chroma_offset: crate::conv::FromLowLevel::from_low_level(
                context,
                value.suggested_y_chroma_offset,
            ),
        }
    }
}
