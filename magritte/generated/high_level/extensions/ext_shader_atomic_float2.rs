pub use crate::common::extensions::ext_shader_atomic_float2::{
    EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME, EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    #[doc(alias = "shaderBufferFloat16Atomics")]
    pub shader_buffer_float16_atomics: bool,
    #[doc(alias = "shaderBufferFloat16AtomicAdd")]
    pub shader_buffer_float16_atomic_add: bool,
    #[doc(alias = "shaderBufferFloat16AtomicMinMax")]
    pub shader_buffer_float16_atomic_min_max: bool,
    #[doc(alias = "shaderBufferFloat32AtomicMinMax")]
    pub shader_buffer_float32_atomic_min_max: bool,
    #[doc(alias = "shaderBufferFloat64AtomicMinMax")]
    pub shader_buffer_float64_atomic_min_max: bool,
    #[doc(alias = "shaderSharedFloat16Atomics")]
    pub shader_shared_float16_atomics: bool,
    #[doc(alias = "shaderSharedFloat16AtomicAdd")]
    pub shader_shared_float16_atomic_add: bool,
    #[doc(alias = "shaderSharedFloat16AtomicMinMax")]
    pub shader_shared_float16_atomic_min_max: bool,
    #[doc(alias = "shaderSharedFloat32AtomicMinMax")]
    pub shader_shared_float32_atomic_min_max: bool,
    #[doc(alias = "shaderSharedFloat64AtomicMinMax")]
    pub shader_shared_float64_atomic_min_max: bool,
    #[doc(alias = "shaderImageFloat32AtomicMinMax")]
    pub shader_image_float32_atomic_min_max: bool,
    #[doc(alias = "sparseImageFloat32AtomicMinMax")]
    pub sparse_image_float32_atomic_min_max: bool,
}
impl PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    ///Get a reference to the `shader_buffer_float16_atomics` field.
    pub fn shader_buffer_float16_atomics(&self) -> &bool {
        &self.shader_buffer_float16_atomics
    }
    ///Get a reference to the `shader_buffer_float16_atomic_add` field.
    pub fn shader_buffer_float16_atomic_add(&self) -> &bool {
        &self.shader_buffer_float16_atomic_add
    }
    ///Get a reference to the `shader_buffer_float16_atomic_min_max` field.
    pub fn shader_buffer_float16_atomic_min_max(&self) -> &bool {
        &self.shader_buffer_float16_atomic_min_max
    }
    ///Get a reference to the `shader_buffer_float32_atomic_min_max` field.
    pub fn shader_buffer_float32_atomic_min_max(&self) -> &bool {
        &self.shader_buffer_float32_atomic_min_max
    }
    ///Get a reference to the `shader_buffer_float64_atomic_min_max` field.
    pub fn shader_buffer_float64_atomic_min_max(&self) -> &bool {
        &self.shader_buffer_float64_atomic_min_max
    }
    ///Get a reference to the `shader_shared_float16_atomics` field.
    pub fn shader_shared_float16_atomics(&self) -> &bool {
        &self.shader_shared_float16_atomics
    }
    ///Get a reference to the `shader_shared_float16_atomic_add` field.
    pub fn shader_shared_float16_atomic_add(&self) -> &bool {
        &self.shader_shared_float16_atomic_add
    }
    ///Get a reference to the `shader_shared_float16_atomic_min_max` field.
    pub fn shader_shared_float16_atomic_min_max(&self) -> &bool {
        &self.shader_shared_float16_atomic_min_max
    }
    ///Get a reference to the `shader_shared_float32_atomic_min_max` field.
    pub fn shader_shared_float32_atomic_min_max(&self) -> &bool {
        &self.shader_shared_float32_atomic_min_max
    }
    ///Get a reference to the `shader_shared_float64_atomic_min_max` field.
    pub fn shader_shared_float64_atomic_min_max(&self) -> &bool {
        &self.shader_shared_float64_atomic_min_max
    }
    ///Get a reference to the `shader_image_float32_atomic_min_max` field.
    pub fn shader_image_float32_atomic_min_max(&self) -> &bool {
        &self.shader_image_float32_atomic_min_max
    }
    ///Get a reference to the `sparse_image_float32_atomic_min_max` field.
    pub fn sparse_image_float32_atomic_min_max(&self) -> &bool {
        &self.sparse_image_float32_atomic_min_max
    }
    ///Get a mutable reference to the `shader_buffer_float16_atomics` field.
    pub fn shader_buffer_float16_atomics_mut(&mut self) -> &mut bool {
        &mut self.shader_buffer_float16_atomics
    }
    ///Get a mutable reference to the `shader_buffer_float16_atomic_add` field.
    pub fn shader_buffer_float16_atomic_add_mut(&mut self) -> &mut bool {
        &mut self.shader_buffer_float16_atomic_add
    }
    ///Get a mutable reference to the `shader_buffer_float16_atomic_min_max` field.
    pub fn shader_buffer_float16_atomic_min_max_mut(&mut self) -> &mut bool {
        &mut self.shader_buffer_float16_atomic_min_max
    }
    ///Get a mutable reference to the `shader_buffer_float32_atomic_min_max` field.
    pub fn shader_buffer_float32_atomic_min_max_mut(&mut self) -> &mut bool {
        &mut self.shader_buffer_float32_atomic_min_max
    }
    ///Get a mutable reference to the `shader_buffer_float64_atomic_min_max` field.
    pub fn shader_buffer_float64_atomic_min_max_mut(&mut self) -> &mut bool {
        &mut self.shader_buffer_float64_atomic_min_max
    }
    ///Get a mutable reference to the `shader_shared_float16_atomics` field.
    pub fn shader_shared_float16_atomics_mut(&mut self) -> &mut bool {
        &mut self.shader_shared_float16_atomics
    }
    ///Get a mutable reference to the `shader_shared_float16_atomic_add` field.
    pub fn shader_shared_float16_atomic_add_mut(&mut self) -> &mut bool {
        &mut self.shader_shared_float16_atomic_add
    }
    ///Get a mutable reference to the `shader_shared_float16_atomic_min_max` field.
    pub fn shader_shared_float16_atomic_min_max_mut(&mut self) -> &mut bool {
        &mut self.shader_shared_float16_atomic_min_max
    }
    ///Get a mutable reference to the `shader_shared_float32_atomic_min_max` field.
    pub fn shader_shared_float32_atomic_min_max_mut(&mut self) -> &mut bool {
        &mut self.shader_shared_float32_atomic_min_max
    }
    ///Get a mutable reference to the `shader_shared_float64_atomic_min_max` field.
    pub fn shader_shared_float64_atomic_min_max_mut(&mut self) -> &mut bool {
        &mut self.shader_shared_float64_atomic_min_max
    }
    ///Get a mutable reference to the `shader_image_float32_atomic_min_max` field.
    pub fn shader_image_float32_atomic_min_max_mut(&mut self) -> &mut bool {
        &mut self.shader_image_float32_atomic_min_max
    }
    ///Get a mutable reference to the `sparse_image_float32_atomic_min_max` field.
    pub fn sparse_image_float32_atomic_min_max_mut(&mut self) -> &mut bool {
        &mut self.sparse_image_float32_atomic_min_max
    }
    ///Sets the `shader_buffer_float16_atomics` field.
    pub fn set_shader_buffer_float16_atomics(&mut self, shader_buffer_float16_atomics: bool) -> &mut Self {
        self.shader_buffer_float16_atomics = shader_buffer_float16_atomics;
        self
    }
    ///Sets the `shader_buffer_float16_atomic_add` field.
    pub fn set_shader_buffer_float16_atomic_add(&mut self, shader_buffer_float16_atomic_add: bool) -> &mut Self {
        self.shader_buffer_float16_atomic_add = shader_buffer_float16_atomic_add;
        self
    }
    ///Sets the `shader_buffer_float16_atomic_min_max` field.
    pub fn set_shader_buffer_float16_atomic_min_max(
        &mut self,
        shader_buffer_float16_atomic_min_max: bool,
    ) -> &mut Self {
        self.shader_buffer_float16_atomic_min_max = shader_buffer_float16_atomic_min_max;
        self
    }
    ///Sets the `shader_buffer_float32_atomic_min_max` field.
    pub fn set_shader_buffer_float32_atomic_min_max(
        &mut self,
        shader_buffer_float32_atomic_min_max: bool,
    ) -> &mut Self {
        self.shader_buffer_float32_atomic_min_max = shader_buffer_float32_atomic_min_max;
        self
    }
    ///Sets the `shader_buffer_float64_atomic_min_max` field.
    pub fn set_shader_buffer_float64_atomic_min_max(
        &mut self,
        shader_buffer_float64_atomic_min_max: bool,
    ) -> &mut Self {
        self.shader_buffer_float64_atomic_min_max = shader_buffer_float64_atomic_min_max;
        self
    }
    ///Sets the `shader_shared_float16_atomics` field.
    pub fn set_shader_shared_float16_atomics(&mut self, shader_shared_float16_atomics: bool) -> &mut Self {
        self.shader_shared_float16_atomics = shader_shared_float16_atomics;
        self
    }
    ///Sets the `shader_shared_float16_atomic_add` field.
    pub fn set_shader_shared_float16_atomic_add(&mut self, shader_shared_float16_atomic_add: bool) -> &mut Self {
        self.shader_shared_float16_atomic_add = shader_shared_float16_atomic_add;
        self
    }
    ///Sets the `shader_shared_float16_atomic_min_max` field.
    pub fn set_shader_shared_float16_atomic_min_max(
        &mut self,
        shader_shared_float16_atomic_min_max: bool,
    ) -> &mut Self {
        self.shader_shared_float16_atomic_min_max = shader_shared_float16_atomic_min_max;
        self
    }
    ///Sets the `shader_shared_float32_atomic_min_max` field.
    pub fn set_shader_shared_float32_atomic_min_max(
        &mut self,
        shader_shared_float32_atomic_min_max: bool,
    ) -> &mut Self {
        self.shader_shared_float32_atomic_min_max = shader_shared_float32_atomic_min_max;
        self
    }
    ///Sets the `shader_shared_float64_atomic_min_max` field.
    pub fn set_shader_shared_float64_atomic_min_max(
        &mut self,
        shader_shared_float64_atomic_min_max: bool,
    ) -> &mut Self {
        self.shader_shared_float64_atomic_min_max = shader_shared_float64_atomic_min_max;
        self
    }
    ///Sets the `shader_image_float32_atomic_min_max` field.
    pub fn set_shader_image_float32_atomic_min_max(&mut self, shader_image_float32_atomic_min_max: bool) -> &mut Self {
        self.shader_image_float32_atomic_min_max = shader_image_float32_atomic_min_max;
        self
    }
    ///Sets the `sparse_image_float32_atomic_min_max` field.
    pub fn set_sparse_image_float32_atomic_min_max(&mut self, sparse_image_float32_atomic_min_max: bool) -> &mut Self {
        self.sparse_image_float32_atomic_min_max = sparse_image_float32_atomic_min_max;
        self
    }
    ///Sets the `shader_buffer_float16_atomics` field in a builder way.
    pub fn with_shader_buffer_float16_atomics(mut self, shader_buffer_float16_atomics: bool) -> Self {
        self.shader_buffer_float16_atomics = shader_buffer_float16_atomics;
        self
    }
    ///Sets the `shader_buffer_float16_atomic_add` field in a builder way.
    pub fn with_shader_buffer_float16_atomic_add(mut self, shader_buffer_float16_atomic_add: bool) -> Self {
        self.shader_buffer_float16_atomic_add = shader_buffer_float16_atomic_add;
        self
    }
    ///Sets the `shader_buffer_float16_atomic_min_max` field in a builder way.
    pub fn with_shader_buffer_float16_atomic_min_max(mut self, shader_buffer_float16_atomic_min_max: bool) -> Self {
        self.shader_buffer_float16_atomic_min_max = shader_buffer_float16_atomic_min_max;
        self
    }
    ///Sets the `shader_buffer_float32_atomic_min_max` field in a builder way.
    pub fn with_shader_buffer_float32_atomic_min_max(mut self, shader_buffer_float32_atomic_min_max: bool) -> Self {
        self.shader_buffer_float32_atomic_min_max = shader_buffer_float32_atomic_min_max;
        self
    }
    ///Sets the `shader_buffer_float64_atomic_min_max` field in a builder way.
    pub fn with_shader_buffer_float64_atomic_min_max(mut self, shader_buffer_float64_atomic_min_max: bool) -> Self {
        self.shader_buffer_float64_atomic_min_max = shader_buffer_float64_atomic_min_max;
        self
    }
    ///Sets the `shader_shared_float16_atomics` field in a builder way.
    pub fn with_shader_shared_float16_atomics(mut self, shader_shared_float16_atomics: bool) -> Self {
        self.shader_shared_float16_atomics = shader_shared_float16_atomics;
        self
    }
    ///Sets the `shader_shared_float16_atomic_add` field in a builder way.
    pub fn with_shader_shared_float16_atomic_add(mut self, shader_shared_float16_atomic_add: bool) -> Self {
        self.shader_shared_float16_atomic_add = shader_shared_float16_atomic_add;
        self
    }
    ///Sets the `shader_shared_float16_atomic_min_max` field in a builder way.
    pub fn with_shader_shared_float16_atomic_min_max(mut self, shader_shared_float16_atomic_min_max: bool) -> Self {
        self.shader_shared_float16_atomic_min_max = shader_shared_float16_atomic_min_max;
        self
    }
    ///Sets the `shader_shared_float32_atomic_min_max` field in a builder way.
    pub fn with_shader_shared_float32_atomic_min_max(mut self, shader_shared_float32_atomic_min_max: bool) -> Self {
        self.shader_shared_float32_atomic_min_max = shader_shared_float32_atomic_min_max;
        self
    }
    ///Sets the `shader_shared_float64_atomic_min_max` field in a builder way.
    pub fn with_shader_shared_float64_atomic_min_max(mut self, shader_shared_float64_atomic_min_max: bool) -> Self {
        self.shader_shared_float64_atomic_min_max = shader_shared_float64_atomic_min_max;
        self
    }
    ///Sets the `shader_image_float32_atomic_min_max` field in a builder way.
    pub fn with_shader_image_float32_atomic_min_max(mut self, shader_image_float32_atomic_min_max: bool) -> Self {
        self.shader_image_float32_atomic_min_max = shader_image_float32_atomic_min_max;
        self
    }
    ///Sets the `sparse_image_float32_atomic_min_max` field in a builder way.
    pub fn with_sparse_image_float32_atomic_min_max(mut self, sparse_image_float32_atomic_min_max: bool) -> Self {
        self.sparse_image_float32_atomic_min_max = sparse_image_float32_atomic_min_max;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    type LowLevel = crate::native::extensions::ext_shader_atomic_float2::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_shader_atomic_float2::PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
            s_type: StructureType::PhysicalDeviceShaderAtomicFloat2FeaturesExt,
            p_next: std::ptr::null_mut(),
            shader_buffer_float16_atomics: self.shader_buffer_float16_atomics.into_low_level(context, bump),
            shader_buffer_float16_atomic_add: self.shader_buffer_float16_atomic_add.into_low_level(context, bump),
            shader_buffer_float16_atomic_min_max: self
                .shader_buffer_float16_atomic_min_max
                .into_low_level(context, bump),
            shader_buffer_float32_atomic_min_max: self
                .shader_buffer_float32_atomic_min_max
                .into_low_level(context, bump),
            shader_buffer_float64_atomic_min_max: self
                .shader_buffer_float64_atomic_min_max
                .into_low_level(context, bump),
            shader_shared_float16_atomics: self.shader_shared_float16_atomics.into_low_level(context, bump),
            shader_shared_float16_atomic_add: self.shader_shared_float16_atomic_add.into_low_level(context, bump),
            shader_shared_float16_atomic_min_max: self
                .shader_shared_float16_atomic_min_max
                .into_low_level(context, bump),
            shader_shared_float32_atomic_min_max: self
                .shader_shared_float32_atomic_min_max
                .into_low_level(context, bump),
            shader_shared_float64_atomic_min_max: self
                .shader_shared_float64_atomic_min_max
                .into_low_level(context, bump),
            shader_image_float32_atomic_min_max: self.shader_image_float32_atomic_min_max.into_low_level(context, bump),
            sparse_image_float32_atomic_min_max: self.sparse_image_float32_atomic_min_max.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_buffer_float16_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_buffer_float16_atomics,
            ),
            shader_buffer_float16_atomic_add: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_buffer_float16_atomic_add,
            ),
            shader_buffer_float16_atomic_min_max: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_buffer_float16_atomic_min_max,
            ),
            shader_buffer_float32_atomic_min_max: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_buffer_float32_atomic_min_max,
            ),
            shader_buffer_float64_atomic_min_max: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_buffer_float64_atomic_min_max,
            ),
            shader_shared_float16_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_shared_float16_atomics,
            ),
            shader_shared_float16_atomic_add: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_shared_float16_atomic_add,
            ),
            shader_shared_float16_atomic_min_max: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_shared_float16_atomic_min_max,
            ),
            shader_shared_float32_atomic_min_max: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_shared_float32_atomic_min_max,
            ),
            shader_shared_float64_atomic_min_max: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_shared_float64_atomic_min_max,
            ),
            shader_image_float32_atomic_min_max: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_image_float32_atomic_min_max,
            ),
            sparse_image_float32_atomic_min_max: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sparse_image_float32_atomic_min_max,
            ),
        }
    }
}
