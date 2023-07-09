pub use crate::common::extensions::khr_pipeline_executable_properties::{
    PipelineExecutableStatisticFormatKHR, KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME,
    KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Pipeline, ShaderStageFlags, StructureType, MAX_DESCRIPTION_SIZE},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    #[doc(alias = "pipelineExecutableInfo")]
    pub pipeline_executable_info: bool,
}
impl PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    ///Get a reference to the `pipeline_executable_info` field.
    pub fn pipeline_executable_info(&self) -> &bool {
        &self.pipeline_executable_info
    }
    ///Get a mutable reference to the `pipeline_executable_info` field.
    pub fn pipeline_executable_info_mut(&mut self) -> &mut bool {
        &mut self.pipeline_executable_info
    }
    ///Sets the `pipeline_executable_info` field.
    pub fn set_pipeline_executable_info(&mut self, pipeline_executable_info: bool) -> &mut Self {
        self.pipeline_executable_info = pipeline_executable_info;
        self
    }
    ///Sets the `pipeline_executable_info` field in a builder way.
    pub fn with_pipeline_executable_info(mut self, pipeline_executable_info: bool) -> Self {
        self.pipeline_executable_info = pipeline_executable_info;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    type LowLevel = crate :: native :: extensions :: khr_pipeline_executable_properties :: PhysicalDevicePipelineExecutablePropertiesFeaturesKHR ;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: khr_pipeline_executable_properties :: PhysicalDevicePipelineExecutablePropertiesFeaturesKHR { s_type : StructureType :: PhysicalDevicePipelineExecutablePropertiesFeaturesKhr , p_next : std :: ptr :: null_mut () , pipeline_executable_info : self . pipeline_executable_info . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            pipeline_executable_info: crate::conv::FromLowLevel::from_low_level(
                context,
                value.pipeline_executable_info,
            ),
        }
    }
}
#[doc(alias = "VkPipelineInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineInfoKHR {
    pub pipeline: Pipeline,
}
impl PipelineInfoKHR {
    ///Get a reference to the `pipeline` field.
    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }
    ///Get a mutable reference to the `pipeline` field.
    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }
    ///Sets the `pipeline` field.
    pub fn set_pipeline(&mut self, pipeline: Pipeline) -> &mut Self {
        self.pipeline = pipeline;
        self
    }
    ///Sets the `pipeline` field in a builder way.
    pub fn with_pipeline(mut self, pipeline: Pipeline) -> Self {
        self.pipeline = pipeline;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineInfoKHR {
    type LowLevel = crate::native::extensions::khr_pipeline_executable_properties::PipelineInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_pipeline_executable_properties::PipelineInfoKHR {
            s_type: StructureType::PipelineInfoKhr,
            p_next: std::ptr::null(),
            pipeline: self.pipeline.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            pipeline: crate::conv::FromLowLevel::from_low_level(context, value.pipeline),
        }
    }
}
#[doc(alias = "VkPipelineExecutablePropertiesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineExecutablePropertiesKHR {
    pub stages: ShaderStageFlags,
    pub name: String,
    pub description: String,
    #[doc(alias = "subgroupSize")]
    pub subgroup_size: u32,
}
impl PipelineExecutablePropertiesKHR {
    ///Get a reference to the `stages` field.
    pub fn stages(&self) -> ShaderStageFlags {
        self.stages
    }
    ///Get a reference to the `name` field.
    pub fn name(&self) -> &String {
        &self.name
    }
    ///Get a reference to the `description` field.
    pub fn description(&self) -> &String {
        &self.description
    }
    ///Get a reference to the `subgroup_size` field.
    pub fn subgroup_size(&self) -> u32 {
        self.subgroup_size
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineExecutablePropertiesKHR {
    type LowLevel = crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutablePropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let name_bytes = self.name.as_bytes();
        debug_assert!(
            memchr::memchr(0, name_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            name_bytes.len() <= MAX_DESCRIPTION_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut name: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize] = [0; MAX_DESCRIPTION_SIZE as usize];
        name[0..name_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            name_bytes.as_ptr() as *const std::ffi::c_char,
            name_bytes.len(),
        ));
        let description_bytes = self.description.as_bytes();
        debug_assert!(
            memchr::memchr(0, description_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            description_bytes.len() <= MAX_DESCRIPTION_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize] = [0; MAX_DESCRIPTION_SIZE as usize];
        description[0..description_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            description_bytes.as_ptr() as *const std::ffi::c_char,
            description_bytes.len(),
        ));
        crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutablePropertiesKHR {
            s_type: StructureType::PipelineExecutablePropertiesKhr,
            p_next: std::ptr::null_mut(),
            stages: self.stages.into_low_level(context, bump),
            name: name,
            description: description,
            subgroup_size: self.subgroup_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineExecutablePropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let name_cstr = std::ffi::CStr::from_ptr(value.name.as_ptr());
        let name = name_cstr.to_string_lossy().into_owned();
        let description_cstr = std::ffi::CStr::from_ptr(value.description.as_ptr());
        let description = description_cstr.to_string_lossy().into_owned();
        Self {
            stages: crate::conv::FromLowLevel::from_low_level(context, value.stages),
            name: name,
            description: description,
            subgroup_size: crate::conv::FromLowLevel::from_low_level(context, value.subgroup_size),
        }
    }
}
#[doc(alias = "VkPipelineExecutableInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineExecutableInfoKHR {
    pub pipeline: Pipeline,
    #[doc(alias = "executableIndex")]
    pub executable_index: u32,
}
impl PipelineExecutableInfoKHR {
    ///Get a reference to the `pipeline` field.
    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }
    ///Get a reference to the `executable_index` field.
    pub fn executable_index(&self) -> u32 {
        self.executable_index
    }
    ///Get a mutable reference to the `pipeline` field.
    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }
    ///Get a mutable reference to the `executable_index` field.
    pub fn executable_index_mut(&mut self) -> &mut u32 {
        &mut self.executable_index
    }
    ///Sets the `pipeline` field.
    pub fn set_pipeline(&mut self, pipeline: Pipeline) -> &mut Self {
        self.pipeline = pipeline;
        self
    }
    ///Sets the `executable_index` field.
    pub fn set_executable_index(&mut self, executable_index: u32) -> &mut Self {
        self.executable_index = executable_index;
        self
    }
    ///Sets the `pipeline` field in a builder way.
    pub fn with_pipeline(mut self, pipeline: Pipeline) -> Self {
        self.pipeline = pipeline;
        self
    }
    ///Sets the `executable_index` field in a builder way.
    pub fn with_executable_index(mut self, executable_index: u32) -> Self {
        self.executable_index = executable_index;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineExecutableInfoKHR {
    type LowLevel = crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableInfoKHR {
            s_type: StructureType::PipelineExecutableInfoKhr,
            p_next: std::ptr::null(),
            pipeline: self.pipeline.into_low_level(context, bump),
            executable_index: self.executable_index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineExecutableInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            pipeline: crate::conv::FromLowLevel::from_low_level(context, value.pipeline),
            executable_index: crate::conv::FromLowLevel::from_low_level(context, value.executable_index),
        }
    }
}
#[doc(alias = "VkPipelineExecutableStatisticKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineExecutableStatisticKHR {
    pub name: String,
    pub description: String,
    pub format: PipelineExecutableStatisticFormatKHR,
    pub value: PipelineExecutableStatisticValueKHR,
}
impl PipelineExecutableStatisticKHR {
    ///Get a reference to the `name` field.
    pub fn name(&self) -> &String {
        &self.name
    }
    ///Get a reference to the `description` field.
    pub fn description(&self) -> &String {
        &self.description
    }
    ///Get a reference to the `format` field.
    pub fn format(&self) -> PipelineExecutableStatisticFormatKHR {
        self.format
    }
    ///Get a reference to the `value` field.
    pub fn value(&self) -> &PipelineExecutableStatisticValueKHR {
        &self.value
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineExecutableStatisticKHR {
    type LowLevel = crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let name_bytes = self.name.as_bytes();
        debug_assert!(
            memchr::memchr(0, name_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            name_bytes.len() <= MAX_DESCRIPTION_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut name: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize] = [0; MAX_DESCRIPTION_SIZE as usize];
        name[0..name_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            name_bytes.as_ptr() as *const std::ffi::c_char,
            name_bytes.len(),
        ));
        let description_bytes = self.description.as_bytes();
        debug_assert!(
            memchr::memchr(0, description_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            description_bytes.len() <= MAX_DESCRIPTION_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize] = [0; MAX_DESCRIPTION_SIZE as usize];
        description[0..description_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            description_bytes.as_ptr() as *const std::ffi::c_char,
            description_bytes.len(),
        ));
        crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticKHR {
            s_type: StructureType::PipelineExecutableStatisticKhr,
            p_next: std::ptr::null_mut(),
            name: name,
            description: description,
            format: self.format.into_low_level(context, bump),
            value: self.value.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineExecutableStatisticKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let name_cstr = std::ffi::CStr::from_ptr(value.name.as_ptr());
        let name = name_cstr.to_string_lossy().into_owned();
        let description_cstr = std::ffi::CStr::from_ptr(value.description.as_ptr());
        let description = description_cstr.to_string_lossy().into_owned();
        Self {
            name: name,
            description: description,
            format: crate::conv::FromLowLevel::from_low_level(context, value.format),
            value: crate::conv::FromLowLevel::from_low_level(context, value.value),
        }
    }
}
#[doc(alias = "VkPipelineExecutableInternalRepresentationKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineExecutableInternalRepresentationKHR {
    pub name: String,
    pub description: String,
    #[doc(alias = "isText")]
    pub is_text: bool,
    #[doc(alias = "pData")]
    pub data: Vec<u8>,
}
impl PipelineExecutableInternalRepresentationKHR {
    ///Get a reference to the `name` field.
    pub fn name(&self) -> &String {
        &self.name
    }
    ///Get a reference to the `description` field.
    pub fn description(&self) -> &String {
        &self.description
    }
    ///Get a reference to the `is_text` field.
    pub fn is_text(&self) -> &bool {
        &self.is_text
    }
    ///Get a reference to the `data` field.
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineExecutableInternalRepresentationKHR {
    type LowLevel =
        crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableInternalRepresentationKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let name_bytes = self.name.as_bytes();
        debug_assert!(
            memchr::memchr(0, name_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            name_bytes.len() <= MAX_DESCRIPTION_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut name: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize] = [0; MAX_DESCRIPTION_SIZE as usize];
        name[0..name_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            name_bytes.as_ptr() as *const std::ffi::c_char,
            name_bytes.len(),
        ));
        let description_bytes = self.description.as_bytes();
        debug_assert!(
            memchr::memchr(0, description_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            description_bytes.len() <= MAX_DESCRIPTION_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize] = [0; MAX_DESCRIPTION_SIZE as usize];
        description[0..description_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            description_bytes.as_ptr() as *const std::ffi::c_char,
            description_bytes.len(),
        ));
        let len_data = self.data.len() as usize;
        crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableInternalRepresentationKHR {
            s_type: StructureType::PipelineExecutableInternalRepresentationKhr,
            p_next: std::ptr::null_mut(),
            name: name,
            description: description,
            is_text: self.is_text.into_low_level(context, bump),
            data_size: len_data,
            data: self.data.as_mut_ptr().cast(),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineExecutableInternalRepresentationKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let name_cstr = std::ffi::CStr::from_ptr(value.name.as_ptr());
        let name = name_cstr.to_string_lossy().into_owned();
        let description_cstr = std::ffi::CStr::from_ptr(value.description.as_ptr());
        let description = description_cstr.to_string_lossy().into_owned();
        let data_len = value.data_size;
        let mut data = Vec::with_capacity(data_len as usize);
        let data_ptr = value.data as *const u8;
        for i in 0..data_len {
            data.push(crate::conv::FromLowLevel::from_low_level(
                context,
                data_ptr.add(i as usize).read(),
            ));
        }
        Self {
            name: name,
            description: description,
            is_text: crate::conv::FromLowLevel::from_low_level(context, value.is_text),
            data: data,
        }
    }
}
#[doc(alias = "VkPipelineExecutableStatisticValueKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PipelineExecutableStatisticValueKHR {
    B32(bool),
    I64(i64),
    U64(u64),
    F64(f64),
}
impl From<bool> for PipelineExecutableStatisticValueKHR {
    fn from(v: bool) -> Self {
        Self::B32(v)
    }
}
impl From<i64> for PipelineExecutableStatisticValueKHR {
    fn from(v: i64) -> Self {
        Self::I64(v)
    }
}
impl From<u64> for PipelineExecutableStatisticValueKHR {
    fn from(v: u64) -> Self {
        Self::U64(v)
    }
}
impl From<f64> for PipelineExecutableStatisticValueKHR {
    fn from(v: f64) -> Self {
        Self::F64(v)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineExecutableStatisticValueKHR {
    type LowLevel = (
        PipelineExecutableStatisticFormatKHR,
        crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticValueKHR,
    );
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::B32(v) => {
                crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticValueKHR {
                    b32: (v.into_low_level(context, bump)),
                }
            },
            Self::I64(v) => {
                crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticValueKHR {
                    i64: (v.into_low_level(context, bump)),
                }
            },
            Self::U64(v) => {
                crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticValueKHR {
                    u64: (v.into_low_level(context, bump)),
                }
            },
            Self::F64(v) => {
                crate::native::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticValueKHR {
                    f64: (v.into_low_level(context, bump)),
                }
            },
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineExecutableStatisticValueKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        (variant, value): <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        match variant {
            BOOL32 => Self::B32(value.b32.into_high_level(context)),
            INT64 => Self::I64(value.i64.into_high_level(context)),
            UINT64 => Self::U64(value.u64.into_high_level(context)),
            FLOAT64 => Self::F64(value.f64.into_high_level(context)),
        }
    }
}
