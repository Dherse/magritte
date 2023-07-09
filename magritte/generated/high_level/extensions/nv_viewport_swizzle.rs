pub use crate::common::extensions::nv_viewport_swizzle::{
    PipelineViewportSwizzleStateCreateFlagsNV, ViewportCoordinateSwizzleNV, ViewportSwizzleNV,
    NV_VIEWPORT_SWIZZLE_EXTENSION_NAME, NV_VIEWPORT_SWIZZLE_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
impl ViewportSwizzleNV {
    ///Get a reference to the `x` field.
    pub fn x(&self) -> ViewportCoordinateSwizzleNV {
        self.x
    }
    ///Get a reference to the `y` field.
    pub fn y(&self) -> ViewportCoordinateSwizzleNV {
        self.y
    }
    ///Get a reference to the `z` field.
    pub fn z(&self) -> ViewportCoordinateSwizzleNV {
        self.z
    }
    ///Get a reference to the `w` field.
    pub fn w(&self) -> ViewportCoordinateSwizzleNV {
        self.w
    }
    ///Get a mutable reference to the `x` field.
    pub fn x_mut(&mut self) -> &mut ViewportCoordinateSwizzleNV {
        &mut self.x
    }
    ///Get a mutable reference to the `y` field.
    pub fn y_mut(&mut self) -> &mut ViewportCoordinateSwizzleNV {
        &mut self.y
    }
    ///Get a mutable reference to the `z` field.
    pub fn z_mut(&mut self) -> &mut ViewportCoordinateSwizzleNV {
        &mut self.z
    }
    ///Get a mutable reference to the `w` field.
    pub fn w_mut(&mut self) -> &mut ViewportCoordinateSwizzleNV {
        &mut self.w
    }
    ///Sets the `x` field.
    pub fn set_x(&mut self, x: ViewportCoordinateSwizzleNV) -> &mut Self {
        self.x = x;
        self
    }
    ///Sets the `y` field.
    pub fn set_y(&mut self, y: ViewportCoordinateSwizzleNV) -> &mut Self {
        self.y = y;
        self
    }
    ///Sets the `z` field.
    pub fn set_z(&mut self, z: ViewportCoordinateSwizzleNV) -> &mut Self {
        self.z = z;
        self
    }
    ///Sets the `w` field.
    pub fn set_w(&mut self, w: ViewportCoordinateSwizzleNV) -> &mut Self {
        self.w = w;
        self
    }
    ///Sets the `x` field in a builder way.
    pub fn with_x(mut self, x: ViewportCoordinateSwizzleNV) -> Self {
        self.x = x;
        self
    }
    ///Sets the `y` field in a builder way.
    pub fn with_y(mut self, y: ViewportCoordinateSwizzleNV) -> Self {
        self.y = y;
        self
    }
    ///Sets the `z` field in a builder way.
    pub fn with_z(mut self, z: ViewportCoordinateSwizzleNV) -> Self {
        self.z = z;
        self
    }
    ///Sets the `w` field in a builder way.
    pub fn with_w(mut self, w: ViewportCoordinateSwizzleNV) -> Self {
        self.w = w;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ViewportSwizzleNV {
    type LowLevel = crate::native::extensions::nv_viewport_swizzle::ViewportSwizzleNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_viewport_swizzle::ViewportSwizzleNV {
            x: self.x.into_low_level(context, bump),
            y: self.y.into_low_level(context, bump),
            z: self.z.into_low_level(context, bump),
            w: self.w.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ViewportSwizzleNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            x: crate::conv::FromLowLevel::from_low_level(context, value.x),
            y: crate::conv::FromLowLevel::from_low_level(context, value.y),
            z: crate::conv::FromLowLevel::from_low_level(context, value.z),
            w: crate::conv::FromLowLevel::from_low_level(context, value.w),
        }
    }
}
#[doc(alias = "VkPipelineViewportSwizzleStateCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineViewportSwizzleStateCreateInfoNV {
    pub flags: PipelineViewportSwizzleStateCreateFlagsNV,
    #[doc(alias = "pViewportSwizzles")]
    pub viewport_swizzles: SmallVec<[ViewportSwizzleNV; 8]>,
}
impl PipelineViewportSwizzleStateCreateInfoNV {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> PipelineViewportSwizzleStateCreateFlagsNV {
        self.flags
    }
    ///Get a reference to the `viewport_swizzles` field.
    pub fn viewport_swizzles(&self) -> &SmallVec<[ViewportSwizzleNV; 8]> {
        &self.viewport_swizzles
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut PipelineViewportSwizzleStateCreateFlagsNV {
        &mut self.flags
    }
    ///Get a mutable reference to the `viewport_swizzles` field.
    pub fn viewport_swizzles_mut(&mut self) -> &mut SmallVec<[ViewportSwizzleNV; 8]> {
        &mut self.viewport_swizzles
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: PipelineViewportSwizzleStateCreateFlagsNV) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `viewport_swizzles` field.
    pub fn set_viewport_swizzles(&mut self, viewport_swizzles: SmallVec<[ViewportSwizzleNV; 8]>) -> &mut Self {
        self.viewport_swizzles = viewport_swizzles;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: PipelineViewportSwizzleStateCreateFlagsNV) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `viewport_swizzles` field in a builder way.
    pub fn with_viewport_swizzles(mut self, viewport_swizzles: SmallVec<[ViewportSwizzleNV; 8]>) -> Self {
        self.viewport_swizzles = viewport_swizzles;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineViewportSwizzleStateCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_viewport_swizzle::PipelineViewportSwizzleStateCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_viewport_swizzles = self.viewport_swizzles.len() as u32;
        let viewport_swizzles = bump
            .alloc_slice_fill_iter(self.viewport_swizzles.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::nv_viewport_swizzle::PipelineViewportSwizzleStateCreateInfoNV {
            s_type: StructureType::PipelineViewportSwizzleStateCreateInfoNv,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            viewport_count: len_viewport_swizzles,
            viewport_swizzles: viewport_swizzles,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineViewportSwizzleStateCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let viewport_swizzles_len = value.viewport_count;
        let mut viewport_swizzles = SmallVec::with_capacity(viewport_swizzles_len as usize);
        for i in 0..viewport_swizzles_len {
            viewport_swizzles.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.viewport_swizzles.add(i as usize).read(),
            ));
        }
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            viewport_swizzles: viewport_swizzles,
        }
    }
}
