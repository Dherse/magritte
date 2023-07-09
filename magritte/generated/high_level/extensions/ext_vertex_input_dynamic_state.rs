pub use crate::common::extensions::ext_vertex_input_dynamic_state::{
    EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME, EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Format, StructureType, VertexInputRate},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    #[doc(alias = "vertexInputDynamicState")]
    pub vertex_input_dynamic_state: bool,
}
impl PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    ///Get a reference to the `vertex_input_dynamic_state` field.
    pub fn vertex_input_dynamic_state(&self) -> &bool {
        &self.vertex_input_dynamic_state
    }
    ///Get a mutable reference to the `vertex_input_dynamic_state` field.
    pub fn vertex_input_dynamic_state_mut(&mut self) -> &mut bool {
        &mut self.vertex_input_dynamic_state
    }
    ///Sets the `vertex_input_dynamic_state` field.
    pub fn set_vertex_input_dynamic_state(&mut self, vertex_input_dynamic_state: bool) -> &mut Self {
        self.vertex_input_dynamic_state = vertex_input_dynamic_state;
        self
    }
    ///Sets the `vertex_input_dynamic_state` field in a builder way.
    pub fn with_vertex_input_dynamic_state(mut self, vertex_input_dynamic_state: bool) -> Self {
        self.vertex_input_dynamic_state = vertex_input_dynamic_state;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    type LowLevel =
        crate::native::extensions::ext_vertex_input_dynamic_state::PhysicalDeviceVertexInputDynamicStateFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_vertex_input_dynamic_state::PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
            s_type: StructureType::PhysicalDeviceVertexInputDynamicStateFeaturesExt,
            p_next: std::ptr::null_mut(),
            vertex_input_dynamic_state: self.vertex_input_dynamic_state.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            vertex_input_dynamic_state: crate::conv::FromLowLevel::from_low_level(
                context,
                value.vertex_input_dynamic_state,
            ),
        }
    }
}
#[doc(alias = "VkVertexInputBindingDescription2EXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VertexInputBindingDescription2EXT {
    pub binding: u32,
    pub stride: u32,
    #[doc(alias = "inputRate")]
    pub input_rate: VertexInputRate,
    pub divisor: u32,
}
impl VertexInputBindingDescription2EXT {
    ///Get a reference to the `binding` field.
    pub fn binding(&self) -> u32 {
        self.binding
    }
    ///Get a reference to the `stride` field.
    pub fn stride(&self) -> u32 {
        self.stride
    }
    ///Get a reference to the `input_rate` field.
    pub fn input_rate(&self) -> VertexInputRate {
        self.input_rate
    }
    ///Get a reference to the `divisor` field.
    pub fn divisor(&self) -> u32 {
        self.divisor
    }
    ///Get a mutable reference to the `binding` field.
    pub fn binding_mut(&mut self) -> &mut u32 {
        &mut self.binding
    }
    ///Get a mutable reference to the `stride` field.
    pub fn stride_mut(&mut self) -> &mut u32 {
        &mut self.stride
    }
    ///Get a mutable reference to the `input_rate` field.
    pub fn input_rate_mut(&mut self) -> &mut VertexInputRate {
        &mut self.input_rate
    }
    ///Get a mutable reference to the `divisor` field.
    pub fn divisor_mut(&mut self) -> &mut u32 {
        &mut self.divisor
    }
    ///Sets the `binding` field.
    pub fn set_binding(&mut self, binding: u32) -> &mut Self {
        self.binding = binding;
        self
    }
    ///Sets the `stride` field.
    pub fn set_stride(&mut self, stride: u32) -> &mut Self {
        self.stride = stride;
        self
    }
    ///Sets the `input_rate` field.
    pub fn set_input_rate(&mut self, input_rate: VertexInputRate) -> &mut Self {
        self.input_rate = input_rate;
        self
    }
    ///Sets the `divisor` field.
    pub fn set_divisor(&mut self, divisor: u32) -> &mut Self {
        self.divisor = divisor;
        self
    }
    ///Sets the `binding` field in a builder way.
    pub fn with_binding(mut self, binding: u32) -> Self {
        self.binding = binding;
        self
    }
    ///Sets the `stride` field in a builder way.
    pub fn with_stride(mut self, stride: u32) -> Self {
        self.stride = stride;
        self
    }
    ///Sets the `input_rate` field in a builder way.
    pub fn with_input_rate(mut self, input_rate: VertexInputRate) -> Self {
        self.input_rate = input_rate;
        self
    }
    ///Sets the `divisor` field in a builder way.
    pub fn with_divisor(mut self, divisor: u32) -> Self {
        self.divisor = divisor;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VertexInputBindingDescription2EXT {
    type LowLevel = crate::native::extensions::ext_vertex_input_dynamic_state::VertexInputBindingDescription2EXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_vertex_input_dynamic_state::VertexInputBindingDescription2EXT {
            s_type: StructureType::VertexInputBindingDescription2Ext,
            p_next: std::ptr::null_mut(),
            binding: self.binding.into_low_level(context, bump),
            stride: self.stride.into_low_level(context, bump),
            input_rate: self.input_rate.into_low_level(context, bump),
            divisor: self.divisor.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VertexInputBindingDescription2EXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            binding: crate::conv::FromLowLevel::from_low_level(context, value.binding),
            stride: crate::conv::FromLowLevel::from_low_level(context, value.stride),
            input_rate: crate::conv::FromLowLevel::from_low_level(context, value.input_rate),
            divisor: crate::conv::FromLowLevel::from_low_level(context, value.divisor),
        }
    }
}
#[doc(alias = "VkVertexInputAttributeDescription2EXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VertexInputAttributeDescription2EXT {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}
impl VertexInputAttributeDescription2EXT {
    ///Get a reference to the `location` field.
    pub fn location(&self) -> u32 {
        self.location
    }
    ///Get a reference to the `binding` field.
    pub fn binding(&self) -> u32 {
        self.binding
    }
    ///Get a reference to the `format` field.
    pub fn format(&self) -> Format {
        self.format
    }
    ///Get a reference to the `offset` field.
    pub fn offset(&self) -> u32 {
        self.offset
    }
    ///Get a mutable reference to the `location` field.
    pub fn location_mut(&mut self) -> &mut u32 {
        &mut self.location
    }
    ///Get a mutable reference to the `binding` field.
    pub fn binding_mut(&mut self) -> &mut u32 {
        &mut self.binding
    }
    ///Get a mutable reference to the `format` field.
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Get a mutable reference to the `offset` field.
    pub fn offset_mut(&mut self) -> &mut u32 {
        &mut self.offset
    }
    ///Sets the `location` field.
    pub fn set_location(&mut self, location: u32) -> &mut Self {
        self.location = location;
        self
    }
    ///Sets the `binding` field.
    pub fn set_binding(&mut self, binding: u32) -> &mut Self {
        self.binding = binding;
        self
    }
    ///Sets the `format` field.
    pub fn set_format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }
    ///Sets the `offset` field.
    pub fn set_offset(&mut self, offset: u32) -> &mut Self {
        self.offset = offset;
        self
    }
    ///Sets the `location` field in a builder way.
    pub fn with_location(mut self, location: u32) -> Self {
        self.location = location;
        self
    }
    ///Sets the `binding` field in a builder way.
    pub fn with_binding(mut self, binding: u32) -> Self {
        self.binding = binding;
        self
    }
    ///Sets the `format` field in a builder way.
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }
    ///Sets the `offset` field in a builder way.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VertexInputAttributeDescription2EXT {
    type LowLevel = crate::native::extensions::ext_vertex_input_dynamic_state::VertexInputAttributeDescription2EXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_vertex_input_dynamic_state::VertexInputAttributeDescription2EXT {
            s_type: StructureType::VertexInputAttributeDescription2Ext,
            p_next: std::ptr::null_mut(),
            location: self.location.into_low_level(context, bump),
            binding: self.binding.into_low_level(context, bump),
            format: self.format.into_low_level(context, bump),
            offset: self.offset.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VertexInputAttributeDescription2EXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            location: crate::conv::FromLowLevel::from_low_level(context, value.location),
            binding: crate::conv::FromLowLevel::from_low_level(context, value.binding),
            format: crate::conv::FromLowLevel::from_low_level(context, value.format),
            offset: crate::conv::FromLowLevel::from_low_level(context, value.offset),
        }
    }
}
