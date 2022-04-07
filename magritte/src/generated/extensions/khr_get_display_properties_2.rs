//![VK_KHR_get_display_properties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_display_properties2.html) - instance extension
//!# Description
//!This extension provides new entry points to query device display properties
//!and capabilities in a way that can be easily extended by other extensions,
//!without introducing any further entry points.
//!This extension can be considered the `[`VK_KHR_display`]` equivalent of
//!the `[`VK_KHR_get_physical_device_properties2`]` extension.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_display`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_get_display_properties2]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_get_display_properties2 extension>>)
//!# New functions & commands
//! - [`get_display_mode_properties2_khr`]
//! - [`get_display_plane_capabilities2_khr`]
//! - [`get_physical_device_display_plane_properties2_khr`]
//! - [`get_physical_device_display_properties2_khr`]
//!# New structures
//! - [`DisplayModeProperties2KHR`]
//! - [`DisplayPlaneCapabilities2KHR`]
//! - [`DisplayPlaneInfo2KHR`]
//! - [`DisplayPlaneProperties2KHR`]
//! - [`DisplayProperties2KHR`]
//!# New constants
//! - [`KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME`]
//! - [`KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR`
//!# Known issues & F.A.Q
//!1) What should this extension be named? **RESOLVED** : [`VK_KHR_get_display_properties2`].
//!Other alternatives:
//! - `VK_KHR_display2`
//! - One extension, combined with `VK_KHR_surface_capabilites2`.
//!2) Should extensible input structs be added for these new functions: **RESOLVED** :
//! - [`get_physical_device_display_properties2_khr`]: No. The only current input is a
//!   [`PhysicalDevice`]. Other inputs would not make sense.
//! - [`get_physical_device_display_plane_properties2_khr`]: No. The only current input is a
//!   [`PhysicalDevice`]. Other inputs would not make sense.
//! - [`get_display_mode_properties2_khr`]: No. The only current inputs are a [`PhysicalDevice`] and
//!   a [`DisplayModeKHR`]. Other inputs would not make sense.
//!3) Should additional display query functions be extended? **RESOLVED** :
//! - [`get_display_plane_supported_displays_khr`]: No. Extensions should instead extend
//!   [`get_display_plane_capabilities_khr`]().
//!# Version History
//! - Revision 1, 2017-02-21 (James Jones)  - Initial draft.
//!# Other info
//! * 2017-02-21
//! * No known IP claims.
//! * - Ian Elliott, Google  - James Jones, NVIDIA
//!# Related
//! - [`DisplayModeProperties2KHR`]
//! - [`DisplayPlaneCapabilities2KHR`]
//! - [`DisplayPlaneInfo2KHR`]
//! - [`DisplayPlaneProperties2KHR`]
//! - [`DisplayProperties2KHR`]
//! - [`get_display_mode_properties2_khr`]
//! - [`get_display_plane_capabilities2_khr`]
//! - [`get_physical_device_display_plane_properties2_khr`]
//! - [`get_physical_device_display_properties2_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::khr_display::{
        DisplayKHR, DisplayModeKHR, DisplayModePropertiesKHR, DisplayPlaneCapabilitiesKHR, DisplayPlanePropertiesKHR,
        DisplayPropertiesKHR,
    },
    vulkan1_0::{BaseInStructure, BaseOutStructure, Instance, PhysicalDevice, StructureType, VulkanResultCodes},
    AsRaw, SmallVec, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION")]
pub const KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME")]
pub const KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_get_display_properties2");
///[vkGetPhysicalDeviceDisplayProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html) - Query information about the available displays
///# C Specifications
///To query information about the available displays, call:
///```c
///// Provided by VK_KHR_get_display_properties2
///VkResult vkGetPhysicalDeviceDisplayProperties2KHR(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t*                                   pPropertyCount,
///    VkDisplayProperties2KHR*                    pProperties);
///```
/// # Parameters
/// - [`physical_device`] is a physical device.
/// - [`p_property_count`] is a pointer to an integer related to the number of display devices
///   available or queried, as described below.
/// - [`p_properties`] is either `NULL` or a pointer to an array of [`DisplayProperties2KHR`]
///   structures.
/// # Description
/// [`get_physical_device_display_properties2_khr`] behaves similarly to
/// [`get_physical_device_display_properties_khr`], with the ability to return
/// extended information via chained output structures.
/// ## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not
///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
///   [`p_property_count`][`DisplayProperties2KHR`] structures
///
/// ## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayProperties2KHR`]
/// - [`PhysicalDevice`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
pub type FNGetPhysicalDeviceDisplayProperties2Khr = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayProperties2KHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDeviceDisplayPlaneProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html) - Query information about the available display planes.
///# C Specifications
///To query the properties of a device’s display planes, call:
///```c
///// Provided by VK_KHR_get_display_properties2
///VkResult vkGetPhysicalDeviceDisplayPlaneProperties2KHR(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t*                                   pPropertyCount,
///    VkDisplayPlaneProperties2KHR*               pProperties);
///```
/// # Parameters
/// - [`physical_device`] is a physical device.
/// - [`p_property_count`] is a pointer to an integer related to the number of display planes
///   available or queried, as described below.
/// - [`p_properties`] is either `NULL` or a pointer to an array of [`DisplayPlaneProperties2KHR`]
///   structures.
/// # Description
/// [`get_physical_device_display_plane_properties2_khr`] behaves similarly to
/// [`get_physical_device_display_plane_properties_khr`], with the ability to
/// return extended information via chained output structures.
/// ## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not
///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
///   [`p_property_count`][`DisplayPlaneProperties2KHR`] structures
///
/// ## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayPlaneProperties2KHR`]
/// - [`PhysicalDevice`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
pub type FNGetPhysicalDeviceDisplayPlaneProperties2Khr = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPlaneProperties2KHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkGetDisplayModeProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html) - Query information about the available display modes.
///# C Specifications
///To query the properties of a device’s built-in display modes, call:
///```c
///// Provided by VK_KHR_get_display_properties2
///VkResult vkGetDisplayModeProperties2KHR(
///    VkPhysicalDevice                            physicalDevice,
///    VkDisplayKHR                                display,
///    uint32_t*                                   pPropertyCount,
///    VkDisplayModeProperties2KHR*                pProperties);
///```
/// # Parameters
/// - [`physical_device`] is the physical device associated with [`display`].
/// - [`display`] is the display to query.
/// - [`p_property_count`] is a pointer to an integer related to the number of display modes
///   available or queried, as described below.
/// - [`p_properties`] is either `NULL` or a pointer to an array of [`DisplayModeProperties2KHR`]
///   structures.
/// # Description
/// [`get_display_mode_properties2_khr`] behaves similarly to
/// [`get_display_mode_properties_khr`], with the ability to return extended
/// information via chained output structures.
/// ## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`display`] **must**  be a valid [`DisplayKHR`] handle
/// - [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not
///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
///   [`p_property_count`][`DisplayModeProperties2KHR`] structures
/// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
///
/// ## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayKHR`]
/// - [`DisplayModeProperties2KHR`]
/// - [`PhysicalDevice`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetDisplayModeProperties2KHR")]
pub type FNGetDisplayModeProperties2Khr = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut DisplayModeProperties2KHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkGetDisplayPlaneCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html) - Query capabilities of a mode and plane combination
///# C Specifications
///To query the capabilities of a given mode and plane combination, call:
///```c
///// Provided by VK_KHR_get_display_properties2
///VkResult vkGetDisplayPlaneCapabilities2KHR(
///    VkPhysicalDevice                            physicalDevice,
///    const VkDisplayPlaneInfo2KHR*               pDisplayPlaneInfo,
///    VkDisplayPlaneCapabilities2KHR*             pCapabilities);
///```
/// # Parameters
/// - [`physical_device`] is the physical device associated with [`p_display_plane_info`].
/// - [`p_display_plane_info`] is a pointer to a [`DisplayPlaneInfo2KHR`] structure describing the
///   plane and mode.
/// - [`p_capabilities`] is a pointer to a [`DisplayPlaneCapabilities2KHR`] structure in which the
///   capabilities are returned.
/// # Description
/// [`get_display_plane_capabilities2_khr`] behaves similarly to
/// [`get_display_plane_capabilities_khr`], with the ability to specify extended
/// inputs via chained input structures, and to return extended information via
/// chained output structures.
/// ## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_display_plane_info`] **must**  be a valid pointer to a valid [`DisplayPlaneInfo2KHR`]
///   structure
/// - [`p_capabilities`] **must**  be a valid pointer to a [`DisplayPlaneCapabilities2KHR`]
///   structure
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayPlaneCapabilities2KHR`]
/// - [`DisplayPlaneInfo2KHR`]
/// - [`PhysicalDevice`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
pub type FNGetDisplayPlaneCapabilities2Khr = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_display_plane_info: *const DisplayPlaneInfo2KHR<'lt>,
        p_capabilities: *mut DisplayPlaneCapabilities2KHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[VkDisplayProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayProperties2KHR.html) - Structure describing an available display device
///# C Specifications
///The [`DisplayProperties2KHR`] structure is defined as:
///```c
///// Provided by VK_KHR_get_display_properties2
///typedef struct VkDisplayProperties2KHR {
///    VkStructureType           sType;
///    void*                     pNext;
///    VkDisplayPropertiesKHR    displayProperties;
///} VkDisplayProperties2KHR;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`display_properties`] is a [`DisplayPropertiesKHR`] structure.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR`
/// - [`p_next`] **must**  be `NULL`
/// # Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayPropertiesKHR`]
/// - [`StructureType`]
/// - [`get_physical_device_display_properties2_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayProperties2KHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayProperties2KHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`display_properties`] is a [`DisplayPropertiesKHR`] structure.
    pub display_properties: DisplayPropertiesKHR<'lt>,
}
impl<'lt> Default for DisplayProperties2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DISPLAY_PROPERTIES2_KHR,
            p_next: std::ptr::null_mut(),
            display_properties: Default::default(),
        }
    }
}
impl<'lt> DisplayProperties2KHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::display_properties`]
    pub fn display_properties(&self) -> DisplayPropertiesKHR<'lt> {
        self.display_properties
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::display_properties`]
    pub fn display_properties_mut(&mut self) -> &mut DisplayPropertiesKHR<'lt> {
        &mut self.display_properties
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::display_properties`]
    pub fn set_display_properties(mut self, value: crate::extensions::khr_display::DisplayPropertiesKHR<'lt>) -> Self {
        self.display_properties = value;
        self
    }
}
///[VkDisplayPlaneProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneProperties2KHR.html) - Structure describing an available display plane
///# C Specifications
///The [`DisplayPlaneProperties2KHR`] structure is defined as:
///```c
///// Provided by VK_KHR_get_display_properties2
///typedef struct VkDisplayPlaneProperties2KHR {
///    VkStructureType                sType;
///    void*                          pNext;
///    VkDisplayPlanePropertiesKHR    displayPlaneProperties;
///} VkDisplayPlaneProperties2KHR;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`display_plane_properties`] is a [`DisplayPlanePropertiesKHR`] structure.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR`
/// - [`p_next`] **must**  be `NULL`
/// # Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayPlanePropertiesKHR`]
/// - [`StructureType`]
/// - [`get_physical_device_display_plane_properties2_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayPlaneProperties2KHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayPlaneProperties2KHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`display_plane_properties`] is a [`DisplayPlanePropertiesKHR`]
    ///structure.
    pub display_plane_properties: DisplayPlanePropertiesKHR,
}
impl<'lt> Default for DisplayPlaneProperties2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DISPLAY_PLANE_PROPERTIES2_KHR,
            p_next: std::ptr::null_mut(),
            display_plane_properties: Default::default(),
        }
    }
}
impl<'lt> DisplayPlaneProperties2KHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::display_plane_properties`]
    pub fn display_plane_properties(&self) -> DisplayPlanePropertiesKHR {
        self.display_plane_properties
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::display_plane_properties`]
    pub fn display_plane_properties_mut(&mut self) -> &mut DisplayPlanePropertiesKHR {
        &mut self.display_plane_properties
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::display_plane_properties`]
    pub fn set_display_plane_properties(
        mut self,
        value: crate::extensions::khr_display::DisplayPlanePropertiesKHR,
    ) -> Self {
        self.display_plane_properties = value;
        self
    }
}
///[VkDisplayModeProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeProperties2KHR.html) - Structure describing an available display mode
///# C Specifications
///The [`DisplayModeProperties2KHR`] structure is defined as:
///```c
///// Provided by VK_KHR_get_display_properties2
///typedef struct VkDisplayModeProperties2KHR {
///    VkStructureType               sType;
///    void*                         pNext;
///    VkDisplayModePropertiesKHR    displayModeProperties;
///} VkDisplayModeProperties2KHR;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`display_mode_properties`] is a [`DisplayModePropertiesKHR`] structure.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR`
/// - [`p_next`] **must**  be `NULL`
/// # Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayModePropertiesKHR`]
/// - [`StructureType`]
/// - [`get_display_mode_properties2_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayModeProperties2KHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayModeProperties2KHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`display_mode_properties`] is a [`DisplayModePropertiesKHR`]
    ///structure.
    pub display_mode_properties: DisplayModePropertiesKHR,
}
impl<'lt> Default for DisplayModeProperties2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DISPLAY_MODE_PROPERTIES2_KHR,
            p_next: std::ptr::null_mut(),
            display_mode_properties: Default::default(),
        }
    }
}
impl<'lt> DisplayModeProperties2KHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::display_mode_properties`]
    pub fn display_mode_properties(&self) -> DisplayModePropertiesKHR {
        self.display_mode_properties
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::display_mode_properties`]
    pub fn display_mode_properties_mut(&mut self) -> &mut DisplayModePropertiesKHR {
        &mut self.display_mode_properties
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::display_mode_properties`]
    pub fn set_display_mode_properties(
        mut self,
        value: crate::extensions::khr_display::DisplayModePropertiesKHR,
    ) -> Self {
        self.display_mode_properties = value;
        self
    }
}
///[VkDisplayPlaneInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneInfo2KHR.html) - Structure defining the intended configuration of a display plane
///# C Specifications
///The [`DisplayPlaneInfo2KHR`] structure is defined as:
///```c
///// Provided by VK_KHR_get_display_properties2
///typedef struct VkDisplayPlaneInfo2KHR {
///    VkStructureType     sType;
///    const void*         pNext;
///    VkDisplayModeKHR    mode;
///    uint32_t            planeIndex;
///} VkDisplayPlaneInfo2KHR;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`mode`] is the display mode the application intends to program when using the specified
///   plane.
/// # Description
/// - [`plane_index`] is the plane which the application intends to use with the display.
/// The members of [`DisplayPlaneInfo2KHR`] correspond to the arguments to
/// [`get_display_plane_capabilities_khr`], with [`s_type`] and [`p_next`]
/// added for extensibility.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`mode`] **must**  be a valid [`DisplayModeKHR`] handle
///
/// ## Host Synchronization
/// - Host access to [`mode`] **must**  be externally synchronized
/// # Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayModeKHR`]
/// - [`StructureType`]
/// - [`get_display_plane_capabilities2_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayPlaneInfo2KHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayPlaneInfo2KHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`mode`] is the display mode the application intends to program when
    ///using the specified plane.
    pub mode: DisplayModeKHR,
    ///No documentation found
    pub plane_index: u32,
}
impl<'lt> Default for DisplayPlaneInfo2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DISPLAY_PLANE_INFO2_KHR,
            p_next: std::ptr::null(),
            mode: Default::default(),
            plane_index: 0,
        }
    }
}
impl<'lt> DisplayPlaneInfo2KHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::mode`]
    pub fn mode(&self) -> DisplayModeKHR {
        self.mode
    }
    ///Gets the value of [`Self::plane_index`]
    pub fn plane_index(&self) -> u32 {
        self.plane_index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::mode`]
    pub fn mode_mut(&mut self) -> &mut DisplayModeKHR {
        &mut self.mode
    }
    ///Gets a mutable reference to the value of [`Self::plane_index`]
    pub fn plane_index_mut(&mut self) -> &mut u32 {
        &mut self.plane_index
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::mode`]
    pub fn set_mode(mut self, value: crate::extensions::khr_display::DisplayModeKHR) -> Self {
        self.mode = value;
        self
    }
    ///Sets the value of [`Self::plane_index`]
    pub fn set_plane_index(mut self, value: u32) -> Self {
        self.plane_index = value;
        self
    }
}
///[VkDisplayPlaneCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilities2KHR.html) - Structure describing the capabilities of a mode and plane combination
///# C Specifications
///The [`DisplayPlaneCapabilities2KHR`] structure is defined as:
///```c
///// Provided by VK_KHR_get_display_properties2
///typedef struct VkDisplayPlaneCapabilities2KHR {
///    VkStructureType                  sType;
///    void*                            pNext;
///    VkDisplayPlaneCapabilitiesKHR    capabilities;
///} VkDisplayPlaneCapabilities2KHR;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`capabilities`] is a [`DisplayPlaneCapabilitiesKHR`] structure.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR`
/// - [`p_next`] **must**  be `NULL`
/// # Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayPlaneCapabilitiesKHR`]
/// - [`StructureType`]
/// - [`get_display_plane_capabilities2_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayPlaneCapabilities2KHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayPlaneCapabilities2KHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`capabilities`] is a [`DisplayPlaneCapabilitiesKHR`] structure.
    pub capabilities: DisplayPlaneCapabilitiesKHR,
}
impl<'lt> Default for DisplayPlaneCapabilities2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DISPLAY_PLANE_CAPABILITIES2_KHR,
            p_next: std::ptr::null_mut(),
            capabilities: Default::default(),
        }
    }
}
impl<'lt> DisplayPlaneCapabilities2KHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::capabilities`]
    pub fn capabilities(&self) -> DisplayPlaneCapabilitiesKHR {
        self.capabilities
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::capabilities`]
    pub fn capabilities_mut(&mut self) -> &mut DisplayPlaneCapabilitiesKHR {
        &mut self.capabilities
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::capabilities`]
    pub fn set_capabilities(mut self, value: crate::extensions::khr_display::DisplayPlaneCapabilitiesKHR) -> Self {
        self.capabilities = value;
        self
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceDisplayProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html) - Query information about the available displays
    ///# C Specifications
    ///To query information about the available displays, call:
    ///```c
    ///// Provided by VK_KHR_get_display_properties2
    ///VkResult vkGetPhysicalDeviceDisplayProperties2KHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    uint32_t*                                   pPropertyCount,
    ///    VkDisplayProperties2KHR*                    pProperties);
    ///```
    /// # Parameters
    /// - [`physical_device`] is a physical device.
    /// - [`p_property_count`] is a pointer to an integer related to the number of display devices
    ///   available or queried, as described below.
    /// - [`p_properties`] is either `NULL` or a pointer to an array of [`DisplayProperties2KHR`]
    ///   structures.
    /// # Description
    /// [`get_physical_device_display_properties2_khr`] behaves similarly to
    /// [`get_physical_device_display_properties_khr`], with the ability to return
    /// extended information via chained output structures.
    /// ## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not
    ///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
    ///   [`p_property_count`][`DisplayProperties2KHR`] structures
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`VK_KHR_get_display_properties2`]
    /// - [`DisplayProperties2KHR`]
    /// - [`PhysicalDevice`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_display_properties2_khr<'lt>(
        self: &Unique<PhysicalDevice>,
        p_property_count: Option<usize>,
    ) -> VulkanResult<SmallVec<DisplayProperties2KHR<'lt>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_get_display_properties_2()
            .and_then(|vtable| vtable.get_physical_device_display_properties2_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_get_display_properties_2()
            .and_then(|vtable| vtable.get_physical_device_display_properties2_khr())
            .unwrap_unchecked();
        let mut p_property_count = match p_property_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_properties =
            SmallVec::<DisplayProperties2KHR<'lt>>::from_elem(Default::default(), p_property_count as usize);
        let _return = _function(self.as_raw(), &mut p_property_count, p_properties.as_mut_ptr());
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => VulkanResult::Success(_return, p_properties),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceDisplayPlaneProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html) - Query information about the available display planes.
    ///# C Specifications
    ///To query the properties of a device’s display planes, call:
    ///```c
    ///// Provided by VK_KHR_get_display_properties2
    ///VkResult vkGetPhysicalDeviceDisplayPlaneProperties2KHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    uint32_t*                                   pPropertyCount,
    ///    VkDisplayPlaneProperties2KHR*               pProperties);
    ///```
    /// # Parameters
    /// - [`physical_device`] is a physical device.
    /// - [`p_property_count`] is a pointer to an integer related to the number of display planes
    ///   available or queried, as described below.
    /// - [`p_properties`] is either `NULL` or a pointer to an array of
    ///   [`DisplayPlaneProperties2KHR`] structures.
    /// # Description
    /// [`get_physical_device_display_plane_properties2_khr`] behaves similarly to
    /// [`get_physical_device_display_plane_properties_khr`], with the ability to
    /// return extended information via chained output structures.
    /// ## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not
    ///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
    ///   [`p_property_count`][`DisplayPlaneProperties2KHR`] structures
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`VK_KHR_get_display_properties2`]
    /// - [`DisplayPlaneProperties2KHR`]
    /// - [`PhysicalDevice`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_display_plane_properties2_khr<'lt>(
        self: &Unique<PhysicalDevice>,
        p_property_count: Option<usize>,
    ) -> VulkanResult<SmallVec<DisplayPlaneProperties2KHR<'lt>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_get_display_properties_2()
            .and_then(|vtable| vtable.get_physical_device_display_plane_properties2_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_get_display_properties_2()
            .and_then(|vtable| vtable.get_physical_device_display_plane_properties2_khr())
            .unwrap_unchecked();
        let mut p_property_count = match p_property_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_properties =
            SmallVec::<DisplayPlaneProperties2KHR<'lt>>::from_elem(Default::default(), p_property_count as usize);
        let _return = _function(self.as_raw(), &mut p_property_count, p_properties.as_mut_ptr());
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => VulkanResult::Success(_return, p_properties),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetDisplayModeProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html) - Query information about the available display modes.
    ///# C Specifications
    ///To query the properties of a device’s built-in display modes, call:
    ///```c
    ///// Provided by VK_KHR_get_display_properties2
    ///VkResult vkGetDisplayModeProperties2KHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    VkDisplayKHR                                display,
    ///    uint32_t*                                   pPropertyCount,
    ///    VkDisplayModeProperties2KHR*                pProperties);
    ///```
    /// # Parameters
    /// - [`physical_device`] is the physical device associated with [`display`].
    /// - [`display`] is the display to query.
    /// - [`p_property_count`] is a pointer to an integer related to the number of display modes
    ///   available or queried, as described below.
    /// - [`p_properties`] is either `NULL` or a pointer to an array of
    ///   [`DisplayModeProperties2KHR`] structures.
    /// # Description
    /// [`get_display_mode_properties2_khr`] behaves similarly to
    /// [`get_display_mode_properties_khr`], with the ability to return extended
    /// information via chained output structures.
    /// ## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`display`] **must**  be a valid [`DisplayKHR`] handle
    /// - [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not
    ///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
    ///   [`p_property_count`][`DisplayModeProperties2KHR`] structures
    /// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`VK_KHR_get_display_properties2`]
    /// - [`DisplayKHR`]
    /// - [`DisplayModeProperties2KHR`]
    /// - [`PhysicalDevice`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetDisplayModeProperties2KHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_display_mode_properties2_khr<'lt>(
        self: &Unique<PhysicalDevice>,
        display: DisplayKHR,
        p_property_count: Option<usize>,
    ) -> VulkanResult<SmallVec<DisplayModeProperties2KHR<'lt>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_get_display_properties_2()
            .and_then(|vtable| vtable.get_display_mode_properties2_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_get_display_properties_2()
            .and_then(|vtable| vtable.get_display_mode_properties2_khr())
            .unwrap_unchecked();
        let mut p_property_count = match p_property_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), display, &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_properties =
            SmallVec::<DisplayModeProperties2KHR<'lt>>::from_elem(Default::default(), p_property_count as usize);
        let _return = _function(self.as_raw(), display, &mut p_property_count, p_properties.as_mut_ptr());
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => VulkanResult::Success(_return, p_properties),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetDisplayPlaneCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html) - Query capabilities of a mode and plane combination
    ///# C Specifications
    ///To query the capabilities of a given mode and plane combination, call:
    ///```c
    ///// Provided by VK_KHR_get_display_properties2
    ///VkResult vkGetDisplayPlaneCapabilities2KHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    const VkDisplayPlaneInfo2KHR*               pDisplayPlaneInfo,
    ///    VkDisplayPlaneCapabilities2KHR*             pCapabilities);
    ///```
    /// # Parameters
    /// - [`physical_device`] is the physical device associated with [`p_display_plane_info`].
    /// - [`p_display_plane_info`] is a pointer to a [`DisplayPlaneInfo2KHR`] structure describing
    ///   the plane and mode.
    /// - [`p_capabilities`] is a pointer to a [`DisplayPlaneCapabilities2KHR`] structure in which
    ///   the capabilities are returned.
    /// # Description
    /// [`get_display_plane_capabilities2_khr`] behaves similarly to
    /// [`get_display_plane_capabilities_khr`], with the ability to specify extended
    /// inputs via chained input structures, and to return extended information via
    /// chained output structures.
    /// ## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`p_display_plane_info`] **must**  be a valid pointer to a valid [`DisplayPlaneInfo2KHR`]
    ///   structure
    /// - [`p_capabilities`] **must**  be a valid pointer to a [`DisplayPlaneCapabilities2KHR`]
    ///   structure
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`VK_KHR_get_display_properties2`]
    /// - [`DisplayPlaneCapabilities2KHR`]
    /// - [`DisplayPlaneInfo2KHR`]
    /// - [`PhysicalDevice`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_display_plane_capabilities2_khr<'lt>(
        self: &Unique<PhysicalDevice>,
        p_display_plane_info: &DisplayPlaneInfo2KHR<'lt>,
        p_capabilities: Option<DisplayPlaneCapabilities2KHR<'lt>>,
    ) -> VulkanResult<DisplayPlaneCapabilities2KHR<'lt>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_get_display_properties_2()
            .and_then(|vtable| vtable.get_display_plane_capabilities2_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_get_display_properties_2()
            .and_then(|vtable| vtable.get_display_plane_capabilities2_khr())
            .unwrap_unchecked();
        let mut p_capabilities = p_capabilities.unwrap_or_default();
        let _return = _function(
            self.as_raw(),
            p_display_plane_info as *const DisplayPlaneInfo2KHR<'lt>,
            &mut p_capabilities,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, {
                p_capabilities.p_next = std::ptr::null_mut();
                p_capabilities
            }),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Instance`] for functions from `VK_KHR_get_display_properties2`
pub struct InstanceKhrGetDisplayProperties2VTable {
    ///See [`FNGetPhysicalDeviceDisplayProperties2Khr`] for more information.
    pub get_physical_device_display_properties2_khr: FNGetPhysicalDeviceDisplayProperties2Khr,
    ///See [`FNGetPhysicalDeviceDisplayPlaneProperties2Khr`] for more information.
    pub get_physical_device_display_plane_properties2_khr: FNGetPhysicalDeviceDisplayPlaneProperties2Khr,
    ///See [`FNGetDisplayModeProperties2Khr`] for more information.
    pub get_display_mode_properties2_khr: FNGetDisplayModeProperties2Khr,
    ///See [`FNGetDisplayPlaneCapabilities2Khr`] for more information.
    pub get_display_plane_capabilities2_khr: FNGetDisplayPlaneCapabilities2Khr,
}
impl InstanceKhrGetDisplayProperties2VTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Instance,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Instance,
    ) -> Self {
        Self {
            get_physical_device_display_properties2_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceDisplayProperties2KHR").as_ptr(),
                ))
            },
            get_physical_device_display_plane_properties2_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceDisplayPlaneProperties2KHR").as_ptr(),
                ))
            },
            get_display_mode_properties2_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDisplayModeProperties2KHR").as_ptr(),
                ))
            },
            get_display_plane_capabilities2_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDisplayPlaneCapabilities2KHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_physical_device_display_properties2_khr`]. See
    /// [`FNGetPhysicalDeviceDisplayProperties2Khr`] for more information.
    pub fn get_physical_device_display_properties2_khr(&self) -> FNGetPhysicalDeviceDisplayProperties2Khr {
        self.get_physical_device_display_properties2_khr
    }
    ///Gets [`Self::get_physical_device_display_plane_properties2_khr`]. See
    /// [`FNGetPhysicalDeviceDisplayPlaneProperties2Khr`] for more information.
    pub fn get_physical_device_display_plane_properties2_khr(&self) -> FNGetPhysicalDeviceDisplayPlaneProperties2Khr {
        self.get_physical_device_display_plane_properties2_khr
    }
    ///Gets [`Self::get_display_mode_properties2_khr`]. See [`FNGetDisplayModeProperties2Khr`] for
    /// more information.
    pub fn get_display_mode_properties2_khr(&self) -> FNGetDisplayModeProperties2Khr {
        self.get_display_mode_properties2_khr
    }
    ///Gets [`Self::get_display_plane_capabilities2_khr`]. See
    /// [`FNGetDisplayPlaneCapabilities2Khr`] for more information.
    pub fn get_display_plane_capabilities2_khr(&self) -> FNGetDisplayPlaneCapabilities2Khr {
        self.get_display_plane_capabilities2_khr
    }
}
