pub use crate::common::extensions::khr_incremental_present::{
    RectLayerKHR, KHR_INCREMENTAL_PRESENT_EXTENSION_NAME, KHR_INCREMENTAL_PRESENT_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Extent2D, Offset2D, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPresentRegionsKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PresentRegionsKHR {
    #[doc(alias = "pRegions")]
    pub regions: SmallVec<[PresentRegionKHR; 8]>,
}
impl PresentRegionsKHR {
    ///Get a reference to the `regions` field.
    pub fn regions(&self) -> &SmallVec<[PresentRegionKHR; 8]> {
        &self.regions
    }
    ///Get a mutable reference to the `regions` field.
    pub fn regions_mut(&mut self) -> &mut SmallVec<[PresentRegionKHR; 8]> {
        &mut self.regions
    }
    ///Sets the `regions` field.
    pub fn set_regions(&mut self, regions: SmallVec<[PresentRegionKHR; 8]>) -> &mut Self {
        self.regions = regions;
        self
    }
    ///Sets the `regions` field in a builder way.
    pub fn with_regions(mut self, regions: SmallVec<[PresentRegionKHR; 8]>) -> Self {
        self.regions = regions;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PresentRegionsKHR {
    type LowLevel = crate::native::extensions::khr_incremental_present::PresentRegionsKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_regions = self.regions.len() as u32;
        let regions = bump
            .alloc_slice_fill_iter(self.regions.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::khr_incremental_present::PresentRegionsKHR {
            s_type: StructureType::PresentRegionsKhr,
            p_next: std::ptr::null(),
            swapchain_count: len_regions,
            regions: regions,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PresentRegionsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let regions_len = value.swapchain_count;
        let mut regions = SmallVec::with_capacity(regions_len as usize);
        for i in 0..regions_len {
            regions.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.regions.add(i as usize).read(),
            ));
        }
        Self { regions: regions }
    }
}
#[doc(alias = "VkPresentRegionKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PresentRegionKHR {
    #[doc(alias = "pRectangles")]
    pub rectangles: SmallVec<[RectLayerKHR; 8]>,
}
impl PresentRegionKHR {
    ///Get a reference to the `rectangles` field.
    pub fn rectangles(&self) -> &SmallVec<[RectLayerKHR; 8]> {
        &self.rectangles
    }
    ///Get a mutable reference to the `rectangles` field.
    pub fn rectangles_mut(&mut self) -> &mut SmallVec<[RectLayerKHR; 8]> {
        &mut self.rectangles
    }
    ///Sets the `rectangles` field.
    pub fn set_rectangles(&mut self, rectangles: SmallVec<[RectLayerKHR; 8]>) -> &mut Self {
        self.rectangles = rectangles;
        self
    }
    ///Sets the `rectangles` field in a builder way.
    pub fn with_rectangles(mut self, rectangles: SmallVec<[RectLayerKHR; 8]>) -> Self {
        self.rectangles = rectangles;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PresentRegionKHR {
    type LowLevel = crate::native::extensions::khr_incremental_present::PresentRegionKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_rectangles = self.rectangles.len() as u32;
        let rectangles = bump
            .alloc_slice_fill_iter(self.rectangles.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::khr_incremental_present::PresentRegionKHR {
            rectangle_count: len_rectangles,
            rectangles: rectangles,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PresentRegionKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let rectangles_len = value.rectangle_count;
        let mut rectangles = SmallVec::with_capacity(rectangles_len as usize);
        for i in 0..rectangles_len {
            rectangles.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.rectangles.add(i as usize).read(),
            ));
        }
        Self { rectangles: rectangles }
    }
}
impl RectLayerKHR {
    ///Get a reference to the `offset` field.
    pub fn offset(&self) -> Offset2D {
        self.offset
    }
    ///Get a reference to the `extent` field.
    pub fn extent(&self) -> Extent2D {
        self.extent
    }
    ///Get a reference to the `layer` field.
    pub fn layer(&self) -> u32 {
        self.layer
    }
    ///Get a mutable reference to the `offset` field.
    pub fn offset_mut(&mut self) -> &mut Offset2D {
        &mut self.offset
    }
    ///Get a mutable reference to the `extent` field.
    pub fn extent_mut(&mut self) -> &mut Extent2D {
        &mut self.extent
    }
    ///Get a mutable reference to the `layer` field.
    pub fn layer_mut(&mut self) -> &mut u32 {
        &mut self.layer
    }
    ///Sets the `offset` field.
    pub fn set_offset(&mut self, offset: Offset2D) -> &mut Self {
        self.offset = offset;
        self
    }
    ///Sets the `extent` field.
    pub fn set_extent(&mut self, extent: Extent2D) -> &mut Self {
        self.extent = extent;
        self
    }
    ///Sets the `layer` field.
    pub fn set_layer(&mut self, layer: u32) -> &mut Self {
        self.layer = layer;
        self
    }
    ///Sets the `offset` field in a builder way.
    pub fn with_offset(mut self, offset: Offset2D) -> Self {
        self.offset = offset;
        self
    }
    ///Sets the `extent` field in a builder way.
    pub fn with_extent(mut self, extent: Extent2D) -> Self {
        self.extent = extent;
        self
    }
    ///Sets the `layer` field in a builder way.
    pub fn with_layer(mut self, layer: u32) -> Self {
        self.layer = layer;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RectLayerKHR {
    type LowLevel = crate::native::extensions::khr_incremental_present::RectLayerKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_incremental_present::RectLayerKHR {
            offset: self.offset.into_low_level(context, bump),
            extent: self.extent.into_low_level(context, bump),
            layer: self.layer.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RectLayerKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            offset: crate::conv::FromLowLevel::from_low_level(context, value.offset),
            extent: crate::conv::FromLowLevel::from_low_level(context, value.extent),
            layer: crate::conv::FromLowLevel::from_low_level(context, value.layer),
        }
    }
}
