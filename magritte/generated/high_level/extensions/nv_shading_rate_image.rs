pub use crate::common::extensions::nv_shading_rate_image::{
    CoarseSampleLocationNV, CoarseSampleOrderTypeNV, ShadingRatePaletteEntryNV, NV_SHADING_RATE_IMAGE_EXTENSION_NAME,
    NV_SHADING_RATE_IMAGE_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Extent2D, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkShadingRatePaletteNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ShadingRatePaletteNV {
    #[doc(alias = "pShadingRatePaletteEntries")]
    pub shading_rate_palette_entries: SmallVec<[ShadingRatePaletteEntryNV; 8]>,
}
impl ShadingRatePaletteNV {
    ///Get a reference to the `shading_rate_palette_entries` field.
    pub fn shading_rate_palette_entries(&self) -> &SmallVec<[ShadingRatePaletteEntryNV; 8]> {
        &self.shading_rate_palette_entries
    }
    ///Get a mutable reference to the `shading_rate_palette_entries` field.
    pub fn shading_rate_palette_entries_mut(&mut self) -> &mut SmallVec<[ShadingRatePaletteEntryNV; 8]> {
        &mut self.shading_rate_palette_entries
    }
    ///Sets the `shading_rate_palette_entries` field.
    pub fn set_shading_rate_palette_entries(
        &mut self,
        shading_rate_palette_entries: SmallVec<[ShadingRatePaletteEntryNV; 8]>,
    ) -> &mut Self {
        self.shading_rate_palette_entries = shading_rate_palette_entries;
        self
    }
    ///Sets the `shading_rate_palette_entries` field in a builder way.
    pub fn with_shading_rate_palette_entries(
        mut self,
        shading_rate_palette_entries: SmallVec<[ShadingRatePaletteEntryNV; 8]>,
    ) -> Self {
        self.shading_rate_palette_entries = shading_rate_palette_entries;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ShadingRatePaletteNV {
    type LowLevel = crate::native::extensions::nv_shading_rate_image::ShadingRatePaletteNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_shading_rate_palette_entries = self.shading_rate_palette_entries.len() as u32;
        let shading_rate_palette_entries = bump
            .alloc_slice_fill_iter(
                self.shading_rate_palette_entries
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::nv_shading_rate_image::ShadingRatePaletteNV {
            shading_rate_palette_entry_count: len_shading_rate_palette_entries,
            shading_rate_palette_entries: shading_rate_palette_entries,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ShadingRatePaletteNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let shading_rate_palette_entries_len = value.shading_rate_palette_entry_count;
        let mut shading_rate_palette_entries = SmallVec::with_capacity(shading_rate_palette_entries_len as usize);
        for i in 0..shading_rate_palette_entries_len {
            shading_rate_palette_entries.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.shading_rate_palette_entries.add(i as usize).read(),
            ));
        }
        Self {
            shading_rate_palette_entries: shading_rate_palette_entries,
        }
    }
}
#[doc(alias = "VkPipelineViewportShadingRateImageStateCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineViewportShadingRateImageStateCreateInfoNV {
    #[doc(alias = "shadingRateImageEnable")]
    pub shading_rate_image_enable: bool,
    #[doc(alias = "pShadingRatePalettes")]
    pub shading_rate_palettes: SmallVec<[ShadingRatePaletteNV; 8]>,
}
impl PipelineViewportShadingRateImageStateCreateInfoNV {
    ///Get a reference to the `shading_rate_image_enable` field.
    pub fn shading_rate_image_enable(&self) -> &bool {
        &self.shading_rate_image_enable
    }
    ///Get a reference to the `shading_rate_palettes` field.
    pub fn shading_rate_palettes(&self) -> &SmallVec<[ShadingRatePaletteNV; 8]> {
        &self.shading_rate_palettes
    }
    ///Get a mutable reference to the `shading_rate_image_enable` field.
    pub fn shading_rate_image_enable_mut(&mut self) -> &mut bool {
        &mut self.shading_rate_image_enable
    }
    ///Get a mutable reference to the `shading_rate_palettes` field.
    pub fn shading_rate_palettes_mut(&mut self) -> &mut SmallVec<[ShadingRatePaletteNV; 8]> {
        &mut self.shading_rate_palettes
    }
    ///Sets the `shading_rate_image_enable` field.
    pub fn set_shading_rate_image_enable(&mut self, shading_rate_image_enable: bool) -> &mut Self {
        self.shading_rate_image_enable = shading_rate_image_enable;
        self
    }
    ///Sets the `shading_rate_palettes` field.
    pub fn set_shading_rate_palettes(
        &mut self,
        shading_rate_palettes: SmallVec<[ShadingRatePaletteNV; 8]>,
    ) -> &mut Self {
        self.shading_rate_palettes = shading_rate_palettes;
        self
    }
    ///Sets the `shading_rate_image_enable` field in a builder way.
    pub fn with_shading_rate_image_enable(mut self, shading_rate_image_enable: bool) -> Self {
        self.shading_rate_image_enable = shading_rate_image_enable;
        self
    }
    ///Sets the `shading_rate_palettes` field in a builder way.
    pub fn with_shading_rate_palettes(mut self, shading_rate_palettes: SmallVec<[ShadingRatePaletteNV; 8]>) -> Self {
        self.shading_rate_palettes = shading_rate_palettes;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineViewportShadingRateImageStateCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_shading_rate_image::PipelineViewportShadingRateImageStateCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_shading_rate_palettes = self.shading_rate_palettes.len() as u32;
        let shading_rate_palettes = bump
            .alloc_slice_fill_iter(
                self.shading_rate_palettes
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::nv_shading_rate_image::PipelineViewportShadingRateImageStateCreateInfoNV {
            s_type: StructureType::PipelineViewportShadingRateImageStateCreateInfoNv,
            p_next: std::ptr::null(),
            shading_rate_image_enable: self.shading_rate_image_enable.into_low_level(context, bump),
            viewport_count: len_shading_rate_palettes,
            shading_rate_palettes: shading_rate_palettes,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineViewportShadingRateImageStateCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let shading_rate_palettes_len = value.viewport_count;
        let mut shading_rate_palettes = SmallVec::with_capacity(shading_rate_palettes_len as usize);
        for i in 0..shading_rate_palettes_len {
            shading_rate_palettes.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.shading_rate_palettes.add(i as usize).read(),
            ));
        }
        Self {
            shading_rate_image_enable: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shading_rate_image_enable,
            ),
            shading_rate_palettes: shading_rate_palettes,
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShadingRateImageFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShadingRateImageFeaturesNV {
    #[doc(alias = "shadingRateImage")]
    pub shading_rate_image: bool,
    #[doc(alias = "shadingRateCoarseSampleOrder")]
    pub shading_rate_coarse_sample_order: bool,
}
impl PhysicalDeviceShadingRateImageFeaturesNV {
    ///Get a reference to the `shading_rate_image` field.
    pub fn shading_rate_image(&self) -> &bool {
        &self.shading_rate_image
    }
    ///Get a reference to the `shading_rate_coarse_sample_order` field.
    pub fn shading_rate_coarse_sample_order(&self) -> &bool {
        &self.shading_rate_coarse_sample_order
    }
    ///Get a mutable reference to the `shading_rate_image` field.
    pub fn shading_rate_image_mut(&mut self) -> &mut bool {
        &mut self.shading_rate_image
    }
    ///Get a mutable reference to the `shading_rate_coarse_sample_order` field.
    pub fn shading_rate_coarse_sample_order_mut(&mut self) -> &mut bool {
        &mut self.shading_rate_coarse_sample_order
    }
    ///Sets the `shading_rate_image` field.
    pub fn set_shading_rate_image(&mut self, shading_rate_image: bool) -> &mut Self {
        self.shading_rate_image = shading_rate_image;
        self
    }
    ///Sets the `shading_rate_coarse_sample_order` field.
    pub fn set_shading_rate_coarse_sample_order(&mut self, shading_rate_coarse_sample_order: bool) -> &mut Self {
        self.shading_rate_coarse_sample_order = shading_rate_coarse_sample_order;
        self
    }
    ///Sets the `shading_rate_image` field in a builder way.
    pub fn with_shading_rate_image(mut self, shading_rate_image: bool) -> Self {
        self.shading_rate_image = shading_rate_image;
        self
    }
    ///Sets the `shading_rate_coarse_sample_order` field in a builder way.
    pub fn with_shading_rate_coarse_sample_order(mut self, shading_rate_coarse_sample_order: bool) -> Self {
        self.shading_rate_coarse_sample_order = shading_rate_coarse_sample_order;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShadingRateImageFeaturesNV {
    type LowLevel = crate::native::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImageFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImageFeaturesNV {
            s_type: StructureType::PhysicalDeviceShadingRateImageFeaturesNv,
            p_next: std::ptr::null_mut(),
            shading_rate_image: self.shading_rate_image.into_low_level(context, bump),
            shading_rate_coarse_sample_order: self.shading_rate_coarse_sample_order.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShadingRateImageFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shading_rate_image: crate::conv::FromLowLevel::from_low_level(context, value.shading_rate_image),
            shading_rate_coarse_sample_order: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shading_rate_coarse_sample_order,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShadingRateImagePropertiesNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShadingRateImagePropertiesNV {
    #[doc(alias = "shadingRateTexelSize")]
    pub shading_rate_texel_size: Extent2D,
    #[doc(alias = "shadingRatePaletteSize")]
    pub shading_rate_palette_size: u32,
    #[doc(alias = "shadingRateMaxCoarseSamples")]
    pub shading_rate_max_coarse_samples: u32,
}
impl PhysicalDeviceShadingRateImagePropertiesNV {
    ///Get a reference to the `shading_rate_texel_size` field.
    pub fn shading_rate_texel_size(&self) -> Extent2D {
        self.shading_rate_texel_size
    }
    ///Get a reference to the `shading_rate_palette_size` field.
    pub fn shading_rate_palette_size(&self) -> u32 {
        self.shading_rate_palette_size
    }
    ///Get a reference to the `shading_rate_max_coarse_samples` field.
    pub fn shading_rate_max_coarse_samples(&self) -> u32 {
        self.shading_rate_max_coarse_samples
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShadingRateImagePropertiesNV {
    type LowLevel = crate::native::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImagePropertiesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImagePropertiesNV {
            s_type: StructureType::PhysicalDeviceShadingRateImagePropertiesNv,
            p_next: std::ptr::null_mut(),
            shading_rate_texel_size: self.shading_rate_texel_size.into_low_level(context, bump),
            shading_rate_palette_size: self.shading_rate_palette_size.into_low_level(context, bump),
            shading_rate_max_coarse_samples: self.shading_rate_max_coarse_samples.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShadingRateImagePropertiesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shading_rate_texel_size: crate::conv::FromLowLevel::from_low_level(context, value.shading_rate_texel_size),
            shading_rate_palette_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shading_rate_palette_size,
            ),
            shading_rate_max_coarse_samples: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shading_rate_max_coarse_samples,
            ),
        }
    }
}
impl CoarseSampleLocationNV {
    ///Get a reference to the `pixel_x` field.
    pub fn pixel_x(&self) -> u32 {
        self.pixel_x
    }
    ///Get a reference to the `pixel_y` field.
    pub fn pixel_y(&self) -> u32 {
        self.pixel_y
    }
    ///Get a reference to the `sample` field.
    pub fn sample(&self) -> u32 {
        self.sample
    }
    ///Get a mutable reference to the `pixel_x` field.
    pub fn pixel_x_mut(&mut self) -> &mut u32 {
        &mut self.pixel_x
    }
    ///Get a mutable reference to the `pixel_y` field.
    pub fn pixel_y_mut(&mut self) -> &mut u32 {
        &mut self.pixel_y
    }
    ///Get a mutable reference to the `sample` field.
    pub fn sample_mut(&mut self) -> &mut u32 {
        &mut self.sample
    }
    ///Sets the `pixel_x` field.
    pub fn set_pixel_x(&mut self, pixel_x: u32) -> &mut Self {
        self.pixel_x = pixel_x;
        self
    }
    ///Sets the `pixel_y` field.
    pub fn set_pixel_y(&mut self, pixel_y: u32) -> &mut Self {
        self.pixel_y = pixel_y;
        self
    }
    ///Sets the `sample` field.
    pub fn set_sample(&mut self, sample: u32) -> &mut Self {
        self.sample = sample;
        self
    }
    ///Sets the `pixel_x` field in a builder way.
    pub fn with_pixel_x(mut self, pixel_x: u32) -> Self {
        self.pixel_x = pixel_x;
        self
    }
    ///Sets the `pixel_y` field in a builder way.
    pub fn with_pixel_y(mut self, pixel_y: u32) -> Self {
        self.pixel_y = pixel_y;
        self
    }
    ///Sets the `sample` field in a builder way.
    pub fn with_sample(mut self, sample: u32) -> Self {
        self.sample = sample;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CoarseSampleLocationNV {
    type LowLevel = crate::native::extensions::nv_shading_rate_image::CoarseSampleLocationNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_shading_rate_image::CoarseSampleLocationNV {
            pixel_x: self.pixel_x.into_low_level(context, bump),
            pixel_y: self.pixel_y.into_low_level(context, bump),
            sample: self.sample.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CoarseSampleLocationNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            pixel_x: crate::conv::FromLowLevel::from_low_level(context, value.pixel_x),
            pixel_y: crate::conv::FromLowLevel::from_low_level(context, value.pixel_y),
            sample: crate::conv::FromLowLevel::from_low_level(context, value.sample),
        }
    }
}
#[doc(alias = "VkCoarseSampleOrderCustomNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CoarseSampleOrderCustomNV {
    #[doc(alias = "shadingRate")]
    pub shading_rate: ShadingRatePaletteEntryNV,
    #[doc(alias = "sampleCount")]
    pub sample_count: u32,
    #[doc(alias = "pSampleLocations")]
    pub sample_locations: SmallVec<[CoarseSampleLocationNV; 8]>,
}
impl CoarseSampleOrderCustomNV {
    ///Get a reference to the `shading_rate` field.
    pub fn shading_rate(&self) -> ShadingRatePaletteEntryNV {
        self.shading_rate
    }
    ///Get a reference to the `sample_count` field.
    pub fn sample_count(&self) -> u32 {
        self.sample_count
    }
    ///Get a reference to the `sample_locations` field.
    pub fn sample_locations(&self) -> &SmallVec<[CoarseSampleLocationNV; 8]> {
        &self.sample_locations
    }
    ///Get a mutable reference to the `shading_rate` field.
    pub fn shading_rate_mut(&mut self) -> &mut ShadingRatePaletteEntryNV {
        &mut self.shading_rate
    }
    ///Get a mutable reference to the `sample_count` field.
    pub fn sample_count_mut(&mut self) -> &mut u32 {
        &mut self.sample_count
    }
    ///Get a mutable reference to the `sample_locations` field.
    pub fn sample_locations_mut(&mut self) -> &mut SmallVec<[CoarseSampleLocationNV; 8]> {
        &mut self.sample_locations
    }
    ///Sets the `shading_rate` field.
    pub fn set_shading_rate(&mut self, shading_rate: ShadingRatePaletteEntryNV) -> &mut Self {
        self.shading_rate = shading_rate;
        self
    }
    ///Sets the `sample_count` field.
    pub fn set_sample_count(&mut self, sample_count: u32) -> &mut Self {
        self.sample_count = sample_count;
        self
    }
    ///Sets the `sample_locations` field.
    pub fn set_sample_locations(&mut self, sample_locations: SmallVec<[CoarseSampleLocationNV; 8]>) -> &mut Self {
        self.sample_locations = sample_locations;
        self
    }
    ///Sets the `shading_rate` field in a builder way.
    pub fn with_shading_rate(mut self, shading_rate: ShadingRatePaletteEntryNV) -> Self {
        self.shading_rate = shading_rate;
        self
    }
    ///Sets the `sample_count` field in a builder way.
    pub fn with_sample_count(mut self, sample_count: u32) -> Self {
        self.sample_count = sample_count;
        self
    }
    ///Sets the `sample_locations` field in a builder way.
    pub fn with_sample_locations(mut self, sample_locations: SmallVec<[CoarseSampleLocationNV; 8]>) -> Self {
        self.sample_locations = sample_locations;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CoarseSampleOrderCustomNV {
    type LowLevel = crate::native::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_sample_locations = self.sample_locations.len() as u32;
        let sample_locations = bump
            .alloc_slice_fill_iter(self.sample_locations.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNV {
            shading_rate: self.shading_rate.into_low_level(context, bump),
            sample_count: self.sample_count.into_low_level(context, bump),
            sample_location_count: len_sample_locations,
            sample_locations: sample_locations,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CoarseSampleOrderCustomNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let sample_locations_len = value.sample_location_count;
        let mut sample_locations = SmallVec::with_capacity(sample_locations_len as usize);
        for i in 0..sample_locations_len {
            sample_locations.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.sample_locations.add(i as usize).read(),
            ));
        }
        Self {
            shading_rate: crate::conv::FromLowLevel::from_low_level(context, value.shading_rate),
            sample_count: crate::conv::FromLowLevel::from_low_level(context, value.sample_count),
            sample_locations: sample_locations,
        }
    }
}
#[doc(alias = "VkPipelineViewportCoarseSampleOrderStateCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    #[doc(alias = "sampleOrderType")]
    pub sample_order_type: CoarseSampleOrderTypeNV,
    #[doc(alias = "pCustomSampleOrders")]
    pub custom_sample_orders: SmallVec<[CoarseSampleOrderCustomNV; 8]>,
}
impl PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    ///Get a reference to the `sample_order_type` field.
    pub fn sample_order_type(&self) -> CoarseSampleOrderTypeNV {
        self.sample_order_type
    }
    ///Get a reference to the `custom_sample_orders` field.
    pub fn custom_sample_orders(&self) -> &SmallVec<[CoarseSampleOrderCustomNV; 8]> {
        &self.custom_sample_orders
    }
    ///Get a mutable reference to the `sample_order_type` field.
    pub fn sample_order_type_mut(&mut self) -> &mut CoarseSampleOrderTypeNV {
        &mut self.sample_order_type
    }
    ///Get a mutable reference to the `custom_sample_orders` field.
    pub fn custom_sample_orders_mut(&mut self) -> &mut SmallVec<[CoarseSampleOrderCustomNV; 8]> {
        &mut self.custom_sample_orders
    }
    ///Sets the `sample_order_type` field.
    pub fn set_sample_order_type(&mut self, sample_order_type: CoarseSampleOrderTypeNV) -> &mut Self {
        self.sample_order_type = sample_order_type;
        self
    }
    ///Sets the `custom_sample_orders` field.
    pub fn set_custom_sample_orders(
        &mut self,
        custom_sample_orders: SmallVec<[CoarseSampleOrderCustomNV; 8]>,
    ) -> &mut Self {
        self.custom_sample_orders = custom_sample_orders;
        self
    }
    ///Sets the `sample_order_type` field in a builder way.
    pub fn with_sample_order_type(mut self, sample_order_type: CoarseSampleOrderTypeNV) -> Self {
        self.sample_order_type = sample_order_type;
        self
    }
    ///Sets the `custom_sample_orders` field in a builder way.
    pub fn with_custom_sample_orders(mut self, custom_sample_orders: SmallVec<[CoarseSampleOrderCustomNV; 8]>) -> Self {
        self.custom_sample_orders = custom_sample_orders;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    type LowLevel =
        crate::native::extensions::nv_shading_rate_image::PipelineViewportCoarseSampleOrderStateCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_custom_sample_orders = self.custom_sample_orders.len() as u32;
        let custom_sample_orders = bump
            .alloc_slice_fill_iter(
                self.custom_sample_orders
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::nv_shading_rate_image::PipelineViewportCoarseSampleOrderStateCreateInfoNV {
            s_type: StructureType::PipelineViewportCoarseSampleOrderStateCreateInfoNv,
            p_next: std::ptr::null(),
            sample_order_type: self.sample_order_type.into_low_level(context, bump),
            custom_sample_order_count: len_custom_sample_orders,
            custom_sample_orders: custom_sample_orders,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let custom_sample_orders_len = value.custom_sample_order_count;
        let mut custom_sample_orders = SmallVec::with_capacity(custom_sample_orders_len as usize);
        for i in 0..custom_sample_orders_len {
            custom_sample_orders.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.custom_sample_orders.add(i as usize).read(),
            ));
        }
        Self {
            sample_order_type: crate::conv::FromLowLevel::from_low_level(context, value.sample_order_type),
            custom_sample_orders: custom_sample_orders,
        }
    }
}
