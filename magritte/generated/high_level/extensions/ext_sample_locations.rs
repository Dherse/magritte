pub use crate::common::extensions::ext_sample_locations::{
    SampleLocationEXT, EXT_SAMPLE_LOCATIONS_EXTENSION_NAME, EXT_SAMPLE_LOCATIONS_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Extent2D, SampleCountFlagBits, SampleCountFlags, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
impl SampleLocationEXT {
    ///Get a reference to the `x` field.
    pub fn x(&self) -> f32 {
        self.x
    }
    ///Get a reference to the `y` field.
    pub fn y(&self) -> f32 {
        self.y
    }
    ///Get a mutable reference to the `x` field.
    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    ///Get a mutable reference to the `y` field.
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    ///Sets the `x` field.
    pub fn set_x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }
    ///Sets the `y` field.
    pub fn set_y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }
    ///Sets the `x` field in a builder way.
    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }
    ///Sets the `y` field in a builder way.
    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SampleLocationEXT {
    type LowLevel = crate::native::extensions::ext_sample_locations::SampleLocationEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_sample_locations::SampleLocationEXT {
            x: self.x.into_low_level(context, bump),
            y: self.y.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SampleLocationEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            x: crate::conv::FromLowLevel::from_low_level(context, value.x),
            y: crate::conv::FromLowLevel::from_low_level(context, value.y),
        }
    }
}
#[doc(alias = "VkSampleLocationsInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SampleLocationsInfoEXT {
    #[doc(alias = "sampleLocationsPerPixel")]
    pub sample_locations_per_pixel: SampleCountFlagBits,
    #[doc(alias = "sampleLocationGridSize")]
    pub sample_location_grid_size: Extent2D,
    #[doc(alias = "pSampleLocations")]
    pub sample_locations: SmallVec<[SampleLocationEXT; 8]>,
}
impl SampleLocationsInfoEXT {
    ///Get a reference to the `sample_locations_per_pixel` field.
    pub fn sample_locations_per_pixel(&self) -> SampleCountFlagBits {
        self.sample_locations_per_pixel
    }
    ///Get a reference to the `sample_location_grid_size` field.
    pub fn sample_location_grid_size(&self) -> Extent2D {
        self.sample_location_grid_size
    }
    ///Get a reference to the `sample_locations` field.
    pub fn sample_locations(&self) -> &SmallVec<[SampleLocationEXT; 8]> {
        &self.sample_locations
    }
    ///Get a mutable reference to the `sample_locations_per_pixel` field.
    pub fn sample_locations_per_pixel_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.sample_locations_per_pixel
    }
    ///Get a mutable reference to the `sample_location_grid_size` field.
    pub fn sample_location_grid_size_mut(&mut self) -> &mut Extent2D {
        &mut self.sample_location_grid_size
    }
    ///Get a mutable reference to the `sample_locations` field.
    pub fn sample_locations_mut(&mut self) -> &mut SmallVec<[SampleLocationEXT; 8]> {
        &mut self.sample_locations
    }
    ///Sets the `sample_locations_per_pixel` field.
    pub fn set_sample_locations_per_pixel(&mut self, sample_locations_per_pixel: SampleCountFlagBits) -> &mut Self {
        self.sample_locations_per_pixel = sample_locations_per_pixel;
        self
    }
    ///Sets the `sample_location_grid_size` field.
    pub fn set_sample_location_grid_size(&mut self, sample_location_grid_size: Extent2D) -> &mut Self {
        self.sample_location_grid_size = sample_location_grid_size;
        self
    }
    ///Sets the `sample_locations` field.
    pub fn set_sample_locations(&mut self, sample_locations: SmallVec<[SampleLocationEXT; 8]>) -> &mut Self {
        self.sample_locations = sample_locations;
        self
    }
    ///Sets the `sample_locations_per_pixel` field in a builder way.
    pub fn with_sample_locations_per_pixel(mut self, sample_locations_per_pixel: SampleCountFlagBits) -> Self {
        self.sample_locations_per_pixel = sample_locations_per_pixel;
        self
    }
    ///Sets the `sample_location_grid_size` field in a builder way.
    pub fn with_sample_location_grid_size(mut self, sample_location_grid_size: Extent2D) -> Self {
        self.sample_location_grid_size = sample_location_grid_size;
        self
    }
    ///Sets the `sample_locations` field in a builder way.
    pub fn with_sample_locations(mut self, sample_locations: SmallVec<[SampleLocationEXT; 8]>) -> Self {
        self.sample_locations = sample_locations;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SampleLocationsInfoEXT {
    type LowLevel = crate::native::extensions::ext_sample_locations::SampleLocationsInfoEXT;
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
        crate::native::extensions::ext_sample_locations::SampleLocationsInfoEXT {
            s_type: StructureType::SampleLocationsInfoExt,
            p_next: std::ptr::null(),
            sample_locations_per_pixel: self.sample_locations_per_pixel.into_low_level(context, bump),
            sample_location_grid_size: self.sample_location_grid_size.into_low_level(context, bump),
            sample_locations_count: len_sample_locations,
            sample_locations: sample_locations,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SampleLocationsInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let sample_locations_len = value.sample_locations_count;
        let mut sample_locations = SmallVec::with_capacity(sample_locations_len as usize);
        for i in 0..sample_locations_len {
            sample_locations.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.sample_locations.add(i as usize).read(),
            ));
        }
        Self {
            sample_locations_per_pixel: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sample_locations_per_pixel,
            ),
            sample_location_grid_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sample_location_grid_size,
            ),
            sample_locations: sample_locations,
        }
    }
}
#[doc(alias = "VkAttachmentSampleLocationsEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttachmentSampleLocationsEXT {
    #[doc(alias = "attachmentIndex")]
    pub attachment_index: u32,
    #[doc(alias = "sampleLocationsInfo")]
    pub sample_locations_info: SampleLocationsInfoEXT,
}
impl AttachmentSampleLocationsEXT {
    ///Get a reference to the `attachment_index` field.
    pub fn attachment_index(&self) -> u32 {
        self.attachment_index
    }
    ///Get a reference to the `sample_locations_info` field.
    pub fn sample_locations_info(&self) -> &SampleLocationsInfoEXT {
        &self.sample_locations_info
    }
    ///Get a mutable reference to the `attachment_index` field.
    pub fn attachment_index_mut(&mut self) -> &mut u32 {
        &mut self.attachment_index
    }
    ///Get a mutable reference to the `sample_locations_info` field.
    pub fn sample_locations_info_mut(&mut self) -> &mut SampleLocationsInfoEXT {
        &mut self.sample_locations_info
    }
    ///Sets the `attachment_index` field.
    pub fn set_attachment_index(&mut self, attachment_index: u32) -> &mut Self {
        self.attachment_index = attachment_index;
        self
    }
    ///Sets the `sample_locations_info` field.
    pub fn set_sample_locations_info(&mut self, sample_locations_info: SampleLocationsInfoEXT) -> &mut Self {
        self.sample_locations_info = sample_locations_info;
        self
    }
    ///Sets the `attachment_index` field in a builder way.
    pub fn with_attachment_index(mut self, attachment_index: u32) -> Self {
        self.attachment_index = attachment_index;
        self
    }
    ///Sets the `sample_locations_info` field in a builder way.
    pub fn with_sample_locations_info(mut self, sample_locations_info: SampleLocationsInfoEXT) -> Self {
        self.sample_locations_info = sample_locations_info;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AttachmentSampleLocationsEXT {
    type LowLevel = crate::native::extensions::ext_sample_locations::AttachmentSampleLocationsEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_sample_locations::AttachmentSampleLocationsEXT {
            attachment_index: self.attachment_index.into_low_level(context, bump),
            sample_locations_info: self.sample_locations_info.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AttachmentSampleLocationsEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            attachment_index: crate::conv::FromLowLevel::from_low_level(context, value.attachment_index),
            sample_locations_info: crate::conv::FromLowLevel::from_low_level(context, value.sample_locations_info),
        }
    }
}
#[doc(alias = "VkSubpassSampleLocationsEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubpassSampleLocationsEXT {
    #[doc(alias = "subpassIndex")]
    pub subpass_index: u32,
    #[doc(alias = "sampleLocationsInfo")]
    pub sample_locations_info: SampleLocationsInfoEXT,
}
impl SubpassSampleLocationsEXT {
    ///Get a reference to the `subpass_index` field.
    pub fn subpass_index(&self) -> u32 {
        self.subpass_index
    }
    ///Get a reference to the `sample_locations_info` field.
    pub fn sample_locations_info(&self) -> &SampleLocationsInfoEXT {
        &self.sample_locations_info
    }
    ///Get a mutable reference to the `subpass_index` field.
    pub fn subpass_index_mut(&mut self) -> &mut u32 {
        &mut self.subpass_index
    }
    ///Get a mutable reference to the `sample_locations_info` field.
    pub fn sample_locations_info_mut(&mut self) -> &mut SampleLocationsInfoEXT {
        &mut self.sample_locations_info
    }
    ///Sets the `subpass_index` field.
    pub fn set_subpass_index(&mut self, subpass_index: u32) -> &mut Self {
        self.subpass_index = subpass_index;
        self
    }
    ///Sets the `sample_locations_info` field.
    pub fn set_sample_locations_info(&mut self, sample_locations_info: SampleLocationsInfoEXT) -> &mut Self {
        self.sample_locations_info = sample_locations_info;
        self
    }
    ///Sets the `subpass_index` field in a builder way.
    pub fn with_subpass_index(mut self, subpass_index: u32) -> Self {
        self.subpass_index = subpass_index;
        self
    }
    ///Sets the `sample_locations_info` field in a builder way.
    pub fn with_sample_locations_info(mut self, sample_locations_info: SampleLocationsInfoEXT) -> Self {
        self.sample_locations_info = sample_locations_info;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubpassSampleLocationsEXT {
    type LowLevel = crate::native::extensions::ext_sample_locations::SubpassSampleLocationsEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_sample_locations::SubpassSampleLocationsEXT {
            subpass_index: self.subpass_index.into_low_level(context, bump),
            sample_locations_info: self.sample_locations_info.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SubpassSampleLocationsEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            subpass_index: crate::conv::FromLowLevel::from_low_level(context, value.subpass_index),
            sample_locations_info: crate::conv::FromLowLevel::from_low_level(context, value.sample_locations_info),
        }
    }
}
#[doc(alias = "VkRenderPassSampleLocationsBeginInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderPassSampleLocationsBeginInfoEXT {
    #[doc(alias = "pAttachmentInitialSampleLocations")]
    pub attachment_initial_sample_locations: SmallVec<[AttachmentSampleLocationsEXT; 8]>,
    #[doc(alias = "pPostSubpassSampleLocations")]
    pub post_subpass_sample_locations: SmallVec<[SubpassSampleLocationsEXT; 8]>,
}
impl RenderPassSampleLocationsBeginInfoEXT {
    ///Get a reference to the `attachment_initial_sample_locations` field.
    pub fn attachment_initial_sample_locations(&self) -> &SmallVec<[AttachmentSampleLocationsEXT; 8]> {
        &self.attachment_initial_sample_locations
    }
    ///Get a reference to the `post_subpass_sample_locations` field.
    pub fn post_subpass_sample_locations(&self) -> &SmallVec<[SubpassSampleLocationsEXT; 8]> {
        &self.post_subpass_sample_locations
    }
    ///Get a mutable reference to the `attachment_initial_sample_locations` field.
    pub fn attachment_initial_sample_locations_mut(&mut self) -> &mut SmallVec<[AttachmentSampleLocationsEXT; 8]> {
        &mut self.attachment_initial_sample_locations
    }
    ///Get a mutable reference to the `post_subpass_sample_locations` field.
    pub fn post_subpass_sample_locations_mut(&mut self) -> &mut SmallVec<[SubpassSampleLocationsEXT; 8]> {
        &mut self.post_subpass_sample_locations
    }
    ///Sets the `attachment_initial_sample_locations` field.
    pub fn set_attachment_initial_sample_locations(
        &mut self,
        attachment_initial_sample_locations: SmallVec<[AttachmentSampleLocationsEXT; 8]>,
    ) -> &mut Self {
        self.attachment_initial_sample_locations = attachment_initial_sample_locations;
        self
    }
    ///Sets the `post_subpass_sample_locations` field.
    pub fn set_post_subpass_sample_locations(
        &mut self,
        post_subpass_sample_locations: SmallVec<[SubpassSampleLocationsEXT; 8]>,
    ) -> &mut Self {
        self.post_subpass_sample_locations = post_subpass_sample_locations;
        self
    }
    ///Sets the `attachment_initial_sample_locations` field in a builder way.
    pub fn with_attachment_initial_sample_locations(
        mut self,
        attachment_initial_sample_locations: SmallVec<[AttachmentSampleLocationsEXT; 8]>,
    ) -> Self {
        self.attachment_initial_sample_locations = attachment_initial_sample_locations;
        self
    }
    ///Sets the `post_subpass_sample_locations` field in a builder way.
    pub fn with_post_subpass_sample_locations(
        mut self,
        post_subpass_sample_locations: SmallVec<[SubpassSampleLocationsEXT; 8]>,
    ) -> Self {
        self.post_subpass_sample_locations = post_subpass_sample_locations;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderPassSampleLocationsBeginInfoEXT {
    type LowLevel = crate::native::extensions::ext_sample_locations::RenderPassSampleLocationsBeginInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_attachment_initial_sample_locations = self.attachment_initial_sample_locations.len() as u32;
        let attachment_initial_sample_locations = bump
            .alloc_slice_fill_iter(
                self.attachment_initial_sample_locations
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        let len_post_subpass_sample_locations = self.post_subpass_sample_locations.len() as u32;
        let post_subpass_sample_locations = bump
            .alloc_slice_fill_iter(
                self.post_subpass_sample_locations
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::ext_sample_locations::RenderPassSampleLocationsBeginInfoEXT {
            s_type: StructureType::RenderPassSampleLocationsBeginInfoExt,
            p_next: std::ptr::null(),
            attachment_initial_sample_locations_count: len_attachment_initial_sample_locations,
            attachment_initial_sample_locations: attachment_initial_sample_locations,
            post_subpass_sample_locations_count: len_post_subpass_sample_locations,
            post_subpass_sample_locations: post_subpass_sample_locations,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RenderPassSampleLocationsBeginInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let attachment_initial_sample_locations_len = value.attachment_initial_sample_locations_count;
        let mut attachment_initial_sample_locations =
            SmallVec::with_capacity(attachment_initial_sample_locations_len as usize);
        for i in 0..attachment_initial_sample_locations_len {
            attachment_initial_sample_locations.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.attachment_initial_sample_locations.add(i as usize).read(),
            ));
        }
        let post_subpass_sample_locations_len = value.post_subpass_sample_locations_count;
        let mut post_subpass_sample_locations = SmallVec::with_capacity(post_subpass_sample_locations_len as usize);
        for i in 0..post_subpass_sample_locations_len {
            post_subpass_sample_locations.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.post_subpass_sample_locations.add(i as usize).read(),
            ));
        }
        Self {
            attachment_initial_sample_locations: attachment_initial_sample_locations,
            post_subpass_sample_locations: post_subpass_sample_locations,
        }
    }
}
#[doc(alias = "VkPipelineSampleLocationsStateCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineSampleLocationsStateCreateInfoEXT {
    #[doc(alias = "sampleLocationsEnable")]
    pub sample_locations_enable: bool,
    #[doc(alias = "sampleLocationsInfo")]
    pub sample_locations_info: SampleLocationsInfoEXT,
}
impl PipelineSampleLocationsStateCreateInfoEXT {
    ///Get a reference to the `sample_locations_enable` field.
    pub fn sample_locations_enable(&self) -> &bool {
        &self.sample_locations_enable
    }
    ///Get a reference to the `sample_locations_info` field.
    pub fn sample_locations_info(&self) -> &SampleLocationsInfoEXT {
        &self.sample_locations_info
    }
    ///Get a mutable reference to the `sample_locations_enable` field.
    pub fn sample_locations_enable_mut(&mut self) -> &mut bool {
        &mut self.sample_locations_enable
    }
    ///Get a mutable reference to the `sample_locations_info` field.
    pub fn sample_locations_info_mut(&mut self) -> &mut SampleLocationsInfoEXT {
        &mut self.sample_locations_info
    }
    ///Sets the `sample_locations_enable` field.
    pub fn set_sample_locations_enable(&mut self, sample_locations_enable: bool) -> &mut Self {
        self.sample_locations_enable = sample_locations_enable;
        self
    }
    ///Sets the `sample_locations_info` field.
    pub fn set_sample_locations_info(&mut self, sample_locations_info: SampleLocationsInfoEXT) -> &mut Self {
        self.sample_locations_info = sample_locations_info;
        self
    }
    ///Sets the `sample_locations_enable` field in a builder way.
    pub fn with_sample_locations_enable(mut self, sample_locations_enable: bool) -> Self {
        self.sample_locations_enable = sample_locations_enable;
        self
    }
    ///Sets the `sample_locations_info` field in a builder way.
    pub fn with_sample_locations_info(mut self, sample_locations_info: SampleLocationsInfoEXT) -> Self {
        self.sample_locations_info = sample_locations_info;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineSampleLocationsStateCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_sample_locations::PipelineSampleLocationsStateCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_sample_locations::PipelineSampleLocationsStateCreateInfoEXT {
            s_type: StructureType::PipelineSampleLocationsStateCreateInfoExt,
            p_next: std::ptr::null(),
            sample_locations_enable: self.sample_locations_enable.into_low_level(context, bump),
            sample_locations_info: self.sample_locations_info.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineSampleLocationsStateCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            sample_locations_enable: crate::conv::FromLowLevel::from_low_level(context, value.sample_locations_enable),
            sample_locations_info: crate::conv::FromLowLevel::from_low_level(context, value.sample_locations_info),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSampleLocationsPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
    #[doc(alias = "sampleLocationSampleCounts")]
    pub sample_location_sample_counts: SampleCountFlags,
    #[doc(alias = "maxSampleLocationGridSize")]
    pub max_sample_location_grid_size: Extent2D,
    #[doc(alias = "sampleLocationCoordinateRange")]
    pub sample_location_coordinate_range: [f32; 2 as usize],
    #[doc(alias = "sampleLocationSubPixelBits")]
    pub sample_location_sub_pixel_bits: u32,
    #[doc(alias = "variableSampleLocations")]
    pub variable_sample_locations: bool,
}
impl PhysicalDeviceSampleLocationsPropertiesEXT {
    ///Get a reference to the `sample_location_sample_counts` field.
    pub fn sample_location_sample_counts(&self) -> SampleCountFlags {
        self.sample_location_sample_counts
    }
    ///Get a reference to the `max_sample_location_grid_size` field.
    pub fn max_sample_location_grid_size(&self) -> Extent2D {
        self.max_sample_location_grid_size
    }
    ///Get a reference to the `sample_location_coordinate_range` field.
    pub fn sample_location_coordinate_range(&self) -> [f32; 2 as usize] {
        self.sample_location_coordinate_range
    }
    ///Get a reference to the `sample_location_sub_pixel_bits` field.
    pub fn sample_location_sub_pixel_bits(&self) -> u32 {
        self.sample_location_sub_pixel_bits
    }
    ///Get a reference to the `variable_sample_locations` field.
    pub fn variable_sample_locations(&self) -> &bool {
        &self.variable_sample_locations
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSampleLocationsPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_sample_locations::PhysicalDeviceSampleLocationsPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_sample_locations::PhysicalDeviceSampleLocationsPropertiesEXT {
            s_type: StructureType::PhysicalDeviceSampleLocationsPropertiesExt,
            p_next: std::ptr::null_mut(),
            sample_location_sample_counts: self.sample_location_sample_counts.into_low_level(context, bump),
            max_sample_location_grid_size: self.max_sample_location_grid_size.into_low_level(context, bump),
            sample_location_coordinate_range: self.sample_location_coordinate_range.into_low_level(context, bump),
            sample_location_sub_pixel_bits: self.sample_location_sub_pixel_bits.into_low_level(context, bump),
            variable_sample_locations: self.variable_sample_locations.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSampleLocationsPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            sample_location_sample_counts: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sample_location_sample_counts,
            ),
            max_sample_location_grid_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_sample_location_grid_size,
            ),
            sample_location_coordinate_range: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sample_location_coordinate_range,
            ),
            sample_location_sub_pixel_bits: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sample_location_sub_pixel_bits,
            ),
            variable_sample_locations: crate::conv::FromLowLevel::from_low_level(
                context,
                value.variable_sample_locations,
            ),
        }
    }
}
#[doc(alias = "VkMultisamplePropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MultisamplePropertiesEXT {
    #[doc(alias = "maxSampleLocationGridSize")]
    pub max_sample_location_grid_size: Extent2D,
}
impl MultisamplePropertiesEXT {
    ///Get a reference to the `max_sample_location_grid_size` field.
    pub fn max_sample_location_grid_size(&self) -> Extent2D {
        self.max_sample_location_grid_size
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MultisamplePropertiesEXT {
    type LowLevel = crate::native::extensions::ext_sample_locations::MultisamplePropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_sample_locations::MultisamplePropertiesEXT {
            s_type: StructureType::MultisamplePropertiesExt,
            p_next: std::ptr::null_mut(),
            max_sample_location_grid_size: self.max_sample_location_grid_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MultisamplePropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_sample_location_grid_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_sample_location_grid_size,
            ),
        }
    }
}
