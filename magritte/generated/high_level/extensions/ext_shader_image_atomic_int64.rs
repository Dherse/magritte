pub use crate::common::extensions::ext_shader_image_atomic_int64::{
    EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME, EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    #[doc(alias = "shaderImageInt64Atomics")]
    pub shader_image_int64_atomics: bool,
    #[doc(alias = "sparseImageInt64Atomics")]
    pub sparse_image_int64_atomics: bool,
}
impl PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    ///Get a reference to the `shader_image_int64_atomics` field.
    pub fn shader_image_int64_atomics(&self) -> &bool {
        &self.shader_image_int64_atomics
    }
    ///Get a reference to the `sparse_image_int64_atomics` field.
    pub fn sparse_image_int64_atomics(&self) -> &bool {
        &self.sparse_image_int64_atomics
    }
    ///Get a mutable reference to the `shader_image_int64_atomics` field.
    pub fn shader_image_int64_atomics_mut(&mut self) -> &mut bool {
        &mut self.shader_image_int64_atomics
    }
    ///Get a mutable reference to the `sparse_image_int64_atomics` field.
    pub fn sparse_image_int64_atomics_mut(&mut self) -> &mut bool {
        &mut self.sparse_image_int64_atomics
    }
    ///Sets the `shader_image_int64_atomics` field.
    pub fn set_shader_image_int64_atomics(&mut self, shader_image_int64_atomics: bool) -> &mut Self {
        self.shader_image_int64_atomics = shader_image_int64_atomics;
        self
    }
    ///Sets the `sparse_image_int64_atomics` field.
    pub fn set_sparse_image_int64_atomics(&mut self, sparse_image_int64_atomics: bool) -> &mut Self {
        self.sparse_image_int64_atomics = sparse_image_int64_atomics;
        self
    }
    ///Sets the `shader_image_int64_atomics` field in a builder way.
    pub fn with_shader_image_int64_atomics(mut self, shader_image_int64_atomics: bool) -> Self {
        self.shader_image_int64_atomics = shader_image_int64_atomics;
        self
    }
    ///Sets the `sparse_image_int64_atomics` field in a builder way.
    pub fn with_sparse_image_int64_atomics(mut self, sparse_image_int64_atomics: bool) -> Self {
        self.sparse_image_int64_atomics = sparse_image_int64_atomics;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    type LowLevel =
        crate::native::extensions::ext_shader_image_atomic_int64::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_shader_image_atomic_int64::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
            s_type: StructureType::PhysicalDeviceShaderImageAtomicInt64FeaturesExt,
            p_next: std::ptr::null_mut(),
            shader_image_int64_atomics: self.shader_image_int64_atomics.into_low_level(context, bump),
            sparse_image_int64_atomics: self.sparse_image_int64_atomics.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_image_int64_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_image_int64_atomics,
            ),
            sparse_image_int64_atomics: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sparse_image_int64_atomics,
            ),
        }
    }
}
