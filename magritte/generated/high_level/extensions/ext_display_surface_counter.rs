pub use crate::common::extensions::ext_display_surface_counter::{
    SurfaceCounterFlagBitsEXT, SurfaceCounterFlagsEXT, EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME,
    EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION,
};
use crate::{
    context::Context,
    extensions::{
        khr_display::SurfaceTransformFlagsKHR,
        khr_surface::{CompositeAlphaFlagsKHR, SurfaceTransformFlagBitsKHR},
    },
    vulkan1_0::{Extent2D, ImageUsageFlags, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkSurfaceCapabilities2EXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SurfaceCapabilities2EXT {
    #[doc(alias = "minImageCount")]
    pub min_image_count: u32,
    #[doc(alias = "maxImageCount")]
    pub max_image_count: u32,
    #[doc(alias = "currentExtent")]
    pub current_extent: Extent2D,
    #[doc(alias = "minImageExtent")]
    pub min_image_extent: Extent2D,
    #[doc(alias = "maxImageExtent")]
    pub max_image_extent: Extent2D,
    #[doc(alias = "maxImageArrayLayers")]
    pub max_image_array_layers: u32,
    #[doc(alias = "supportedTransforms")]
    pub supported_transforms: SurfaceTransformFlagsKHR,
    #[doc(alias = "currentTransform")]
    pub current_transform: SurfaceTransformFlagBitsKHR,
    #[doc(alias = "supportedCompositeAlpha")]
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    #[doc(alias = "supportedUsageFlags")]
    pub supported_usage_flags: ImageUsageFlags,
    #[doc(alias = "supportedSurfaceCounters")]
    pub supported_surface_counters: SurfaceCounterFlagsEXT,
}
impl SurfaceCapabilities2EXT {
    ///Get a reference to the `min_image_count` field.
    pub fn min_image_count(&self) -> u32 {
        self.min_image_count
    }
    ///Get a reference to the `max_image_count` field.
    pub fn max_image_count(&self) -> u32 {
        self.max_image_count
    }
    ///Get a reference to the `current_extent` field.
    pub fn current_extent(&self) -> Extent2D {
        self.current_extent
    }
    ///Get a reference to the `min_image_extent` field.
    pub fn min_image_extent(&self) -> Extent2D {
        self.min_image_extent
    }
    ///Get a reference to the `max_image_extent` field.
    pub fn max_image_extent(&self) -> Extent2D {
        self.max_image_extent
    }
    ///Get a reference to the `max_image_array_layers` field.
    pub fn max_image_array_layers(&self) -> u32 {
        self.max_image_array_layers
    }
    ///Get a reference to the `supported_transforms` field.
    pub fn supported_transforms(&self) -> SurfaceTransformFlagsKHR {
        self.supported_transforms
    }
    ///Get a reference to the `current_transform` field.
    pub fn current_transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.current_transform
    }
    ///Get a reference to the `supported_composite_alpha` field.
    pub fn supported_composite_alpha(&self) -> CompositeAlphaFlagsKHR {
        self.supported_composite_alpha
    }
    ///Get a reference to the `supported_usage_flags` field.
    pub fn supported_usage_flags(&self) -> ImageUsageFlags {
        self.supported_usage_flags
    }
    ///Get a reference to the `supported_surface_counters` field.
    pub fn supported_surface_counters(&self) -> SurfaceCounterFlagsEXT {
        self.supported_surface_counters
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SurfaceCapabilities2EXT {
    type LowLevel = crate::native::extensions::ext_display_surface_counter::SurfaceCapabilities2EXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_display_surface_counter::SurfaceCapabilities2EXT {
            s_type: StructureType::SurfaceCapabilities2Ext,
            p_next: std::ptr::null_mut(),
            min_image_count: self.min_image_count.into_low_level(context, bump),
            max_image_count: self.max_image_count.into_low_level(context, bump),
            current_extent: self.current_extent.into_low_level(context, bump),
            min_image_extent: self.min_image_extent.into_low_level(context, bump),
            max_image_extent: self.max_image_extent.into_low_level(context, bump),
            max_image_array_layers: self.max_image_array_layers.into_low_level(context, bump),
            supported_transforms: self.supported_transforms.into_low_level(context, bump),
            current_transform: self.current_transform.into_low_level(context, bump),
            supported_composite_alpha: self.supported_composite_alpha.into_low_level(context, bump),
            supported_usage_flags: self.supported_usage_flags.into_low_level(context, bump),
            supported_surface_counters: self.supported_surface_counters.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SurfaceCapabilities2EXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            min_image_count: crate::conv::FromLowLevel::from_low_level(context, value.min_image_count),
            max_image_count: crate::conv::FromLowLevel::from_low_level(context, value.max_image_count),
            current_extent: crate::conv::FromLowLevel::from_low_level(context, value.current_extent),
            min_image_extent: crate::conv::FromLowLevel::from_low_level(context, value.min_image_extent),
            max_image_extent: crate::conv::FromLowLevel::from_low_level(context, value.max_image_extent),
            max_image_array_layers: crate::conv::FromLowLevel::from_low_level(context, value.max_image_array_layers),
            supported_transforms: crate::conv::FromLowLevel::from_low_level(context, value.supported_transforms),
            current_transform: crate::conv::FromLowLevel::from_low_level(context, value.current_transform),
            supported_composite_alpha: crate::conv::FromLowLevel::from_low_level(
                context,
                value.supported_composite_alpha,
            ),
            supported_usage_flags: crate::conv::FromLowLevel::from_low_level(context, value.supported_usage_flags),
            supported_surface_counters: crate::conv::FromLowLevel::from_low_level(
                context,
                value.supported_surface_counters,
            ),
        }
    }
}
