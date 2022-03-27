use crate::{
    extensions::khr_display::{
        DisplayModeKHR, DisplayModePropertiesKHR, DisplayPlaneCapabilitiesKHR, DisplayPlanePropertiesKHR,
        DisplayPropertiesKHR,
    },
    vulkan1_0::{BaseInStructure, BaseOutStructure, StructureType},
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`display_properties`] is a [`DisplayPropertiesKHR`] structure.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayPropertiesKHR`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceDisplayProperties2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DisplayProperties2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`display_properties`] is a [`DisplayPropertiesKHR`] structure.
    display_properties: DisplayPropertiesKHR<'lt>,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`display_plane_properties`] is a [`DisplayPlanePropertiesKHR`] structure.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayPlanePropertiesKHR`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceDisplayPlaneProperties2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DisplayPlaneProperties2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`display_plane_properties`] is a [`DisplayPlanePropertiesKHR`]
    ///structure.
    display_plane_properties: DisplayPlanePropertiesKHR,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`display_mode_properties`] is a [`DisplayModePropertiesKHR`] structure.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayModePropertiesKHR`]
/// - [`StructureType`]
/// - [`GetDisplayModeProperties2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DisplayModeProperties2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`display_mode_properties`] is a [`DisplayModePropertiesKHR`]
    ///structure.
    display_mode_properties: DisplayModePropertiesKHR,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`mode`] is the display mode the application intends to program when using the specified
///   plane.
///# Description
/// - [`plane_index`] is the plane which the application intends to use with the display.
///The members of [`DisplayPlaneInfo2KHR`] correspond to the arguments to
///[`GetDisplayPlaneCapabilitiesKHR`], with [`s_type`] and [`p_next`]
///added for extensibility.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`mode`]**must** be a valid [`DisplayModeKHR`] handle
///Host Synchronization
/// - Host access to [`mode`]**must** be externally synchronized
///# Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayModeKHR`]
/// - [`StructureType`]
/// - [`GetDisplayPlaneCapabilities2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DisplayPlaneInfo2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`mode`] is the display mode the application intends to program when
    ///using the specified plane.
    mode: DisplayModeKHR,
    ///No documentation found
    plane_index: u32,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`capabilities`] is a [`DisplayPlaneCapabilitiesKHR`] structure.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_get_display_properties2`]
/// - [`DisplayPlaneCapabilitiesKHR`]
/// - [`StructureType`]
/// - [`GetDisplayPlaneCapabilities2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DisplayPlaneCapabilities2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`capabilities`] is a [`DisplayPlaneCapabilitiesKHR`] structure.
    capabilities: DisplayPlaneCapabilitiesKHR,
}
