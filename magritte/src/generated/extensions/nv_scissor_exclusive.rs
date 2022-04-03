//![VK_NV_scissor_exclusive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_scissor_exclusive.html) - device extension
//!# Description
//!This extension adds support for an exclusive scissor test to Vulkan.
//!The exclusive scissor test behaves like the scissor test, except that the
//!exclusive scissor test fails for pixels inside the corresponding rectangle
//!and passes for pixels outside the rectangle.
//!If the same rectangle is used for both the scissor and exclusive scissor
//!tests, the exclusive scissor test will pass if and only if the scissor test
//!fails.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Pat Brown [nvpbrown](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_scissor_exclusive]
//!   @nvpbrown%0A<<Here describe the issue or question you have about the VK_NV_scissor_exclusive
//!   extension>>)
//!# New functions & commands
//! - [`cmd_set_exclusive_scissor_nv`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceExclusiveScissorFeaturesNV`]
//! - Extending [`PipelineViewportStateCreateInfo`]:  -
//!   [`PipelineViewportExclusiveScissorStateCreateInfoNV`]
//!# New constants
//! - [`NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME`]
//! - [`NV_SCISSOR_EXCLUSIVE_SPEC_VERSION`]
//! - Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV`
//!# Known issues & F.A.Q
//!1) For the scissor test, the viewport state must be created with a matching
//!   number of scissor and viewport rectangles.
//!   Should we have the same requirement for exclusive scissors? **RESOLVED** : For exclusive
//! scissors, we relax this requirement and allow an
//!exclusive scissor rectangle count that is either zero or equal to the number
//!of viewport rectangles.
//!If you pass in an exclusive scissor count of zero, the exclusive scissor
//!test is treated as disabled.
//!# Version History
//! - Revision 1, 2018-07-31 (Pat Brown)  - Internal revisions
//!# Other info
//! * 2018-07-31
//! * No known IP claims.
//! * None
//! * - Pat Brown, NVIDIA  - Jeff Bolz, NVIDIA  - Piers Daniell, NVIDIA  - Daniel Koch, NVIDIA
//!# Related
//! - [`PhysicalDeviceExclusiveScissorFeaturesNV`]
//! - [`PipelineViewportExclusiveScissorStateCreateInfoNV`]
//! - [`cmd_set_exclusive_scissor_nv`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Device, Rect2D, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_SCISSOR_EXCLUSIVE_SPEC_VERSION")]
pub const NV_SCISSOR_EXCLUSIVE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME")]
pub const NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_scissor_exclusive");
///[vkCmdSetExclusiveScissorNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html) - Set exclusive scissor rectangles dynamically for a command buffer
///# C Specifications
///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the exclusive scissor
///rectangles, call:
///```c
///// Provided by VK_NV_scissor_exclusive
///void vkCmdSetExclusiveScissorNV(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    firstExclusiveScissor,
///    uint32_t                                    exclusiveScissorCount,
///    const VkRect2D*                             pExclusiveScissors);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`first_exclusive_scissor`] is the index of the first exclusive scissor rectangle whose state
///   is updated by the command.
/// - [`exclusive_scissor_count`] is the number of exclusive scissor rectangles updated by the
///   command.
/// - [`p_exclusive_scissors`] is a pointer to an array of [`Rect2D`] structures defining exclusive
///   scissor rectangles.
///# Description
///The scissor rectangles taken from element i of
///[`p_exclusive_scissors`] replace the current state for the scissor index
///[`first_exclusive_scissor`] +  i, for i in [0,
///[`exclusive_scissor_count`]).This command sets the exclusive scissor rectangles for subsequent
/// drawing
///commands when the graphics pipeline is created with
///`VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV` set in
///[`PipelineDynamicStateCreateInfo::dynamic_states`].
///Otherwise, this state is specified by the
///[`PipelineViewportExclusiveScissorStateCreateInfoNV`]::[`p_exclusive_scissors`]
///values used to create the currently active pipeline.
///## Valid Usage
/// - The [exclusive scissor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-exclusiveScissor)
///   feature  **must**  be enabled
/// - The sum of [`first_exclusive_scissor`] and [`exclusive_scissor_count`] **must**  be between
///   `1` and [`PhysicalDeviceLimits::max_viewports`], inclusive
/// - If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport)
///   feature is not enabled, [`first_exclusive_scissor`] **must**  be `0`
/// - If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport)
///   feature is not enabled, [`exclusive_scissor_count`] **must**  be `1`
/// - The `x` and `y` members of `offset` in each member of [`p_exclusive_scissors`] **must**  be
///   greater than or equal to `0`
/// - Evaluation of (`offset.x` +  `extent.width`) for each member of [`p_exclusive_scissors`]
///   **must**  not cause a signed integer addition overflow
/// - Evaluation of (`offset.y` +  `extent.height`) for each member of [`p_exclusive_scissors`]
///   **must**  not cause a signed integer addition overflow
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_exclusive_scissors`] **must**  be a valid pointer to an array of
///   [`exclusive_scissor_count`][`Rect2D`] structures
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - [`exclusive_scissor_count`] **must**  be greater than `0`
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_NV_scissor_exclusive`]
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
#[doc(alias = "vkCmdSetExclusiveScissorNV")]
pub type FNCmdSetExclusiveScissorNv = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissors: *const Rect2D,
    ),
>;
///[VkPhysicalDeviceExclusiveScissorFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExclusiveScissorFeaturesNV.html) - Structure describing exclusive scissor features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceExclusiveScissorFeaturesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_scissor_exclusive
///typedef struct VkPhysicalDeviceExclusiveScissorFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           exclusiveScissor;
///} VkPhysicalDeviceExclusiveScissorFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`exclusive_scissor`] indicates that the implementation supports the exclusive scissor test.
///See [Exclusive Scissor Test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-exclusive-scissor) for more
///information.If the [`PhysicalDeviceExclusiveScissorFeaturesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceExclusiveScissorFeaturesNV`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV`
///# Related
/// - [`VK_NV_scissor_exclusive`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceExclusiveScissorFeaturesNV")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`exclusive_scissor`] indicates that the
    ///implementation supports the exclusive scissor test.
    pub exclusive_scissor: Bool32,
}
impl<'lt> Default for PhysicalDeviceExclusiveScissorFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceExclusiveScissorFeaturesNv,
            p_next: std::ptr::null_mut(),
            exclusive_scissor: 0,
        }
    }
}
impl<'lt> PhysicalDeviceExclusiveScissorFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::exclusive_scissor`]
    pub fn exclusive_scissor_raw(&self) -> Bool32 {
        self.exclusive_scissor
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::exclusive_scissor`]
    pub fn set_exclusive_scissor_raw(&mut self, value: Bool32) -> &mut Self {
        self.exclusive_scissor = value;
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
    ///Gets the value of [`Self::exclusive_scissor`]
    pub fn exclusive_scissor(&self) -> bool {
        unsafe { std::mem::transmute(self.exclusive_scissor as u8) }
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
    ///Gets a mutable reference to the value of [`Self::exclusive_scissor`]
    pub fn exclusive_scissor_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.exclusive_scissor as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.exclusive_scissor as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the raw value of [`Self::exclusive_scissor`]
    pub fn set_exclusive_scissor(&mut self, value: bool) -> &mut Self {
        self.exclusive_scissor = value as u8 as u32;
        self
    }
}
///[VkPipelineViewportExclusiveScissorStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html) - Structure specifying parameters controlling exclusive scissor testing
///# C Specifications
///The [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_scissor_exclusive
///typedef struct VkPipelineViewportExclusiveScissorStateCreateInfoNV {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           exclusiveScissorCount;
///    const VkRect2D*    pExclusiveScissors;
///} VkPipelineViewportExclusiveScissorStateCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`exclusive_scissor_count`] is the number of exclusive scissor rectangles.
/// - [`exclusive_scissors`] is a pointer to an array of [`Rect2D`] structures defining exclusive
///   scissor rectangles.
///# Description
///If the `VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV` dynamic state is enabled
///for a pipeline, the [`exclusive_scissors`] member is ignored.When this structure is included in
/// the [`p_next`] chain of
///[`GraphicsPipelineCreateInfo`], it defines parameters of the exclusive
///scissor test.
///If this structure is not included in the [`p_next`] chain, it is equivalent
///to specifying this structure with a [`exclusive_scissor_count`] of `0`.
///## Valid Usage
/// - If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport)
///   feature is not enabled, [`exclusive_scissor_count`] **must**  be `0` or `1`
/// - [`exclusive_scissor_count`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_viewports`]
/// - [`exclusive_scissor_count`] **must**  be `0` or greater than or equal to the `viewportCount`
///   member of [`PipelineViewportStateCreateInfo`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV`
///# Related
/// - [`VK_NV_scissor_exclusive`]
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
#[doc(alias = "VkPipelineViewportExclusiveScissorStateCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`exclusive_scissor_count`] is the number of exclusive scissor
    ///rectangles.
    pub exclusive_scissor_count: u32,
    ///[`exclusive_scissors`] is a pointer to an array of [`Rect2D`]
    ///structures defining exclusive scissor rectangles.
    pub exclusive_scissors: *const Rect2D,
}
impl<'lt> Default for PipelineViewportExclusiveScissorStateCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PipelineViewportExclusiveScissorStateCreateInfoNv,
            p_next: std::ptr::null(),
            exclusive_scissor_count: 0,
            exclusive_scissors: std::ptr::null(),
        }
    }
}
impl<'lt> PipelineViewportExclusiveScissorStateCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::exclusive_scissors`]
    pub fn exclusive_scissors_raw(&self) -> *const Rect2D {
        self.exclusive_scissors
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::exclusive_scissors`]
    pub fn set_exclusive_scissors_raw(&mut self, value: *const Rect2D) -> &mut Self {
        self.exclusive_scissors = value;
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
    ///Gets the value of [`Self::exclusive_scissor_count`]
    pub fn exclusive_scissor_count(&self) -> u32 {
        self.exclusive_scissor_count
    }
    ///Gets the value of [`Self::exclusive_scissors`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn exclusive_scissors(&self) -> &[Rect2D] {
        std::slice::from_raw_parts(self.exclusive_scissors, self.exclusive_scissor_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::exclusive_scissor_count`]
    pub fn exclusive_scissor_count_mut(&mut self) -> &mut u32 {
        &mut self.exclusive_scissor_count
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
    ///Sets the raw value of [`Self::exclusive_scissor_count`]
    pub fn set_exclusive_scissor_count(&mut self, value: u32) -> &mut Self {
        self.exclusive_scissor_count = value;
        self
    }
    ///Sets the raw value of [`Self::exclusive_scissors`]
    pub fn set_exclusive_scissors(&mut self, value: &'lt [crate::vulkan1_0::Rect2D]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.exclusive_scissors = value.as_ptr();
        self.exclusive_scissor_count = len_;
        self
    }
}
///The V-table of [`Device`] for functions from VK_NV_scissor_exclusive
pub struct DeviceNvScissorExclusiveVTable {
    ///See [`FNCmdSetExclusiveScissorNv`] for more information.
    pub cmd_set_exclusive_scissor_nv: FNCmdSetExclusiveScissorNv,
}
impl DeviceNvScissorExclusiveVTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Device) -> Self
    where
        F: Fn(Device, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            cmd_set_exclusive_scissor_nv: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetExclusiveScissorNV")))
            },
        }
    }
    ///Gets [`Self::cmd_set_exclusive_scissor_nv`]. See [`FNCmdSetExclusiveScissorNv`] for more
    /// information.
    pub fn cmd_set_exclusive_scissor_nv(&self) -> FNCmdSetExclusiveScissorNv {
        self.cmd_set_exclusive_scissor_nv
    }
}
