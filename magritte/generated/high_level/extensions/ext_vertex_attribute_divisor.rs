pub use crate::common::extensions::ext_vertex_attribute_divisor::{
    VertexInputBindingDivisorDescriptionEXT, EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME,
    EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
impl VertexInputBindingDivisorDescriptionEXT {
    ///Get a reference to the `binding` field.
    pub fn binding(&self) -> u32 {
        self.binding
    }
    ///Get a reference to the `divisor` field.
    pub fn divisor(&self) -> u32 {
        self.divisor
    }
    ///Get a mutable reference to the `binding` field.
    pub fn binding_mut(&mut self) -> &mut u32 {
        &mut self.binding
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
    ///Sets the `divisor` field in a builder way.
    pub fn with_divisor(mut self, divisor: u32) -> Self {
        self.divisor = divisor;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for VertexInputBindingDivisorDescriptionEXT {
    type LowLevel = crate::native::extensions::ext_vertex_attribute_divisor::VertexInputBindingDivisorDescriptionEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_vertex_attribute_divisor::VertexInputBindingDivisorDescriptionEXT {
            binding: self.binding.into_low_level(context, bump),
            divisor: self.divisor.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for VertexInputBindingDivisorDescriptionEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            binding: crate::conv::FromLowLevel::from_low_level(context, value.binding),
            divisor: crate::conv::FromLowLevel::from_low_level(context, value.divisor),
        }
    }
}
#[doc(alias = "VkPipelineVertexInputDivisorStateCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineVertexInputDivisorStateCreateInfoEXT {
    #[doc(alias = "pVertexBindingDivisors")]
    pub vertex_binding_divisors: SmallVec<[VertexInputBindingDivisorDescriptionEXT; 8]>,
}
impl PipelineVertexInputDivisorStateCreateInfoEXT {
    ///Get a reference to the `vertex_binding_divisors` field.
    pub fn vertex_binding_divisors(&self) -> &SmallVec<[VertexInputBindingDivisorDescriptionEXT; 8]> {
        &self.vertex_binding_divisors
    }
    ///Get a mutable reference to the `vertex_binding_divisors` field.
    pub fn vertex_binding_divisors_mut(&mut self) -> &mut SmallVec<[VertexInputBindingDivisorDescriptionEXT; 8]> {
        &mut self.vertex_binding_divisors
    }
    ///Sets the `vertex_binding_divisors` field.
    pub fn set_vertex_binding_divisors(
        &mut self,
        vertex_binding_divisors: SmallVec<[VertexInputBindingDivisorDescriptionEXT; 8]>,
    ) -> &mut Self {
        self.vertex_binding_divisors = vertex_binding_divisors;
        self
    }
    ///Sets the `vertex_binding_divisors` field in a builder way.
    pub fn with_vertex_binding_divisors(
        mut self,
        vertex_binding_divisors: SmallVec<[VertexInputBindingDivisorDescriptionEXT; 8]>,
    ) -> Self {
        self.vertex_binding_divisors = vertex_binding_divisors;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineVertexInputDivisorStateCreateInfoEXT {
    type LowLevel =
        crate::native::extensions::ext_vertex_attribute_divisor::PipelineVertexInputDivisorStateCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_vertex_binding_divisors = self.vertex_binding_divisors.len() as u32;
        let vertex_binding_divisors = bump
            .alloc_slice_fill_iter(
                self.vertex_binding_divisors
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::ext_vertex_attribute_divisor::PipelineVertexInputDivisorStateCreateInfoEXT {
            s_type: StructureType::PipelineVertexInputDivisorStateCreateInfoExt,
            p_next: std::ptr::null(),
            vertex_binding_divisor_count: len_vertex_binding_divisors,
            vertex_binding_divisors: vertex_binding_divisors,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineVertexInputDivisorStateCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let vertex_binding_divisors_len = value.vertex_binding_divisor_count;
        let mut vertex_binding_divisors = SmallVec::with_capacity(vertex_binding_divisors_len as usize);
        for i in 0..vertex_binding_divisors_len {
            vertex_binding_divisors.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.vertex_binding_divisors.add(i as usize).read(),
            ));
        }
        Self {
            vertex_binding_divisors: vertex_binding_divisors,
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    #[doc(alias = "maxVertexAttribDivisor")]
    pub max_vertex_attrib_divisor: u32,
}
impl PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    ///Get a reference to the `max_vertex_attrib_divisor` field.
    pub fn max_vertex_attrib_divisor(&self) -> u32 {
        self.max_vertex_attrib_divisor
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    type LowLevel =
        crate::native::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
            s_type: StructureType::PhysicalDeviceVertexAttributeDivisorPropertiesExt,
            p_next: std::ptr::null_mut(),
            max_vertex_attrib_divisor: self.max_vertex_attrib_divisor.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_vertex_attrib_divisor: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_vertex_attrib_divisor,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    #[doc(alias = "vertexAttributeInstanceRateDivisor")]
    pub vertex_attribute_instance_rate_divisor: bool,
    #[doc(alias = "vertexAttributeInstanceRateZeroDivisor")]
    pub vertex_attribute_instance_rate_zero_divisor: bool,
}
impl PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    ///Get a reference to the `vertex_attribute_instance_rate_divisor` field.
    pub fn vertex_attribute_instance_rate_divisor(&self) -> &bool {
        &self.vertex_attribute_instance_rate_divisor
    }
    ///Get a reference to the `vertex_attribute_instance_rate_zero_divisor` field.
    pub fn vertex_attribute_instance_rate_zero_divisor(&self) -> &bool {
        &self.vertex_attribute_instance_rate_zero_divisor
    }
    ///Get a mutable reference to the `vertex_attribute_instance_rate_divisor` field.
    pub fn vertex_attribute_instance_rate_divisor_mut(&mut self) -> &mut bool {
        &mut self.vertex_attribute_instance_rate_divisor
    }
    ///Get a mutable reference to the `vertex_attribute_instance_rate_zero_divisor` field.
    pub fn vertex_attribute_instance_rate_zero_divisor_mut(&mut self) -> &mut bool {
        &mut self.vertex_attribute_instance_rate_zero_divisor
    }
    ///Sets the `vertex_attribute_instance_rate_divisor` field.
    pub fn set_vertex_attribute_instance_rate_divisor(
        &mut self,
        vertex_attribute_instance_rate_divisor: bool,
    ) -> &mut Self {
        self.vertex_attribute_instance_rate_divisor = vertex_attribute_instance_rate_divisor;
        self
    }
    ///Sets the `vertex_attribute_instance_rate_zero_divisor` field.
    pub fn set_vertex_attribute_instance_rate_zero_divisor(
        &mut self,
        vertex_attribute_instance_rate_zero_divisor: bool,
    ) -> &mut Self {
        self.vertex_attribute_instance_rate_zero_divisor = vertex_attribute_instance_rate_zero_divisor;
        self
    }
    ///Sets the `vertex_attribute_instance_rate_divisor` field in a builder way.
    pub fn with_vertex_attribute_instance_rate_divisor(mut self, vertex_attribute_instance_rate_divisor: bool) -> Self {
        self.vertex_attribute_instance_rate_divisor = vertex_attribute_instance_rate_divisor;
        self
    }
    ///Sets the `vertex_attribute_instance_rate_zero_divisor` field in a builder way.
    pub fn with_vertex_attribute_instance_rate_zero_divisor(
        mut self,
        vertex_attribute_instance_rate_zero_divisor: bool,
    ) -> Self {
        self.vertex_attribute_instance_rate_zero_divisor = vertex_attribute_instance_rate_zero_divisor;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    type LowLevel =
        crate::native::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
            s_type: StructureType::PhysicalDeviceVertexAttributeDivisorFeaturesExt,
            p_next: std::ptr::null_mut(),
            vertex_attribute_instance_rate_divisor: self
                .vertex_attribute_instance_rate_divisor
                .into_low_level(context, bump),
            vertex_attribute_instance_rate_zero_divisor: self
                .vertex_attribute_instance_rate_zero_divisor
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            vertex_attribute_instance_rate_divisor: crate::conv::FromLowLevel::from_low_level(
                context,
                value.vertex_attribute_instance_rate_divisor,
            ),
            vertex_attribute_instance_rate_zero_divisor: crate::conv::FromLowLevel::from_low_level(
                context,
                value.vertex_attribute_instance_rate_zero_divisor,
            ),
        }
    }
}
