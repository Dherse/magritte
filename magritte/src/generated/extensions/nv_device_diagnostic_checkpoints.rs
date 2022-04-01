//![VK_NV_device_diagnostic_checkpoints](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_device_diagnostic_checkpoints.html) - device extension
//!# Description
//!This extension allows applications to insert markers in the command stream
//!and associate them with custom data.If a device lost error occurs, the application  **may**
//! then query the
//!implementation for the last markers to cross specific implementation-defined
//!pipeline stages, in order to narrow down which commands were executing at
//!the time and might have caused the failure.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Nuno Subtil [nsubtil](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_device_diagnostic_checkpoints]
//!   @nsubtil%0A<<Here describe the issue or question you have about the
//!   VK_NV_device_diagnostic_checkpoints extension>>)
//!# New functions & commands
//! - [`CmdSetCheckpointNV`]
//! - [`GetQueueCheckpointDataNV`]
//!# New structures
//! - [`CheckpointDataNV`]
//! - Extending [`QueueFamilyProperties2`]:  - [`QueueFamilyCheckpointPropertiesNV`]
//!# New constants
//! - [`NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME`]
//! - [`NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV`  -
//!   `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV`
//!# Version History
//! - Revision 1, 2018-07-16 (Nuno Subtil)  - Internal revisions
//! - Revision 2, 2018-07-16 (Nuno Subtil)  - ???
//!# Other info
//! * 2018-07-16
//! * - Oleg Kuznetsov, NVIDIA  - Alex Dunn, NVIDIA  - Jeff Bolz, NVIDIA  - Eric Werness, NVIDIA  -
//!   Daniel Koch, NVIDIA
//!# Related
//! - [`CheckpointDataNV`]
//! - [`QueueFamilyCheckpointPropertiesNV`]
//! - [`CmdSetCheckpointNV`]
//! - [`GetQueueCheckpointDataNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, PipelineStageFlagBits, PipelineStageFlags, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION")]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME")]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_device_diagnostic_checkpoints");
///[VkQueueFamilyCheckpointPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyCheckpointPropertiesNV.html) - Return structure for queue family checkpoint information query
///# C Specifications
///The [`QueueFamilyCheckpointPropertiesNV`] structure is defined as:
///```c
///// Provided by VK_NV_device_diagnostic_checkpoints
///typedef struct VkQueueFamilyCheckpointPropertiesNV {
///    VkStructureType         sType;
///    void*                   pNext;
///    VkPipelineStageFlags    checkpointExecutionStageMask;
///} VkQueueFamilyCheckpointPropertiesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`checkpoint_execution_stage_mask`] is a mask indicating which pipeline stages the
///   implementation can execute checkpoint markers in.
///# Description
///Additional queue family information can be queried by setting
///[`QueueFamilyProperties2`]::[`p_next`] to point to a
///[`QueueFamilyCheckpointPropertiesNV`] structure.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV`
///# Related
/// - [`VK_NV_device_diagnostic_checkpoints`]
/// - [`PipelineStageFlags`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueueFamilyCheckpointPropertiesNV")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct QueueFamilyCheckpointPropertiesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`checkpoint_execution_stage_mask`] is a mask indicating which pipeline
    ///stages the implementation can execute checkpoint markers in.
    pub checkpoint_execution_stage_mask: PipelineStageFlags,
}
impl<'lt> Default for QueueFamilyCheckpointPropertiesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::QueueFamilyCheckpointPropertiesNv,
            p_next: std::ptr::null_mut(),
            checkpoint_execution_stage_mask: Default::default(),
        }
    }
}
impl<'lt> QueueFamilyCheckpointPropertiesNV<'lt> {
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
    ///Gets the value of [`Self::checkpoint_execution_stage_mask`]
    pub fn checkpoint_execution_stage_mask(&self) -> PipelineStageFlags {
        self.checkpoint_execution_stage_mask
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
    ///Gets a mutable reference to the value of [`Self::checkpoint_execution_stage_mask`]
    pub fn checkpoint_execution_stage_mask_mut(&mut self) -> &mut PipelineStageFlags {
        &mut self.checkpoint_execution_stage_mask
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
    ///Sets the raw value of [`Self::checkpoint_execution_stage_mask`]
    pub fn set_checkpoint_execution_stage_mask(&mut self, value: crate::vulkan1_0::PipelineStageFlags) -> &mut Self {
        self.checkpoint_execution_stage_mask = value;
        self
    }
}
///[VkCheckpointDataNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCheckpointDataNV.html) - Return structure for command buffer checkpoint data
///# C Specifications
///The [`CheckpointDataNV`] structure is defined as:
///```c
///// Provided by VK_NV_device_diagnostic_checkpoints
///typedef struct VkCheckpointDataNV {
///    VkStructureType            sType;
///    void*                      pNext;
///    VkPipelineStageFlagBits    stage;
///    void*                      pCheckpointMarker;
///} VkCheckpointDataNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`stage`] is a [`PipelineStageFlagBits`] value specifying which pipeline stage the checkpoint
///   marker data refers to.
/// - [`checkpoint_marker`] contains the value of the last checkpoint marker executed in the stage
///   that [`stage`] refers to.
///# Description
///The stages at which a checkpoint marker  **can**  be executed are
///implementation-defined and  **can**  be queried by calling
///[`GetPhysicalDeviceQueueFamilyProperties2`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_NV_device_diagnostic_checkpoints`]
/// - [`PipelineStageFlagBits`]
/// - [`StructureType`]
/// - [`GetQueueCheckpointDataNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCheckpointDataNV")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CheckpointDataNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`stage`] is a [`PipelineStageFlagBits`] value specifying which
    ///pipeline stage the checkpoint marker data refers to.
    pub stage: PipelineStageFlagBits,
    ///[`checkpoint_marker`] contains the value of the last checkpoint marker
    ///executed in the stage that [`stage`] refers to.
    pub checkpoint_marker: *mut c_void,
}
impl<'lt> Default for CheckpointDataNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::CheckpointDataNv,
            p_next: std::ptr::null_mut(),
            stage: Default::default(),
            checkpoint_marker: std::ptr::null_mut(),
        }
    }
}
impl<'lt> CheckpointDataNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::checkpoint_marker`]
    pub fn checkpoint_marker_raw(&self) -> &*mut c_void {
        &self.checkpoint_marker
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::checkpoint_marker`]
    pub fn set_checkpoint_marker_raw(&mut self, value: *mut c_void) -> &mut Self {
        self.checkpoint_marker = value;
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
    ///Gets the value of [`Self::stage`]
    pub fn stage(&self) -> PipelineStageFlagBits {
        self.stage
    }
    ///Gets the value of [`Self::checkpoint_marker`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn checkpoint_marker(&self) -> &c_void {
        &*self.checkpoint_marker
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
    ///Gets a mutable reference to the value of [`Self::stage`]
    pub fn stage_mut(&mut self) -> &mut PipelineStageFlagBits {
        &mut self.stage
    }
    ///Gets a mutable reference to the value of [`Self::checkpoint_marker`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn checkpoint_marker_mut(&mut self) -> &mut c_void {
        &mut *self.checkpoint_marker
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
    ///Sets the raw value of [`Self::stage`]
    pub fn set_stage(&mut self, value: crate::vulkan1_0::PipelineStageFlagBits) -> &mut Self {
        self.stage = value;
        self
    }
    ///Sets the raw value of [`Self::checkpoint_marker`]
    pub fn set_checkpoint_marker(&mut self, value: &'lt mut std::ffi::c_void) -> &mut Self {
        self.checkpoint_marker = value as *mut _;
        self
    }
}
