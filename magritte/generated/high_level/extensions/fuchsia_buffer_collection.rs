pub use crate::common::extensions::fuchsia_buffer_collection::{
    ImageConstraintsInfoFlagBitsFUCHSIA, ImageConstraintsInfoFlagsFUCHSIA, ImageFormatConstraintsFlagsFUCHSIA,
    FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME, FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION,
};
use crate::{
    context::{Container, Context, Error, ObjectId},
    vulkan1_0::{BufferCreateInfo, ComponentMapping, FormatFeatureFlags, ImageCreateInfo, StructureType},
    vulkan1_1::{ChromaLocation, SamplerYcbcrModelConversion, SamplerYcbcrRange},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkImportMemoryBufferCollectionFUCHSIA")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImportMemoryBufferCollectionFUCHSIA {
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}
impl ImportMemoryBufferCollectionFUCHSIA {
    ///Get a reference to the `collection` field.
    pub fn collection(&self) -> &BufferCollectionFUCHSIA {
        &self.collection
    }
    ///Get a reference to the `index` field.
    pub fn index(&self) -> u32 {
        self.index
    }
    ///Get a mutable reference to the `collection` field.
    pub fn collection_mut(&mut self) -> &mut BufferCollectionFUCHSIA {
        &mut self.collection
    }
    ///Get a mutable reference to the `index` field.
    pub fn index_mut(&mut self) -> &mut u32 {
        &mut self.index
    }
    ///Sets the `collection` field.
    pub fn set_collection(&mut self, collection: BufferCollectionFUCHSIA) -> &mut Self {
        self.collection = collection;
        self
    }
    ///Sets the `index` field.
    pub fn set_index(&mut self, index: u32) -> &mut Self {
        self.index = index;
        self
    }
    ///Sets the `collection` field in a builder way.
    pub fn with_collection(mut self, collection: BufferCollectionFUCHSIA) -> Self {
        self.collection = collection;
        self
    }
    ///Sets the `index` field in a builder way.
    pub fn with_index(mut self, index: u32) -> Self {
        self.index = index;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImportMemoryBufferCollectionFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_buffer_collection::ImportMemoryBufferCollectionFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::fuchsia_buffer_collection::ImportMemoryBufferCollectionFUCHSIA {
            s_type: StructureType::ImportMemoryBufferCollectionFuchsia,
            p_next: std::ptr::null(),
            collection: self.collection.into_low_level(context, bump),
            index: self.index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImportMemoryBufferCollectionFUCHSIA {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            collection: crate::conv::FromLowLevel::from_low_level(context, value.collection),
            index: crate::conv::FromLowLevel::from_low_level(context, value.index),
        }
    }
}
#[doc(alias = "VkBufferCollectionImageCreateInfoFUCHSIA")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferCollectionImageCreateInfoFUCHSIA {
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}
impl BufferCollectionImageCreateInfoFUCHSIA {
    ///Get a reference to the `collection` field.
    pub fn collection(&self) -> &BufferCollectionFUCHSIA {
        &self.collection
    }
    ///Get a reference to the `index` field.
    pub fn index(&self) -> u32 {
        self.index
    }
    ///Get a mutable reference to the `collection` field.
    pub fn collection_mut(&mut self) -> &mut BufferCollectionFUCHSIA {
        &mut self.collection
    }
    ///Get a mutable reference to the `index` field.
    pub fn index_mut(&mut self) -> &mut u32 {
        &mut self.index
    }
    ///Sets the `collection` field.
    pub fn set_collection(&mut self, collection: BufferCollectionFUCHSIA) -> &mut Self {
        self.collection = collection;
        self
    }
    ///Sets the `index` field.
    pub fn set_index(&mut self, index: u32) -> &mut Self {
        self.index = index;
        self
    }
    ///Sets the `collection` field in a builder way.
    pub fn with_collection(mut self, collection: BufferCollectionFUCHSIA) -> Self {
        self.collection = collection;
        self
    }
    ///Sets the `index` field in a builder way.
    pub fn with_index(mut self, index: u32) -> Self {
        self.index = index;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferCollectionImageCreateInfoFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_buffer_collection::BufferCollectionImageCreateInfoFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::fuchsia_buffer_collection::BufferCollectionImageCreateInfoFUCHSIA {
            s_type: StructureType::BufferCollectionImageCreateInfoFuchsia,
            p_next: std::ptr::null(),
            collection: self.collection.into_low_level(context, bump),
            index: self.index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferCollectionImageCreateInfoFUCHSIA {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            collection: crate::conv::FromLowLevel::from_low_level(context, value.collection),
            index: crate::conv::FromLowLevel::from_low_level(context, value.index),
        }
    }
}
#[doc(alias = "VkBufferCollectionBufferCreateInfoFUCHSIA")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferCollectionBufferCreateInfoFUCHSIA {
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}
impl BufferCollectionBufferCreateInfoFUCHSIA {
    ///Get a reference to the `collection` field.
    pub fn collection(&self) -> &BufferCollectionFUCHSIA {
        &self.collection
    }
    ///Get a reference to the `index` field.
    pub fn index(&self) -> u32 {
        self.index
    }
    ///Get a mutable reference to the `collection` field.
    pub fn collection_mut(&mut self) -> &mut BufferCollectionFUCHSIA {
        &mut self.collection
    }
    ///Get a mutable reference to the `index` field.
    pub fn index_mut(&mut self) -> &mut u32 {
        &mut self.index
    }
    ///Sets the `collection` field.
    pub fn set_collection(&mut self, collection: BufferCollectionFUCHSIA) -> &mut Self {
        self.collection = collection;
        self
    }
    ///Sets the `index` field.
    pub fn set_index(&mut self, index: u32) -> &mut Self {
        self.index = index;
        self
    }
    ///Sets the `collection` field in a builder way.
    pub fn with_collection(mut self, collection: BufferCollectionFUCHSIA) -> Self {
        self.collection = collection;
        self
    }
    ///Sets the `index` field in a builder way.
    pub fn with_index(mut self, index: u32) -> Self {
        self.index = index;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferCollectionBufferCreateInfoFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_buffer_collection::BufferCollectionBufferCreateInfoFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::fuchsia_buffer_collection::BufferCollectionBufferCreateInfoFUCHSIA {
            s_type: StructureType::BufferCollectionBufferCreateInfoFuchsia,
            p_next: std::ptr::null(),
            collection: self.collection.into_low_level(context, bump),
            index: self.index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferCollectionBufferCreateInfoFUCHSIA {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            collection: crate::conv::FromLowLevel::from_low_level(context, value.collection),
            index: crate::conv::FromLowLevel::from_low_level(context, value.index),
        }
    }
}
#[doc(alias = "VkBufferCollectionPropertiesFUCHSIA")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferCollectionPropertiesFUCHSIA {
    #[doc(alias = "memoryTypeBits")]
    pub memory_type_bits: u32,
    #[doc(alias = "bufferCount")]
    pub buffer_count: u32,
    #[doc(alias = "createInfoIndex")]
    pub create_info_index: u32,
    #[doc(alias = "sysmemPixelFormat")]
    pub sysmem_pixel_format: u64,
    #[doc(alias = "formatFeatures")]
    pub format_features: FormatFeatureFlags,
    #[doc(alias = "sysmemColorSpaceIndex")]
    pub sysmem_color_space_index: SysmemColorSpaceFUCHSIA,
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
impl BufferCollectionPropertiesFUCHSIA {
    ///Get a reference to the `memory_type_bits` field.
    pub fn memory_type_bits(&self) -> u32 {
        self.memory_type_bits
    }
    ///Get a reference to the `buffer_count` field.
    pub fn buffer_count(&self) -> u32 {
        self.buffer_count
    }
    ///Get a reference to the `create_info_index` field.
    pub fn create_info_index(&self) -> u32 {
        self.create_info_index
    }
    ///Get a reference to the `sysmem_pixel_format` field.
    pub fn sysmem_pixel_format(&self) -> u64 {
        self.sysmem_pixel_format
    }
    ///Get a reference to the `format_features` field.
    pub fn format_features(&self) -> FormatFeatureFlags {
        self.format_features
    }
    ///Get a reference to the `sysmem_color_space_index` field.
    pub fn sysmem_color_space_index(&self) -> &SysmemColorSpaceFUCHSIA {
        &self.sysmem_color_space_index
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
    ///Get a mutable reference to the `memory_type_bits` field.
    pub fn memory_type_bits_mut(&mut self) -> &mut u32 {
        &mut self.memory_type_bits
    }
    ///Get a mutable reference to the `buffer_count` field.
    pub fn buffer_count_mut(&mut self) -> &mut u32 {
        &mut self.buffer_count
    }
    ///Get a mutable reference to the `create_info_index` field.
    pub fn create_info_index_mut(&mut self) -> &mut u32 {
        &mut self.create_info_index
    }
    ///Get a mutable reference to the `sysmem_pixel_format` field.
    pub fn sysmem_pixel_format_mut(&mut self) -> &mut u64 {
        &mut self.sysmem_pixel_format
    }
    ///Get a mutable reference to the `format_features` field.
    pub fn format_features_mut(&mut self) -> &mut FormatFeatureFlags {
        &mut self.format_features
    }
    ///Get a mutable reference to the `sysmem_color_space_index` field.
    pub fn sysmem_color_space_index_mut(&mut self) -> &mut SysmemColorSpaceFUCHSIA {
        &mut self.sysmem_color_space_index
    }
    ///Get a mutable reference to the `sampler_ycbcr_conversion_components` field.
    pub fn sampler_ycbcr_conversion_components_mut(&mut self) -> &mut ComponentMapping {
        &mut self.sampler_ycbcr_conversion_components
    }
    ///Get a mutable reference to the `suggested_ycbcr_model` field.
    pub fn suggested_ycbcr_model_mut(&mut self) -> &mut SamplerYcbcrModelConversion {
        &mut self.suggested_ycbcr_model
    }
    ///Get a mutable reference to the `suggested_ycbcr_range` field.
    pub fn suggested_ycbcr_range_mut(&mut self) -> &mut SamplerYcbcrRange {
        &mut self.suggested_ycbcr_range
    }
    ///Get a mutable reference to the `suggested_x_chroma_offset` field.
    pub fn suggested_x_chroma_offset_mut(&mut self) -> &mut ChromaLocation {
        &mut self.suggested_x_chroma_offset
    }
    ///Get a mutable reference to the `suggested_y_chroma_offset` field.
    pub fn suggested_y_chroma_offset_mut(&mut self) -> &mut ChromaLocation {
        &mut self.suggested_y_chroma_offset
    }
    ///Sets the `memory_type_bits` field.
    pub fn set_memory_type_bits(&mut self, memory_type_bits: u32) -> &mut Self {
        self.memory_type_bits = memory_type_bits;
        self
    }
    ///Sets the `buffer_count` field.
    pub fn set_buffer_count(&mut self, buffer_count: u32) -> &mut Self {
        self.buffer_count = buffer_count;
        self
    }
    ///Sets the `create_info_index` field.
    pub fn set_create_info_index(&mut self, create_info_index: u32) -> &mut Self {
        self.create_info_index = create_info_index;
        self
    }
    ///Sets the `sysmem_pixel_format` field.
    pub fn set_sysmem_pixel_format(&mut self, sysmem_pixel_format: u64) -> &mut Self {
        self.sysmem_pixel_format = sysmem_pixel_format;
        self
    }
    ///Sets the `format_features` field.
    pub fn set_format_features(&mut self, format_features: FormatFeatureFlags) -> &mut Self {
        self.format_features = format_features;
        self
    }
    ///Sets the `sysmem_color_space_index` field.
    pub fn set_sysmem_color_space_index(&mut self, sysmem_color_space_index: SysmemColorSpaceFUCHSIA) -> &mut Self {
        self.sysmem_color_space_index = sysmem_color_space_index;
        self
    }
    ///Sets the `sampler_ycbcr_conversion_components` field.
    pub fn set_sampler_ycbcr_conversion_components(
        &mut self,
        sampler_ycbcr_conversion_components: ComponentMapping,
    ) -> &mut Self {
        self.sampler_ycbcr_conversion_components = sampler_ycbcr_conversion_components;
        self
    }
    ///Sets the `suggested_ycbcr_model` field.
    pub fn set_suggested_ycbcr_model(&mut self, suggested_ycbcr_model: SamplerYcbcrModelConversion) -> &mut Self {
        self.suggested_ycbcr_model = suggested_ycbcr_model;
        self
    }
    ///Sets the `suggested_ycbcr_range` field.
    pub fn set_suggested_ycbcr_range(&mut self, suggested_ycbcr_range: SamplerYcbcrRange) -> &mut Self {
        self.suggested_ycbcr_range = suggested_ycbcr_range;
        self
    }
    ///Sets the `suggested_x_chroma_offset` field.
    pub fn set_suggested_x_chroma_offset(&mut self, suggested_x_chroma_offset: ChromaLocation) -> &mut Self {
        self.suggested_x_chroma_offset = suggested_x_chroma_offset;
        self
    }
    ///Sets the `suggested_y_chroma_offset` field.
    pub fn set_suggested_y_chroma_offset(&mut self, suggested_y_chroma_offset: ChromaLocation) -> &mut Self {
        self.suggested_y_chroma_offset = suggested_y_chroma_offset;
        self
    }
    ///Sets the `memory_type_bits` field in a builder way.
    pub fn with_memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.memory_type_bits = memory_type_bits;
        self
    }
    ///Sets the `buffer_count` field in a builder way.
    pub fn with_buffer_count(mut self, buffer_count: u32) -> Self {
        self.buffer_count = buffer_count;
        self
    }
    ///Sets the `create_info_index` field in a builder way.
    pub fn with_create_info_index(mut self, create_info_index: u32) -> Self {
        self.create_info_index = create_info_index;
        self
    }
    ///Sets the `sysmem_pixel_format` field in a builder way.
    pub fn with_sysmem_pixel_format(mut self, sysmem_pixel_format: u64) -> Self {
        self.sysmem_pixel_format = sysmem_pixel_format;
        self
    }
    ///Sets the `format_features` field in a builder way.
    pub fn with_format_features(mut self, format_features: FormatFeatureFlags) -> Self {
        self.format_features = format_features;
        self
    }
    ///Sets the `sysmem_color_space_index` field in a builder way.
    pub fn with_sysmem_color_space_index(mut self, sysmem_color_space_index: SysmemColorSpaceFUCHSIA) -> Self {
        self.sysmem_color_space_index = sysmem_color_space_index;
        self
    }
    ///Sets the `sampler_ycbcr_conversion_components` field in a builder way.
    pub fn with_sampler_ycbcr_conversion_components(
        mut self,
        sampler_ycbcr_conversion_components: ComponentMapping,
    ) -> Self {
        self.sampler_ycbcr_conversion_components = sampler_ycbcr_conversion_components;
        self
    }
    ///Sets the `suggested_ycbcr_model` field in a builder way.
    pub fn with_suggested_ycbcr_model(mut self, suggested_ycbcr_model: SamplerYcbcrModelConversion) -> Self {
        self.suggested_ycbcr_model = suggested_ycbcr_model;
        self
    }
    ///Sets the `suggested_ycbcr_range` field in a builder way.
    pub fn with_suggested_ycbcr_range(mut self, suggested_ycbcr_range: SamplerYcbcrRange) -> Self {
        self.suggested_ycbcr_range = suggested_ycbcr_range;
        self
    }
    ///Sets the `suggested_x_chroma_offset` field in a builder way.
    pub fn with_suggested_x_chroma_offset(mut self, suggested_x_chroma_offset: ChromaLocation) -> Self {
        self.suggested_x_chroma_offset = suggested_x_chroma_offset;
        self
    }
    ///Sets the `suggested_y_chroma_offset` field in a builder way.
    pub fn with_suggested_y_chroma_offset(mut self, suggested_y_chroma_offset: ChromaLocation) -> Self {
        self.suggested_y_chroma_offset = suggested_y_chroma_offset;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferCollectionPropertiesFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_buffer_collection::BufferCollectionPropertiesFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::fuchsia_buffer_collection::BufferCollectionPropertiesFUCHSIA {
            s_type: StructureType::BufferCollectionPropertiesFuchsia,
            p_next: std::ptr::null_mut(),
            memory_type_bits: self.memory_type_bits.into_low_level(context, bump),
            buffer_count: self.buffer_count.into_low_level(context, bump),
            create_info_index: self.create_info_index.into_low_level(context, bump),
            sysmem_pixel_format: self.sysmem_pixel_format.into_low_level(context, bump),
            format_features: self.format_features.into_low_level(context, bump),
            sysmem_color_space_index: self.sysmem_color_space_index.into_low_level(context, bump),
            sampler_ycbcr_conversion_components: self.sampler_ycbcr_conversion_components.into_low_level(context, bump),
            suggested_ycbcr_model: self.suggested_ycbcr_model.into_low_level(context, bump),
            suggested_ycbcr_range: self.suggested_ycbcr_range.into_low_level(context, bump),
            suggested_x_chroma_offset: self.suggested_x_chroma_offset.into_low_level(context, bump),
            suggested_y_chroma_offset: self.suggested_y_chroma_offset.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferCollectionPropertiesFUCHSIA {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            memory_type_bits: crate::conv::FromLowLevel::from_low_level(context, value.memory_type_bits),
            buffer_count: crate::conv::FromLowLevel::from_low_level(context, value.buffer_count),
            create_info_index: crate::conv::FromLowLevel::from_low_level(context, value.create_info_index),
            sysmem_pixel_format: crate::conv::FromLowLevel::from_low_level(context, value.sysmem_pixel_format),
            format_features: crate::conv::FromLowLevel::from_low_level(context, value.format_features),
            sysmem_color_space_index: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sysmem_color_space_index,
            ),
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
#[doc(alias = "VkBufferConstraintsInfoFUCHSIA")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferConstraintsInfoFUCHSIA {
    #[doc(alias = "createInfo")]
    pub create_info: BufferCreateInfo,
    #[doc(alias = "requiredFormatFeatures")]
    pub required_format_features: FormatFeatureFlags,
    #[doc(alias = "bufferCollectionConstraints")]
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
}
impl BufferConstraintsInfoFUCHSIA {
    ///Get a reference to the `create_info` field.
    pub fn create_info(&self) -> &BufferCreateInfo {
        &self.create_info
    }
    ///Get a reference to the `required_format_features` field.
    pub fn required_format_features(&self) -> FormatFeatureFlags {
        self.required_format_features
    }
    ///Get a reference to the `buffer_collection_constraints` field.
    pub fn buffer_collection_constraints(&self) -> &BufferCollectionConstraintsInfoFUCHSIA {
        &self.buffer_collection_constraints
    }
    ///Get a mutable reference to the `create_info` field.
    pub fn create_info_mut(&mut self) -> &mut BufferCreateInfo {
        &mut self.create_info
    }
    ///Get a mutable reference to the `required_format_features` field.
    pub fn required_format_features_mut(&mut self) -> &mut FormatFeatureFlags {
        &mut self.required_format_features
    }
    ///Get a mutable reference to the `buffer_collection_constraints` field.
    pub fn buffer_collection_constraints_mut(&mut self) -> &mut BufferCollectionConstraintsInfoFUCHSIA {
        &mut self.buffer_collection_constraints
    }
    ///Sets the `create_info` field.
    pub fn set_create_info(&mut self, create_info: BufferCreateInfo) -> &mut Self {
        self.create_info = create_info;
        self
    }
    ///Sets the `required_format_features` field.
    pub fn set_required_format_features(&mut self, required_format_features: FormatFeatureFlags) -> &mut Self {
        self.required_format_features = required_format_features;
        self
    }
    ///Sets the `buffer_collection_constraints` field.
    pub fn set_buffer_collection_constraints(
        &mut self,
        buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
    ) -> &mut Self {
        self.buffer_collection_constraints = buffer_collection_constraints;
        self
    }
    ///Sets the `create_info` field in a builder way.
    pub fn with_create_info(mut self, create_info: BufferCreateInfo) -> Self {
        self.create_info = create_info;
        self
    }
    ///Sets the `required_format_features` field in a builder way.
    pub fn with_required_format_features(mut self, required_format_features: FormatFeatureFlags) -> Self {
        self.required_format_features = required_format_features;
        self
    }
    ///Sets the `buffer_collection_constraints` field in a builder way.
    pub fn with_buffer_collection_constraints(
        mut self,
        buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
    ) -> Self {
        self.buffer_collection_constraints = buffer_collection_constraints;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferConstraintsInfoFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_buffer_collection::BufferConstraintsInfoFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::fuchsia_buffer_collection::BufferConstraintsInfoFUCHSIA {
            s_type: StructureType::BufferConstraintsInfoFuchsia,
            p_next: std::ptr::null(),
            create_info: self.create_info.into_low_level(context, bump),
            required_format_features: self.required_format_features.into_low_level(context, bump),
            buffer_collection_constraints: self.buffer_collection_constraints.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferConstraintsInfoFUCHSIA {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            create_info: crate::conv::FromLowLevel::from_low_level(context, value.create_info),
            required_format_features: crate::conv::FromLowLevel::from_low_level(
                context,
                value.required_format_features,
            ),
            buffer_collection_constraints: crate::conv::FromLowLevel::from_low_level(
                context,
                value.buffer_collection_constraints,
            ),
        }
    }
}
#[doc(alias = "VkSysmemColorSpaceFUCHSIA")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SysmemColorSpaceFUCHSIA {
    #[doc(alias = "colorSpace")]
    pub color_space: u32,
}
impl SysmemColorSpaceFUCHSIA {
    ///Get a reference to the `color_space` field.
    pub fn color_space(&self) -> u32 {
        self.color_space
    }
    ///Get a mutable reference to the `color_space` field.
    pub fn color_space_mut(&mut self) -> &mut u32 {
        &mut self.color_space
    }
    ///Sets the `color_space` field.
    pub fn set_color_space(&mut self, color_space: u32) -> &mut Self {
        self.color_space = color_space;
        self
    }
    ///Sets the `color_space` field in a builder way.
    pub fn with_color_space(mut self, color_space: u32) -> Self {
        self.color_space = color_space;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SysmemColorSpaceFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA {
            s_type: StructureType::SysmemColorSpaceFuchsia,
            p_next: std::ptr::null(),
            color_space: self.color_space.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SysmemColorSpaceFUCHSIA {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            color_space: crate::conv::FromLowLevel::from_low_level(context, value.color_space),
        }
    }
}
#[doc(alias = "VkImageFormatConstraintsInfoFUCHSIA")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageFormatConstraintsInfoFUCHSIA {
    #[doc(alias = "imageCreateInfo")]
    pub image_create_info: ImageCreateInfo,
    #[doc(alias = "requiredFormatFeatures")]
    pub required_format_features: FormatFeatureFlags,
    pub flags: ImageFormatConstraintsFlagsFUCHSIA,
    #[doc(alias = "sysmemPixelFormat")]
    pub sysmem_pixel_format: u64,
    #[doc(alias = "pColorSpaces")]
    pub color_spaces: SmallVec<[SysmemColorSpaceFUCHSIA; 8]>,
}
impl ImageFormatConstraintsInfoFUCHSIA {
    ///Get a reference to the `image_create_info` field.
    pub fn image_create_info(&self) -> &ImageCreateInfo {
        &self.image_create_info
    }
    ///Get a reference to the `required_format_features` field.
    pub fn required_format_features(&self) -> FormatFeatureFlags {
        self.required_format_features
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> ImageFormatConstraintsFlagsFUCHSIA {
        self.flags
    }
    ///Get a reference to the `sysmem_pixel_format` field.
    pub fn sysmem_pixel_format(&self) -> u64 {
        self.sysmem_pixel_format
    }
    ///Get a reference to the `color_spaces` field.
    pub fn color_spaces(&self) -> &SmallVec<[SysmemColorSpaceFUCHSIA; 8]> {
        &self.color_spaces
    }
    ///Get a mutable reference to the `image_create_info` field.
    pub fn image_create_info_mut(&mut self) -> &mut ImageCreateInfo {
        &mut self.image_create_info
    }
    ///Get a mutable reference to the `required_format_features` field.
    pub fn required_format_features_mut(&mut self) -> &mut FormatFeatureFlags {
        &mut self.required_format_features
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut ImageFormatConstraintsFlagsFUCHSIA {
        &mut self.flags
    }
    ///Get a mutable reference to the `sysmem_pixel_format` field.
    pub fn sysmem_pixel_format_mut(&mut self) -> &mut u64 {
        &mut self.sysmem_pixel_format
    }
    ///Get a mutable reference to the `color_spaces` field.
    pub fn color_spaces_mut(&mut self) -> &mut SmallVec<[SysmemColorSpaceFUCHSIA; 8]> {
        &mut self.color_spaces
    }
    ///Sets the `image_create_info` field.
    pub fn set_image_create_info(&mut self, image_create_info: ImageCreateInfo) -> &mut Self {
        self.image_create_info = image_create_info;
        self
    }
    ///Sets the `required_format_features` field.
    pub fn set_required_format_features(&mut self, required_format_features: FormatFeatureFlags) -> &mut Self {
        self.required_format_features = required_format_features;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: ImageFormatConstraintsFlagsFUCHSIA) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `sysmem_pixel_format` field.
    pub fn set_sysmem_pixel_format(&mut self, sysmem_pixel_format: u64) -> &mut Self {
        self.sysmem_pixel_format = sysmem_pixel_format;
        self
    }
    ///Sets the `color_spaces` field.
    pub fn set_color_spaces(&mut self, color_spaces: SmallVec<[SysmemColorSpaceFUCHSIA; 8]>) -> &mut Self {
        self.color_spaces = color_spaces;
        self
    }
    ///Sets the `image_create_info` field in a builder way.
    pub fn with_image_create_info(mut self, image_create_info: ImageCreateInfo) -> Self {
        self.image_create_info = image_create_info;
        self
    }
    ///Sets the `required_format_features` field in a builder way.
    pub fn with_required_format_features(mut self, required_format_features: FormatFeatureFlags) -> Self {
        self.required_format_features = required_format_features;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: ImageFormatConstraintsFlagsFUCHSIA) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `sysmem_pixel_format` field in a builder way.
    pub fn with_sysmem_pixel_format(mut self, sysmem_pixel_format: u64) -> Self {
        self.sysmem_pixel_format = sysmem_pixel_format;
        self
    }
    ///Sets the `color_spaces` field in a builder way.
    pub fn with_color_spaces(mut self, color_spaces: SmallVec<[SysmemColorSpaceFUCHSIA; 8]>) -> Self {
        self.color_spaces = color_spaces;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageFormatConstraintsInfoFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_buffer_collection::ImageFormatConstraintsInfoFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_color_spaces = self.color_spaces.len() as u32;
        let color_spaces = bump
            .alloc_slice_fill_iter(self.color_spaces.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::fuchsia_buffer_collection::ImageFormatConstraintsInfoFUCHSIA {
            s_type: StructureType::ImageFormatConstraintsInfoFuchsia,
            p_next: std::ptr::null(),
            image_create_info: self.image_create_info.into_low_level(context, bump),
            required_format_features: self.required_format_features.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
            sysmem_pixel_format: self.sysmem_pixel_format.into_low_level(context, bump),
            color_space_count: len_color_spaces,
            color_spaces: color_spaces,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageFormatConstraintsInfoFUCHSIA {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let color_spaces_len = value.color_space_count;
        let mut color_spaces = SmallVec::with_capacity(color_spaces_len as usize);
        for i in 0..color_spaces_len {
            color_spaces.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.color_spaces.add(i as usize).read(),
            ));
        }
        Self {
            image_create_info: crate::conv::FromLowLevel::from_low_level(context, value.image_create_info),
            required_format_features: crate::conv::FromLowLevel::from_low_level(
                context,
                value.required_format_features,
            ),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            sysmem_pixel_format: crate::conv::FromLowLevel::from_low_level(context, value.sysmem_pixel_format),
            color_spaces: color_spaces,
        }
    }
}
#[doc(alias = "VkImageConstraintsInfoFUCHSIA")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageConstraintsInfoFUCHSIA {
    #[doc(alias = "pFormatConstraints")]
    pub format_constraints: SmallVec<[ImageFormatConstraintsInfoFUCHSIA; 8]>,
    #[doc(alias = "bufferCollectionConstraints")]
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
    pub flags: ImageConstraintsInfoFlagsFUCHSIA,
}
impl ImageConstraintsInfoFUCHSIA {
    ///Get a reference to the `format_constraints` field.
    pub fn format_constraints(&self) -> &SmallVec<[ImageFormatConstraintsInfoFUCHSIA; 8]> {
        &self.format_constraints
    }
    ///Get a reference to the `buffer_collection_constraints` field.
    pub fn buffer_collection_constraints(&self) -> &BufferCollectionConstraintsInfoFUCHSIA {
        &self.buffer_collection_constraints
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> ImageConstraintsInfoFlagsFUCHSIA {
        self.flags
    }
    ///Get a mutable reference to the `format_constraints` field.
    pub fn format_constraints_mut(&mut self) -> &mut SmallVec<[ImageFormatConstraintsInfoFUCHSIA; 8]> {
        &mut self.format_constraints
    }
    ///Get a mutable reference to the `buffer_collection_constraints` field.
    pub fn buffer_collection_constraints_mut(&mut self) -> &mut BufferCollectionConstraintsInfoFUCHSIA {
        &mut self.buffer_collection_constraints
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut ImageConstraintsInfoFlagsFUCHSIA {
        &mut self.flags
    }
    ///Sets the `format_constraints` field.
    pub fn set_format_constraints(
        &mut self,
        format_constraints: SmallVec<[ImageFormatConstraintsInfoFUCHSIA; 8]>,
    ) -> &mut Self {
        self.format_constraints = format_constraints;
        self
    }
    ///Sets the `buffer_collection_constraints` field.
    pub fn set_buffer_collection_constraints(
        &mut self,
        buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
    ) -> &mut Self {
        self.buffer_collection_constraints = buffer_collection_constraints;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: ImageConstraintsInfoFlagsFUCHSIA) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `format_constraints` field in a builder way.
    pub fn with_format_constraints(
        mut self,
        format_constraints: SmallVec<[ImageFormatConstraintsInfoFUCHSIA; 8]>,
    ) -> Self {
        self.format_constraints = format_constraints;
        self
    }
    ///Sets the `buffer_collection_constraints` field in a builder way.
    pub fn with_buffer_collection_constraints(
        mut self,
        buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
    ) -> Self {
        self.buffer_collection_constraints = buffer_collection_constraints;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: ImageConstraintsInfoFlagsFUCHSIA) -> Self {
        self.flags = flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageConstraintsInfoFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_format_constraints = self.format_constraints.len() as u32;
        let format_constraints = bump
            .alloc_slice_fill_iter(self.format_constraints.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFUCHSIA {
            s_type: StructureType::ImageConstraintsInfoFuchsia,
            p_next: std::ptr::null(),
            format_constraints_count: len_format_constraints,
            format_constraints: format_constraints,
            buffer_collection_constraints: self.buffer_collection_constraints.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageConstraintsInfoFUCHSIA {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let format_constraints_len = value.format_constraints_count;
        let mut format_constraints = SmallVec::with_capacity(format_constraints_len as usize);
        for i in 0..format_constraints_len {
            format_constraints.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.format_constraints.add(i as usize).read(),
            ));
        }
        Self {
            format_constraints: format_constraints,
            buffer_collection_constraints: crate::conv::FromLowLevel::from_low_level(
                context,
                value.buffer_collection_constraints,
            ),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
        }
    }
}
#[doc(alias = "VkBufferCollectionConstraintsInfoFUCHSIA")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferCollectionConstraintsInfoFUCHSIA {
    #[doc(alias = "minBufferCount")]
    pub min_buffer_count: u32,
    #[doc(alias = "maxBufferCount")]
    pub max_buffer_count: u32,
    #[doc(alias = "minBufferCountForCamping")]
    pub min_buffer_count_for_camping: u32,
    #[doc(alias = "minBufferCountForDedicatedSlack")]
    pub min_buffer_count_for_dedicated_slack: u32,
    #[doc(alias = "minBufferCountForSharedSlack")]
    pub min_buffer_count_for_shared_slack: u32,
}
impl BufferCollectionConstraintsInfoFUCHSIA {
    ///Get a reference to the `min_buffer_count` field.
    pub fn min_buffer_count(&self) -> u32 {
        self.min_buffer_count
    }
    ///Get a reference to the `max_buffer_count` field.
    pub fn max_buffer_count(&self) -> u32 {
        self.max_buffer_count
    }
    ///Get a reference to the `min_buffer_count_for_camping` field.
    pub fn min_buffer_count_for_camping(&self) -> u32 {
        self.min_buffer_count_for_camping
    }
    ///Get a reference to the `min_buffer_count_for_dedicated_slack` field.
    pub fn min_buffer_count_for_dedicated_slack(&self) -> u32 {
        self.min_buffer_count_for_dedicated_slack
    }
    ///Get a reference to the `min_buffer_count_for_shared_slack` field.
    pub fn min_buffer_count_for_shared_slack(&self) -> u32 {
        self.min_buffer_count_for_shared_slack
    }
    ///Get a mutable reference to the `min_buffer_count` field.
    pub fn min_buffer_count_mut(&mut self) -> &mut u32 {
        &mut self.min_buffer_count
    }
    ///Get a mutable reference to the `max_buffer_count` field.
    pub fn max_buffer_count_mut(&mut self) -> &mut u32 {
        &mut self.max_buffer_count
    }
    ///Get a mutable reference to the `min_buffer_count_for_camping` field.
    pub fn min_buffer_count_for_camping_mut(&mut self) -> &mut u32 {
        &mut self.min_buffer_count_for_camping
    }
    ///Get a mutable reference to the `min_buffer_count_for_dedicated_slack` field.
    pub fn min_buffer_count_for_dedicated_slack_mut(&mut self) -> &mut u32 {
        &mut self.min_buffer_count_for_dedicated_slack
    }
    ///Get a mutable reference to the `min_buffer_count_for_shared_slack` field.
    pub fn min_buffer_count_for_shared_slack_mut(&mut self) -> &mut u32 {
        &mut self.min_buffer_count_for_shared_slack
    }
    ///Sets the `min_buffer_count` field.
    pub fn set_min_buffer_count(&mut self, min_buffer_count: u32) -> &mut Self {
        self.min_buffer_count = min_buffer_count;
        self
    }
    ///Sets the `max_buffer_count` field.
    pub fn set_max_buffer_count(&mut self, max_buffer_count: u32) -> &mut Self {
        self.max_buffer_count = max_buffer_count;
        self
    }
    ///Sets the `min_buffer_count_for_camping` field.
    pub fn set_min_buffer_count_for_camping(&mut self, min_buffer_count_for_camping: u32) -> &mut Self {
        self.min_buffer_count_for_camping = min_buffer_count_for_camping;
        self
    }
    ///Sets the `min_buffer_count_for_dedicated_slack` field.
    pub fn set_min_buffer_count_for_dedicated_slack(&mut self, min_buffer_count_for_dedicated_slack: u32) -> &mut Self {
        self.min_buffer_count_for_dedicated_slack = min_buffer_count_for_dedicated_slack;
        self
    }
    ///Sets the `min_buffer_count_for_shared_slack` field.
    pub fn set_min_buffer_count_for_shared_slack(&mut self, min_buffer_count_for_shared_slack: u32) -> &mut Self {
        self.min_buffer_count_for_shared_slack = min_buffer_count_for_shared_slack;
        self
    }
    ///Sets the `min_buffer_count` field in a builder way.
    pub fn with_min_buffer_count(mut self, min_buffer_count: u32) -> Self {
        self.min_buffer_count = min_buffer_count;
        self
    }
    ///Sets the `max_buffer_count` field in a builder way.
    pub fn with_max_buffer_count(mut self, max_buffer_count: u32) -> Self {
        self.max_buffer_count = max_buffer_count;
        self
    }
    ///Sets the `min_buffer_count_for_camping` field in a builder way.
    pub fn with_min_buffer_count_for_camping(mut self, min_buffer_count_for_camping: u32) -> Self {
        self.min_buffer_count_for_camping = min_buffer_count_for_camping;
        self
    }
    ///Sets the `min_buffer_count_for_dedicated_slack` field in a builder way.
    pub fn with_min_buffer_count_for_dedicated_slack(mut self, min_buffer_count_for_dedicated_slack: u32) -> Self {
        self.min_buffer_count_for_dedicated_slack = min_buffer_count_for_dedicated_slack;
        self
    }
    ///Sets the `min_buffer_count_for_shared_slack` field in a builder way.
    pub fn with_min_buffer_count_for_shared_slack(mut self, min_buffer_count_for_shared_slack: u32) -> Self {
        self.min_buffer_count_for_shared_slack = min_buffer_count_for_shared_slack;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferCollectionConstraintsInfoFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA {
            s_type: StructureType::BufferCollectionConstraintsInfoFuchsia,
            p_next: std::ptr::null(),
            min_buffer_count: self.min_buffer_count.into_low_level(context, bump),
            max_buffer_count: self.max_buffer_count.into_low_level(context, bump),
            min_buffer_count_for_camping: self.min_buffer_count_for_camping.into_low_level(context, bump),
            min_buffer_count_for_dedicated_slack: self
                .min_buffer_count_for_dedicated_slack
                .into_low_level(context, bump),
            min_buffer_count_for_shared_slack: self.min_buffer_count_for_shared_slack.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferCollectionConstraintsInfoFUCHSIA {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            min_buffer_count: crate::conv::FromLowLevel::from_low_level(context, value.min_buffer_count),
            max_buffer_count: crate::conv::FromLowLevel::from_low_level(context, value.max_buffer_count),
            min_buffer_count_for_camping: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_buffer_count_for_camping,
            ),
            min_buffer_count_for_dedicated_slack: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_buffer_count_for_dedicated_slack,
            ),
            min_buffer_count_for_shared_slack: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_buffer_count_for_shared_slack,
            ),
        }
    }
}
#[doc(alias = "VkBufferCollectionFUCHSIA")]
#[derive(Debug)]
pub struct BufferCollectionFUCHSIA {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for BufferCollectionFUCHSIA {
    fn clone(&self) -> Self {
        self.context.clone_buffer_collection_fuchsia(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BufferCollectionFUCHSIA {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for BufferCollectionFUCHSIA {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = ObjectId::deserialize(deserializer)?;
        crate::context::CONTEXT.with(|context| {
            let borrow = context.borrow();
            let context = borrow.as_ref().expect("Context not set.");
            Ok(Self {
                context: Arc::clone(context),
                id,
            })
        })
    }
}
impl Drop for BufferCollectionFUCHSIA {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_buffer_collection_fuchsia(&self.id);
        }
    }
}
impl PartialEq for BufferCollectionFUCHSIA {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl BufferCollectionFUCHSIA {
    ///Creates a new instance of this handle from its core components.
    pub(crate) const unsafe fn new(context: Arc<Context>, id: ObjectId) -> Self {
        Self { context, id }
    }
    ///Gets the object id
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
    ///Gets a reference to the context
    pub fn context(&self) -> &Context {
        &self.context
    }
    ///Gets a reference to the context wrapped in an [`Arc`]
    pub fn arc_context(&self) -> &Arc<Context> {
        &self.context
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferCollectionFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .buffer_collection_fuchsia()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferCollectionFUCHSIA {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context
            .buffer_collection_fuchsia()
            .insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
