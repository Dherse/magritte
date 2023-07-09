pub use crate::common::extensions::khr_pipeline_library::{
    KHR_PIPELINE_LIBRARY_EXTENSION_NAME, KHR_PIPELINE_LIBRARY_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Pipeline, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPipelineLibraryCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineLibraryCreateInfoKHR {
    #[doc(alias = "pLibraries")]
    pub libraries: SmallVec<[Pipeline; 8]>,
}
impl PipelineLibraryCreateInfoKHR {
    ///Get a reference to the `libraries` field.
    pub fn libraries(&self) -> &SmallVec<[Pipeline; 8]> {
        &self.libraries
    }
    ///Get a mutable reference to the `libraries` field.
    pub fn libraries_mut(&mut self) -> &mut SmallVec<[Pipeline; 8]> {
        &mut self.libraries
    }
    ///Sets the `libraries` field.
    pub fn set_libraries(&mut self, libraries: SmallVec<[Pipeline; 8]>) -> &mut Self {
        self.libraries = libraries;
        self
    }
    ///Sets the `libraries` field in a builder way.
    pub fn with_libraries(mut self, libraries: SmallVec<[Pipeline; 8]>) -> Self {
        self.libraries = libraries;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineLibraryCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_libraries = self.libraries.len() as u32;
        let libraries = bump
            .alloc_slice_fill_iter(self.libraries.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR {
            s_type: StructureType::PipelineLibraryCreateInfoKhr,
            p_next: std::ptr::null(),
            library_count: len_libraries,
            libraries: libraries,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineLibraryCreateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let libraries_len = value.library_count;
        let mut libraries = SmallVec::with_capacity(libraries_len as usize);
        for i in 0..libraries_len {
            libraries.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.libraries.add(i as usize).read(),
            ));
        }
        Self { libraries: libraries }
    }
}
