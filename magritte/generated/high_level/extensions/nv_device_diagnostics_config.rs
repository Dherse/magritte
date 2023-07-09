pub use crate::common::extensions::nv_device_diagnostics_config::{
    DeviceDiagnosticsConfigFlagBitsNV, DeviceDiagnosticsConfigFlagsNV, NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME,
    NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceDiagnosticsConfigFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV {
    #[doc(alias = "diagnosticsConfig")]
    pub diagnostics_config: bool,
}
impl PhysicalDeviceDiagnosticsConfigFeaturesNV {
    ///Get a reference to the `diagnostics_config` field.
    pub fn diagnostics_config(&self) -> &bool {
        &self.diagnostics_config
    }
    ///Get a mutable reference to the `diagnostics_config` field.
    pub fn diagnostics_config_mut(&mut self) -> &mut bool {
        &mut self.diagnostics_config
    }
    ///Sets the `diagnostics_config` field.
    pub fn set_diagnostics_config(&mut self, diagnostics_config: bool) -> &mut Self {
        self.diagnostics_config = diagnostics_config;
        self
    }
    ///Sets the `diagnostics_config` field in a builder way.
    pub fn with_diagnostics_config(mut self, diagnostics_config: bool) -> Self {
        self.diagnostics_config = diagnostics_config;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDiagnosticsConfigFeaturesNV {
    type LowLevel = crate::native::extensions::nv_device_diagnostics_config::PhysicalDeviceDiagnosticsConfigFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_device_diagnostics_config::PhysicalDeviceDiagnosticsConfigFeaturesNV {
            s_type: StructureType::PhysicalDeviceDiagnosticsConfigFeaturesNv,
            p_next: std::ptr::null_mut(),
            diagnostics_config: self.diagnostics_config.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDiagnosticsConfigFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            diagnostics_config: crate::conv::FromLowLevel::from_low_level(context, value.diagnostics_config),
        }
    }
}
#[doc(alias = "VkDeviceDiagnosticsConfigCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceDiagnosticsConfigCreateInfoNV {
    pub flags: DeviceDiagnosticsConfigFlagsNV,
}
impl DeviceDiagnosticsConfigCreateInfoNV {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> DeviceDiagnosticsConfigFlagsNV {
        self.flags
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut DeviceDiagnosticsConfigFlagsNV {
        &mut self.flags
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: DeviceDiagnosticsConfigFlagsNV) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: DeviceDiagnosticsConfigFlagsNV) -> Self {
        self.flags = flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceDiagnosticsConfigCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_device_diagnostics_config::DeviceDiagnosticsConfigCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_device_diagnostics_config::DeviceDiagnosticsConfigCreateInfoNV {
            s_type: StructureType::DeviceDiagnosticsConfigCreateInfoNv,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceDiagnosticsConfigCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
        }
    }
}
