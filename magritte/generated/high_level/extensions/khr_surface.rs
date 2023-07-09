pub use crate::common::extensions::khr_surface::{
    ColorSpaceKHR, CompositeAlphaFlagBitsKHR, CompositeAlphaFlagsKHR, PresentModeKHR, SurfaceCapabilitiesKHR,
    SurfaceFormatKHR, SurfaceTransformFlagBitsKHR, KHR_SURFACE_EXTENSION_NAME, KHR_SURFACE_SPEC_VERSION,
};
#[cfg(feature = "VK_KHR_display")]
use crate::extensions::khr_display::SurfaceTransformFlagsKHR;
use crate::{
    context::{Container, Context, Error, ObjectId},
    vulkan1_0::{Extent2D, Format, ImageUsageFlags},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
impl SurfaceCapabilitiesKHR {
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
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SurfaceCapabilitiesKHR {
    type LowLevel = crate::native::extensions::khr_surface::SurfaceCapabilitiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_surface::SurfaceCapabilitiesKHR {
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
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SurfaceCapabilitiesKHR {
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
        }
    }
}
impl SurfaceFormatKHR {
    ///Get a reference to the `format` field.
    pub fn format(&self) -> Format {
        self.format
    }
    ///Get a reference to the `color_space` field.
    pub fn color_space(&self) -> ColorSpaceKHR {
        self.color_space
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SurfaceFormatKHR {
    type LowLevel = crate::native::extensions::khr_surface::SurfaceFormatKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_surface::SurfaceFormatKHR {
            format: self.format.into_low_level(context, bump),
            color_space: self.color_space.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SurfaceFormatKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            format: crate::conv::FromLowLevel::from_low_level(context, value.format),
            color_space: crate::conv::FromLowLevel::from_low_level(context, value.color_space),
        }
    }
}
#[doc(alias = "VkSurfaceKHR")]
#[derive(Debug)]
pub struct SurfaceKHR {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for SurfaceKHR {
    fn clone(&self) -> Self {
        self.context.clone_surface_khr(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SurfaceKHR {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for SurfaceKHR {
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
impl Drop for SurfaceKHR {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_surface_khr(&self.id);
        }
    }
}
impl PartialEq for SurfaceKHR {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl SurfaceKHR {
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
unsafe impl crate::conv::IntoLowLevel for SurfaceKHR {
    type LowLevel = crate::native::extensions::khr_surface::SurfaceKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context.surface_khr().get(&self.id).expect("unknwon handle").handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SurfaceKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context.surface_khr().insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
