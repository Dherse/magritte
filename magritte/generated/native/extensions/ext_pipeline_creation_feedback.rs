use crate::native::vulkan1_3::{
    PipelineCreationFeedback, PipelineCreationFeedbackCreateInfo, PipelineCreationFeedbackFlagBits,
    PipelineCreationFeedbackFlags,
};
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
pub use crate::common::extensions::ext_pipeline_creation_feedback::{
    EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME, EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION,
};
