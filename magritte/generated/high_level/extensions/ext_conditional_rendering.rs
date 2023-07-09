pub use crate::common::extensions::ext_conditional_rendering::{
    ConditionalRenderingFlagBitsEXT, ConditionalRenderingFlagsEXT, EXT_CONDITIONAL_RENDERING_EXTENSION_NAME,
    EXT_CONDITIONAL_RENDERING_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Buffer, DeviceSize, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkConditionalRenderingBeginInfoEXT")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConditionalRenderingBeginInfoEXT {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub flags: ConditionalRenderingFlagsEXT,
}
impl ConditionalRenderingBeginInfoEXT {
    ///Get a reference to the `buffer` field.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    ///Get a reference to the `offset` field.
    pub fn offset(&self) -> &DeviceSize {
        &self.offset
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> ConditionalRenderingFlagsEXT {
        self.flags
    }
    ///Get a mutable reference to the `buffer` field.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Get a mutable reference to the `offset` field.
    pub fn offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.offset
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut ConditionalRenderingFlagsEXT {
        &mut self.flags
    }
    ///Sets the `buffer` field.
    pub fn set_buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `offset` field.
    pub fn set_offset(&mut self, offset: DeviceSize) -> &mut Self {
        self.offset = offset;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: ConditionalRenderingFlagsEXT) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `buffer` field in a builder way.
    pub fn with_buffer(mut self, buffer: Buffer) -> Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `offset` field in a builder way.
    pub fn with_offset(mut self, offset: DeviceSize) -> Self {
        self.offset = offset;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: ConditionalRenderingFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ConditionalRenderingBeginInfoEXT {
    type LowLevel = crate::native::extensions::ext_conditional_rendering::ConditionalRenderingBeginInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_conditional_rendering::ConditionalRenderingBeginInfoEXT {
            s_type: StructureType::ConditionalRenderingBeginInfoExt,
            p_next: std::ptr::null(),
            buffer: self.buffer.into_low_level(context, bump),
            offset: self.offset.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ConditionalRenderingBeginInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            buffer: crate::conv::FromLowLevel::from_low_level(context, value.buffer),
            offset: crate::conv::FromLowLevel::from_low_level(context, value.offset),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
        }
    }
}
#[doc(alias = "VkCommandBufferInheritanceConditionalRenderingInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT {
    #[doc(alias = "conditionalRenderingEnable")]
    pub conditional_rendering_enable: bool,
}
impl CommandBufferInheritanceConditionalRenderingInfoEXT {
    ///Get a reference to the `conditional_rendering_enable` field.
    pub fn conditional_rendering_enable(&self) -> &bool {
        &self.conditional_rendering_enable
    }
    ///Get a mutable reference to the `conditional_rendering_enable` field.
    pub fn conditional_rendering_enable_mut(&mut self) -> &mut bool {
        &mut self.conditional_rendering_enable
    }
    ///Sets the `conditional_rendering_enable` field.
    pub fn set_conditional_rendering_enable(&mut self, conditional_rendering_enable: bool) -> &mut Self {
        self.conditional_rendering_enable = conditional_rendering_enable;
        self
    }
    ///Sets the `conditional_rendering_enable` field in a builder way.
    pub fn with_conditional_rendering_enable(mut self, conditional_rendering_enable: bool) -> Self {
        self.conditional_rendering_enable = conditional_rendering_enable;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CommandBufferInheritanceConditionalRenderingInfoEXT {
    type LowLevel =
        crate::native::extensions::ext_conditional_rendering::CommandBufferInheritanceConditionalRenderingInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_conditional_rendering::CommandBufferInheritanceConditionalRenderingInfoEXT {
            s_type: StructureType::CommandBufferInheritanceConditionalRenderingInfoExt,
            p_next: std::ptr::null(),
            conditional_rendering_enable: self.conditional_rendering_enable.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CommandBufferInheritanceConditionalRenderingInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            conditional_rendering_enable: crate::conv::FromLowLevel::from_low_level(
                context,
                value.conditional_rendering_enable,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceConditionalRenderingFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT {
    #[doc(alias = "conditionalRendering")]
    pub conditional_rendering: bool,
    #[doc(alias = "inheritedConditionalRendering")]
    pub inherited_conditional_rendering: bool,
}
impl PhysicalDeviceConditionalRenderingFeaturesEXT {
    ///Get a reference to the `conditional_rendering` field.
    pub fn conditional_rendering(&self) -> &bool {
        &self.conditional_rendering
    }
    ///Get a reference to the `inherited_conditional_rendering` field.
    pub fn inherited_conditional_rendering(&self) -> &bool {
        &self.inherited_conditional_rendering
    }
    ///Get a mutable reference to the `conditional_rendering` field.
    pub fn conditional_rendering_mut(&mut self) -> &mut bool {
        &mut self.conditional_rendering
    }
    ///Get a mutable reference to the `inherited_conditional_rendering` field.
    pub fn inherited_conditional_rendering_mut(&mut self) -> &mut bool {
        &mut self.inherited_conditional_rendering
    }
    ///Sets the `conditional_rendering` field.
    pub fn set_conditional_rendering(&mut self, conditional_rendering: bool) -> &mut Self {
        self.conditional_rendering = conditional_rendering;
        self
    }
    ///Sets the `inherited_conditional_rendering` field.
    pub fn set_inherited_conditional_rendering(&mut self, inherited_conditional_rendering: bool) -> &mut Self {
        self.inherited_conditional_rendering = inherited_conditional_rendering;
        self
    }
    ///Sets the `conditional_rendering` field in a builder way.
    pub fn with_conditional_rendering(mut self, conditional_rendering: bool) -> Self {
        self.conditional_rendering = conditional_rendering;
        self
    }
    ///Sets the `inherited_conditional_rendering` field in a builder way.
    pub fn with_inherited_conditional_rendering(mut self, inherited_conditional_rendering: bool) -> Self {
        self.inherited_conditional_rendering = inherited_conditional_rendering;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceConditionalRenderingFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_conditional_rendering::PhysicalDeviceConditionalRenderingFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_conditional_rendering::PhysicalDeviceConditionalRenderingFeaturesEXT {
            s_type: StructureType::PhysicalDeviceConditionalRenderingFeaturesExt,
            p_next: std::ptr::null_mut(),
            conditional_rendering: self.conditional_rendering.into_low_level(context, bump),
            inherited_conditional_rendering: self.inherited_conditional_rendering.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceConditionalRenderingFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            conditional_rendering: crate::conv::FromLowLevel::from_low_level(context, value.conditional_rendering),
            inherited_conditional_rendering: crate::conv::FromLowLevel::from_low_level(
                context,
                value.inherited_conditional_rendering,
            ),
        }
    }
}
