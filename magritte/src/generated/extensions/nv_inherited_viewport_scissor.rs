use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType, Viewport};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION")]
pub const NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME")]
pub const NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_inherited_viewport_scissor");
///[VkPhysicalDeviceInheritedViewportScissorFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInheritedViewportScissorFeaturesNV.html) - Structure describing the viewport scissor inheritance behavior for an implementation
///# C Specifications
///The [`PhysicalDeviceInheritedViewportScissorFeaturesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_inherited_viewport_scissor
///typedef struct VkPhysicalDeviceInheritedViewportScissorFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           inheritedViewportScissor2D;
///} VkPhysicalDeviceInheritedViewportScissorFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`inherited_viewport_scissor_2_d`] indicates whether secondary command buffers can inherit
///   most of the dynamic state affected by `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`,
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT`, `VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT`,
///   `VK_DYNAMIC_STATE_VIEWPORT` or `VK_DYNAMIC_STATE_SCISSOR`, from a primary command buffer.
///If the [`PhysicalDeviceInheritedViewportScissorFeaturesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceInheritedViewportScissorFeaturesNV`] **can**  also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV`
///# Related
/// - [`VK_NV_inherited_viewport_scissor`]
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`inherited_viewport_scissor_2_d`] indicates whether secondary command
    ///buffers can inherit most of the dynamic state affected by
    ///`VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`,
    ///`VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT`,
    ///`VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT`,
    ///`VK_DYNAMIC_STATE_VIEWPORT` or `VK_DYNAMIC_STATE_SCISSOR`,
    ///from a primary command buffer.
    inherited_viewport_scissor_2_d: Bool32,
}
impl<'lt> Default for PhysicalDeviceInheritedViewportScissorFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            inherited_viewport_scissor_2_d: 0,
        }
    }
}
impl<'lt> PhysicalDeviceInheritedViewportScissorFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::inherited_viewport_scissor_2_d`]
    pub fn inherited_viewport_scissor_2_d_raw(&self) -> Bool32 {
        self.inherited_viewport_scissor_2_d
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::inherited_viewport_scissor_2_d`]
    pub fn set_inherited_viewport_scissor_2_d_raw(&mut self, value: Bool32) -> &mut Self {
        self.inherited_viewport_scissor_2_d = value;
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
    ///Gets the value of [`Self::inherited_viewport_scissor_2_d`]
    pub fn inherited_viewport_scissor_2_d(&self) -> bool {
        unsafe { std::mem::transmute(self.inherited_viewport_scissor_2_d as u8) }
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
    ///Gets a mutable reference to the value of [`Self::inherited_viewport_scissor_2_d`]
    pub fn inherited_viewport_scissor_2_d_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.inherited_viewport_scissor_2_d as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.inherited_viewport_scissor_2_d as *mut Bool32)
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
    ///Sets the raw value of [`Self::inherited_viewport_scissor_2_d`]
    pub fn set_inherited_viewport_scissor_2_d(&mut self, value: bool) -> &mut Self {
        self.inherited_viewport_scissor_2_d = value as u8 as u32;
        self
    }
}
///[VkCommandBufferInheritanceViewportScissorInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceViewportScissorInfoNV.html) - Structure specifying command buffer inheritance information
///# C Specifications
///The [`CommandBufferInheritanceViewportScissorInfoNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_inherited_viewport_scissor
///typedef struct VkCommandBufferInheritanceViewportScissorInfoNV {
///    VkStructureType      sType;
///    const void*          pNext;
///    VkBool32             viewportScissor2D;
///    uint32_t             viewportDepthCount;
///    const VkViewport*    pViewportDepths;
///} VkCommandBufferInheritanceViewportScissorInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`viewport_scissor_2_d`] specifies whether the listed dynamic state is inherited.
/// - [`viewport_depth_count`] specifies the maximum number of viewports to inherit. When
///   [`viewport_scissor_2_d`] is [`FALSE`], the behavior is as if this value is zero.
/// - [`viewport_depths`] is a pointer to a [`Viewport`] structure specifying the expected depth
///   range for each inherited viewport.
///# Description
///If the [`p_next`] chain of [`CommandBufferInheritanceInfo`] includes a
///[`CommandBufferInheritanceViewportScissorInfoNV`] structure, then that
///structure controls whether a command buffer  **can**  inherit the following state
///from other command buffers:
/// - `VK_DYNAMIC_STATE_SCISSOR`
/// - `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT`
/// - `VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT`
///as well as the following state, with restrictions on inherited depth values
///and viewport count:
/// - `VK_DYNAMIC_STATE_VIEWPORT`
/// - `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`
///If [`viewport_scissor_2_d`] is [`FALSE`], then the command buffer does
///not inherit the listed dynamic state, and  **should**  set this state itself.
///If this structure is not present, the behavior is as if
///[`viewport_scissor_2_d`] is [`FALSE`].If [`viewport_scissor_2_d`] is [`TRUE`], then the listed
/// dynamic state
///is inherited, and the command buffer  **must**  not set this
///state, except that the viewport and scissor count  **may**  be set by binding a
///graphics pipeline that does not specify this state as dynamic.When the command buffer is
/// executed as part of a the execution of a
///[`CmdExecuteCommands`] command, the inherited state (if enabled) is
///determined by the following procedure, performed separately for each dynamic
///state, and separately for each value for dynamic state that consists of
///multiple values (e.g. multiple viewports).
/// - With i being the index of the executed command buffer in the `pCommandBuffers` array of
///   [`CmdExecuteCommands`], if i > 0 and any secondary command buffer from index 0 to i-1 modifies
///   the state, the inherited state is provisionally set to the final value set by the last such
///   secondary command buffer. Binding a graphics pipeline defining the state statically is
///   equivalent to setting the state to an undefined value.
/// - Otherwise, the tentatative inherited state is that of the primary command buffer at the point
///   the [`CmdExecuteCommands`] command was recorded; if the state is undefined, then so is the
///   provisional inherited state.
/// - If the provisional inherited state is an undefined value, then the state is not inherited.
/// - If the provisional inherited state is a viewport, with n being its viewport index, then if n ≥
///   [`viewport_depth_count`], or if either [`Viewport::min_depth`] or [`Viewport::max_depth`] are
///   not equal to the respective values of the n<sup>th</sup> element of [`viewport_depths`], then
///   the state is not inherited.
/// - If the provisional inherited state passes both checks, then it becomes the actual inherited
///   state.
///
///## Valid Usage
/// - If the [inherited viewport scissor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-inheritedViewportScissor2D)
///   feature is not enabled, [`viewport_scissor_2_d`] **must**  be [`FALSE`]
/// - If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport)
///   feature is not enabled and [`viewport_scissor_2_d`] is [`TRUE`], then [`viewport_depth_count`]
///   **must**  be `1`
/// - If [`viewport_scissor_2_d`] is [`TRUE`], then [`viewport_depth_count`] **must**  be greater
///   than `0`
/// - If [`viewport_scissor_2_d`] is [`TRUE`], then [`viewport_depths`] **must**  be a valid pointer
///   to an array of [`viewport_depth_count`] valid [`Viewport`] structures, except any requirements
///   on `x`, `y`, `width`, and `height` do not apply
/// - If [`viewport_scissor_2_d`] is [`TRUE`], then the command buffer  **must**  be recorded with
///   the `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV`
///# Related
/// - [`VK_NV_inherited_viewport_scissor`]
/// - [`Bool32`]
/// - [`StructureType`]
/// - [`Viewport`]
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
pub struct CommandBufferInheritanceViewportScissorInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`viewport_scissor_2_d`] specifies whether the listed dynamic state is
    ///inherited.
    viewport_scissor_2_d: Bool32,
    ///[`viewport_depth_count`] specifies the maximum number of viewports to
    ///inherit.
    ///When [`viewport_scissor_2_d`] is [`FALSE`], the behavior is as if
    ///this value is zero.
    viewport_depth_count: u32,
    ///[`viewport_depths`] is a pointer to a [`Viewport`] structure
    ///specifying the expected depth range for each inherited viewport.
    viewport_depths: *const Viewport,
}
impl<'lt> Default for CommandBufferInheritanceViewportScissorInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            viewport_scissor_2_d: 0,
            viewport_depth_count: 0,
            viewport_depths: std::ptr::null(),
        }
    }
}
impl<'lt> CommandBufferInheritanceViewportScissorInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::viewport_scissor_2_d`]
    pub fn viewport_scissor_2_d_raw(&self) -> Bool32 {
        self.viewport_scissor_2_d
    }
    ///Gets the raw value of [`Self::viewport_depths`]
    pub fn viewport_depths_raw(&self) -> *const Viewport {
        self.viewport_depths
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::viewport_scissor_2_d`]
    pub fn set_viewport_scissor_2_d_raw(&mut self, value: Bool32) -> &mut Self {
        self.viewport_scissor_2_d = value;
        self
    }
    ///Sets the raw value of [`Self::viewport_depths`]
    pub fn set_viewport_depths_raw(&mut self, value: *const Viewport) -> &mut Self {
        self.viewport_depths = value;
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
    ///Gets the value of [`Self::viewport_scissor_2_d`]
    pub fn viewport_scissor_2_d(&self) -> bool {
        unsafe { std::mem::transmute(self.viewport_scissor_2_d as u8) }
    }
    ///Gets the value of [`Self::viewport_depth_count`]
    pub fn viewport_depth_count(&self) -> u32 {
        self.viewport_depth_count
    }
    ///Gets the value of [`Self::viewport_depths`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn viewport_depths(&self) -> &Viewport {
        &*self.viewport_depths
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::viewport_scissor_2_d`]
    pub fn viewport_scissor_2_d_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.viewport_scissor_2_d as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.viewport_scissor_2_d as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::viewport_depth_count`]
    pub fn viewport_depth_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::viewport_scissor_2_d`]
    pub fn set_viewport_scissor_2_d(&mut self, value: bool) -> &mut Self {
        self.viewport_scissor_2_d = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::viewport_depth_count`]
    pub fn set_viewport_depth_count(&mut self, value: u32) -> &mut Self {
        self.viewport_depth_count = value;
        self
    }
    ///Sets the raw value of [`Self::viewport_depths`]
    pub fn set_viewport_depths(&mut self, value: &'lt crate::vulkan1_0::Viewport) -> &mut Self {
        self.viewport_depths = value as *const _;
        self
    }
}
