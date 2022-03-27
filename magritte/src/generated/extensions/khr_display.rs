use crate::{
    extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    vulkan1_0::{BaseInStructure, Bool32, Extent2D, Offset2D, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DISPLAY_SPEC_VERSION")]
pub const KHR_DISPLAY_SPEC_VERSION: u32 = 23;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DISPLAY_EXTENSION_NAME")]
pub const KHR_DISPLAY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_display");
///[VkDisplayPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPropertiesKHR.html) - Structure describing an available display device
///# C Specifications
///The [`DisplayPropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayPropertiesKHR {
///    VkDisplayKHR                  display;
///    const char*                   displayName;
///    VkExtent2D                    physicalDimensions;
///    VkExtent2D                    physicalResolution;
///    VkSurfaceTransformFlagsKHR    supportedTransforms;
///    VkBool32                      planeReorderPossible;
///    VkBool32                      persistentContent;
///} VkDisplayPropertiesKHR;
///```
///# Members
/// - [`display`] is a handle that is used to refer to the display described here. This handle will
///   be valid for the lifetime of the Vulkan instance.
/// - [`display_name`] is `NULL` or a pointer to a null-terminated UTF-8 string containing the name
///   of the display. Generally, this will be the name provided by the display’s EDID. If `NULL`, no
///   suitable name is available. If not `NULL`, the string pointed to **must** remain accessible
///   and unmodified as long as [`display`] is valid.
/// - [`physical_dimensions`] describes the physical width and height of the visible portion of the
///   display, in millimeters.
/// - [`physical_resolution`] describes the physical, native, or preferred resolution of the
///   display.
///# Description
/// - [`supported_transforms`] is a bitmask of [`SurfaceTransformFlagBitsKHR`] describing which
///   transforms are supported by this display.
/// - [`plane_reorder_possible`] tells whether the planes on this display **can** have their z order
///   changed. If this is [`TRUE`], the application **can** re-arrange the planes on this display in
///   any order relative to each other.
/// - [`persistent_content`] tells whether the display supports self-refresh/internal buffering. If
///   this is true, the application **can** submit persistent present operations on swapchains
///   created against this display.
///# Related
/// - [`VK_KHR_display`]
/// - [`Bool32`]
/// - [`DisplayKHR`]
/// - [`DisplayProperties2KHR`]
/// - [`Extent2D`]
/// - [`SurfaceTransformFlagsKHR`]
/// - [`GetPhysicalDeviceDisplayPropertiesKHR`]
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
pub struct DisplayPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`display`] is a handle that is used to refer to the display described
    ///here.
    ///This handle will be valid for the lifetime of the Vulkan instance.
    display: DisplayKHR,
    ///[`display_name`] is `NULL` or a pointer to a null-terminated UTF-8
    ///string containing the name of the display.
    ///Generally, this will be the name provided by the display’s EDID.
    ///If `NULL`, no suitable name is available.
    ///If not `NULL`, the string pointed to **must** remain accessible and
    ///unmodified as long as [`display`] is valid.
    display_name: &'lt CStr,
    ///[`physical_dimensions`] describes the physical width and height of the
    ///visible portion of the display, in millimeters.
    physical_dimensions: Extent2D,
    ///[`physical_resolution`] describes the physical, native, or preferred
    ///resolution of the display.
    physical_resolution: Extent2D,
    ///No documentation found
    supported_transforms: SurfaceTransformFlagsKHR,
    ///No documentation found
    plane_reorder_possible: Bool32,
    ///No documentation found
    persistent_content: Bool32,
}
impl<'lt> Default for DisplayPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            display: Default::default(),
            display_name: std::ptr::null(),
            physical_dimensions: Default::default(),
            physical_resolution: Default::default(),
            supported_transforms: Default::default(),
            plane_reorder_possible: 0,
            persistent_content: 0,
        }
    }
}
impl<'lt> DisplayPropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::display_name`]
    pub fn display_name_raw(&self) -> &'lt CStr {
        self.display_name
    }
    ///Gets the raw value of [`Self::plane_reorder_possible`]
    pub fn plane_reorder_possible_raw(&self) -> Bool32 {
        self.plane_reorder_possible
    }
    ///Gets the raw value of [`Self::persistent_content`]
    pub fn persistent_content_raw(&self) -> Bool32 {
        self.persistent_content
    }
    ///Sets the raw value of [`Self::display_name`]
    pub fn set_display_name_raw(&mut self, value: &'lt CStr) -> &mut Self {
        self.display_name = value;
        self
    }
    ///Sets the raw value of [`Self::plane_reorder_possible`]
    pub fn set_plane_reorder_possible_raw(&mut self, value: Bool32) -> &mut Self {
        self.plane_reorder_possible = value;
        self
    }
    ///Sets the raw value of [`Self::persistent_content`]
    pub fn set_persistent_content_raw(&mut self, value: Bool32) -> &mut Self {
        self.persistent_content = value;
        self
    }
    ///Gets the value of [`Self::display`]
    pub fn display(&self) -> DisplayKHR {
        self.display
    }
    ///Gets the value of [`Self::display_name`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn display_name(&self) -> &'lt CStr {
        self.display_name
    }
    ///Gets the value of [`Self::physical_dimensions`]
    pub fn physical_dimensions(&self) -> Extent2D {
        self.physical_dimensions
    }
    ///Gets the value of [`Self::physical_resolution`]
    pub fn physical_resolution(&self) -> Extent2D {
        self.physical_resolution
    }
    ///Gets the value of [`Self::supported_transforms`]
    pub fn supported_transforms(&self) -> SurfaceTransformFlagsKHR {
        self.supported_transforms
    }
    ///Gets the value of [`Self::plane_reorder_possible`]
    pub fn plane_reorder_possible(&self) -> bool {
        unsafe { std::mem::transmute(self.plane_reorder_possible as u8) }
    }
    ///Gets the value of [`Self::persistent_content`]
    pub fn persistent_content(&self) -> bool {
        unsafe { std::mem::transmute(self.persistent_content as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::display`]
    pub fn display_mut(&mut self) -> &mut DisplayKHR {
        &mut self.display
    }
    ///Gets a mutable reference to the value of [`Self::physical_dimensions`]
    pub fn physical_dimensions_mut(&mut self) -> &mut Extent2D {
        &mut self.physical_dimensions
    }
    ///Gets a mutable reference to the value of [`Self::physical_resolution`]
    pub fn physical_resolution_mut(&mut self) -> &mut Extent2D {
        &mut self.physical_resolution
    }
    ///Gets a mutable reference to the value of [`Self::supported_transforms`]
    pub fn supported_transforms_mut(&mut self) -> &mut SurfaceTransformFlagsKHR {
        &mut self.supported_transforms
    }
    ///Gets a mutable reference to the value of [`Self::plane_reorder_possible`]
    pub fn plane_reorder_possible_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.plane_reorder_possible as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.plane_reorder_possible as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::persistent_content`]
    pub fn persistent_content_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.persistent_content as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.persistent_content as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::display`]
    pub fn set_display(&mut self, value: crate::extensions::khr_display::DisplayKHR) -> &mut Self {
        self.display = value;
        self
    }
    ///Sets the raw value of [`Self::display_name`]
    pub fn set_display_name(&mut self, value: &'lt std::ffi::CStr) -> &mut Self {
        self.display_name = value;
        self
    }
    ///Sets the raw value of [`Self::physical_dimensions`]
    pub fn set_physical_dimensions(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.physical_dimensions = value;
        self
    }
    ///Sets the raw value of [`Self::physical_resolution`]
    pub fn set_physical_resolution(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.physical_resolution = value;
        self
    }
    ///Sets the raw value of [`Self::supported_transforms`]
    pub fn set_supported_transforms(
        &mut self,
        value: crate::extensions::khr_display::SurfaceTransformFlagsKHR,
    ) -> &mut Self {
        self.supported_transforms = value;
        self
    }
    ///Sets the raw value of [`Self::plane_reorder_possible`]
    pub fn set_plane_reorder_possible(&mut self, value: bool) -> &mut Self {
        self.plane_reorder_possible = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::persistent_content`]
    pub fn set_persistent_content(&mut self, value: bool) -> &mut Self {
        self.persistent_content = value as u8 as u32;
        self
    }
}
///[VkDisplayPlanePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlanePropertiesKHR.html) - Structure describing display plane properties
///# C Specifications
///The [`DisplayPlanePropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayPlanePropertiesKHR {
///    VkDisplayKHR    currentDisplay;
///    uint32_t        currentStackIndex;
///} VkDisplayPlanePropertiesKHR;
///```
///# Members
/// - [`current_display`] is the handle of the display the plane is currently associated with. If
///   the plane is not currently attached to any displays, this will be
///   [`crate::utils::Handle::null`].
/// - [`current_stack_index`] is the current z-order of the plane. This will be between 0 and the
///   value returned by [`GetPhysicalDeviceDisplayPlanePropertiesKHR`] in `pPropertyCount`.
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayKHR`]
/// - [`DisplayPlaneProperties2KHR`]
/// - [`GetPhysicalDeviceDisplayPlanePropertiesKHR`]
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
pub struct DisplayPlanePropertiesKHR {
    ///[`current_display`] is the handle of the display the plane is currently
    ///associated with.
    ///If the plane is not currently attached to any displays, this will be
    ///[`crate::utils::Handle::null`].
    current_display: DisplayKHR,
    ///[`current_stack_index`] is the current z-order of the plane.
    ///This will be between 0 and the value returned by
    ///[`GetPhysicalDeviceDisplayPlanePropertiesKHR`] in
    ///`pPropertyCount`.
    current_stack_index: u32,
}
impl Default for DisplayPlanePropertiesKHR {
    fn default() -> Self {
        Self {
            current_display: Default::default(),
            current_stack_index: 0,
        }
    }
}
impl DisplayPlanePropertiesKHR {
    ///Gets the raw value of [`Self::current_stack_index`]
    pub fn current_stack_index_raw(&self) -> u32 {
        self.current_stack_index
    }
    ///Sets the raw value of [`Self::current_stack_index`]
    pub fn set_current_stack_index_raw(&mut self, value: u32) -> &mut Self {
        self.current_stack_index = value;
        self
    }
    ///Gets the value of [`Self::current_display`]
    pub fn current_display(&self) -> DisplayKHR {
        self.current_display
    }
    ///Gets the value of [`Self::current_stack_index`]
    pub fn current_stack_index(&self) -> u32 {
        self.current_stack_index
    }
    ///Gets a mutable reference to the value of [`Self::current_display`]
    pub fn current_display_mut(&mut self) -> &mut DisplayKHR {
        &mut self.current_display
    }
    ///Gets a mutable reference to the value of [`Self::current_stack_index`]
    pub fn current_stack_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::current_display`]
    pub fn set_current_display(&mut self, value: crate::extensions::khr_display::DisplayKHR) -> &mut Self {
        self.current_display = value;
        self
    }
    ///Sets the raw value of [`Self::current_stack_index`]
    pub fn set_current_stack_index(&mut self, value: u32) -> &mut Self {
        self.current_stack_index = value;
        self
    }
}
///[VkDisplayModeParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeParametersKHR.html) - Structure describing display parameters associated with a display mode
///# C Specifications
///The [`DisplayModeParametersKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayModeParametersKHR {
///    VkExtent2D    visibleRegion;
///    uint32_t      refreshRate;
///} VkDisplayModeParametersKHR;
///```
///# Members
/// - [`visible_region`] is the 2D extents of the visible region.
/// - [`refresh_rate`] is a `uint32_t` that is the number of times the display is refreshed each
///   second multiplied by 1000.
///# Description
///Valid Usage
/// - The `width` member of [`visible_region`]**must** be greater than `0`
/// - The `height` member of [`visible_region`]**must** be greater than `0`
/// - [`refresh_rate`]**must** be greater than `0`
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeCreateInfoKHR`]
/// - [`DisplayModePropertiesKHR`]
/// - [`Extent2D`]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DisplayModeParametersKHR {
    ///[`visible_region`] is the 2D extents of the visible region.
    visible_region: Extent2D,
    ///[`refresh_rate`] is a `uint32_t` that is the number of times the
    ///display is refreshed each second multiplied by 1000.
    refresh_rate: u32,
}
impl Default for DisplayModeParametersKHR {
    fn default() -> Self {
        Self {
            visible_region: Default::default(),
            refresh_rate: 0,
        }
    }
}
impl DisplayModeParametersKHR {
    ///Gets the raw value of [`Self::refresh_rate`]
    pub fn refresh_rate_raw(&self) -> u32 {
        self.refresh_rate
    }
    ///Sets the raw value of [`Self::refresh_rate`]
    pub fn set_refresh_rate_raw(&mut self, value: u32) -> &mut Self {
        self.refresh_rate = value;
        self
    }
    ///Gets the value of [`Self::visible_region`]
    pub fn visible_region(&self) -> Extent2D {
        self.visible_region
    }
    ///Gets the value of [`Self::refresh_rate`]
    pub fn refresh_rate(&self) -> u32 {
        self.refresh_rate
    }
    ///Gets a mutable reference to the value of [`Self::visible_region`]
    pub fn visible_region_mut(&mut self) -> &mut Extent2D {
        &mut self.visible_region
    }
    ///Gets a mutable reference to the value of [`Self::refresh_rate`]
    pub fn refresh_rate_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::visible_region`]
    pub fn set_visible_region(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.visible_region = value;
        self
    }
    ///Sets the raw value of [`Self::refresh_rate`]
    pub fn set_refresh_rate(&mut self, value: u32) -> &mut Self {
        self.refresh_rate = value;
        self
    }
}
///[VkDisplayModePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModePropertiesKHR.html) - Structure describing display mode properties
///# C Specifications
///The [`DisplayModePropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayModePropertiesKHR {
///    VkDisplayModeKHR              displayMode;
///    VkDisplayModeParametersKHR    parameters;
///} VkDisplayModePropertiesKHR;
///```
///# Members
/// - [`display_mode`] is a handle to the display mode described in this structure. This handle will
///   be valid for the lifetime of the Vulkan instance.
/// - [`parameters`] is a [`DisplayModeParametersKHR`] structure describing the display parameters
///   associated with [`display_mode`].
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeKHR`]
/// - [`DisplayModeParametersKHR`]
/// - [`DisplayModeProperties2KHR`]
/// - [`GetDisplayModePropertiesKHR`]
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
pub struct DisplayModePropertiesKHR {
    ///[`display_mode`] is a handle to the display mode described in this
    ///structure.
    ///This handle will be valid for the lifetime of the Vulkan instance.
    display_mode: DisplayModeKHR,
    ///[`parameters`] is a [`DisplayModeParametersKHR`] structure
    ///describing the display parameters associated with [`display_mode`].
    parameters: DisplayModeParametersKHR,
}
impl Default for DisplayModePropertiesKHR {
    fn default() -> Self {
        Self {
            display_mode: Default::default(),
            parameters: Default::default(),
        }
    }
}
impl DisplayModePropertiesKHR {
    ///Gets the value of [`Self::display_mode`]
    pub fn display_mode(&self) -> DisplayModeKHR {
        self.display_mode
    }
    ///Gets the value of [`Self::parameters`]
    pub fn parameters(&self) -> DisplayModeParametersKHR {
        self.parameters
    }
    ///Gets a mutable reference to the value of [`Self::display_mode`]
    pub fn display_mode_mut(&mut self) -> &mut DisplayModeKHR {
        &mut self.display_mode
    }
    ///Gets a mutable reference to the value of [`Self::parameters`]
    pub fn parameters_mut(&mut self) -> &mut DisplayModeParametersKHR {
        &mut self.parameters
    }
    ///Sets the raw value of [`Self::display_mode`]
    pub fn set_display_mode(&mut self, value: crate::extensions::khr_display::DisplayModeKHR) -> &mut Self {
        self.display_mode = value;
        self
    }
    ///Sets the raw value of [`Self::parameters`]
    pub fn set_parameters(&mut self, value: crate::extensions::khr_display::DisplayModeParametersKHR) -> &mut Self {
        self.parameters = value;
        self
    }
}
///[VkDisplayModeCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeCreateInfoKHR.html) - Structure specifying parameters of a newly created display mode object
///# C Specifications
///The [`DisplayModeCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayModeCreateInfoKHR {
///    VkStructureType                sType;
///    const void*                    pNext;
///    VkDisplayModeCreateFlagsKHR    flags;
///    VkDisplayModeParametersKHR     parameters;
///} VkDisplayModeCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use, and **must** be zero.
/// - [`parameters`] is a [`DisplayModeParametersKHR`] structure describing the display parameters
///   to use in creating the new mode. If the parameters are not compatible with the specified
///   display, the implementation **must** return `VK_ERROR_INITIALIZATION_FAILED`.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
/// - [`parameters`]**must** be a valid [`DisplayModeParametersKHR`] structure
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeCreateFlagsKHR`]
/// - [`DisplayModeParametersKHR`]
/// - [`StructureType`]
/// - [`CreateDisplayModeKHR`]
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
pub struct DisplayModeCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use, and **must** be zero.
    flags: DisplayModeCreateFlagsKHR,
    ///[`parameters`] is a [`DisplayModeParametersKHR`] structure
    ///describing the display parameters to use in creating the new mode.
    ///If the parameters are not compatible with the specified display, the
    ///implementation **must** return `VK_ERROR_INITIALIZATION_FAILED`.
    parameters: DisplayModeParametersKHR,
}
impl<'lt> Default for DisplayModeCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            parameters: Default::default(),
        }
    }
}
impl<'lt> DisplayModeCreateInfoKHR<'lt> {
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> DisplayModeCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::parameters`]
    pub fn parameters(&self) -> DisplayModeParametersKHR {
        self.parameters
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DisplayModeCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::parameters`]
    pub fn parameters_mut(&mut self) -> &mut DisplayModeParametersKHR {
        &mut self.parameters
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
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::khr_display::DisplayModeCreateFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::parameters`]
    pub fn set_parameters(&mut self, value: crate::extensions::khr_display::DisplayModeParametersKHR) -> &mut Self {
        self.parameters = value;
        self
    }
}
///[VkDisplayPlaneCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilitiesKHR.html) - Structure describing capabilities of a mode and plane combination
///# C Specifications
///The [`DisplayPlaneCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayPlaneCapabilitiesKHR {
///    VkDisplayPlaneAlphaFlagsKHR    supportedAlpha;
///    VkOffset2D                     minSrcPosition;
///    VkOffset2D                     maxSrcPosition;
///    VkExtent2D                     minSrcExtent;
///    VkExtent2D                     maxSrcExtent;
///    VkOffset2D                     minDstPosition;
///    VkOffset2D                     maxDstPosition;
///    VkExtent2D                     minDstExtent;
///    VkExtent2D                     maxDstExtent;
///} VkDisplayPlaneCapabilitiesKHR;
///```
///# Members
/// - [`supported_alpha`] is a bitmask of [`DisplayPlaneAlphaFlagBitsKHR`] describing the supported
///   alpha blending modes.
/// - [`min_src_position`] is the minimum source rectangle offset supported by this plane using the
///   specified mode.
/// - [`max_src_position`] is the maximum source rectangle offset supported by this plane using the
///   specified mode. The `x` and `y` components of [`max_src_position`]**must** each be greater
///   than or equal to the `x` and `y` components of [`min_src_position`], respectively.
/// - [`min_src_extent`] is the minimum source rectangle size supported by this plane using the
///   specified mode.
/// - [`max_src_extent`] is the maximum source rectangle size supported by this plane using the
///   specified mode.
/// - [`min_dst_position`], [`max_dst_position`], [`min_dst_extent`], [`max_dst_extent`] all have
///   similar semantics to their corresponding `*Src*` equivalents, but apply to the output region
///   within the mode rather than the input region within the source image. Unlike the `*Src*`
///   offsets, [`min_dst_position`] and [`max_dst_position`]**may** contain negative values.
///# Description
///The minimum and maximum position and extent fields describe the
///implementation limits, if any, as they apply to the specified display mode
///and plane.
///Vendors **may** support displaying a subset of a swapchain’s presentable images
///on the specified display plane.
///This is expressed by returning [`min_src_position`], [`max_src_position`],
///[`min_src_extent`], and [`max_src_extent`] values that indicate a range of
///possible positions and sizes which **may** be used to specify the region within
///the presentable images that source pixels will be read from when creating a
///swapchain on the specified display mode and plane.Vendors **may** also support mapping the
/// presentable images’ content to a
///subset or superset of the visible region in the specified display mode.
///This is expressed by returning [`min_dst_position`], [`max_dst_position`],
///[`min_dst_extent`] and [`max_dst_extent`] values that indicate a range of
///possible positions and sizes which **may** be used to describe the region
///within the display mode that the source pixels will be mapped to.Other vendors **may** support
/// only a 1-1 mapping between pixels in the
///presentable images and the display mode.
///This **may** be indicated by returning (0,0) for [`min_src_position`],
///[`max_src_position`], [`min_dst_position`], and [`max_dst_position`], and
///(display mode width, display mode height) for [`min_src_extent`],
///[`max_src_extent`], [`min_dst_extent`], and [`max_dst_extent`].The value
/// [`supported_alpha`]**must** contain at least one valid
///[`DisplayPlaneAlphaFlagBitsKHR`] bit.These values indicate the limits of the implementation’s
/// individual fields.
///Not all combinations of values within the offset and extent ranges returned
///in [`DisplayPlaneCapabilitiesKHR`] are guaranteed to be supported.
///Presentation requests specifying unsupported combinations **may** fail.
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayPlaneAlphaFlagsKHR`]
/// - [`DisplayPlaneCapabilities2KHR`]
/// - [`Extent2D`]
/// - [`Offset2D`]
/// - [`GetDisplayPlaneCapabilitiesKHR`]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DisplayPlaneCapabilitiesKHR {
    ///[`supported_alpha`] is a bitmask of
    ///[`DisplayPlaneAlphaFlagBitsKHR`] describing the supported alpha
    ///blending modes.
    supported_alpha: DisplayPlaneAlphaFlagsKHR,
    ///[`min_src_position`] is the minimum source rectangle offset supported by
    ///this plane using the specified mode.
    min_src_position: Offset2D,
    ///[`max_src_position`] is the maximum source rectangle offset supported by
    ///this plane using the specified mode.
    ///The `x` and `y` components of [`max_src_position`]**must** each be
    ///greater than or equal to the `x` and `y` components of
    ///[`min_src_position`], respectively.
    max_src_position: Offset2D,
    ///[`min_src_extent`] is the minimum source rectangle size supported by
    ///this plane using the specified mode.
    min_src_extent: Extent2D,
    ///[`max_src_extent`] is the maximum source rectangle size supported by
    ///this plane using the specified mode.
    max_src_extent: Extent2D,
    ///[`min_dst_position`], [`max_dst_position`], [`min_dst_extent`],
    ///[`max_dst_extent`] all have similar semantics to their corresponding
    ///`*Src*` equivalents, but apply to the output region within the mode
    ///rather than the input region within the source image.
    ///Unlike the `*Src*` offsets, [`min_dst_position`] and
    ///[`max_dst_position`]**may** contain negative values.
    min_dst_position: Offset2D,
    ///No documentation found
    max_dst_position: Offset2D,
    ///No documentation found
    min_dst_extent: Extent2D,
    ///No documentation found
    max_dst_extent: Extent2D,
}
impl Default for DisplayPlaneCapabilitiesKHR {
    fn default() -> Self {
        Self {
            supported_alpha: Default::default(),
            min_src_position: Default::default(),
            max_src_position: Default::default(),
            min_src_extent: Default::default(),
            max_src_extent: Default::default(),
            min_dst_position: Default::default(),
            max_dst_position: Default::default(),
            min_dst_extent: Default::default(),
            max_dst_extent: Default::default(),
        }
    }
}
impl DisplayPlaneCapabilitiesKHR {
    ///Gets the value of [`Self::supported_alpha`]
    pub fn supported_alpha(&self) -> DisplayPlaneAlphaFlagsKHR {
        self.supported_alpha
    }
    ///Gets the value of [`Self::min_src_position`]
    pub fn min_src_position(&self) -> Offset2D {
        self.min_src_position
    }
    ///Gets the value of [`Self::max_src_position`]
    pub fn max_src_position(&self) -> Offset2D {
        self.max_src_position
    }
    ///Gets the value of [`Self::min_src_extent`]
    pub fn min_src_extent(&self) -> Extent2D {
        self.min_src_extent
    }
    ///Gets the value of [`Self::max_src_extent`]
    pub fn max_src_extent(&self) -> Extent2D {
        self.max_src_extent
    }
    ///Gets the value of [`Self::min_dst_position`]
    pub fn min_dst_position(&self) -> Offset2D {
        self.min_dst_position
    }
    ///Gets the value of [`Self::max_dst_position`]
    pub fn max_dst_position(&self) -> Offset2D {
        self.max_dst_position
    }
    ///Gets the value of [`Self::min_dst_extent`]
    pub fn min_dst_extent(&self) -> Extent2D {
        self.min_dst_extent
    }
    ///Gets the value of [`Self::max_dst_extent`]
    pub fn max_dst_extent(&self) -> Extent2D {
        self.max_dst_extent
    }
    ///Gets a mutable reference to the value of [`Self::supported_alpha`]
    pub fn supported_alpha_mut(&mut self) -> &mut DisplayPlaneAlphaFlagsKHR {
        &mut self.supported_alpha
    }
    ///Gets a mutable reference to the value of [`Self::min_src_position`]
    pub fn min_src_position_mut(&mut self) -> &mut Offset2D {
        &mut self.min_src_position
    }
    ///Gets a mutable reference to the value of [`Self::max_src_position`]
    pub fn max_src_position_mut(&mut self) -> &mut Offset2D {
        &mut self.max_src_position
    }
    ///Gets a mutable reference to the value of [`Self::min_src_extent`]
    pub fn min_src_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.min_src_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_src_extent`]
    pub fn max_src_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_src_extent
    }
    ///Gets a mutable reference to the value of [`Self::min_dst_position`]
    pub fn min_dst_position_mut(&mut self) -> &mut Offset2D {
        &mut self.min_dst_position
    }
    ///Gets a mutable reference to the value of [`Self::max_dst_position`]
    pub fn max_dst_position_mut(&mut self) -> &mut Offset2D {
        &mut self.max_dst_position
    }
    ///Gets a mutable reference to the value of [`Self::min_dst_extent`]
    pub fn min_dst_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.min_dst_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_dst_extent`]
    pub fn max_dst_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_dst_extent
    }
    ///Sets the raw value of [`Self::supported_alpha`]
    pub fn set_supported_alpha(
        &mut self,
        value: crate::extensions::khr_display::DisplayPlaneAlphaFlagsKHR,
    ) -> &mut Self {
        self.supported_alpha = value;
        self
    }
    ///Sets the raw value of [`Self::min_src_position`]
    pub fn set_min_src_position(&mut self, value: crate::vulkan1_0::Offset2D) -> &mut Self {
        self.min_src_position = value;
        self
    }
    ///Sets the raw value of [`Self::max_src_position`]
    pub fn set_max_src_position(&mut self, value: crate::vulkan1_0::Offset2D) -> &mut Self {
        self.max_src_position = value;
        self
    }
    ///Sets the raw value of [`Self::min_src_extent`]
    pub fn set_min_src_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.min_src_extent = value;
        self
    }
    ///Sets the raw value of [`Self::max_src_extent`]
    pub fn set_max_src_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_src_extent = value;
        self
    }
    ///Sets the raw value of [`Self::min_dst_position`]
    pub fn set_min_dst_position(&mut self, value: crate::vulkan1_0::Offset2D) -> &mut Self {
        self.min_dst_position = value;
        self
    }
    ///Sets the raw value of [`Self::max_dst_position`]
    pub fn set_max_dst_position(&mut self, value: crate::vulkan1_0::Offset2D) -> &mut Self {
        self.max_dst_position = value;
        self
    }
    ///Sets the raw value of [`Self::min_dst_extent`]
    pub fn set_min_dst_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.min_dst_extent = value;
        self
    }
    ///Sets the raw value of [`Self::max_dst_extent`]
    pub fn set_max_dst_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_dst_extent = value;
        self
    }
}
///[VkDisplaySurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created display plane surface object
///# C Specifications
///The [`DisplaySurfaceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplaySurfaceCreateInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkDisplaySurfaceCreateFlagsKHR    flags;
///    VkDisplayModeKHR                  displayMode;
///    uint32_t                          planeIndex;
///    uint32_t                          planeStackIndex;
///    VkSurfaceTransformFlagBitsKHR     transform;
///    float                             globalAlpha;
///    VkDisplayPlaneAlphaFlagBitsKHR    alphaMode;
///    VkExtent2D                        imageExtent;
///} VkDisplaySurfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use, and **must** be zero.
/// - [`display_mode`] is a [`DisplayModeKHR`] handle specifying the mode to use when displaying
///   this surface.
/// - [`plane_index`] is the plane on which this surface appears.
/// - [`plane_stack_index`] is the z-order of the plane.
/// - [`transform`] is a [`SurfaceTransformFlagBitsKHR`] value specifying the transformation to
///   apply to images as part of the scanout operation.
/// - [`global_alpha`] is the global alpha value. This value is ignored if [`alpha_mode`] is not
///   `VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR`.
/// - [`alpha_mode`] is a [`DisplayPlaneAlphaFlagBitsKHR`] value specifying the type of alpha
///   blending to use.
/// - [`image_extent`] is the size of the presentable images to use with the surface.
///# Description
///Valid Usage
/// - [`plane_index`]**must** be less than the number of display planes supported by the device as
///   determined by calling [`GetPhysicalDeviceDisplayPlanePropertiesKHR`]
/// - If the `planeReorderPossible` member of the [`DisplayPropertiesKHR`] structure returned by
///   [`GetPhysicalDeviceDisplayPropertiesKHR`] for the display corresponding to [`display_mode`] is
///   [`TRUE`] then [`plane_stack_index`]**must** be less than the number of display planes
///   supported by the device as determined by calling
///   [`GetPhysicalDeviceDisplayPlanePropertiesKHR`]; otherwise [`plane_stack_index`]**must** equal
///   the `currentStackIndex` member of [`DisplayPlanePropertiesKHR`] returned by
///   [`GetPhysicalDeviceDisplayPlanePropertiesKHR`] for the display plane corresponding to
///   [`display_mode`]
/// - If [`alpha_mode`] is `VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR` then [`global_alpha`]**must** be
///   between `0` and `1`, inclusive
/// - [`alpha_mode`]**must** be one of the bits present in the `supportedAlpha` member of
///   [`DisplayPlaneCapabilitiesKHR`] for the display plane corresponding to [`display_mode`]
/// - The `width` and `height` members of [`image_extent`]**must** be less than or equal to
///   [`PhysicalDeviceLimits::max_image_dimension_2_d`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
/// - [`display_mode`]**must** be a valid [`DisplayModeKHR`] handle
/// - [`transform`]**must** be a valid [`SurfaceTransformFlagBitsKHR`] value
/// - [`alpha_mode`]**must** be a valid [`DisplayPlaneAlphaFlagBitsKHR`] value
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeKHR`]
/// - [`DisplayPlaneAlphaFlagBitsKHR`]
/// - [`DisplaySurfaceCreateFlagsKHR`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`SurfaceTransformFlagBitsKHR`]
/// - [`CreateDisplayPlaneSurfaceKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplaySurfaceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use, and **must** be zero.
    flags: DisplaySurfaceCreateFlagsKHR,
    ///[`display_mode`] is a [`DisplayModeKHR`] handle specifying the mode
    ///to use when displaying this surface.
    display_mode: DisplayModeKHR,
    ///[`plane_index`] is the plane on which this surface appears.
    plane_index: u32,
    ///[`plane_stack_index`] is the z-order of the plane.
    plane_stack_index: u32,
    ///[`transform`] is a [`SurfaceTransformFlagBitsKHR`] value
    ///specifying the transformation to apply to images as part of the scanout
    ///operation.
    transform: SurfaceTransformFlagBitsKHR,
    ///[`global_alpha`] is the global alpha value.
    ///This value is ignored if [`alpha_mode`] is not
    ///`VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR`.
    global_alpha: f32,
    ///[`alpha_mode`] is a [`DisplayPlaneAlphaFlagBitsKHR`] value
    ///specifying the type of alpha blending to use.
    alpha_mode: DisplayPlaneAlphaFlagBitsKHR,
    ///[`image_extent`] is the size of the presentable images to use with the
    ///surface.
    image_extent: Extent2D,
}
impl<'lt> Default for DisplaySurfaceCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            display_mode: Default::default(),
            plane_index: 0,
            plane_stack_index: 0,
            transform: Default::default(),
            global_alpha: 0.0,
            alpha_mode: Default::default(),
            image_extent: Default::default(),
        }
    }
}
impl<'lt> DisplaySurfaceCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::plane_index`]
    pub fn plane_index_raw(&self) -> u32 {
        self.plane_index
    }
    ///Gets the raw value of [`Self::plane_stack_index`]
    pub fn plane_stack_index_raw(&self) -> u32 {
        self.plane_stack_index
    }
    ///Gets the raw value of [`Self::global_alpha`]
    pub fn global_alpha_raw(&self) -> f32 {
        self.global_alpha
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
    ///Sets the raw value of [`Self::plane_stack_index`]
    pub fn set_plane_stack_index_raw(&mut self, value: u32) -> &mut Self {
        self.plane_stack_index = value;
        self
    }
    ///Sets the raw value of [`Self::global_alpha`]
    pub fn set_global_alpha_raw(&mut self, value: f32) -> &mut Self {
        self.global_alpha = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> DisplaySurfaceCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::display_mode`]
    pub fn display_mode(&self) -> DisplayModeKHR {
        self.display_mode
    }
    ///Gets the value of [`Self::plane_index`]
    pub fn plane_index(&self) -> u32 {
        self.plane_index
    }
    ///Gets the value of [`Self::plane_stack_index`]
    pub fn plane_stack_index(&self) -> u32 {
        self.plane_stack_index
    }
    ///Gets the value of [`Self::transform`]
    pub fn transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.transform
    }
    ///Gets the value of [`Self::global_alpha`]
    pub fn global_alpha(&self) -> f32 {
        self.global_alpha
    }
    ///Gets the value of [`Self::alpha_mode`]
    pub fn alpha_mode(&self) -> DisplayPlaneAlphaFlagBitsKHR {
        self.alpha_mode
    }
    ///Gets the value of [`Self::image_extent`]
    pub fn image_extent(&self) -> Extent2D {
        self.image_extent
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DisplaySurfaceCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::display_mode`]
    pub fn display_mode_mut(&mut self) -> &mut DisplayModeKHR {
        &mut self.display_mode
    }
    ///Gets a mutable reference to the value of [`Self::plane_index`]
    pub fn plane_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::plane_stack_index`]
    pub fn plane_stack_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::transform`]
    pub fn transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.transform
    }
    ///Gets a mutable reference to the value of [`Self::global_alpha`]
    pub fn global_alpha_mut(&mut self) -> &mut f32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::alpha_mode`]
    pub fn alpha_mode_mut(&mut self) -> &mut DisplayPlaneAlphaFlagBitsKHR {
        &mut self.alpha_mode
    }
    ///Gets a mutable reference to the value of [`Self::image_extent`]
    pub fn image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.image_extent
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
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::khr_display::DisplaySurfaceCreateFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::display_mode`]
    pub fn set_display_mode(&mut self, value: crate::extensions::khr_display::DisplayModeKHR) -> &mut Self {
        self.display_mode = value;
        self
    }
    ///Sets the raw value of [`Self::plane_index`]
    pub fn set_plane_index(&mut self, value: u32) -> &mut Self {
        self.plane_index = value;
        self
    }
    ///Sets the raw value of [`Self::plane_stack_index`]
    pub fn set_plane_stack_index(&mut self, value: u32) -> &mut Self {
        self.plane_stack_index = value;
        self
    }
    ///Sets the raw value of [`Self::transform`]
    pub fn set_transform(&mut self, value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> &mut Self {
        self.transform = value;
        self
    }
    ///Sets the raw value of [`Self::global_alpha`]
    pub fn set_global_alpha(&mut self, value: f32) -> &mut Self {
        self.global_alpha = value;
        self
    }
    ///Sets the raw value of [`Self::alpha_mode`]
    pub fn set_alpha_mode(&mut self, value: crate::extensions::khr_display::DisplayPlaneAlphaFlagBitsKHR) -> &mut Self {
        self.alpha_mode = value;
        self
    }
    ///Sets the raw value of [`Self::image_extent`]
    pub fn set_image_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.image_extent = value;
        self
    }
}
