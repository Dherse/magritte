//![VK_EXT_extended_dynamic_state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state.html) - device extension
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
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_extended_dynamic_state]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_extended_dynamic_state extension>>)
//!# New functions & commands
//! - [`CmdBindVertexBuffers2EXT`]
//! - [`CmdSetCullModeEXT`]
//! - [`CmdSetDepthBoundsTestEnableEXT`]
//! - [`CmdSetDepthCompareOpEXT`]
//! - [`CmdSetDepthTestEnableEXT`]
//! - [`CmdSetDepthWriteEnableEXT`]
//! - [`CmdSetFrontFaceEXT`]
//! - [`CmdSetPrimitiveTopologyEXT`]
//! - [`CmdSetScissorWithCountEXT`]
//! - [`CmdSetStencilOpEXT`]
//! - [`CmdSetStencilTestEnableEXT`]
//! - [`CmdSetViewportWithCountEXT`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceExtendedDynamicStateFeaturesEXT`]
//!# New constants
//! - [`EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME`]
//! - [`EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION`]
//! - Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_CULL_MODE_EXT`  -
//!   `VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE_EXT`  - `VK_DYNAMIC_STATE_DEPTH_COMPARE_OP_EXT`  -
//!   `VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE_EXT`  - `VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE_EXT`  -
//!   `VK_DYNAMIC_STATE_FRONT_FACE_EXT`  - `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT`  -
//!   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT_EXT`  - `VK_DYNAMIC_STATE_STENCIL_OP_EXT`  -
//!   `VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE_EXT`  -
//!   `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT`  -
//!   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2019-12-09 (Piers Daniell)  - Internal revisions
//!# Other info
//! * 2019-12-09
//! * - Promoted to Vulkan 1.3 Core
//! * No known IP claims.
//! * - Dan Ginsburg, Valve Corporation  - Graeme Leese, Broadcom  - Hans-Kristian Arntzen, Valve
//!   Corporation  - Jan-Harald Fredriksen, Arm Limited  - Jason Ekstrand, Intel  - Jeff Bolz,
//!   NVIDIA  - Jesse Hall, Google  - Philip Rebohle, Valve Corporation  - Stuart Smith, Imagination
//!   Technologies  - Tobias Hector, AMD
//!# Related
//! - [`PhysicalDeviceExtendedDynamicStateFeaturesEXT`]
//! - [`CmdBindVertexBuffers2EXT`]
//! - [`CmdSetCullModeEXT`]
//! - [`CmdSetDepthBoundsTestEnableEXT`]
//! - [`CmdSetDepthCompareOpEXT`]
//! - [`CmdSetDepthTestEnableEXT`]
//! - [`CmdSetDepthWriteEnableEXT`]
//! - [`CmdSetFrontFaceEXT`]
//! - [`CmdSetPrimitiveTopologyEXT`]
//! - [`CmdSetScissorWithCountEXT`]
//! - [`CmdSetStencilOpEXT`]
//! - [`CmdSetStencilTestEnableEXT`]
//! - [`CmdSetViewportWithCountEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseOutStructure, Bool32, Device, StructureType},
    vulkan1_3::{
        FNCmdBindVertexBuffers2, FNCmdSetCullMode, FNCmdSetDepthBoundsTestEnable, FNCmdSetDepthCompareOp,
        FNCmdSetDepthTestEnable, FNCmdSetDepthWriteEnable, FNCmdSetFrontFace, FNCmdSetPrimitiveTopology,
        FNCmdSetScissorWithCount, FNCmdSetStencilOp, FNCmdSetStencilTestEnable, FNCmdSetViewportWithCount,
    },
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION")]
pub const EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME")]
pub const EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_extended_dynamic_state");
///[VkPhysicalDeviceExtendedDynamicStateFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicStateFeaturesEXT.html) - Structure describing what extended dynamic state can be used
///# C Specifications
///The [`PhysicalDeviceExtendedDynamicStateFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_extended_dynamic_state
///typedef struct VkPhysicalDeviceExtendedDynamicStateFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           extendedDynamicState;
///} VkPhysicalDeviceExtendedDynamicStateFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`extended_dynamic_state`] indicates that the implementation supports the following dynamic
///   states:  - `VK_DYNAMIC_STATE_CULL_MODE`  - `VK_DYNAMIC_STATE_FRONT_FACE`  -
///   `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY`  - `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`  -
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT`  - `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE`  -
///   `VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE`  - `VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE`  -
///   `VK_DYNAMIC_STATE_DEPTH_COMPARE_OP`  - `VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE`  -
///   `VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE`  - `VK_DYNAMIC_STATE_STENCIL_OP`
///If the [`PhysicalDeviceExtendedDynamicStateFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceExtendedDynamicStateFeaturesEXT`] **can**  also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT`
///# Related
/// - [`VK_EXT_extended_dynamic_state`]
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
#[doc(alias = "VkPhysicalDeviceExtendedDynamicStateFeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`extended_dynamic_state`] indicates
    ///that the implementation supports the following dynamic states:
    /// - `VK_DYNAMIC_STATE_CULL_MODE`
    /// - `VK_DYNAMIC_STATE_FRONT_FACE`
    /// - `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY`
    /// - `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`
    /// - `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT`
    /// - `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE`
    /// - `VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE`
    /// - `VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE`
    /// - `VK_DYNAMIC_STATE_DEPTH_COMPARE_OP`
    /// - `VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE`
    /// - `VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE`
    /// - `VK_DYNAMIC_STATE_STENCIL_OP`
    pub extended_dynamic_state: Bool32,
}
impl<'lt> Default for PhysicalDeviceExtendedDynamicStateFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            extended_dynamic_state: 0,
        }
    }
}
impl<'lt> PhysicalDeviceExtendedDynamicStateFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::extended_dynamic_state`]
    pub fn extended_dynamic_state_raw(&self) -> Bool32 {
        self.extended_dynamic_state
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::extended_dynamic_state`]
    pub fn set_extended_dynamic_state_raw(mut self, value: Bool32) -> Self {
        self.extended_dynamic_state = value;
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
    ///Gets the value of [`Self::extended_dynamic_state`]
    pub fn extended_dynamic_state(&self) -> bool {
        unsafe { std::mem::transmute(self.extended_dynamic_state as u8) }
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
    ///Gets a mutable reference to the value of [`Self::extended_dynamic_state`]
    pub fn extended_dynamic_state_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.extended_dynamic_state as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.extended_dynamic_state as *mut Bool32)
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
    ///Sets the value of [`Self::extended_dynamic_state`]
    pub fn set_extended_dynamic_state(mut self, value: bool) -> Self {
        self.extended_dynamic_state = value as u8 as u32;
        self
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_extended_dynamic_state`
pub struct DeviceExtExtendedDynamicStateVTable {
    ///See [`FNCmdSetCullMode`] for more information.
    pub cmd_set_cull_mode: FNCmdSetCullMode,
    ///See [`FNCmdSetFrontFace`] for more information.
    pub cmd_set_front_face: FNCmdSetFrontFace,
    ///See [`FNCmdSetPrimitiveTopology`] for more information.
    pub cmd_set_primitive_topology: FNCmdSetPrimitiveTopology,
    ///See [`FNCmdSetViewportWithCount`] for more information.
    pub cmd_set_viewport_with_count: FNCmdSetViewportWithCount,
    ///See [`FNCmdSetScissorWithCount`] for more information.
    pub cmd_set_scissor_with_count: FNCmdSetScissorWithCount,
    ///See [`FNCmdBindVertexBuffers2`] for more information.
    pub cmd_bind_vertex_buffers2: FNCmdBindVertexBuffers2,
    ///See [`FNCmdSetDepthTestEnable`] for more information.
    pub cmd_set_depth_test_enable: FNCmdSetDepthTestEnable,
    ///See [`FNCmdSetDepthWriteEnable`] for more information.
    pub cmd_set_depth_write_enable: FNCmdSetDepthWriteEnable,
    ///See [`FNCmdSetDepthCompareOp`] for more information.
    pub cmd_set_depth_compare_op: FNCmdSetDepthCompareOp,
    ///See [`FNCmdSetDepthBoundsTestEnable`] for more information.
    pub cmd_set_depth_bounds_test_enable: FNCmdSetDepthBoundsTestEnable,
    ///See [`FNCmdSetStencilTestEnable`] for more information.
    pub cmd_set_stencil_test_enable: FNCmdSetStencilTestEnable,
    ///See [`FNCmdSetStencilOp`] for more information.
    pub cmd_set_stencil_op: FNCmdSetStencilOp,
}
impl DeviceExtExtendedDynamicStateVTable {
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
            cmd_set_cull_mode: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetCullModeEXT").as_ptr()))
            },
            cmd_set_front_face: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetFrontFaceEXT").as_ptr()))
            },
            cmd_set_primitive_topology: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetPrimitiveTopologyEXT").as_ptr()))
            },
            cmd_set_viewport_with_count: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetViewportWithCountEXT").as_ptr()))
            },
            cmd_set_scissor_with_count: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetScissorWithCountEXT").as_ptr()))
            },
            cmd_bind_vertex_buffers2: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdBindVertexBuffers2EXT").as_ptr()))
            },
            cmd_set_depth_test_enable: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetDepthTestEnableEXT").as_ptr()))
            },
            cmd_set_depth_write_enable: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetDepthWriteEnableEXT").as_ptr()))
            },
            cmd_set_depth_compare_op: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetDepthCompareOpEXT").as_ptr()))
            },
            cmd_set_depth_bounds_test_enable: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdSetDepthBoundsTestEnableEXT").as_ptr(),
                ))
            },
            cmd_set_stencil_test_enable: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetStencilTestEnableEXT").as_ptr()))
            },
            cmd_set_stencil_op: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetStencilOpEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::cmd_set_cull_mode`]. See [`FNCmdSetCullMode`] for more information.
    pub fn cmd_set_cull_mode(&self) -> FNCmdSetCullMode {
        self.cmd_set_cull_mode
    }
    ///Gets [`Self::cmd_set_front_face`]. See [`FNCmdSetFrontFace`] for more information.
    pub fn cmd_set_front_face(&self) -> FNCmdSetFrontFace {
        self.cmd_set_front_face
    }
    ///Gets [`Self::cmd_set_primitive_topology`]. See [`FNCmdSetPrimitiveTopology`] for more
    /// information.
    pub fn cmd_set_primitive_topology(&self) -> FNCmdSetPrimitiveTopology {
        self.cmd_set_primitive_topology
    }
    ///Gets [`Self::cmd_set_viewport_with_count`]. See [`FNCmdSetViewportWithCount`] for more
    /// information.
    pub fn cmd_set_viewport_with_count(&self) -> FNCmdSetViewportWithCount {
        self.cmd_set_viewport_with_count
    }
    ///Gets [`Self::cmd_set_scissor_with_count`]. See [`FNCmdSetScissorWithCount`] for more
    /// information.
    pub fn cmd_set_scissor_with_count(&self) -> FNCmdSetScissorWithCount {
        self.cmd_set_scissor_with_count
    }
    ///Gets [`Self::cmd_bind_vertex_buffers2`]. See [`FNCmdBindVertexBuffers2`] for more
    /// information.
    pub fn cmd_bind_vertex_buffers2(&self) -> FNCmdBindVertexBuffers2 {
        self.cmd_bind_vertex_buffers2
    }
    ///Gets [`Self::cmd_set_depth_test_enable`]. See [`FNCmdSetDepthTestEnable`] for more
    /// information.
    pub fn cmd_set_depth_test_enable(&self) -> FNCmdSetDepthTestEnable {
        self.cmd_set_depth_test_enable
    }
    ///Gets [`Self::cmd_set_depth_write_enable`]. See [`FNCmdSetDepthWriteEnable`] for more
    /// information.
    pub fn cmd_set_depth_write_enable(&self) -> FNCmdSetDepthWriteEnable {
        self.cmd_set_depth_write_enable
    }
    ///Gets [`Self::cmd_set_depth_compare_op`]. See [`FNCmdSetDepthCompareOp`] for more
    /// information.
    pub fn cmd_set_depth_compare_op(&self) -> FNCmdSetDepthCompareOp {
        self.cmd_set_depth_compare_op
    }
    ///Gets [`Self::cmd_set_depth_bounds_test_enable`]. See [`FNCmdSetDepthBoundsTestEnable`] for
    /// more information.
    pub fn cmd_set_depth_bounds_test_enable(&self) -> FNCmdSetDepthBoundsTestEnable {
        self.cmd_set_depth_bounds_test_enable
    }
    ///Gets [`Self::cmd_set_stencil_test_enable`]. See [`FNCmdSetStencilTestEnable`] for more
    /// information.
    pub fn cmd_set_stencil_test_enable(&self) -> FNCmdSetStencilTestEnable {
        self.cmd_set_stencil_test_enable
    }
    ///Gets [`Self::cmd_set_stencil_op`]. See [`FNCmdSetStencilOp`] for more information.
    pub fn cmd_set_stencil_op(&self) -> FNCmdSetStencilOp {
        self.cmd_set_stencil_op
    }
}
