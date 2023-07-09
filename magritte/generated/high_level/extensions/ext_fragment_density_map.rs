pub use crate::common::extensions::ext_fragment_density_map::{
    EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME, EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{AttachmentReference, Extent2D, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT {
    #[doc(alias = "fragmentDensityMap")]
    pub fragment_density_map: bool,
    #[doc(alias = "fragmentDensityMapDynamic")]
    pub fragment_density_map_dynamic: bool,
    #[doc(alias = "fragmentDensityMapNonSubsampledImages")]
    pub fragment_density_map_non_subsampled_images: bool,
}
impl PhysicalDeviceFragmentDensityMapFeaturesEXT {
    ///Get a reference to the `fragment_density_map` field.
    pub fn fragment_density_map(&self) -> &bool {
        &self.fragment_density_map
    }
    ///Get a reference to the `fragment_density_map_dynamic` field.
    pub fn fragment_density_map_dynamic(&self) -> &bool {
        &self.fragment_density_map_dynamic
    }
    ///Get a reference to the `fragment_density_map_non_subsampled_images` field.
    pub fn fragment_density_map_non_subsampled_images(&self) -> &bool {
        &self.fragment_density_map_non_subsampled_images
    }
    ///Get a mutable reference to the `fragment_density_map` field.
    pub fn fragment_density_map_mut(&mut self) -> &mut bool {
        &mut self.fragment_density_map
    }
    ///Get a mutable reference to the `fragment_density_map_dynamic` field.
    pub fn fragment_density_map_dynamic_mut(&mut self) -> &mut bool {
        &mut self.fragment_density_map_dynamic
    }
    ///Get a mutable reference to the `fragment_density_map_non_subsampled_images` field.
    pub fn fragment_density_map_non_subsampled_images_mut(&mut self) -> &mut bool {
        &mut self.fragment_density_map_non_subsampled_images
    }
    ///Sets the `fragment_density_map` field.
    pub fn set_fragment_density_map(&mut self, fragment_density_map: bool) -> &mut Self {
        self.fragment_density_map = fragment_density_map;
        self
    }
    ///Sets the `fragment_density_map_dynamic` field.
    pub fn set_fragment_density_map_dynamic(&mut self, fragment_density_map_dynamic: bool) -> &mut Self {
        self.fragment_density_map_dynamic = fragment_density_map_dynamic;
        self
    }
    ///Sets the `fragment_density_map_non_subsampled_images` field.
    pub fn set_fragment_density_map_non_subsampled_images(
        &mut self,
        fragment_density_map_non_subsampled_images: bool,
    ) -> &mut Self {
        self.fragment_density_map_non_subsampled_images = fragment_density_map_non_subsampled_images;
        self
    }
    ///Sets the `fragment_density_map` field in a builder way.
    pub fn with_fragment_density_map(mut self, fragment_density_map: bool) -> Self {
        self.fragment_density_map = fragment_density_map;
        self
    }
    ///Sets the `fragment_density_map_dynamic` field in a builder way.
    pub fn with_fragment_density_map_dynamic(mut self, fragment_density_map_dynamic: bool) -> Self {
        self.fragment_density_map_dynamic = fragment_density_map_dynamic;
        self
    }
    ///Sets the `fragment_density_map_non_subsampled_images` field in a builder way.
    pub fn with_fragment_density_map_non_subsampled_images(
        mut self,
        fragment_density_map_non_subsampled_images: bool,
    ) -> Self {
        self.fragment_density_map_non_subsampled_images = fragment_density_map_non_subsampled_images;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFragmentDensityMapFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapFeaturesEXT {
            s_type: StructureType::PhysicalDeviceFragmentDensityMapFeaturesExt,
            p_next: std::ptr::null_mut(),
            fragment_density_map: self.fragment_density_map.into_low_level(context, bump),
            fragment_density_map_dynamic: self.fragment_density_map_dynamic.into_low_level(context, bump),
            fragment_density_map_non_subsampled_images: self
                .fragment_density_map_non_subsampled_images
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFragmentDensityMapFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            fragment_density_map: crate::conv::FromLowLevel::from_low_level(context, value.fragment_density_map),
            fragment_density_map_dynamic: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_density_map_dynamic,
            ),
            fragment_density_map_non_subsampled_images: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_density_map_non_subsampled_images,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT {
    #[doc(alias = "minFragmentDensityTexelSize")]
    pub min_fragment_density_texel_size: Extent2D,
    #[doc(alias = "maxFragmentDensityTexelSize")]
    pub max_fragment_density_texel_size: Extent2D,
    #[doc(alias = "fragmentDensityInvocations")]
    pub fragment_density_invocations: bool,
}
impl PhysicalDeviceFragmentDensityMapPropertiesEXT {
    ///Get a reference to the `min_fragment_density_texel_size` field.
    pub fn min_fragment_density_texel_size(&self) -> Extent2D {
        self.min_fragment_density_texel_size
    }
    ///Get a reference to the `max_fragment_density_texel_size` field.
    pub fn max_fragment_density_texel_size(&self) -> Extent2D {
        self.max_fragment_density_texel_size
    }
    ///Get a reference to the `fragment_density_invocations` field.
    pub fn fragment_density_invocations(&self) -> &bool {
        &self.fragment_density_invocations
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFragmentDensityMapPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapPropertiesEXT {
            s_type: StructureType::PhysicalDeviceFragmentDensityMapPropertiesExt,
            p_next: std::ptr::null_mut(),
            min_fragment_density_texel_size: self.min_fragment_density_texel_size.into_low_level(context, bump),
            max_fragment_density_texel_size: self.max_fragment_density_texel_size.into_low_level(context, bump),
            fragment_density_invocations: self.fragment_density_invocations.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFragmentDensityMapPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            min_fragment_density_texel_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_fragment_density_texel_size,
            ),
            max_fragment_density_texel_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_fragment_density_texel_size,
            ),
            fragment_density_invocations: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_density_invocations,
            ),
        }
    }
}
#[doc(alias = "VkRenderPassFragmentDensityMapCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderPassFragmentDensityMapCreateInfoEXT {
    #[doc(alias = "fragmentDensityMapAttachment")]
    pub fragment_density_map_attachment: AttachmentReference,
}
impl RenderPassFragmentDensityMapCreateInfoEXT {
    ///Get a reference to the `fragment_density_map_attachment` field.
    pub fn fragment_density_map_attachment(&self) -> AttachmentReference {
        self.fragment_density_map_attachment
    }
    ///Get a mutable reference to the `fragment_density_map_attachment` field.
    pub fn fragment_density_map_attachment_mut(&mut self) -> &mut AttachmentReference {
        &mut self.fragment_density_map_attachment
    }
    ///Sets the `fragment_density_map_attachment` field.
    pub fn set_fragment_density_map_attachment(
        &mut self,
        fragment_density_map_attachment: AttachmentReference,
    ) -> &mut Self {
        self.fragment_density_map_attachment = fragment_density_map_attachment;
        self
    }
    ///Sets the `fragment_density_map_attachment` field in a builder way.
    pub fn with_fragment_density_map_attachment(
        mut self,
        fragment_density_map_attachment: AttachmentReference,
    ) -> Self {
        self.fragment_density_map_attachment = fragment_density_map_attachment;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderPassFragmentDensityMapCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_fragment_density_map::RenderPassFragmentDensityMapCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_fragment_density_map::RenderPassFragmentDensityMapCreateInfoEXT {
            s_type: StructureType::RenderPassFragmentDensityMapCreateInfoExt,
            p_next: std::ptr::null(),
            fragment_density_map_attachment: self.fragment_density_map_attachment.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RenderPassFragmentDensityMapCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            fragment_density_map_attachment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_density_map_attachment,
            ),
        }
    }
}
