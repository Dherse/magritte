use crate::native::vulkan1_0::{BaseInStructure, StructureType};
#[doc(alias = "VkValidationFlagsEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ValidationFlagsEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "disabledValidationCheckCount")]
    pub disabled_validation_check_count: u32,
    #[doc(alias = "pDisabledValidationChecks")]
    pub disabled_validation_checks: *const ValidationCheckEXT,
}
impl Default for ValidationFlagsEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::ValidationFlagsExt,
            p_next: unsafe { std::mem::zeroed() },
            disabled_validation_check_count: unsafe { std::mem::zeroed() },
            disabled_validation_checks: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_validation_flags::{
    ValidationCheckEXT, EXT_VALIDATION_FLAGS_EXTENSION_NAME, EXT_VALIDATION_FLAGS_SPEC_VERSION,
};
