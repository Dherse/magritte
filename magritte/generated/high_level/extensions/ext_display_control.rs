pub use crate::common::extensions::ext_display_control::{
    DeviceEventTypeEXT, DisplayEventTypeEXT, DisplayPowerStateEXT, EXT_DISPLAY_CONTROL_EXTENSION_NAME,
    EXT_DISPLAY_CONTROL_SPEC_VERSION,
};
use crate::{
    context::Context, extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT, vulkan1_0::StructureType,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkDisplayPowerInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayPowerInfoEXT {
    #[doc(alias = "powerState")]
    pub power_state: DisplayPowerStateEXT,
}
impl DisplayPowerInfoEXT {
    ///Get a reference to the `power_state` field.
    pub fn power_state(&self) -> DisplayPowerStateEXT {
        self.power_state
    }
    ///Get a mutable reference to the `power_state` field.
    pub fn power_state_mut(&mut self) -> &mut DisplayPowerStateEXT {
        &mut self.power_state
    }
    ///Sets the `power_state` field.
    pub fn set_power_state(&mut self, power_state: DisplayPowerStateEXT) -> &mut Self {
        self.power_state = power_state;
        self
    }
    ///Sets the `power_state` field in a builder way.
    pub fn with_power_state(mut self, power_state: DisplayPowerStateEXT) -> Self {
        self.power_state = power_state;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayPowerInfoEXT {
    type LowLevel = crate::native::extensions::ext_display_control::DisplayPowerInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_display_control::DisplayPowerInfoEXT {
            s_type: StructureType::DisplayPowerInfoExt,
            p_next: std::ptr::null(),
            power_state: self.power_state.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayPowerInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            power_state: crate::conv::FromLowLevel::from_low_level(context, value.power_state),
        }
    }
}
#[doc(alias = "VkDeviceEventInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceEventInfoEXT {
    #[doc(alias = "deviceEvent")]
    pub device_event: DeviceEventTypeEXT,
}
impl DeviceEventInfoEXT {
    ///Get a reference to the `device_event` field.
    pub fn device_event(&self) -> DeviceEventTypeEXT {
        self.device_event
    }
    ///Get a mutable reference to the `device_event` field.
    pub fn device_event_mut(&mut self) -> &mut DeviceEventTypeEXT {
        &mut self.device_event
    }
    ///Sets the `device_event` field.
    pub fn set_device_event(&mut self, device_event: DeviceEventTypeEXT) -> &mut Self {
        self.device_event = device_event;
        self
    }
    ///Sets the `device_event` field in a builder way.
    pub fn with_device_event(mut self, device_event: DeviceEventTypeEXT) -> Self {
        self.device_event = device_event;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceEventInfoEXT {
    type LowLevel = crate::native::extensions::ext_display_control::DeviceEventInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_display_control::DeviceEventInfoEXT {
            s_type: StructureType::DeviceEventInfoExt,
            p_next: std::ptr::null(),
            device_event: self.device_event.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceEventInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            device_event: crate::conv::FromLowLevel::from_low_level(context, value.device_event),
        }
    }
}
#[doc(alias = "VkDisplayEventInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayEventInfoEXT {
    #[doc(alias = "displayEvent")]
    pub display_event: DisplayEventTypeEXT,
}
impl DisplayEventInfoEXT {
    ///Get a reference to the `display_event` field.
    pub fn display_event(&self) -> DisplayEventTypeEXT {
        self.display_event
    }
    ///Get a mutable reference to the `display_event` field.
    pub fn display_event_mut(&mut self) -> &mut DisplayEventTypeEXT {
        &mut self.display_event
    }
    ///Sets the `display_event` field.
    pub fn set_display_event(&mut self, display_event: DisplayEventTypeEXT) -> &mut Self {
        self.display_event = display_event;
        self
    }
    ///Sets the `display_event` field in a builder way.
    pub fn with_display_event(mut self, display_event: DisplayEventTypeEXT) -> Self {
        self.display_event = display_event;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayEventInfoEXT {
    type LowLevel = crate::native::extensions::ext_display_control::DisplayEventInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_display_control::DisplayEventInfoEXT {
            s_type: StructureType::DisplayEventInfoExt,
            p_next: std::ptr::null(),
            display_event: self.display_event.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayEventInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            display_event: crate::conv::FromLowLevel::from_low_level(context, value.display_event),
        }
    }
}
#[doc(alias = "VkSwapchainCounterCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwapchainCounterCreateInfoEXT {
    #[doc(alias = "surfaceCounters")]
    pub surface_counters: SurfaceCounterFlagsEXT,
}
impl SwapchainCounterCreateInfoEXT {
    ///Get a reference to the `surface_counters` field.
    pub fn surface_counters(&self) -> SurfaceCounterFlagsEXT {
        self.surface_counters
    }
    ///Get a mutable reference to the `surface_counters` field.
    pub fn surface_counters_mut(&mut self) -> &mut SurfaceCounterFlagsEXT {
        &mut self.surface_counters
    }
    ///Sets the `surface_counters` field.
    pub fn set_surface_counters(&mut self, surface_counters: SurfaceCounterFlagsEXT) -> &mut Self {
        self.surface_counters = surface_counters;
        self
    }
    ///Sets the `surface_counters` field in a builder way.
    pub fn with_surface_counters(mut self, surface_counters: SurfaceCounterFlagsEXT) -> Self {
        self.surface_counters = surface_counters;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SwapchainCounterCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_display_control::SwapchainCounterCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_display_control::SwapchainCounterCreateInfoEXT {
            s_type: StructureType::SwapchainCounterCreateInfoExt,
            p_next: std::ptr::null(),
            surface_counters: self.surface_counters.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SwapchainCounterCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            surface_counters: crate::conv::FromLowLevel::from_low_level(context, value.surface_counters),
        }
    }
}
