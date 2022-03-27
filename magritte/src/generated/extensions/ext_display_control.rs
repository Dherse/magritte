use crate::{
    extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
    vulkan1_0::{BaseInStructure, StructureType},
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
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
/// - [`DisplayPowerStateOffExt`] specifies that the display is powered down.
/// - [`DisplayPowerStateSuspendExt`] specifies that the display is put into a low power mode, from
///   which it **may** be able to transition back to [`DisplayPowerStateOnExt`] more quickly than if
///   it were in [`DisplayPowerStateOffExt`]. This state **may** be the same as
///   [`DisplayPowerStateOffExt`].
/// - [`DisplayPowerStateOnExt`] specifies that the display is powered on.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum DisplayPowerStateEXT {
    ///[`DisplayPowerStateOffExt`] specifies that the display is
    ///powered down.
    DisplayPowerStateOffExt = 0,
    ///[`DisplayPowerStateSuspendExt`] specifies that the display is
    ///put into a low power mode, from which it **may** be able to transition back
    ///to [`DisplayPowerStateOnExt`] more quickly than if it were in
    ///[`DisplayPowerStateOffExt`].
    ///This state **may** be the same as [`DisplayPowerStateOffExt`].
    DisplayPowerStateSuspendExt = 1,
    ///[`DisplayPowerStateOnExt`] specifies that the display is
    ///powered on.
    DisplayPowerStateOnExt = 2,
}
impl const Default for DisplayPowerStateEXT {
    fn default() -> Self {
        DisplayPowerStateOffExt
    }
}
impl DisplayPowerStateEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
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
/// - [`DeviceEventTypeDisplayHotplugExt`] specifies that the fence is signaled when a display is
///   plugged into or unplugged from the specified device. Applications **can** use this
///   notification to determine when they need to re-enumerate the available displays on a device.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum DeviceEventTypeEXT {
    ///[`DeviceEventTypeDisplayHotplugExt`] specifies that the fence
    ///is signaled when a display is plugged into or unplugged from the
    ///specified device.
    ///Applications **can** use this notification to determine when they need to
    ///re-enumerate the available displays on a device.
    DeviceEventTypeDisplayHotplugExt = 0,
}
impl const Default for DeviceEventTypeEXT {
    fn default() -> Self {
        DeviceEventTypeDisplayHotplugExt
    }
}
impl DeviceEventTypeEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
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
/// - [`DisplayEventTypeFirstPixelOutExt`] specifies that the fence is signaled when the first pixel
///   of the next display refresh cycle leaves the display engine for the display.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum DisplayEventTypeEXT {
    ///[`DisplayEventTypeFirstPixelOutExt`] specifies that the fence
    ///is signaled when the first pixel of the next display refresh cycle
    ///leaves the display engine for the display.
    DisplayEventTypeFirstPixelOutExt = 0,
}
impl const Default for DisplayEventTypeEXT {
    fn default() -> Self {
        DisplayEventTypeFirstPixelOutExt
    }
}
impl DisplayEventTypeEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkDisplayPowerInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPowerInfoEXT.html) - Describe the power state of a display
///# C Specifications
///The [`DisplayPowerInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_display_control
///typedef struct VkDisplayPowerInfoEXT {
///    VkStructureType           sType;
///    const void*               pNext;
///    VkDisplayPowerStateEXT    powerState;
///} VkDisplayPowerInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`power_state`] is a [`DisplayPowerStateEXT`] value specifying the new power state of the
///   display.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`power_state`]**must** be a valid [`DisplayPowerStateEXT`] value
///# Related
/// - [`VK_EXT_display_control`]
/// - [`DisplayPowerStateEXT`]
/// - [`StructureType`]
/// - [`DisplayPowerControlEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayPowerInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`power_state`] is a [`DisplayPowerStateEXT`] value specifying the
    ///new power state of the display.
    power_state: DisplayPowerStateEXT,
}
impl<'lt> Default for DisplayPowerInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            power_state: Default::default(),
        }
    }
}
impl<'lt> DisplayPowerInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::power_state`]
    pub fn power_state(&self) -> DisplayPowerStateEXT {
        self.power_state
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::power_state`]
    pub fn power_state_mut(&mut self) -> &mut DisplayPowerStateEXT {
        &mut self.power_state
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::power_state`]
    pub fn set_power_state(
        &mut self,
        value: crate::extensions::ext_display_control::DisplayPowerStateEXT,
    ) -> &mut Self {
        self.power_state = value;
        self
    }
}
///[VkDeviceEventInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceEventInfoEXT.html) - Describe a device event to create
///# C Specifications
///The [`DeviceEventInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_display_control
///typedef struct VkDeviceEventInfoEXT {
///    VkStructureType         sType;
///    const void*             pNext;
///    VkDeviceEventTypeEXT    deviceEvent;
///} VkDeviceEventInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - `device` is a [`DeviceEventTypeEXT`] value specifying when the fence will be signaled.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`device_event`]**must** be a valid [`DeviceEventTypeEXT`] value
///# Related
/// - [`VK_EXT_display_control`]
/// - [`DeviceEventTypeEXT`]
/// - [`StructureType`]
/// - [`RegisterDeviceEventEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceEventInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///No documentation found
    device_event: DeviceEventTypeEXT,
}
impl<'lt> Default for DeviceEventInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            device_event: Default::default(),
        }
    }
}
impl<'lt> DeviceEventInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::device_event`]
    pub fn device_event(&self) -> DeviceEventTypeEXT {
        self.device_event
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::device_event`]
    pub fn device_event_mut(&mut self) -> &mut DeviceEventTypeEXT {
        &mut self.device_event
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::device_event`]
    pub fn set_device_event(&mut self, value: crate::extensions::ext_display_control::DeviceEventTypeEXT) -> &mut Self {
        self.device_event = value;
        self
    }
}
///[VkDisplayEventInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayEventInfoEXT.html) - Describe a display event to create
///# C Specifications
///The [`DisplayEventInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_display_control
///typedef struct VkDisplayEventInfoEXT {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkDisplayEventTypeEXT    displayEvent;
///} VkDisplayEventInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`display_event`] is a [`DisplayEventTypeEXT`] specifying when the fence will be signaled.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`display_event`]**must** be a valid [`DisplayEventTypeEXT`] value
///# Related
/// - [`VK_EXT_display_control`]
/// - [`DisplayEventTypeEXT`]
/// - [`StructureType`]
/// - [`RegisterDisplayEventEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayEventInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`display_event`] is a [`DisplayEventTypeEXT`] specifying when the
    ///fence will be signaled.
    display_event: DisplayEventTypeEXT,
}
impl<'lt> Default for DisplayEventInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            display_event: Default::default(),
        }
    }
}
impl<'lt> DisplayEventInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::display_event`]
    pub fn display_event(&self) -> DisplayEventTypeEXT {
        self.display_event
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::display_event`]
    pub fn display_event_mut(&mut self) -> &mut DisplayEventTypeEXT {
        &mut self.display_event
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::display_event`]
    pub fn set_display_event(
        &mut self,
        value: crate::extensions::ext_display_control::DisplayEventTypeEXT,
    ) -> &mut Self {
        self.display_event = value;
        self
    }
}
///[VkSwapchainCounterCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainCounterCreateInfoEXT.html) - Specify the surface counters desired
///# C Specifications
///To enable surface counters when creating a swapchain, add a
///[`SwapchainCounterCreateInfoEXT`] structure to the [`p_next`] chain of
///[`SwapchainCreateInfoKHR`].
///[`SwapchainCounterCreateInfoEXT`] is defined as:
///```c
///// Provided by VK_EXT_display_control
///typedef struct VkSwapchainCounterCreateInfoEXT {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkSurfaceCounterFlagsEXT    surfaceCounters;
///} VkSwapchainCounterCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`surface_counters`] is a bitmask of [`SurfaceCounterFlagBitsEXT`] specifying surface counters
///   to enable for the swapchain.
///# Description
///Valid Usage
/// - The bits in [`surface_counters`]**must** be supported by [`SwapchainCreateInfoKHR::surface`],
///   as reported by [`GetPhysicalDeviceSurfaceCapabilities2EXT`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT`
/// - [`surface_counters`]**must** be a valid combination of [`SurfaceCounterFlagBitsEXT`] values
///# Related
/// - [`VK_EXT_display_control`]
/// - [`StructureType`]
/// - [`SurfaceCounterFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SwapchainCounterCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`surface_counters`] is a bitmask of [`SurfaceCounterFlagBitsEXT`]
    ///specifying surface counters to enable for the swapchain.
    surface_counters: SurfaceCounterFlagsEXT,
}
impl<'lt> Default for SwapchainCounterCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            surface_counters: Default::default(),
        }
    }
}
impl<'lt> SwapchainCounterCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::surface_counters`]
    pub fn surface_counters(&self) -> SurfaceCounterFlagsEXT {
        self.surface_counters
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::surface_counters`]
    pub fn surface_counters_mut(&mut self) -> &mut SurfaceCounterFlagsEXT {
        &mut self.surface_counters
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::surface_counters`]
    pub fn set_surface_counters(
        &mut self,
        value: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
    ) -> &mut Self {
        self.surface_counters = value;
        self
    }
}
