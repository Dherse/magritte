pub use crate::common::extensions::ext_validation_flags::{
    ValidationCheckEXT, EXT_VALIDATION_FLAGS_EXTENSION_NAME, EXT_VALIDATION_FLAGS_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkValidationFlagsEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationFlagsEXT {
    #[doc(alias = "pDisabledValidationChecks")]
    pub disabled_validation_checks: SmallVec<[ValidationCheckEXT; 8]>,
}
impl ValidationFlagsEXT {
    ///Get a reference to the `disabled_validation_checks` field.
    pub fn disabled_validation_checks(&self) -> &SmallVec<[ValidationCheckEXT; 8]> {
        &self.disabled_validation_checks
    }
    ///Get a mutable reference to the `disabled_validation_checks` field.
    pub fn disabled_validation_checks_mut(&mut self) -> &mut SmallVec<[ValidationCheckEXT; 8]> {
        &mut self.disabled_validation_checks
    }
    ///Sets the `disabled_validation_checks` field.
    pub fn set_disabled_validation_checks(
        &mut self,
        disabled_validation_checks: SmallVec<[ValidationCheckEXT; 8]>,
    ) -> &mut Self {
        self.disabled_validation_checks = disabled_validation_checks;
        self
    }
    ///Sets the `disabled_validation_checks` field in a builder way.
    pub fn with_disabled_validation_checks(
        mut self,
        disabled_validation_checks: SmallVec<[ValidationCheckEXT; 8]>,
    ) -> Self {
        self.disabled_validation_checks = disabled_validation_checks;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ValidationFlagsEXT {
    type LowLevel = crate::native::extensions::ext_validation_flags::ValidationFlagsEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_disabled_validation_checks = self.disabled_validation_checks.len() as u32;
        let disabled_validation_checks = bump
            .alloc_slice_fill_iter(
                self.disabled_validation_checks
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::ext_validation_flags::ValidationFlagsEXT {
            s_type: StructureType::ValidationFlagsExt,
            p_next: std::ptr::null(),
            disabled_validation_check_count: len_disabled_validation_checks,
            disabled_validation_checks: disabled_validation_checks,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ValidationFlagsEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let disabled_validation_checks_len = value.disabled_validation_check_count;
        let mut disabled_validation_checks = SmallVec::with_capacity(disabled_validation_checks_len as usize);
        for i in 0..disabled_validation_checks_len {
            disabled_validation_checks.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.disabled_validation_checks.add(i as usize).read(),
            ));
        }
        Self {
            disabled_validation_checks: disabled_validation_checks,
        }
    }
}
