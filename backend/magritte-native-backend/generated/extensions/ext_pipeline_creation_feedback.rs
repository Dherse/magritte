use crate::{
    cstr,
    vulkan1_3::{
        PipelineCreationFeedback, PipelineCreationFeedbackCreateInfo, PipelineCreationFeedbackFlagBits,
        PipelineCreationFeedbackFlags,
    },
};
use std::ffi::CStr;
///See [`PipelineCreationFeedbackFlags`]
#[doc(alias = "VkPipelineCreationFeedbackFlagsEXT")]
pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;
///See [`PipelineCreationFeedbackFlagBits`]
#[doc(alias = "VkPipelineCreationFeedbackFlagBitsEXT")]
pub type PipelineCreationFeedbackFlagBitsEXT = PipelineCreationFeedbackFlagBits;
///See [`PipelineCreationFeedback`]
#[doc(alias = "VkPipelineCreationFeedbackEXT")]
pub type PipelineCreationFeedbackEXT = PipelineCreationFeedback;
///See [`PipelineCreationFeedbackCreateInfo`]
#[doc(alias = "VkPipelineCreationFeedbackCreateInfoEXT")]
pub type PipelineCreationFeedbackCreateInfoEXT = PipelineCreationFeedbackCreateInfo;
#[doc(alias = "VK_EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION")]
pub const EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME")]
pub const EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_pipeline_creation_feedback");
