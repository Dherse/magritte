pub use crate::common::extensions::ext_shader_atomic_float::{
    EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME, EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloatFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    #[doc(alias = "shaderBufferFloat32Atomics")]
    pub shader_buffer_float32_atomics: bool,
    #[doc(alias = "shaderBufferFloat32AtomicAdd")]
    pub shader_buffer_float32_atomic_add: bool,
    #[doc(alias = "shaderBufferFloat64Atomics")]
    pub shader_buffer_float64_atomics: bool,
    #[doc(alias = "shaderBufferFloat64AtomicAdd")]
    pub shader_buffer_float64_atomic_add: bool,
    #[doc(alias = "shaderSharedFloat32Atomics")]
    pub shader_shared_float32_atomics: bool,
    #[doc(alias = "shaderSharedFloat32AtomicAdd")]
    pub shader_shared_float32_atomic_add: bool,
    #[doc(alias = "shaderSharedFloat64Atomics")]
    pub shader_shared_float64_atomics: bool,
    #[doc(alias = "shaderSharedFloat64AtomicAdd")]
    pub shader_shared_float64_atomic_add: bool,
    #[doc(alias = "shaderImageFloat32Atomics")]
    pub shader_image_float32_atomics: bool,
    #[doc(alias = "shaderImageFloat32AtomicAdd")]
    pub shader_image_float32_atomic_add: bool,
    #[doc(alias = "sparseImageFloat32Atomics")]
    pub sparse_image_float32_atomics: bool,
    #[doc(alias = "sparseImageFloat32AtomicAdd")]
    pub sparse_image_float32_atomic_add: bool,
}
impl PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    ///Get a reference to the `shader_buffer_float32_atomics` field.
    pub fn shader_buffer_float32_atomics(&self) -> &bool {
        &self.shader_buffer_float32_atomics
    }
    ///Get a reference to the `shader_buffer_float32_atomic_add` field.
    pub fn shader_buffer_float32_atomic_add(&self) -> &bool {
        &self.shader_buffer_float32_atomic_add
    }
    ///Get a reference to the `shader_buffer_float64_atomics` field.
    pub fn shader_buffer_float64_atomics(&self) -> &bool {
        &self.shader_buffer_float64_atomics
    }
    ///Get a reference to the `shader_buffer_float64_atomic_add` field.
    pub fn shader_buffer_float64_atomic_add(&self) -> &bool {
        &self.shader_buffer_float64_atomic_add
    }
    ///Get a reference to the `shader_shared_float32_atomics` field.
    pub fn shader_shared_float32_atomics(&self) -> &bool {
        &self.shader_shared_float32_atomics
    }
    ///Get a reference to the `shader_shared_float32_atomic_add` field.
    pub fn shader_shared_float32_atomic_add(&self) -> &bool {
        &self.shader_shared_float32_atomic_add
    }
    ///Get a reference to the `shader_shared_float64_atomics` field.
    pub fn shader_shared_float64_atomics(&self) -> &bool {
        &self.shader_shared_float64_atomics
    }
    ///Get a reference to the `shader_shared_float64_atomic_add` field.
    pub fn shader_shared_float64_atomic_add(&self) -> &bool {
        &self.shader_shared_float64_atomic_add
    }
    ///Get a reference to the `shader_image_float32_atomics` field.
    pub fn shader_image_float32_atomics(&self) -> &bool {
        &self.shader_image_float32_atomics
    }
    ///Get a reference to the `shader_image_float32_atomic_add` field.
    pub fn shader_image_float32_atomic_add(&self) -> &bool {
        &self.shader_image_float32_atomic_add
    }
    ///Get a reference to the `sparse_image_float32_atomics` field.
    pub fn sparse_image_float32_atomics(&self) -> &bool {
        &self.sparse_image_float32_atomics
    }
    ///Get a reference to the `sparse_image_float32_atomic_add` field.
    pub fn sparse_image_float32_atomic_add(&self) -> &bool {
        &self.sparse_image_float32_atomic_add
    }
    ///Get a mutable reference to the `shader_buffer_float32_atomics` field.
    pub fn shader_buffer_float32_atomics_mut(&mut self) -> &mut bool {
        &mut self.shader_buffer_float32_atomics
    }
    ///Get a mutable reference to the `shader_buffer_float32_atomic_add` field.
    pub fn shader_buffer_float32_atomic_add_mut(&mut self) -> &mut bool {
        &mut self.shader_buffer_float32_atomic_add
    }
    ///Get a mutable reference to the `shader_buffer_float64_atomics` field.
    pub fn shader_buffer_float64_atomics_mut(&mut self) -> &mut bool {
        &mut self.shader_buffer_float64_atomics
    }
    ///Get a mutable reference to the `shader_buffer_float64_atomic_add` field.
    pub fn shader_buffer_float64_atomic_add_mut(&mut self) -> &mut bool {
        &mut self.shader_buffer_float64_atomic_add
    }
    ///Get a mutable reference to the `shader_shared_float32_atomics` field.
    pub fn shader_shared_float32_atomics_mut(&mut self) -> &mut bool {
        &mut self.shader_shared_float32_atomics
    }
    ///Get a mutable reference to the `shader_shared_float32_atomic_add` field.
    pub fn shader_shared_float32_atomic_add_mut(&mut self) -> &mut bool {
        &mut self.shader_shared_float32_atomic_add
    }
    ///Get a mutable reference to the `shader_shared_float64_atomics` field.
    pub fn shader_shared_float64_atomics_mut(&mut self) -> &mut bool {
        &mut self.shader_shared_float64_atomics
    }
    ///Get a mutable reference to the `shader_shared_float64_atomic_add` field.
    pub fn shader_shared_float64_atomic_add_mut(&mut self) -> &mut bool {
        &mut self.shader_shared_float64_atomic_add
    }
    ///Get a mutable reference to the `shader_image_float32_atomics` field.
    pub fn shader_image_float32_atomics_mut(&mut self) -> &mut bool {
        &mut self.shader_image_float32_atomics
    }
    ///Get a mutable reference to the `shader_image_float32_atomic_add` field.
    pub fn shader_image_float32_atomic_add_mut(&mut self) -> &mut bool {
        &mut self.shader_image_float32_atomic_add
    }
    ///Get a mutable reference to the `sparse_image_float32_atomics` field.
    pub fn sparse_image_float32_atomics_mut(&mut self) -> &mut bool {
        &mut self.sparse_image_float32_atomics
    }
    ///Get a mutable reference to the `sparse_image_float32_atomic_add` field.
    pub fn sparse_image_float32_atomic_add_mut(&mut self) -> &mut bool {
        &mut self.sparse_image_float32_atomic_add
    }
    ///Sets the `shader_buffer_float32_atomics` field.
    pub fn set_shader_buffer_float32_atomics(&mut self, shader_buffer_float32_atomics: bool) -> &mut Self {
        self.shader_buffer_float32_atomics = shader_buffer_float32_atomics;
        self
    }
    ///Sets the `shader_buffer_float32_atomic_add` field.
    pub fn set_shader_buffer_float32_atomic_add(&mut self, shader_buffer_float32_atomic_add: bool) -> &mut Self {
        self.shader_buffer_float32_atomic_add = shader_buffer_float32_atomic_add;
        self
    }
    ///Sets the `shader_buffer_float64_atomics` field.
    pub fn set_shader_buffer_float64_atomics(&mut self, shader_buffer_float64_atomics: bool) -> &mut Self {
        self.shader_buffer_float64_atomics = shader_buffer_float64_atomics;
        self
    }
    ///Sets the `shader_buffer_float64_atomic_add` field.
    pub fn set_shader_buffer_float64_atomic_add(&mut self, shader_buffer_float64_atomic_add: bool) -> &mut Self {
        self.shader_buffer_float64_atomic_add = shader_buffer_float64_atomic_add;
        self
    }
    ///Sets the `shader_shared_float32_atomics` field.
    pub fn set_shader_shared_float32_atomics(&mut self, shader_shared_float32_atomics: bool) -> &mut Self {
        self.shader_shared_float32_atomics = shader_shared_float32_atomics;
        self
    }
    ///Sets the `shader_shared_float32_atomic_add` field.
    pub fn set_shader_shared_float32_atomic_add(&mut self, shader_shared_float32_atomic_add: bool) -> &mut Self {
        self.shader_shared_float32_atomic_add = shader_shared_float32_atomic_add;
        self
    }
    ///Sets the `shader_shared_float64_atomics` field.
    pub fn set_shader_shared_float64_atomics(&mut self, shader_shared_float64_atomics: bool) -> &mut Self {
        self.shader_shared_float64_atomics = shader_shared_float64_atomics;
        self
    }
    ///Sets the `shader_shared_float64_atomic_add` field.
    pub fn set_shader_shared_float64_atomic_add(&mut self, shader_shared_float64_atomic_add: bool) -> &mut Self {
        self.shader_shared_float64_atomic_add = shader_shared_float64_atomic_add;
        self
    }
    ///Sets the `shader_image_float32_atomics` field.
    pub fn set_shader_image_float32_atomics(&mut self, shader_image_float32_atomics: bool) -> &mut Self {
        self.shader_image_float32_atomics = shader_image_float32_atomics;
        self
    }
    ///Sets the `shader_image_float32_atomic_add` field.
    pub fn set_shader_image_float32_atomic_add(&mut self, shader_image_float32_atomic_add: bool) -> &mut Self {
        self.shader_image_float32_atomic_add = shader_image_float32_atomic_add;
        self
    }
    ///Sets the `sparse_image_float32_atomics` field.
    pub fn set_sparse_image_float32_atomics(&mut self, sparse_image_float32_atomics: bool) -> &mut Self {
        self.sparse_image_float32_atomics = sparse_image_float32_atomics;
        self
    }
    ///Sets the `sparse_image_float32_atomic_add` field.
    pub fn set_sparse_image_float32_atomic_add(&mut self, sparse_image_float32_atomic_add: bool) -> &mut Self {
        self.sparse_image_float32_atomic_add = sparse_image_float32_atomic_add;
        self
    }
    ///Sets the `shader_buffer_float32_atomics` field in a builder way.
    pub fn with_shader_buffer_float32_atomics(mut self, shader_buffer_float32_atomics: bool) -> Self {
        self.shader_buffer_float32_atomics = shader_buffer_float32_atomics;
        self
    }
    ///Sets the `shader_buffer_float32_atomic_add` field in a builder way.
    pub fn with_shader_buffer_float32_atomic_add(mut self, shader_buffer_float32_atomic_add: bool) -> Self {
        self.shader_buffer_float32_atomic_add = shader_buffer_float32_atomic_add;
        self
    }
    ///Sets the `shader_buffer_float64_atomics` field in a builder way.
    pub fn with_shader_buffer_float64_atomics(mut self, shader_buffer_float64_atomics: bool) -> Self {
        self.shader_buffer_float64_atomics = shader_buffer_float64_atomics;
        self
    }
    ///Sets the `shader_buffer_float64_atomic_add` field in a builder way.
    pub fn with_shader_buffer_float64_atomic_add(mut self, shader_buffer_float64_atomic_add: bool) -> Self {
        self.shader_buffer_float64_atomic_add = shader_buffer_float64_atomic_add;
        self
    }
    ///Sets the `shader_shared_float32_atomics` field in a builder way.
    pub fn with_shader_shared_float32_atomics(mut self, shader_shared_float32_atomics: bool) -> Self {
        self.shader_shared_float32_atomics = shader_shared_float32_atomics;
        self
    }
    ///Sets the `shader_shared_float32_atomic_add` field in a builder way.
    pub fn with_shader_shared_float32_atomic_add(mut self, shader_shared_float32_atomic_add: bool) -> Self {
        self.shader_shared_float32_atomic_add = shader_shared_float32_atomic_add;
        self
    }
    ///Sets the `shader_shared_float64_atomics` field in a builder way.
    pub fn with_shader_shared_float64_atomics(mut self, shader_shared_float64_atomics: bool) -> Self {
        self.shader_shared_float64_atomics = shader_shared_float64_atomics;
        self
    }
    ///Sets the `shader_shared_float64_atomic_add` field in a builder way.
    pub fn with_shader_shared_float64_atomic_add(mut self, shader_shared_float64_atomic_add: bool) -> Self {
        self.shader_shared_float64_atomic_add = shader_shared_float64_atomic_add;
        self
    }
    ///Sets the `shader_image_float32_atomics` field in a builder way.
    pub fn with_shader_image_float32_atomics(mut self, shader_image_float32_atomics: bool) -> Self {
        self.shader_image_float32_atomics = shader_image_float32_atomics;
        self
    }
    ///Sets the `shader_image_float32_atomic_add` field in a builder way.
    pub fn with_shader_image_float32_atomic_add(mut self, shader_image_float32_atomic_add: bool) -> Self {
        self.shader_image_float32_atomic_add = shader_image_float32_atomic_add;
        self
    }
    ///Sets the `sparse_image_float32_atomics` field in a builder way.
    pub fn with_sparse_image_float32_atomics(mut self, sparse_image_float32_atomics: bool) -> Self {
        self.sparse_image_float32_atomics = sparse_image_float32_atomics;
        self
    }
    ///Sets the `sparse_image_float32_atomic_add` field in a builder way.
    pub fn with_sparse_image_float32_atomic_add(mut self, sparse_image_float32_atomic_add: bool) -> Self {
        self.sparse_image_float32_atomic_add = sparse_image_float32_atomic_add;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_shader_atomic_float::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_shader_atomic_float::PhysicalDeviceShaderAtomicFloatFeaturesEXT {
            s_type: StructureType::PhysicalDeviceShaderAtomicFloatFeaturesExt,
            p_next: std::ptr::null_mut(),
            shader_buffer_float32_atomics: self.shader_buffer_float32_atomics.into_low_level(context, bump),
            shader_buffer_float32_atomic_add: self.shader_buffer_float32_atomic_add.into_low_level(context, bump),
            shader_buffer_float64_atomics: self.shader_buffer_float64_atomics.into_low_level(context, bump),
            shader_buffer_float64_atomic_add: self.shader_buffer_float64_atomic_add.into_low_level(context, bump),
            shader_shared_float32_atomics: self.shader_shared_float32_atomics.into_low_level(context, bump),
            shader_shared_float32_atomic_add: self.shader_shared_float32_atomic_add.into_low_level(context, bump),
            shader_shared_float64_atomics: self.shader_shared_float64_atomics.into_low_level(context, bump),
            shader_shared_float64_atomic_add: self.shader_shared_float64_atomic_add.into_low_level(context, bump),
            shader_image_float32_atomics: self.shader_image_float32_atomics.into_low_level(context, bump),
            shader_image_float32_atomic_add: self.shader_image_float32_atomic_add.into_low_level(context, bump),
            sparse_image_float32_atomics: self.sparse_image_float32_atomics.into_low_level(context, bump),
            sparse_image_float32_atomic_add: self.sparse_image_float32_atomic_add.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_buffer_float32_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_buffer_float32_atomics,
            ),
            shader_buffer_float32_atomic_add: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_buffer_float32_atomic_add,
            ),
            shader_buffer_float64_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_buffer_float64_atomics,
            ),
            shader_buffer_float64_atomic_add: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_buffer_float64_atomic_add,
            ),
            shader_shared_float32_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_shared_float32_atomics,
            ),
            shader_shared_float32_atomic_add: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_shared_float32_atomic_add,
            ),
            shader_shared_float64_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_shared_float64_atomics,
            ),
            shader_shared_float64_atomic_add: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_shared_float64_atomic_add,
            ),
            shader_image_float32_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_image_float32_atomics,
            ),
            shader_image_float32_atomic_add: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_image_float32_atomic_add,
            ),
            sparse_image_float32_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sparse_image_float32_atomics,
            ),
            sparse_image_float32_atomic_add: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sparse_image_float32_atomic_add,
            ),
        }
    }
}
