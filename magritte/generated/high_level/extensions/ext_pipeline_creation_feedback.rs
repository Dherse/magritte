pub use crate::common::extensions::ext_pipeline_creation_feedback::{
    EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME, EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION,
};
use crate::vulkan1_3::{
    PipelineCreationFeedback, PipelineCreationFeedbackCreateInfo, PipelineCreationFeedbackFlagBits,
    PipelineCreationFeedbackFlags,
};
#[doc(alias = "VkPipelineCreationFeedbackFlagsEXT")]
pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;
#[doc(alias = "VkPipelineCreationFeedbackFlagBitsEXT")]
pub type PipelineCreationFeedbackFlagBitsEXT = PipelineCreationFeedbackFlagBits;
#[doc(alias = "VkPipelineCreationFeedbackEXT")]
pub type PipelineCreationFeedbackEXT = PipelineCreationFeedback;
#[doc(alias = "VkPipelineCreationFeedbackCreateInfoEXT")]
pub type PipelineCreationFeedbackCreateInfoEXT = PipelineCreationFeedbackCreateInfo;
