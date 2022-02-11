#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DISPLAY_CONTROL_SPEC_VERSION")]
pub const EXT_DISPLAY_CONTROL_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME")]
pub const EXT_DISPLAY_CONTROL_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_display_control");
///[VkDisplayPowerStateEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPowerStateEXT.html) - Possible power states for a display
///# C Specifications
///Possible values of [`DisplayPowerInfoEXT::power_state`], specifying
///the new power state of a display, are:
///```c
///// Provided by VK_EXT_display_control
///typedef enum VkDisplayPowerStateEXT {
///    VK_DISPLAY_POWER_STATE_OFF_EXT = 0,
///    VK_DISPLAY_POWER_STATE_SUSPEND_EXT = 1,
///    VK_DISPLAY_POWER_STATE_ON_EXT = 2,
///} VkDisplayPowerStateEXT;
///```
///# Description
/// - [`DISPLAY_POWER_STATE_OFF`] specifies that the display is
///powered down.
/// - [`DISPLAY_POWER_STATE_SUSPEND`] specifies that the display is
///put into a low power mode, from which it **may** be able to transition back
///to [`DISPLAY_POWER_STATE_ON`] more quickly than if it were in
///[`DISPLAY_POWER_STATE_OFF`].
///This state **may** be the same as [`DISPLAY_POWER_STATE_OFF`].
/// - [`DISPLAY_POWER_STATE_ON`] specifies that the display is
///powered on.
///# Related
/// - [`VK_EXT_display_control`]
/// - [`DisplayPowerInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayPowerStateEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DisplayPowerStateEXT(i32);
impl const Default for DisplayPowerStateEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DisplayPowerStateEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("DisplayPowerStateEXT")
            .field(match *self {
                Self::DISPLAY_POWER_STATE_OFF => &"DISPLAY_POWER_STATE_OFF",
                Self::DISPLAY_POWER_STATE_SUSPEND => &"DISPLAY_POWER_STATE_SUSPEND",
                Self::DISPLAY_POWER_STATE_ON => &"DISPLAY_POWER_STATE_ON",
                other => unreachable!("invalid value for `DisplayPowerStateEXT`: {:?}", other),
            })
            .finish()
    }
}
impl DisplayPowerStateEXT {
    ///[`DISPLAY_POWER_STATE_OFF`] specifies that the display is
    ///powered down.
    pub const DISPLAY_POWER_STATE_OFF: Self = Self(0);
    ///[`DISPLAY_POWER_STATE_SUSPEND`] specifies that the display is
    ///put into a low power mode, from which it **may** be able to transition back
    ///to [`DISPLAY_POWER_STATE_ON`] more quickly than if it were in
    ///[`DISPLAY_POWER_STATE_OFF`].
    ///This state **may** be the same as [`DISPLAY_POWER_STATE_OFF`].
    pub const DISPLAY_POWER_STATE_SUSPEND: Self = Self(1);
    ///[`DISPLAY_POWER_STATE_ON`] specifies that the display is
    ///powered on.
    pub const DISPLAY_POWER_STATE_ON: Self = Self(2);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
///[VkDeviceEventTypeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceEventTypeEXT.html) - Events that can occur on a device object
///# C Specifications
///Possible values of [`DeviceEventInfoEXT`]`::device`, specifying when
///a fence will be signaled, are:
///```c
///// Provided by VK_EXT_display_control
///typedef enum VkDeviceEventTypeEXT {
///    VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT = 0,
///} VkDeviceEventTypeEXT;
///```
///# Description
/// - [`DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG`] specifies that the fence
///is signaled when a display is plugged into or unplugged from the
///specified device.
///Applications **can** use this notification to determine when they need to
///re-enumerate the available displays on a device.
///# Related
/// - [`VK_EXT_display_control`]
/// - [`DeviceEventInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceEventTypeEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DeviceEventTypeEXT(i32);
impl const Default for DeviceEventTypeEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DeviceEventTypeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("DeviceEventTypeEXT")
            .field(match *self {
                Self::DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG => &"DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG",
                other => unreachable!("invalid value for `DeviceEventTypeEXT`: {:?}", other),
            })
            .finish()
    }
}
impl DeviceEventTypeEXT {
    ///[`DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG`] specifies that the fence
    ///is signaled when a display is plugged into or unplugged from the
    ///specified device.
    ///Applications **can** use this notification to determine when they need to
    ///re-enumerate the available displays on a device.
    pub const DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG: Self = Self(0);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
///[VkDisplayEventTypeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayEventTypeEXT.html) - Events that can occur on a display object
///# C Specifications
///Possible values of [`DisplayEventInfoEXT::display_event`],
///specifying when a fence will be signaled, are:
///```c
///// Provided by VK_EXT_display_control
///typedef enum VkDisplayEventTypeEXT {
///    VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT = 0,
///} VkDisplayEventTypeEXT;
///```
///# Description
/// - [`DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT`] specifies that the fence
///is signaled when the first pixel of the next display refresh cycle
///leaves the display engine for the display.
///# Related
/// - [`VK_EXT_display_control`]
/// - [`DisplayEventInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayEventTypeEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DisplayEventTypeEXT(i32);
impl const Default for DisplayEventTypeEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DisplayEventTypeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("DisplayEventTypeEXT")
            .field(match *self {
                Self::DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT => &"DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT",
                other => unreachable!("invalid value for `DisplayEventTypeEXT`: {:?}", other),
            })
            .finish()
    }
}
impl DisplayEventTypeEXT {
    ///[`DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT`] specifies that the fence
    ///is signaled when the first pixel of the next display refresh cycle
    ///leaves the display engine for the display.
    pub const DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT: Self = Self(0);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
