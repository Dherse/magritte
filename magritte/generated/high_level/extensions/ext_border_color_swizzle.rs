pub use crate::common::extensions::ext_border_color_swizzle::{
    EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME, EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{ComponentMapping, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkSamplerBorderColorComponentMappingCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SamplerBorderColorComponentMappingCreateInfoEXT {
    pub components: ComponentMapping,
    pub srgb: bool,
}
impl SamplerBorderColorComponentMappingCreateInfoEXT {
    ///Get a reference to the `components` field.
    pub fn components(&self) -> ComponentMapping {
        self.components
    }
    ///Get a reference to the `srgb` field.
    pub fn srgb(&self) -> &bool {
        &self.srgb
    }
    ///Get a mutable reference to the `components` field.
    pub fn components_mut(&mut self) -> &mut ComponentMapping {
        &mut self.components
    }
    ///Get a mutable reference to the `srgb` field.
    pub fn srgb_mut(&mut self) -> &mut bool {
        &mut self.srgb
    }
    ///Sets the `components` field.
    pub fn set_components(&mut self, components: ComponentMapping) -> &mut Self {
        self.components = components;
        self
    }
    ///Sets the `srgb` field.
    pub fn set_srgb(&mut self, srgb: bool) -> &mut Self {
        self.srgb = srgb;
        self
    }
    ///Sets the `components` field in a builder way.
    pub fn with_components(mut self, components: ComponentMapping) -> Self {
        self.components = components;
        self
    }
    ///Sets the `srgb` field in a builder way.
    pub fn with_srgb(mut self, srgb: bool) -> Self {
        self.srgb = srgb;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SamplerBorderColorComponentMappingCreateInfoEXT {
    type LowLevel =
        crate::native::extensions::ext_border_color_swizzle::SamplerBorderColorComponentMappingCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_border_color_swizzle::SamplerBorderColorComponentMappingCreateInfoEXT {
            s_type: StructureType::SamplerBorderColorComponentMappingCreateInfoExt,
            p_next: std::ptr::null(),
            components: self.components.into_low_level(context, bump),
            srgb: self.srgb.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SamplerBorderColorComponentMappingCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            components: crate::conv::FromLowLevel::from_low_level(context, value.components),
            srgb: crate::conv::FromLowLevel::from_low_level(context, value.srgb),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceBorderColorSwizzleFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    #[doc(alias = "borderColorSwizzle")]
    pub border_color_swizzle: bool,
    #[doc(alias = "borderColorSwizzleFromImage")]
    pub border_color_swizzle_from_image: bool,
}
impl PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    ///Get a reference to the `border_color_swizzle` field.
    pub fn border_color_swizzle(&self) -> &bool {
        &self.border_color_swizzle
    }
    ///Get a reference to the `border_color_swizzle_from_image` field.
    pub fn border_color_swizzle_from_image(&self) -> &bool {
        &self.border_color_swizzle_from_image
    }
    ///Get a mutable reference to the `border_color_swizzle` field.
    pub fn border_color_swizzle_mut(&mut self) -> &mut bool {
        &mut self.border_color_swizzle
    }
    ///Get a mutable reference to the `border_color_swizzle_from_image` field.
    pub fn border_color_swizzle_from_image_mut(&mut self) -> &mut bool {
        &mut self.border_color_swizzle_from_image
    }
    ///Sets the `border_color_swizzle` field.
    pub fn set_border_color_swizzle(&mut self, border_color_swizzle: bool) -> &mut Self {
        self.border_color_swizzle = border_color_swizzle;
        self
    }
    ///Sets the `border_color_swizzle_from_image` field.
    pub fn set_border_color_swizzle_from_image(&mut self, border_color_swizzle_from_image: bool) -> &mut Self {
        self.border_color_swizzle_from_image = border_color_swizzle_from_image;
        self
    }
    ///Sets the `border_color_swizzle` field in a builder way.
    pub fn with_border_color_swizzle(mut self, border_color_swizzle: bool) -> Self {
        self.border_color_swizzle = border_color_swizzle;
        self
    }
    ///Sets the `border_color_swizzle_from_image` field in a builder way.
    pub fn with_border_color_swizzle_from_image(mut self, border_color_swizzle_from_image: bool) -> Self {
        self.border_color_swizzle_from_image = border_color_swizzle_from_image;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_border_color_swizzle::PhysicalDeviceBorderColorSwizzleFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_border_color_swizzle::PhysicalDeviceBorderColorSwizzleFeaturesEXT {
            s_type: StructureType::PhysicalDeviceBorderColorSwizzleFeaturesExt,
            p_next: std::ptr::null_mut(),
            border_color_swizzle: self.border_color_swizzle.into_low_level(context, bump),
            border_color_swizzle_from_image: self.border_color_swizzle_from_image.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            border_color_swizzle: crate::conv::FromLowLevel::from_low_level(context, value.border_color_swizzle),
            border_color_swizzle_from_image: crate::conv::FromLowLevel::from_low_level(
                context,
                value.border_color_swizzle_from_image,
            ),
        }
    }
}
