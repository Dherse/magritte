//![VK_NV_device_diagnostic_checkpoints](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_device_diagnostic_checkpoints.html) - device extension
//!# Description
//!This extension allows applications to insert markers in the command stream
//!and associate them with custom data.If a device lost error occurs, the application  **may**
//! then query the
//!implementation for the last markers to cross specific implementation-defined
//!pipeline stages, in order to narrow down which commands were executing at
//!the time and might have caused the failure.
# ! [doc = concat ! ("# " , "Revision")]
//!2
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
# ! [doc = concat ! ("# " , "Contacts")]
//! - Nuno Subtil [nsubtil](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_device_diagnostic_checkpoints]
//!   @nsubtil%0A<<Here describe the issue or question you have about the
//!   VK_NV_device_diagnostic_checkpoints extension>>)
# ! [doc = concat ! ("# " , "New commands")]
//! - [`cmd_set_checkpoint_nv`]
//! - [`get_queue_checkpoint_data_nv`]
# ! [doc = concat ! ("# " , "New structures")]
//! - [`CheckpointDataNV`]
//! - Extending [`QueueFamilyProperties2`]:  - [`QueueFamilyCheckpointPropertiesNV`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME`]
//! - [`NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV`  -
//!   `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV`
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2018-07-16 (Nuno Subtil)  - Internal revisions
//! - Revision 2, 2018-07-16 (Nuno Subtil)  - ???
//!# Other info
//! * 2018-07-16
//! * - Oleg Kuznetsov, NVIDIA  - Alex Dunn, NVIDIA  - Jeff Bolz, NVIDIA  - Eric Werness, NVIDIA  -
//!   Daniel Koch, NVIDIA
//!# Related
//! - [`CheckpointDataNV`]
//! - [`QueueFamilyCheckpointPropertiesNV`]
//! - [`cmd_set_checkpoint_nv`]
//! - [`get_queue_checkpoint_data_nv`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{
        BaseOutStructure, CommandBuffer, Device, PipelineStageFlagBits, PipelineStageFlags, Queue, StructureType,
    },
    AsRaw, SmallVec, Unique,
};
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
///[vkGetQueueCheckpointDataNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointDataNV.html) - Retrieve diagnostic checkpoint data
///# C Specifications
///If the device encounters an error during execution, the implementation will
///return a `VK_ERROR_DEVICE_LOST` error to the application at a certain
///point during host execution.
///When this happens, the application  **can**  call
///[`get_queue_checkpoint_data_nv`] to retrieve information on the most recent
///diagnostic checkpoints that were executed by the device.
///```c
///// Provided by VK_NV_device_diagnostic_checkpoints
///void vkGetQueueCheckpointDataNV(
///    VkQueue                                     queue,
///    uint32_t*                                   pCheckpointDataCount,
///    VkCheckpointDataNV*                         pCheckpointData);
///```
///# Parameters
/// - [`queue`] is the [`Queue`] object the caller would like to retrieve checkpoint data for
/// - [`p_checkpoint_data_count`] is a pointer to an integer related to the number of checkpoint
///   markers available or queried, as described below.
/// - [`p_checkpoint_data`] is either `NULL` or a pointer to an array of [`CheckpointDataNV`]
///   structures.
///# Description
///If [`p_checkpoint_data`] is `NULL`, then the number of checkpoint markers
///available is returned in [`p_checkpoint_data_count`].Otherwise, [`p_checkpoint_data_count`]
/// **must**  point to a variable set by the
///user to the number of elements in the [`p_checkpoint_data`] array, and on
///return the variable is overwritten with the number of structures actually
///written to [`p_checkpoint_data`].If [`p_checkpoint_data_count`] is less than the number of
/// checkpoint markers
///available, at most [`p_checkpoint_data_count`] structures will be written.
///## Valid Usage
/// - The device that [`queue`] belongs to  **must**  be in the lost state
///
///## Valid Usage (Implicit)
/// - [`queue`] **must**  be a valid [`Queue`] handle
/// - [`p_checkpoint_data_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_checkpoint_data_count`] is not `0`, and [`p_checkpoint_data`]
///   is not `NULL`, [`p_checkpoint_data`] **must**  be a valid pointer to an array of
///   [`p_checkpoint_data_count`][`CheckpointDataNV`] structures
///# Related
/// - [`nv_device_diagnostic_checkpoints`]
/// - [`CheckpointDataNV`]
/// - [`Queue`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetQueueCheckpointDataNV")]
pub type FNGetQueueCheckpointDataNv = Option<
    for<'lt> unsafe extern "system" fn(
        queue: Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut CheckpointDataNV<'lt>,
    ),
>;
///[vkCmdSetCheckpointNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCheckpointNV.html) - Insert diagnostic checkpoint in command stream
///# C Specifications
///Device diagnostic checkpoints are inserted into the command stream by
///calling [`cmd_set_checkpoint_nv`].
///```c
///// Provided by VK_NV_device_diagnostic_checkpoints
///void vkCmdSetCheckpointNV(
///    VkCommandBuffer                             commandBuffer,
///    const void*                                 pCheckpointMarker);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer that will receive the marker
/// - [`p_checkpoint_marker`] is an opaque application-provided value that will be associated with
///   the checkpoint.
///# Description
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics,
///   compute, or transfer operations
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`nv_device_diagnostic_checkpoints`]
/// - [`CommandBuffer`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdSetCheckpointNV")]
pub type FNCmdSetCheckpointNv =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, p_checkpoint_marker: *const c_void)>;
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
/// - [`nv_device_diagnostic_checkpoints`]
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
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
            s_type: StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            checkpoint_execution_stage_mask: Default::default(),
        }
    }
}
impl<'lt> QueueFamilyCheckpointPropertiesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::checkpoint_execution_stage_mask`]
    pub fn set_checkpoint_execution_stage_mask(&mut self, value: crate::vulkan1_0::PipelineStageFlags) -> &mut Self {
        self.checkpoint_execution_stage_mask = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::checkpoint_execution_stage_mask`]
    pub fn with_checkpoint_execution_stage_mask(mut self, value: crate::vulkan1_0::PipelineStageFlags) -> Self {
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
///[`get_physical_device_queue_family_properties2`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`nv_device_diagnostic_checkpoints`]
/// - [`PipelineStageFlagBits`]
/// - [`StructureType`]
/// - [`get_queue_checkpoint_data_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCheckpointDataNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
            s_type: StructureType::CHECKPOINT_DATA_NV,
            p_next: std::ptr::null_mut(),
            stage: Default::default(),
            checkpoint_marker: std::ptr::null_mut(),
        }
    }
}
impl<'lt> CheckpointDataNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::checkpoint_marker`]
    pub fn checkpoint_marker_raw(&self) -> *mut c_void {
        self.checkpoint_marker
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
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::checkpoint_marker`]
    pub fn with_checkpoint_marker_raw(mut self, value: *mut c_void) -> Self {
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::stage`]
    pub fn set_stage(&mut self, value: crate::vulkan1_0::PipelineStageFlagBits) -> &mut Self {
        self.stage = value;
        self
    }
    ///Sets the value of [`Self::checkpoint_marker`]
    pub fn set_checkpoint_marker(&mut self, value: &'lt mut std::ffi::c_void) -> &mut Self {
        self.checkpoint_marker = value as *mut _;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::stage`]
    pub fn with_stage(mut self, value: crate::vulkan1_0::PipelineStageFlagBits) -> Self {
        self.stage = value;
        self
    }
    ///Sets the value of [`Self::checkpoint_marker`]
    pub fn with_checkpoint_marker(mut self, value: &'lt mut std::ffi::c_void) -> Self {
        self.checkpoint_marker = value as *mut _;
        self
    }
}
impl Queue {
    ///[vkGetQueueCheckpointDataNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointDataNV.html) - Retrieve diagnostic checkpoint data
    ///# C Specifications
    ///If the device encounters an error during execution, the implementation will
    ///return a `VK_ERROR_DEVICE_LOST` error to the application at a certain
    ///point during host execution.
    ///When this happens, the application  **can**  call
    ///[`get_queue_checkpoint_data_nv`] to retrieve information on the most recent
    ///diagnostic checkpoints that were executed by the device.
    ///```c
    ///// Provided by VK_NV_device_diagnostic_checkpoints
    ///void vkGetQueueCheckpointDataNV(
    ///    VkQueue                                     queue,
    ///    uint32_t*                                   pCheckpointDataCount,
    ///    VkCheckpointDataNV*                         pCheckpointData);
    ///```
    ///# Parameters
    /// - [`queue`] is the [`Queue`] object the caller would like to retrieve checkpoint data for
    /// - [`p_checkpoint_data_count`] is a pointer to an integer related to the number of checkpoint
    ///   markers available or queried, as described below.
    /// - [`p_checkpoint_data`] is either `NULL` or a pointer to an array of [`CheckpointDataNV`]
    ///   structures.
    ///# Description
    ///If [`p_checkpoint_data`] is `NULL`, then the number of checkpoint markers
    ///available is returned in [`p_checkpoint_data_count`].Otherwise, [`p_checkpoint_data_count`]
    /// **must**  point to a variable set by the
    ///user to the number of elements in the [`p_checkpoint_data`] array, and on
    ///return the variable is overwritten with the number of structures actually
    ///written to [`p_checkpoint_data`].If [`p_checkpoint_data_count`] is less than the number of
    /// checkpoint markers
    ///available, at most [`p_checkpoint_data_count`] structures will be written.
    ///## Valid Usage
    /// - The device that [`queue`] belongs to  **must**  be in the lost state
    ///
    ///## Valid Usage (Implicit)
    /// - [`queue`] **must**  be a valid [`Queue`] handle
    /// - [`p_checkpoint_data_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_checkpoint_data_count`] is not `0`, and
    ///   [`p_checkpoint_data`] is not `NULL`, [`p_checkpoint_data`] **must**  be a valid pointer to
    ///   an array of [`p_checkpoint_data_count`][`CheckpointDataNV`] structures
    ///# Related
    /// - [`nv_device_diagnostic_checkpoints`]
    /// - [`CheckpointDataNV`]
    /// - [`Queue`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetQueueCheckpointDataNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_queue_checkpoint_data_nv<'lt>(
        self: &Unique<Queue>,
        p_checkpoint_data_count: Option<usize>,
    ) -> SmallVec<CheckpointDataNV<'lt>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_device_diagnostic_checkpoints()
            .and_then(|vtable| vtable.get_queue_checkpoint_data_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_device_diagnostic_checkpoints()
            .and_then(|vtable| vtable.get_queue_checkpoint_data_nv())
            .unwrap_unchecked();
        let mut p_checkpoint_data_count = match p_checkpoint_data_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_checkpoint_data =
            SmallVec::<CheckpointDataNV<'lt>>::from_elem(Default::default(), p_checkpoint_data_count as usize);
        let _return = _function(
            self.as_raw(),
            &mut p_checkpoint_data_count,
            p_checkpoint_data.as_mut_ptr(),
        );
        p_checkpoint_data
    }
}
impl CommandBuffer {
    ///[vkCmdSetCheckpointNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCheckpointNV.html) - Insert diagnostic checkpoint in command stream
    ///# C Specifications
    ///Device diagnostic checkpoints are inserted into the command stream by
    ///calling [`cmd_set_checkpoint_nv`].
    ///```c
    ///// Provided by VK_NV_device_diagnostic_checkpoints
    ///void vkCmdSetCheckpointNV(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const void*                                 pCheckpointMarker);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer that will receive the marker
    /// - [`p_checkpoint_marker`] is an opaque application-provided value that will be associated
    ///   with the checkpoint.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, compute, or transfer operations
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`nv_device_diagnostic_checkpoints`]
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
    #[doc(alias = "vkCmdSetCheckpointNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_set_checkpoint_nv(self: &Unique<CommandBuffer>, p_checkpoint_marker: *const c_void) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_device_diagnostic_checkpoints()
            .and_then(|vtable| vtable.cmd_set_checkpoint_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_device_diagnostic_checkpoints()
            .and_then(|vtable| vtable.cmd_set_checkpoint_nv())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_checkpoint_marker);
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_NV_device_diagnostic_checkpoints`
pub struct DeviceNvDeviceDiagnosticCheckpointsVTable {
    ///See [`FNGetQueueCheckpointDataNv`] for more information.
    pub get_queue_checkpoint_data_nv: FNGetQueueCheckpointDataNv,
    ///See [`FNCmdSetCheckpointNv`] for more information.
    pub cmd_set_checkpoint_nv: FNCmdSetCheckpointNv,
}
impl DeviceNvDeviceDiagnosticCheckpointsVTable {
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
            get_queue_checkpoint_data_nv: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetQueueCheckpointDataNV").as_ptr()))
            },
            cmd_set_checkpoint_nv: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetCheckpointNV").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_queue_checkpoint_data_nv`]. See [`FNGetQueueCheckpointDataNv`] for more
    /// information.
    pub fn get_queue_checkpoint_data_nv(&self) -> FNGetQueueCheckpointDataNv {
        self.get_queue_checkpoint_data_nv
    }
    ///Gets [`Self::cmd_set_checkpoint_nv`]. See [`FNCmdSetCheckpointNv`] for more information.
    pub fn cmd_set_checkpoint_nv(&self) -> FNCmdSetCheckpointNv {
        self.cmd_set_checkpoint_nv
    }
}
