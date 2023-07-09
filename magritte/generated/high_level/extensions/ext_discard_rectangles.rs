pub use crate::common::extensions::ext_discard_rectangles::{
    DiscardRectangleModeEXT, PipelineDiscardRectangleStateCreateFlagsEXT, EXT_DISCARD_RECTANGLES_EXTENSION_NAME,
    EXT_DISCARD_RECTANGLES_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Rect2D, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceDiscardRectanglePropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
    #[doc(alias = "maxDiscardRectangles")]
    pub max_discard_rectangles: u32,
}
impl PhysicalDeviceDiscardRectanglePropertiesEXT {
    ///Get a reference to the `max_discard_rectangles` field.
    pub fn max_discard_rectangles(&self) -> u32 {
        self.max_discard_rectangles
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDiscardRectanglePropertiesEXT {
    type LowLevel = crate::native::extensions::ext_discard_rectangles::PhysicalDeviceDiscardRectanglePropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_discard_rectangles::PhysicalDeviceDiscardRectanglePropertiesEXT {
            s_type: StructureType::PhysicalDeviceDiscardRectanglePropertiesExt,
            p_next: std::ptr::null_mut(),
            max_discard_rectangles: self.max_discard_rectangles.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDiscardRectanglePropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_discard_rectangles: crate::conv::FromLowLevel::from_low_level(context, value.max_discard_rectangles),
        }
    }
}
#[doc(alias = "VkPipelineDiscardRectangleStateCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
    pub flags: PipelineDiscardRectangleStateCreateFlagsEXT,
    #[doc(alias = "discardRectangleMode")]
    pub discard_rectangle_mode: DiscardRectangleModeEXT,
    #[doc(alias = "pDiscardRectangles")]
    pub discard_rectangles: SmallVec<[Rect2D; 8]>,
}
impl PipelineDiscardRectangleStateCreateInfoEXT {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> PipelineDiscardRectangleStateCreateFlagsEXT {
        self.flags
    }
    ///Get a reference to the `discard_rectangle_mode` field.
    pub fn discard_rectangle_mode(&self) -> DiscardRectangleModeEXT {
        self.discard_rectangle_mode
    }
    ///Get a reference to the `discard_rectangles` field.
    pub fn discard_rectangles(&self) -> &SmallVec<[Rect2D; 8]> {
        &self.discard_rectangles
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut PipelineDiscardRectangleStateCreateFlagsEXT {
        &mut self.flags
    }
    ///Get a mutable reference to the `discard_rectangle_mode` field.
    pub fn discard_rectangle_mode_mut(&mut self) -> &mut DiscardRectangleModeEXT {
        &mut self.discard_rectangle_mode
    }
    ///Get a mutable reference to the `discard_rectangles` field.
    pub fn discard_rectangles_mut(&mut self) -> &mut SmallVec<[Rect2D; 8]> {
        &mut self.discard_rectangles
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: PipelineDiscardRectangleStateCreateFlagsEXT) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `discard_rectangle_mode` field.
    pub fn set_discard_rectangle_mode(&mut self, discard_rectangle_mode: DiscardRectangleModeEXT) -> &mut Self {
        self.discard_rectangle_mode = discard_rectangle_mode;
        self
    }
    ///Sets the `discard_rectangles` field.
    pub fn set_discard_rectangles(&mut self, discard_rectangles: SmallVec<[Rect2D; 8]>) -> &mut Self {
        self.discard_rectangles = discard_rectangles;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: PipelineDiscardRectangleStateCreateFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `discard_rectangle_mode` field in a builder way.
    pub fn with_discard_rectangle_mode(mut self, discard_rectangle_mode: DiscardRectangleModeEXT) -> Self {
        self.discard_rectangle_mode = discard_rectangle_mode;
        self
    }
    ///Sets the `discard_rectangles` field in a builder way.
    pub fn with_discard_rectangles(mut self, discard_rectangles: SmallVec<[Rect2D; 8]>) -> Self {
        self.discard_rectangles = discard_rectangles;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineDiscardRectangleStateCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_discard_rectangles::PipelineDiscardRectangleStateCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_discard_rectangles = self.discard_rectangles.len() as u32;
        let discard_rectangles = bump
            .alloc_slice_fill_iter(self.discard_rectangles.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::ext_discard_rectangles::PipelineDiscardRectangleStateCreateInfoEXT {
            s_type: StructureType::PipelineDiscardRectangleStateCreateInfoExt,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            discard_rectangle_mode: self.discard_rectangle_mode.into_low_level(context, bump),
            discard_rectangle_count: len_discard_rectangles,
            discard_rectangles: discard_rectangles,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineDiscardRectangleStateCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let discard_rectangles_len = value.discard_rectangle_count;
        let mut discard_rectangles = SmallVec::with_capacity(discard_rectangles_len as usize);
        for i in 0..discard_rectangles_len {
            discard_rectangles.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.discard_rectangles.add(i as usize).read(),
            ));
        }
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            discard_rectangle_mode: crate::conv::FromLowLevel::from_low_level(context, value.discard_rectangle_mode),
            discard_rectangles: discard_rectangles,
        }
    }
}
