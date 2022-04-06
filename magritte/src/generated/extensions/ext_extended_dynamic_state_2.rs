//![VK_EXT_extended_dynamic_state2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state2.html) - device extension
//!# Description
//!This extension adds some more dynamic state to support applications that
//!need to reduce the number of pipeline state objects they compile and bind.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Vikram Kushwaha [vkushwaha-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_extended_dynamic_state2]
//!   @vkushwaha-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_extended_dynamic_state2 extension>>)
//!# New functions & commands
//! - [`CmdSetDepthBiasEnableEXT`]
//! - [`cmd_set_logic_op_ext`]
//! - [`cmd_set_patch_control_points_ext`]
//! - [`CmdSetPrimitiveRestartEnableEXT`]
//! - [`CmdSetRasterizerDiscardEnableEXT`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`]
//!# New constants
//! - [`EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME`]
//! - [`EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION`]
//! - Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE_EXT`  -
//!   `VK_DYNAMIC_STATE_LOGIC_OP_EXT`  - `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT`  -
//!   `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT`  -
//!   `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2021-04-12 (Vikram Kushwaha)  - Internal revisions
//!# Other info
//! * 2021-04-12
//! * - Promoted to Vulkan 1.3 Core
//! * No known IP claims.
//! * - Vikram Kushwaha, NVIDIA  - Piers Daniell, NVIDIA  - Jeff Bolz, NVIDIA
//!# Related
//! - [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`]
//! - [`CmdSetDepthBiasEnableEXT`]
//! - [`cmd_set_logic_op_ext`]
//! - [`cmd_set_patch_control_points_ext`]
//! - [`CmdSetPrimitiveRestartEnableEXT`]
//! - [`CmdSetRasterizerDiscardEnableEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, Device, LogicOp, StructureType},
    vulkan1_3::{FNCmdSetDepthBiasEnable, FNCmdSetPrimitiveRestartEnable, FNCmdSetRasterizerDiscardEnable},
    AsRaw, Unique,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_extended_dynamic_state2");
///[vkCmdSetPatchControlPointsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html) - Specify the number of control points per patch dynamically for a command buffer
///# C Specifications
///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the number of control points
///per patch, call:
///```c
///// Provided by VK_EXT_extended_dynamic_state2
///void vkCmdSetPatchControlPointsEXT(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    patchControlPoints);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`patch_control_points`] specifies the number of control points per patch.
///# Description
///This command sets the number of control points per patch for subsequent
///drawing commands when the graphics pipeline is created with
///`VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT` set in
///[`PipelineDynamicStateCreateInfo::dynamic_states`].
///Otherwise, this state is specified by the
///[`PipelineTessellationStateCreateInfo`]::[`patch_control_points`] value
///used to create the currently active pipeline.
///## Valid Usage
/// - The [extendedDynamicState2PatchControlPoints](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-extendedDynamicState2PatchControlPoints)
///   feature  **must**  be enabled
/// - [`patch_control_points`] **must**  be greater than zero and less than or equal to
///   [`PhysicalDeviceLimits::max_tessellation_patch_size`]
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_EXT_extended_dynamic_state2`]
/// - [`CommandBuffer`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdSetPatchControlPointsEXT")]
pub type FNCmdSetPatchControlPointsExt =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, patch_control_points: u32)>;
///[vkCmdSetLogicOpEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html) - Select which logical operation to apply for blend state dynamically for a command buffer
///# C Specifications
///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the logical operation to
///apply for blend state, call:
///```c
///// Provided by VK_EXT_extended_dynamic_state2
///void vkCmdSetLogicOpEXT(
///    VkCommandBuffer                             commandBuffer,
///    VkLogicOp                                   logicOp);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`logic_op`] specifies the logical operation to apply for blend state.
///# Description
///This command sets the logical operation for blend state for subsequent
///drawing commands when the graphics pipeline is created with
///`VK_DYNAMIC_STATE_LOGIC_OP_EXT` set in
///[`PipelineDynamicStateCreateInfo::dynamic_states`].
///Otherwise, this state is specified by the
///[`PipelineColorBlendStateCreateInfo`]::[`logic_op`] value used to
///create the currently active pipeline.
///## Valid Usage
/// - The [extendedDynamicState2LogicOp](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-extendedDynamicState2LogicOp)
///   feature  **must**  be enabled
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`logic_op`] **must**  be a valid [`LogicOp`] value
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_EXT_extended_dynamic_state2`]
/// - [`CommandBuffer`]
/// - [`LogicOp`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdSetLogicOpEXT")]
pub type FNCmdSetLogicOpExt = Option<unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op: LogicOp)>;
///[VkPhysicalDeviceExtendedDynamicState2FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html) - Structure describing what extended dynamic state can be used
///# C Specifications
///The [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_extended_dynamic_state2
///typedef struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           extendedDynamicState2;
///    VkBool32           extendedDynamicState2LogicOp;
///    VkBool32           extendedDynamicState2PatchControlPoints;
///} VkPhysicalDeviceExtendedDynamicState2FeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`extended_dynamic_state_2`] indicates that the implementation supports the following dynamic
///   states:  - `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE`  - `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE`
///   - `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE`
/// - [`extended_dynamic_state_2_logic_op`] indicates that the implementation supports the following
///   dynamic state:  - `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
/// - [`extended_dynamic_state_2_patch_control_points`] indicates that the implementation supports
///   the following dynamic state:  - `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT`
///If the [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceExtendedDynamicState2FeaturesEXT`] **can**  also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT`
///# Related
/// - [`VK_EXT_extended_dynamic_state2`]
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
#[doc(alias = "VkPhysicalDeviceExtendedDynamicState2FeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`extended_dynamic_state_2`] indicates
    ///that the implementation supports the following dynamic states:
    /// - `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE`
    /// - `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE`
    /// - `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE`
    pub extended_dynamic_state_2: Bool32,
    ///[`extended_dynamic_state_2_logic_op`] indicates that the implementation
    ///supports the following dynamic state:
    /// - `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
    pub extended_dynamic_state_2_logic_op: Bool32,
    ///[`extended_dynamic_state_2_patch_control_points`] indicates that the
    ///implementation supports the following dynamic state:
    /// - `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT`
    pub extended_dynamic_state_2_patch_control_points: Bool32,
}
impl<'lt> Default for PhysicalDeviceExtendedDynamicState2FeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE2_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            extended_dynamic_state_2: 0,
            extended_dynamic_state_2_logic_op: 0,
            extended_dynamic_state_2_patch_control_points: 0,
        }
    }
}
impl<'lt> PhysicalDeviceExtendedDynamicState2FeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::extended_dynamic_state_2`]
    pub fn extended_dynamic_state_2_raw(&self) -> Bool32 {
        self.extended_dynamic_state_2
    }
    ///Gets the raw value of [`Self::extended_dynamic_state_2_logic_op`]
    pub fn extended_dynamic_state_2_logic_op_raw(&self) -> Bool32 {
        self.extended_dynamic_state_2_logic_op
    }
    ///Gets the raw value of [`Self::extended_dynamic_state_2_patch_control_points`]
    pub fn extended_dynamic_state_2_patch_control_points_raw(&self) -> Bool32 {
        self.extended_dynamic_state_2_patch_control_points
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::extended_dynamic_state_2`]
    pub fn set_extended_dynamic_state_2_raw(mut self, value: Bool32) -> Self {
        self.extended_dynamic_state_2 = value;
        self
    }
    ///Sets the raw value of [`Self::extended_dynamic_state_2_logic_op`]
    pub fn set_extended_dynamic_state_2_logic_op_raw(mut self, value: Bool32) -> Self {
        self.extended_dynamic_state_2_logic_op = value;
        self
    }
    ///Sets the raw value of [`Self::extended_dynamic_state_2_patch_control_points`]
    pub fn set_extended_dynamic_state_2_patch_control_points_raw(mut self, value: Bool32) -> Self {
        self.extended_dynamic_state_2_patch_control_points = value;
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
    ///Gets the value of [`Self::extended_dynamic_state_2`]
    pub fn extended_dynamic_state_2(&self) -> bool {
        unsafe { std::mem::transmute(self.extended_dynamic_state_2 as u8) }
    }
    ///Gets the value of [`Self::extended_dynamic_state_2_logic_op`]
    pub fn extended_dynamic_state_2_logic_op(&self) -> bool {
        unsafe { std::mem::transmute(self.extended_dynamic_state_2_logic_op as u8) }
    }
    ///Gets the value of [`Self::extended_dynamic_state_2_patch_control_points`]
    pub fn extended_dynamic_state_2_patch_control_points(&self) -> bool {
        unsafe { std::mem::transmute(self.extended_dynamic_state_2_patch_control_points as u8) }
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
    ///Gets a mutable reference to the value of [`Self::extended_dynamic_state_2`]
    pub fn extended_dynamic_state_2_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.extended_dynamic_state_2 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.extended_dynamic_state_2 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::extended_dynamic_state_2_logic_op`]
    pub fn extended_dynamic_state_2_logic_op_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.extended_dynamic_state_2_logic_op as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.extended_dynamic_state_2_logic_op as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::extended_dynamic_state_2_patch_control_points`]
    pub fn extended_dynamic_state_2_patch_control_points_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.extended_dynamic_state_2_patch_control_points as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.extended_dynamic_state_2_patch_control_points as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the value of [`Self::extended_dynamic_state_2`]
    pub fn set_extended_dynamic_state_2(mut self, value: bool) -> Self {
        self.extended_dynamic_state_2 = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::extended_dynamic_state_2_logic_op`]
    pub fn set_extended_dynamic_state_2_logic_op(mut self, value: bool) -> Self {
        self.extended_dynamic_state_2_logic_op = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::extended_dynamic_state_2_patch_control_points`]
    pub fn set_extended_dynamic_state_2_patch_control_points(mut self, value: bool) -> Self {
        self.extended_dynamic_state_2_patch_control_points = value as u8 as u32;
        self
    }
}
impl CommandBuffer {
    ///[vkCmdSetPatchControlPointsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html) - Specify the number of control points per patch dynamically for a command buffer
    ///# C Specifications
    ///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the number of control points
    ///per patch, call:
    ///```c
    ///// Provided by VK_EXT_extended_dynamic_state2
    ///void vkCmdSetPatchControlPointsEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    uint32_t                                    patchControlPoints);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`patch_control_points`] specifies the number of control points per patch.
    ///# Description
    ///This command sets the number of control points per patch for subsequent
    ///drawing commands when the graphics pipeline is created with
    ///`VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT` set in
    ///[`PipelineDynamicStateCreateInfo::dynamic_states`].
    ///Otherwise, this state is specified by the
    ///[`PipelineTessellationStateCreateInfo`]::[`patch_control_points`] value
    ///used to create the currently active pipeline.
    ///## Valid Usage
    /// - The [extendedDynamicState2PatchControlPoints](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-extendedDynamicState2PatchControlPoints)
    ///   feature  **must**  be enabled
    /// - [`patch_control_points`] **must**  be greater than zero and less than or equal to
    ///   [`PhysicalDeviceLimits::max_tessellation_patch_size`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_EXT_extended_dynamic_state2`]
    /// - [`CommandBuffer`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdSetPatchControlPointsEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_set_patch_control_points_ext<'a: 'this, 'this>(
        self: &'this mut Unique<'a, CommandBuffer>,
        patch_control_points: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_extended_dynamic_state_2()
            .expect("extension/version not loaded")
            .cmd_set_patch_control_points_ext()
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_extended_dynamic_state_2()
            .unwrap_unchecked()
            .cmd_set_patch_control_points_ext()
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), patch_control_points.unwrap_or_default() as _);
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdSetLogicOpEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html) - Select which logical operation to apply for blend state dynamically for a command buffer
    ///# C Specifications
    ///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the logical operation to
    ///apply for blend state, call:
    ///```c
    ///// Provided by VK_EXT_extended_dynamic_state2
    ///void vkCmdSetLogicOpEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkLogicOp                                   logicOp);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`logic_op`] specifies the logical operation to apply for blend state.
    ///# Description
    ///This command sets the logical operation for blend state for subsequent
    ///drawing commands when the graphics pipeline is created with
    ///`VK_DYNAMIC_STATE_LOGIC_OP_EXT` set in
    ///[`PipelineDynamicStateCreateInfo::dynamic_states`].
    ///Otherwise, this state is specified by the
    ///[`PipelineColorBlendStateCreateInfo`]::[`logic_op`] value used to
    ///create the currently active pipeline.
    ///## Valid Usage
    /// - The [extendedDynamicState2LogicOp](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-extendedDynamicState2LogicOp)
    ///   feature  **must**  be enabled
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`logic_op`] **must**  be a valid [`LogicOp`] value
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_EXT_extended_dynamic_state2`]
    /// - [`CommandBuffer`]
    /// - [`LogicOp`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdSetLogicOpEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_set_logic_op_ext<'a: 'this, 'this>(
        self: &'this mut Unique<'a, CommandBuffer>,
        logic_op: LogicOp,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_extended_dynamic_state_2()
            .expect("extension/version not loaded")
            .cmd_set_logic_op_ext()
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_extended_dynamic_state_2()
            .unwrap_unchecked()
            .cmd_set_logic_op_ext()
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), logic_op);
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_extended_dynamic_state2`
pub struct DeviceExtExtendedDynamicState2VTable {
    ///See [`FNCmdSetPatchControlPointsExt`] for more information.
    pub cmd_set_patch_control_points_ext: FNCmdSetPatchControlPointsExt,
    ///See [`FNCmdSetLogicOpExt`] for more information.
    pub cmd_set_logic_op_ext: FNCmdSetLogicOpExt,
    ///See [`FNCmdSetRasterizerDiscardEnable`] for more information.
    pub cmd_set_rasterizer_discard_enable: FNCmdSetRasterizerDiscardEnable,
    ///See [`FNCmdSetDepthBiasEnable`] for more information.
    pub cmd_set_depth_bias_enable: FNCmdSetDepthBiasEnable,
    ///See [`FNCmdSetPrimitiveRestartEnable`] for more information.
    pub cmd_set_primitive_restart_enable: FNCmdSetPrimitiveRestartEnable,
}
impl DeviceExtExtendedDynamicState2VTable {
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
            cmd_set_patch_control_points_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdSetPatchControlPointsEXT").as_ptr(),
                ))
            },
            cmd_set_logic_op_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetLogicOpEXT").as_ptr()))
            },
            cmd_set_rasterizer_discard_enable: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdSetRasterizerDiscardEnableEXT").as_ptr(),
                ))
            },
            cmd_set_depth_bias_enable: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetDepthBiasEnableEXT").as_ptr()))
            },
            cmd_set_primitive_restart_enable: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdSetPrimitiveRestartEnableEXT").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::cmd_set_patch_control_points_ext`]. See [`FNCmdSetPatchControlPointsExt`] for
    /// more information.
    pub fn cmd_set_patch_control_points_ext(&self) -> FNCmdSetPatchControlPointsExt {
        self.cmd_set_patch_control_points_ext
    }
    ///Gets [`Self::cmd_set_logic_op_ext`]. See [`FNCmdSetLogicOpExt`] for more information.
    pub fn cmd_set_logic_op_ext(&self) -> FNCmdSetLogicOpExt {
        self.cmd_set_logic_op_ext
    }
    ///Gets [`Self::cmd_set_rasterizer_discard_enable`]. See [`FNCmdSetRasterizerDiscardEnable`]
    /// for more information.
    pub fn cmd_set_rasterizer_discard_enable(&self) -> FNCmdSetRasterizerDiscardEnable {
        self.cmd_set_rasterizer_discard_enable
    }
    ///Gets [`Self::cmd_set_depth_bias_enable`]. See [`FNCmdSetDepthBiasEnable`] for more
    /// information.
    pub fn cmd_set_depth_bias_enable(&self) -> FNCmdSetDepthBiasEnable {
        self.cmd_set_depth_bias_enable
    }
    ///Gets [`Self::cmd_set_primitive_restart_enable`]. See [`FNCmdSetPrimitiveRestartEnable`] for
    /// more information.
    pub fn cmd_set_primitive_restart_enable(&self) -> FNCmdSetPrimitiveRestartEnable {
        self.cmd_set_primitive_restart_enable
    }
}
