//![VK_EXT_discard_rectangles](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_discard_rectangles.html) - device extension
//!# Description
//!This extension provides additional orthogonally aligned “discard
//!rectangles” specified in framebuffer-space coordinates that restrict
//!rasterization of all points, lines and triangles.From zero to an implementation-dependent limit
//! (specified by
//!`maxDiscardRectangles`) number of discard rectangles can be operational
//!at once.
//!When one or more discard rectangles are active, rasterized fragments can
//!either survive if the fragment is within any of the operational discard
//!rectangles (`VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT` mode) or be
//!rejected if the fragment is within any of the operational discard rectangles
//!(`VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT` mode).These discard rectangles operate orthogonally
//! to the existing scissor test
//!functionality.
//!The discard rectangles can be different for each physical device in a device
//!group by specifying the device mask and setting discard rectangle dynamic
//!state.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_discard_rectangles]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_discard_rectangles extension>>)
//!# New functions & commands
//! - [`cmd_set_discard_rectangle_ext`]
//!# New structures
//! - Extending [`GraphicsPipelineCreateInfo`]:  - [`PipelineDiscardRectangleStateCreateInfoEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceDiscardRectanglePropertiesEXT`]
//!# New enums
//! - [`DiscardRectangleModeEXT`]
//!# New bitmasks
//! - [`PipelineDiscardRectangleStateCreateFlagsEXT`]
//!# New constants
//! - [`EXT_DISCARD_RECTANGLES_EXTENSION_NAME`]
//! - [`EXT_DISCARD_RECTANGLES_SPEC_VERSION`]
//! - Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2016-12-22 (Piers Daniell)  - Internal revisions
//!# Other info
//! * 2016-12-22
//! * - Interacts with `[`VK_KHR_device_group`]`  - Interacts with Vulkan 1.1
//! * - Daniel Koch, NVIDIA  - Jeff Bolz, NVIDIA
//!# Related
//! - [`DiscardRectangleModeEXT`]
//! - [`PhysicalDeviceDiscardRectanglePropertiesEXT`]
//! - [`PipelineDiscardRectangleStateCreateFlagsEXT`]
//! - [`PipelineDiscardRectangleStateCreateInfoEXT`]
//! - [`cmd_set_discard_rectangle_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, CommandBuffer, Device, Rect2D, StructureType},
    AsRaw, Unique,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION")]
pub const EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME")]
pub const EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_discard_rectangles");
///[vkCmdSetDiscardRectangleEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEXT.html) - Set discard rectangles dynamically for a command buffer
///# C Specifications
///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the discard rectangles,
///call:
///```c
///// Provided by VK_EXT_discard_rectangles
///void vkCmdSetDiscardRectangleEXT(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    firstDiscardRectangle,
///    uint32_t                                    discardRectangleCount,
///    const VkRect2D*                             pDiscardRectangles);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`first_discard_rectangle`] is the index of the first discard rectangle whose state is updated
///   by the command.
/// - [`discard_rectangle_count`] is the number of discard rectangles whose state are updated by the
///   command.
/// - [`p_discard_rectangles`] is a pointer to an array of [`Rect2D`] structures specifying discard
///   rectangles.
///# Description
///The discard rectangle taken from element i of [`p_discard_rectangles`]
///replace the current state for the discard rectangle at index
///[`first_discard_rectangle`] +  i, for i in [0,
///[`discard_rectangle_count`]).This command sets the discard rectangles for subsequent drawing
/// commands
///when the graphics pipeline is created with
///`VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT` set in
///[`PipelineDynamicStateCreateInfo::dynamic_states`].
///Otherwise, this state is specified by the
///[`PipelineDiscardRectangleStateCreateInfoEXT`]::[`p_discard_rectangles`]
///values used to create the currently active pipeline.
///## Valid Usage
/// - The sum of [`first_discard_rectangle`] and [`discard_rectangle_count`] **must**  be less than
///   or equal to [`PhysicalDeviceDiscardRectanglePropertiesEXT::max_discard_rectangles`]
/// - The `x` and `y` member of `offset` in each [`Rect2D`] element of [`p_discard_rectangles`]
///   **must**  be greater than or equal to `0`
/// - Evaluation of (`offset.x` +  `extent.width`) in each [`Rect2D`] element of
///   [`p_discard_rectangles`] **must**  not cause a signed integer addition overflow
/// - Evaluation of (`offset.y` +  `extent.height`) in each [`Rect2D`] element of
///   [`p_discard_rectangles`] **must**  not cause a signed integer addition overflow
/// - If this command is recorded in a secondary command buffer with
///   [`CommandBufferInheritanceViewportScissorInfoNV::viewport_scissor_2_d`] enabled, then this
///   function  **must**  not be called
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_discard_rectangles`] **must**  be a valid pointer to an array of
///   [`discard_rectangle_count`][`Rect2D`] structures
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - [`discard_rectangle_count`] **must**  be greater than `0`
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_EXT_discard_rectangles`]
/// - [`CommandBuffer`]
/// - [`Rect2D`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdSetDiscardRectangleEXT")]
pub type FNCmdSetDiscardRectangleExt = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangle_count: u32,
        p_discard_rectangles: *const Rect2D,
    ),
>;
///[VkDiscardRectangleModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDiscardRectangleModeEXT.html) - Specify the discard rectangle mode
///# C Specifications
///[`DiscardRectangleModeEXT`] values are:
///```c
///// Provided by VK_EXT_discard_rectangles
///typedef enum VkDiscardRectangleModeEXT {
///    VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT = 0,
///    VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT = 1,
///} VkDiscardRectangleModeEXT;
///```
///# Description
/// - [`INCLUSIVE`] specifies that the discard rectangle test is inclusive.
/// - [`EXCLUSIVE`] specifies that the discard rectangle test is exclusive.
///# Related
/// - [`VK_EXT_discard_rectangles`]
/// - [`PipelineDiscardRectangleStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDiscardRectangleModeEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DiscardRectangleModeEXT(i32);
impl const Default for DiscardRectangleModeEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl DiscardRectangleModeEXT {
    ///[`INCLUSIVE`] specifies that the discard
    ///rectangle test is inclusive.
    pub const INCLUSIVE: Self = Self(0);
    ///[`EXCLUSIVE`] specifies that the discard
    ///rectangle test is exclusive.
    pub const EXCLUSIVE: Self = Self(1);
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
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for DiscardRectangleModeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(DiscardRectangleModeEXT))
            .field(match *self {
                Self::INCLUSIVE => &"INCLUSIVE",
                Self::EXCLUSIVE => &"EXCLUSIVE",
                other => unreachable!(
                    concat!("invalid value for", stringify!(DiscardRectangleModeEXT), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for DiscardRectangleModeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::INCLUSIVE => &"INCLUSIVE",
            Self::EXCLUSIVE => &"EXCLUSIVE",
            other => unreachable!(
                concat!("invalid value for", stringify!(DiscardRectangleModeEXT), ": {:?}"),
                other
            ),
        })
    }
}
///[VkPipelineDiscardRectangleStateCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateFlagsEXT.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_EXT_discard_rectangles
///typedef VkFlags VkPipelineDiscardRectangleStateCreateFlagsEXT;
///```
///# Related
/// - [`VK_EXT_discard_rectangles`]
/// - [`PipelineDiscardRectangleStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PipelineDiscardRectangleStateCreateFlagsEXT(u32);
impl const Default for PipelineDiscardRectangleStateCreateFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PipelineDiscardRectangleStateCreateFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PipelineDiscardRectangleStateCreateFlagsEXT))
            .field(&self.0)
            .finish()
    }
}
///[VkPhysicalDeviceDiscardRectanglePropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html) - Structure describing discard rectangle limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDiscardRectanglePropertiesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_discard_rectangles
///typedef struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxDiscardRectangles;
///} VkPhysicalDeviceDiscardRectanglePropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_discard_rectangles`] is the maximum number of active discard rectangles that  **can**
///   be specified.
///# Description
///If the [`PhysicalDeviceDiscardRectanglePropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_discard_rectangles`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceDiscardRectanglePropertiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_discard_rectangles`] is the
    ///maximum number of active discard rectangles that  **can**  be specified.
    pub max_discard_rectangles: u32,
}
impl<'lt> Default for PhysicalDeviceDiscardRectanglePropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_discard_rectangles: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDiscardRectanglePropertiesEXT<'lt> {
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
    ///Gets the value of [`Self::max_discard_rectangles`]
    pub fn max_discard_rectangles(&self) -> u32 {
        self.max_discard_rectangles
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
    ///Gets a mutable reference to the value of [`Self::max_discard_rectangles`]
    pub fn max_discard_rectangles_mut(&mut self) -> &mut u32 {
        &mut self.max_discard_rectangles
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
    ///Sets the value of [`Self::max_discard_rectangles`]
    pub fn set_max_discard_rectangles(mut self, value: u32) -> Self {
        self.max_discard_rectangles = value;
        self
    }
}
///[VkPipelineDiscardRectangleStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateInfoEXT.html) - Structure specifying discard rectangle
///# C Specifications
///The [`PipelineDiscardRectangleStateCreateInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_discard_rectangles
///typedef struct VkPipelineDiscardRectangleStateCreateInfoEXT {
///    VkStructureType                                  sType;
///    const void*                                      pNext;
///    VkPipelineDiscardRectangleStateCreateFlagsEXT    flags;
///    VkDiscardRectangleModeEXT                        discardRectangleMode;
///    uint32_t                                         discardRectangleCount;
///    const VkRect2D*                                  pDiscardRectangles;
///} VkPipelineDiscardRectangleStateCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`discard_rectangle_mode`] is a [`DiscardRectangleModeEXT`] value determining whether the
///   discard rectangle test is inclusive or exclusive.
/// - [`discard_rectangle_count`] is the number of discard rectangles to use.
/// - [`discard_rectangles`] is a pointer to an array of [`Rect2D`] structures defining discard
///   rectangles.
///# Description
///If the `VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT` dynamic state is enabled
///for a pipeline, the [`discard_rectangles`] member is ignored.When this structure is included in
/// the [`p_next`] chain of
///[`GraphicsPipelineCreateInfo`], it defines parameters of the discard
///rectangle test.
///If this structure is not included in the [`p_next`] chain, it is equivalent
///to specifying this structure with a [`discard_rectangle_count`] of `0`.
///## Valid Usage
/// - [`discard_rectangle_count`] **must**  be less than or equal to
///   [`PhysicalDeviceDiscardRectanglePropertiesEXT::max_discard_rectangles`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT`
/// - [`flags`] **must**  be `0`
/// - [`discard_rectangle_mode`] **must**  be a valid [`DiscardRectangleModeEXT`] value
///# Related
/// - [`VK_EXT_discard_rectangles`]
/// - [`DiscardRectangleModeEXT`]
/// - [`PipelineDiscardRectangleStateCreateFlagsEXT`]
/// - [`Rect2D`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineDiscardRectangleStateCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: PipelineDiscardRectangleStateCreateFlagsEXT,
    ///[`discard_rectangle_mode`] is a [`DiscardRectangleModeEXT`] value
    ///determining whether the discard rectangle test is inclusive or
    ///exclusive.
    pub discard_rectangle_mode: DiscardRectangleModeEXT,
    ///[`discard_rectangle_count`] is the number of discard rectangles to use.
    pub discard_rectangle_count: u32,
    ///[`discard_rectangles`] is a pointer to an array of [`Rect2D`]
    ///structures defining discard rectangles.
    pub discard_rectangles: *const Rect2D,
}
impl<'lt> Default for PipelineDiscardRectangleStateCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            discard_rectangle_mode: Default::default(),
            discard_rectangle_count: 0,
            discard_rectangles: std::ptr::null(),
        }
    }
}
impl<'lt> PipelineDiscardRectangleStateCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::discard_rectangles`]
    pub fn discard_rectangles_raw(&self) -> *const Rect2D {
        self.discard_rectangles
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::discard_rectangles`]
    pub fn set_discard_rectangles_raw(mut self, value: *const Rect2D) -> Self {
        self.discard_rectangles = value;
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
    pub fn flags(&self) -> PipelineDiscardRectangleStateCreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::discard_rectangle_mode`]
    pub fn discard_rectangle_mode(&self) -> DiscardRectangleModeEXT {
        self.discard_rectangle_mode
    }
    ///Gets the value of [`Self::discard_rectangle_count`]
    pub fn discard_rectangle_count(&self) -> u32 {
        self.discard_rectangle_count
    }
    ///Gets the value of [`Self::discard_rectangles`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn discard_rectangles(&self) -> &[Rect2D] {
        std::slice::from_raw_parts(self.discard_rectangles, self.discard_rectangle_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PipelineDiscardRectangleStateCreateFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::discard_rectangle_mode`]
    pub fn discard_rectangle_mode_mut(&mut self) -> &mut DiscardRectangleModeEXT {
        &mut self.discard_rectangle_mode
    }
    ///Gets a mutable reference to the value of [`Self::discard_rectangle_count`]
    pub fn discard_rectangle_count_mut(&mut self) -> &mut u32 {
        &mut self.discard_rectangle_count
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
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        mut self,
        value: crate::extensions::ext_discard_rectangles::PipelineDiscardRectangleStateCreateFlagsEXT,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::discard_rectangle_mode`]
    pub fn set_discard_rectangle_mode(
        mut self,
        value: crate::extensions::ext_discard_rectangles::DiscardRectangleModeEXT,
    ) -> Self {
        self.discard_rectangle_mode = value;
        self
    }
    ///Sets the value of [`Self::discard_rectangle_count`]
    pub fn set_discard_rectangle_count(mut self, value: u32) -> Self {
        self.discard_rectangle_count = value;
        self
    }
    ///Sets the value of [`Self::discard_rectangles`]
    pub fn set_discard_rectangles(mut self, value: &'lt [crate::vulkan1_0::Rect2D]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.discard_rectangles = value.as_ptr();
        self.discard_rectangle_count = len_;
        self
    }
}
impl CommandBuffer {
    ///[vkCmdSetDiscardRectangleEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEXT.html) - Set discard rectangles dynamically for a command buffer
    ///# C Specifications
    ///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the discard rectangles,
    ///call:
    ///```c
    ///// Provided by VK_EXT_discard_rectangles
    ///void vkCmdSetDiscardRectangleEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    uint32_t                                    firstDiscardRectangle,
    ///    uint32_t                                    discardRectangleCount,
    ///    const VkRect2D*                             pDiscardRectangles);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`first_discard_rectangle`] is the index of the first discard rectangle whose state is
    ///   updated by the command.
    /// - [`discard_rectangle_count`] is the number of discard rectangles whose state are updated by
    ///   the command.
    /// - [`p_discard_rectangles`] is a pointer to an array of [`Rect2D`] structures specifying
    ///   discard rectangles.
    ///# Description
    ///The discard rectangle taken from element i of [`p_discard_rectangles`]
    ///replace the current state for the discard rectangle at index
    ///[`first_discard_rectangle`] +  i, for i in [0,
    ///[`discard_rectangle_count`]).This command sets the discard rectangles for subsequent drawing
    /// commands
    ///when the graphics pipeline is created with
    ///`VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT` set in
    ///[`PipelineDynamicStateCreateInfo::dynamic_states`].
    ///Otherwise, this state is specified by the
    ///[`PipelineDiscardRectangleStateCreateInfoEXT`]::[`p_discard_rectangles`]
    ///values used to create the currently active pipeline.
    ///## Valid Usage
    /// - The sum of [`first_discard_rectangle`] and [`discard_rectangle_count`] **must**  be less
    ///   than or equal to [`PhysicalDeviceDiscardRectanglePropertiesEXT::max_discard_rectangles`]
    /// - The `x` and `y` member of `offset` in each [`Rect2D`] element of [`p_discard_rectangles`]
    ///   **must**  be greater than or equal to `0`
    /// - Evaluation of (`offset.x` +  `extent.width`) in each [`Rect2D`] element of
    ///   [`p_discard_rectangles`] **must**  not cause a signed integer addition overflow
    /// - Evaluation of (`offset.y` +  `extent.height`) in each [`Rect2D`] element of
    ///   [`p_discard_rectangles`] **must**  not cause a signed integer addition overflow
    /// - If this command is recorded in a secondary command buffer with
    ///   [`CommandBufferInheritanceViewportScissorInfoNV::viewport_scissor_2_d`] enabled, then this
    ///   function  **must**  not be called
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_discard_rectangles`] **must**  be a valid pointer to an array of
    ///   [`discard_rectangle_count`][`Rect2D`] structures
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    /// - [`discard_rectangle_count`] **must**  be greater than `0`
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_EXT_discard_rectangles`]
    /// - [`CommandBuffer`]
    /// - [`Rect2D`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdSetDiscardRectangleEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_set_discard_rectangle_ext(
        self: &Unique<CommandBuffer>,
        first_discard_rectangle: Option<u32>,
        p_discard_rectangles: &[crate::vulkan1_0::Rect2D],
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_discard_rectangles()
            .and_then(|vtable| vtable.cmd_set_discard_rectangle_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_discard_rectangles()
            .and_then(|vtable| vtable.cmd_set_discard_rectangle_ext())
            .unwrap_unchecked();
        let discard_rectangle_count = (|len: usize| len)(p_discard_rectangles.len()) as _;
        let _return = _function(
            self.as_raw(),
            first_discard_rectangle.unwrap_or_default() as _,
            discard_rectangle_count,
            p_discard_rectangles.as_ptr(),
        );
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_discard_rectangles`
pub struct DeviceExtDiscardRectanglesVTable {
    ///See [`FNCmdSetDiscardRectangleExt`] for more information.
    pub cmd_set_discard_rectangle_ext: FNCmdSetDiscardRectangleExt,
}
impl DeviceExtDiscardRectanglesVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            cmd_set_discard_rectangle_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetDiscardRectangleEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::cmd_set_discard_rectangle_ext`]. See [`FNCmdSetDiscardRectangleExt`] for more
    /// information.
    pub fn cmd_set_discard_rectangle_ext(&self) -> FNCmdSetDiscardRectangleExt {
        self.cmd_set_discard_rectangle_ext
    }
}
