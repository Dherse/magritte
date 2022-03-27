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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayProperties2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`display_properties`] is a [`DisplayPropertiesKHR`] structure.
    display_properties: DisplayPropertiesKHR<'lt>,
}
impl<'lt> Default for DisplayProperties2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            display_properties: Default::default(),
        }
    }
}
impl<'lt> DisplayProperties2KHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::display_properties`]
    pub fn set_display_properties(
        &mut self,
        value: crate::extensions::khr_display::DisplayPropertiesKHR<'lt>,
    ) -> &mut Self {
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayPlaneProperties2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`display_plane_properties`] is a [`DisplayPlanePropertiesKHR`]
    ///structure.
    display_plane_properties: DisplayPlanePropertiesKHR,
}
impl<'lt> Default for DisplayPlaneProperties2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            display_plane_properties: Default::default(),
        }
    }
}
impl<'lt> DisplayPlaneProperties2KHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::display_plane_properties`]
    pub fn set_display_plane_properties(
        &mut self,
        value: crate::extensions::khr_display::DisplayPlanePropertiesKHR,
    ) -> &mut Self {
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayModeProperties2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`display_mode_properties`] is a [`DisplayModePropertiesKHR`]
    ///structure.
    display_mode_properties: DisplayModePropertiesKHR,
}
impl<'lt> Default for DisplayModeProperties2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            display_mode_properties: Default::default(),
        }
    }
}
impl<'lt> DisplayModeProperties2KHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::display_mode_properties`]
    pub fn set_display_mode_properties(
        &mut self,
        value: crate::extensions::khr_display::DisplayModePropertiesKHR,
    ) -> &mut Self {
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayPlaneInfo2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`mode`] is the display mode the application intends to program when
    ///using the specified plane.
    mode: DisplayModeKHR,
    ///No documentation found
    plane_index: u32,
}
impl<'lt> Default for DisplayPlaneInfo2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    ///Gets the raw value of [`Self::plane_index`]
    pub fn plane_index_raw(&self) -> u32 {
        self.plane_index
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::plane_index`]
    pub fn set_plane_index_raw(&mut self, value: u32) -> &mut Self {
        self.plane_index = value;
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
        &mut getter
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
    ///Sets the raw value of [`Self::mode`]
    pub fn set_mode(&mut self, value: crate::extensions::khr_display::DisplayModeKHR) -> &mut Self {
        self.mode = value;
        self
    }
    ///Sets the raw value of [`Self::plane_index`]
    pub fn set_plane_index(&mut self, value: u32) -> &mut Self {
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayPlaneCapabilities2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`capabilities`] is a [`DisplayPlaneCapabilitiesKHR`] structure.
    capabilities: DisplayPlaneCapabilitiesKHR,
}
impl<'lt> Default for DisplayPlaneCapabilities2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            capabilities: Default::default(),
        }
    }
}
impl<'lt> DisplayPlaneCapabilities2KHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::capabilities`]
    pub fn set_capabilities(
        &mut self,
        value: crate::extensions::khr_display::DisplayPlaneCapabilitiesKHR,
    ) -> &mut Self {
        self.capabilities = value;
        self
    }
}
