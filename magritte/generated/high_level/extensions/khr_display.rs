pub use crate::common::extensions::khr_display::{
    DisplayModeCreateFlagsKHR, DisplayModeParametersKHR, DisplayPlaneAlphaFlagBitsKHR, DisplayPlaneAlphaFlagsKHR,
    DisplayPlaneCapabilitiesKHR, DisplaySurfaceCreateFlagsKHR, SurfaceTransformFlagsKHR, KHR_DISPLAY_EXTENSION_NAME,
    KHR_DISPLAY_SPEC_VERSION,
};
use crate::{
    context::{Container, Context, Error, ObjectId},
    extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    vulkan1_0::{Extent2D, Offset2D, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkDisplayPropertiesKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayPropertiesKHR {
    pub display: DisplayKHR,
    #[doc(alias = "displayName")]
    pub display_name: String,
    #[doc(alias = "physicalDimensions")]
    pub physical_dimensions: Extent2D,
    #[doc(alias = "physicalResolution")]
    pub physical_resolution: Extent2D,
    #[doc(alias = "supportedTransforms")]
    pub supported_transforms: SurfaceTransformFlagsKHR,
    #[doc(alias = "planeReorderPossible")]
    pub plane_reorder_possible: bool,
    #[doc(alias = "persistentContent")]
    pub persistent_content: bool,
}
impl DisplayPropertiesKHR {
    ///Get a reference to the `display` field.
    pub fn display(&self) -> &DisplayKHR {
        &self.display
    }
    ///Get a reference to the `display_name` field.
    pub fn display_name(&self) -> &String {
        &self.display_name
    }
    ///Get a reference to the `physical_dimensions` field.
    pub fn physical_dimensions(&self) -> Extent2D {
        self.physical_dimensions
    }
    ///Get a reference to the `physical_resolution` field.
    pub fn physical_resolution(&self) -> Extent2D {
        self.physical_resolution
    }
    ///Get a reference to the `supported_transforms` field.
    pub fn supported_transforms(&self) -> SurfaceTransformFlagsKHR {
        self.supported_transforms
    }
    ///Get a reference to the `plane_reorder_possible` field.
    pub fn plane_reorder_possible(&self) -> &bool {
        &self.plane_reorder_possible
    }
    ///Get a reference to the `persistent_content` field.
    pub fn persistent_content(&self) -> &bool {
        &self.persistent_content
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayPropertiesKHR {
    type LowLevel = crate::native::extensions::khr_display::DisplayPropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_display::DisplayPropertiesKHR {
            display: self.display.into_low_level(context, bump),
            display_name: self.display_name.into_low_level(context, bump),
            physical_dimensions: self.physical_dimensions.into_low_level(context, bump),
            physical_resolution: self.physical_resolution.into_low_level(context, bump),
            supported_transforms: self.supported_transforms.into_low_level(context, bump),
            plane_reorder_possible: self.plane_reorder_possible.into_low_level(context, bump),
            persistent_content: self.persistent_content.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayPropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            display: crate::conv::FromLowLevel::from_low_level(context, value.display),
            display_name: crate::conv::FromLowLevel::from_low_level(context, value.display_name),
            physical_dimensions: crate::conv::FromLowLevel::from_low_level(context, value.physical_dimensions),
            physical_resolution: crate::conv::FromLowLevel::from_low_level(context, value.physical_resolution),
            supported_transforms: crate::conv::FromLowLevel::from_low_level(context, value.supported_transforms),
            plane_reorder_possible: crate::conv::FromLowLevel::from_low_level(context, value.plane_reorder_possible),
            persistent_content: crate::conv::FromLowLevel::from_low_level(context, value.persistent_content),
        }
    }
}
#[doc(alias = "VkDisplayPlanePropertiesKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayPlanePropertiesKHR {
    #[doc(alias = "currentDisplay")]
    pub current_display: DisplayKHR,
    #[doc(alias = "currentStackIndex")]
    pub current_stack_index: u32,
}
impl DisplayPlanePropertiesKHR {
    ///Get a reference to the `current_display` field.
    pub fn current_display(&self) -> &DisplayKHR {
        &self.current_display
    }
    ///Get a reference to the `current_stack_index` field.
    pub fn current_stack_index(&self) -> u32 {
        self.current_stack_index
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayPlanePropertiesKHR {
    type LowLevel = crate::native::extensions::khr_display::DisplayPlanePropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_display::DisplayPlanePropertiesKHR {
            current_display: self.current_display.into_low_level(context, bump),
            current_stack_index: self.current_stack_index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayPlanePropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            current_display: crate::conv::FromLowLevel::from_low_level(context, value.current_display),
            current_stack_index: crate::conv::FromLowLevel::from_low_level(context, value.current_stack_index),
        }
    }
}
impl DisplayModeParametersKHR {
    ///Get a reference to the `visible_region` field.
    pub fn visible_region(&self) -> Extent2D {
        self.visible_region
    }
    ///Get a reference to the `refresh_rate` field.
    pub fn refresh_rate(&self) -> u32 {
        self.refresh_rate
    }
    ///Get a mutable reference to the `visible_region` field.
    pub fn visible_region_mut(&mut self) -> &mut Extent2D {
        &mut self.visible_region
    }
    ///Get a mutable reference to the `refresh_rate` field.
    pub fn refresh_rate_mut(&mut self) -> &mut u32 {
        &mut self.refresh_rate
    }
    ///Sets the `visible_region` field.
    pub fn set_visible_region(&mut self, visible_region: Extent2D) -> &mut Self {
        self.visible_region = visible_region;
        self
    }
    ///Sets the `refresh_rate` field.
    pub fn set_refresh_rate(&mut self, refresh_rate: u32) -> &mut Self {
        self.refresh_rate = refresh_rate;
        self
    }
    ///Sets the `visible_region` field in a builder way.
    pub fn with_visible_region(mut self, visible_region: Extent2D) -> Self {
        self.visible_region = visible_region;
        self
    }
    ///Sets the `refresh_rate` field in a builder way.
    pub fn with_refresh_rate(mut self, refresh_rate: u32) -> Self {
        self.refresh_rate = refresh_rate;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayModeParametersKHR {
    type LowLevel = crate::native::extensions::khr_display::DisplayModeParametersKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_display::DisplayModeParametersKHR {
            visible_region: self.visible_region.into_low_level(context, bump),
            refresh_rate: self.refresh_rate.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayModeParametersKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            visible_region: crate::conv::FromLowLevel::from_low_level(context, value.visible_region),
            refresh_rate: crate::conv::FromLowLevel::from_low_level(context, value.refresh_rate),
        }
    }
}
#[doc(alias = "VkDisplayModePropertiesKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayModePropertiesKHR {
    #[doc(alias = "displayMode")]
    pub display_mode: DisplayModeKHR,
    pub parameters: DisplayModeParametersKHR,
}
impl DisplayModePropertiesKHR {
    ///Get a reference to the `display_mode` field.
    pub fn display_mode(&self) -> &DisplayModeKHR {
        &self.display_mode
    }
    ///Get a reference to the `parameters` field.
    pub fn parameters(&self) -> DisplayModeParametersKHR {
        self.parameters
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayModePropertiesKHR {
    type LowLevel = crate::native::extensions::khr_display::DisplayModePropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_display::DisplayModePropertiesKHR {
            display_mode: self.display_mode.into_low_level(context, bump),
            parameters: self.parameters.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayModePropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            display_mode: crate::conv::FromLowLevel::from_low_level(context, value.display_mode),
            parameters: crate::conv::FromLowLevel::from_low_level(context, value.parameters),
        }
    }
}
#[doc(alias = "VkDisplayModeCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayModeCreateInfoKHR {
    pub flags: DisplayModeCreateFlagsKHR,
    pub parameters: DisplayModeParametersKHR,
}
impl DisplayModeCreateInfoKHR {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> DisplayModeCreateFlagsKHR {
        self.flags
    }
    ///Get a reference to the `parameters` field.
    pub fn parameters(&self) -> DisplayModeParametersKHR {
        self.parameters
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut DisplayModeCreateFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `parameters` field.
    pub fn parameters_mut(&mut self) -> &mut DisplayModeParametersKHR {
        &mut self.parameters
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: DisplayModeCreateFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `parameters` field.
    pub fn set_parameters(&mut self, parameters: DisplayModeParametersKHR) -> &mut Self {
        self.parameters = parameters;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: DisplayModeCreateFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `parameters` field in a builder way.
    pub fn with_parameters(mut self, parameters: DisplayModeParametersKHR) -> Self {
        self.parameters = parameters;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayModeCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_display::DisplayModeCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_display::DisplayModeCreateInfoKHR {
            s_type: StructureType::DisplayModeCreateInfoKhr,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            parameters: self.parameters.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayModeCreateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            parameters: crate::conv::FromLowLevel::from_low_level(context, value.parameters),
        }
    }
}
impl DisplayPlaneCapabilitiesKHR {
    ///Get a reference to the `supported_alpha` field.
    pub fn supported_alpha(&self) -> DisplayPlaneAlphaFlagsKHR {
        self.supported_alpha
    }
    ///Get a reference to the `min_src_position` field.
    pub fn min_src_position(&self) -> Offset2D {
        self.min_src_position
    }
    ///Get a reference to the `max_src_position` field.
    pub fn max_src_position(&self) -> Offset2D {
        self.max_src_position
    }
    ///Get a reference to the `min_src_extent` field.
    pub fn min_src_extent(&self) -> Extent2D {
        self.min_src_extent
    }
    ///Get a reference to the `max_src_extent` field.
    pub fn max_src_extent(&self) -> Extent2D {
        self.max_src_extent
    }
    ///Get a reference to the `min_dst_position` field.
    pub fn min_dst_position(&self) -> Offset2D {
        self.min_dst_position
    }
    ///Get a reference to the `max_dst_position` field.
    pub fn max_dst_position(&self) -> Offset2D {
        self.max_dst_position
    }
    ///Get a reference to the `min_dst_extent` field.
    pub fn min_dst_extent(&self) -> Extent2D {
        self.min_dst_extent
    }
    ///Get a reference to the `max_dst_extent` field.
    pub fn max_dst_extent(&self) -> Extent2D {
        self.max_dst_extent
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayPlaneCapabilitiesKHR {
    type LowLevel = crate::native::extensions::khr_display::DisplayPlaneCapabilitiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_display::DisplayPlaneCapabilitiesKHR {
            supported_alpha: self.supported_alpha.into_low_level(context, bump),
            min_src_position: self.min_src_position.into_low_level(context, bump),
            max_src_position: self.max_src_position.into_low_level(context, bump),
            min_src_extent: self.min_src_extent.into_low_level(context, bump),
            max_src_extent: self.max_src_extent.into_low_level(context, bump),
            min_dst_position: self.min_dst_position.into_low_level(context, bump),
            max_dst_position: self.max_dst_position.into_low_level(context, bump),
            min_dst_extent: self.min_dst_extent.into_low_level(context, bump),
            max_dst_extent: self.max_dst_extent.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayPlaneCapabilitiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            supported_alpha: crate::conv::FromLowLevel::from_low_level(context, value.supported_alpha),
            min_src_position: crate::conv::FromLowLevel::from_low_level(context, value.min_src_position),
            max_src_position: crate::conv::FromLowLevel::from_low_level(context, value.max_src_position),
            min_src_extent: crate::conv::FromLowLevel::from_low_level(context, value.min_src_extent),
            max_src_extent: crate::conv::FromLowLevel::from_low_level(context, value.max_src_extent),
            min_dst_position: crate::conv::FromLowLevel::from_low_level(context, value.min_dst_position),
            max_dst_position: crate::conv::FromLowLevel::from_low_level(context, value.max_dst_position),
            min_dst_extent: crate::conv::FromLowLevel::from_low_level(context, value.min_dst_extent),
            max_dst_extent: crate::conv::FromLowLevel::from_low_level(context, value.max_dst_extent),
        }
    }
}
#[doc(alias = "VkDisplaySurfaceCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplaySurfaceCreateInfoKHR {
    pub flags: DisplaySurfaceCreateFlagsKHR,
    #[doc(alias = "displayMode")]
    pub display_mode: DisplayModeKHR,
    #[doc(alias = "planeIndex")]
    pub plane_index: u32,
    #[doc(alias = "planeStackIndex")]
    pub plane_stack_index: u32,
    pub transform: SurfaceTransformFlagBitsKHR,
    #[doc(alias = "globalAlpha")]
    pub global_alpha: f32,
    #[doc(alias = "alphaMode")]
    pub alpha_mode: DisplayPlaneAlphaFlagBitsKHR,
    #[doc(alias = "imageExtent")]
    pub image_extent: Extent2D,
}
impl DisplaySurfaceCreateInfoKHR {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> DisplaySurfaceCreateFlagsKHR {
        self.flags
    }
    ///Get a reference to the `display_mode` field.
    pub fn display_mode(&self) -> &DisplayModeKHR {
        &self.display_mode
    }
    ///Get a reference to the `plane_index` field.
    pub fn plane_index(&self) -> u32 {
        self.plane_index
    }
    ///Get a reference to the `plane_stack_index` field.
    pub fn plane_stack_index(&self) -> u32 {
        self.plane_stack_index
    }
    ///Get a reference to the `transform` field.
    pub fn transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.transform
    }
    ///Get a reference to the `global_alpha` field.
    pub fn global_alpha(&self) -> f32 {
        self.global_alpha
    }
    ///Get a reference to the `alpha_mode` field.
    pub fn alpha_mode(&self) -> DisplayPlaneAlphaFlagBitsKHR {
        self.alpha_mode
    }
    ///Get a reference to the `image_extent` field.
    pub fn image_extent(&self) -> Extent2D {
        self.image_extent
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut DisplaySurfaceCreateFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `display_mode` field.
    pub fn display_mode_mut(&mut self) -> &mut DisplayModeKHR {
        &mut self.display_mode
    }
    ///Get a mutable reference to the `plane_index` field.
    pub fn plane_index_mut(&mut self) -> &mut u32 {
        &mut self.plane_index
    }
    ///Get a mutable reference to the `plane_stack_index` field.
    pub fn plane_stack_index_mut(&mut self) -> &mut u32 {
        &mut self.plane_stack_index
    }
    ///Get a mutable reference to the `transform` field.
    pub fn transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.transform
    }
    ///Get a mutable reference to the `global_alpha` field.
    pub fn global_alpha_mut(&mut self) -> &mut f32 {
        &mut self.global_alpha
    }
    ///Get a mutable reference to the `alpha_mode` field.
    pub fn alpha_mode_mut(&mut self) -> &mut DisplayPlaneAlphaFlagBitsKHR {
        &mut self.alpha_mode
    }
    ///Get a mutable reference to the `image_extent` field.
    pub fn image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.image_extent
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: DisplaySurfaceCreateFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `display_mode` field.
    pub fn set_display_mode(&mut self, display_mode: DisplayModeKHR) -> &mut Self {
        self.display_mode = display_mode;
        self
    }
    ///Sets the `plane_index` field.
    pub fn set_plane_index(&mut self, plane_index: u32) -> &mut Self {
        self.plane_index = plane_index;
        self
    }
    ///Sets the `plane_stack_index` field.
    pub fn set_plane_stack_index(&mut self, plane_stack_index: u32) -> &mut Self {
        self.plane_stack_index = plane_stack_index;
        self
    }
    ///Sets the `transform` field.
    pub fn set_transform(&mut self, transform: SurfaceTransformFlagBitsKHR) -> &mut Self {
        self.transform = transform;
        self
    }
    ///Sets the `global_alpha` field.
    pub fn set_global_alpha(&mut self, global_alpha: f32) -> &mut Self {
        self.global_alpha = global_alpha;
        self
    }
    ///Sets the `alpha_mode` field.
    pub fn set_alpha_mode(&mut self, alpha_mode: DisplayPlaneAlphaFlagBitsKHR) -> &mut Self {
        self.alpha_mode = alpha_mode;
        self
    }
    ///Sets the `image_extent` field.
    pub fn set_image_extent(&mut self, image_extent: Extent2D) -> &mut Self {
        self.image_extent = image_extent;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: DisplaySurfaceCreateFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `display_mode` field in a builder way.
    pub fn with_display_mode(mut self, display_mode: DisplayModeKHR) -> Self {
        self.display_mode = display_mode;
        self
    }
    ///Sets the `plane_index` field in a builder way.
    pub fn with_plane_index(mut self, plane_index: u32) -> Self {
        self.plane_index = plane_index;
        self
    }
    ///Sets the `plane_stack_index` field in a builder way.
    pub fn with_plane_stack_index(mut self, plane_stack_index: u32) -> Self {
        self.plane_stack_index = plane_stack_index;
        self
    }
    ///Sets the `transform` field in a builder way.
    pub fn with_transform(mut self, transform: SurfaceTransformFlagBitsKHR) -> Self {
        self.transform = transform;
        self
    }
    ///Sets the `global_alpha` field in a builder way.
    pub fn with_global_alpha(mut self, global_alpha: f32) -> Self {
        self.global_alpha = global_alpha;
        self
    }
    ///Sets the `alpha_mode` field in a builder way.
    pub fn with_alpha_mode(mut self, alpha_mode: DisplayPlaneAlphaFlagBitsKHR) -> Self {
        self.alpha_mode = alpha_mode;
        self
    }
    ///Sets the `image_extent` field in a builder way.
    pub fn with_image_extent(mut self, image_extent: Extent2D) -> Self {
        self.image_extent = image_extent;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplaySurfaceCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_display::DisplaySurfaceCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_display::DisplaySurfaceCreateInfoKHR {
            s_type: StructureType::DisplaySurfaceCreateInfoKhr,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            display_mode: self.display_mode.into_low_level(context, bump),
            plane_index: self.plane_index.into_low_level(context, bump),
            plane_stack_index: self.plane_stack_index.into_low_level(context, bump),
            transform: self.transform.into_low_level(context, bump),
            global_alpha: self.global_alpha.into_low_level(context, bump),
            alpha_mode: self.alpha_mode.into_low_level(context, bump),
            image_extent: self.image_extent.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplaySurfaceCreateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            display_mode: crate::conv::FromLowLevel::from_low_level(context, value.display_mode),
            plane_index: crate::conv::FromLowLevel::from_low_level(context, value.plane_index),
            plane_stack_index: crate::conv::FromLowLevel::from_low_level(context, value.plane_stack_index),
            transform: crate::conv::FromLowLevel::from_low_level(context, value.transform),
            global_alpha: crate::conv::FromLowLevel::from_low_level(context, value.global_alpha),
            alpha_mode: crate::conv::FromLowLevel::from_low_level(context, value.alpha_mode),
            image_extent: crate::conv::FromLowLevel::from_low_level(context, value.image_extent),
        }
    }
}
#[doc(alias = "VkDisplayKHR")]
#[derive(Debug)]
pub struct DisplayKHR {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for DisplayKHR {
    fn clone(&self) -> Self {
        self.context.clone_display_khr(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DisplayKHR {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for DisplayKHR {
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
impl Drop for DisplayKHR {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_display_khr(&self.id);
        }
    }
}
impl PartialEq for DisplayKHR {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl DisplayKHR {
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
unsafe impl crate::conv::IntoLowLevel for DisplayKHR {
    type LowLevel = crate::native::extensions::khr_display::DisplayKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context.display_khr().get(&self.id).expect("unknwon handle").handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context.display_khr().insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
#[doc(alias = "VkDisplayModeKHR")]
#[derive(Debug)]
pub struct DisplayModeKHR {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for DisplayModeKHR {
    fn clone(&self) -> Self {
        self.context.clone_display_mode_khr(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DisplayModeKHR {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for DisplayModeKHR {
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
impl Drop for DisplayModeKHR {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_display_mode_khr(&self.id);
        }
    }
}
impl PartialEq for DisplayModeKHR {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl DisplayModeKHR {
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
unsafe impl crate::conv::IntoLowLevel for DisplayModeKHR {
    type LowLevel = crate::native::extensions::khr_display::DisplayModeKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .display_mode_khr()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayModeKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context.display_mode_khr().insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
