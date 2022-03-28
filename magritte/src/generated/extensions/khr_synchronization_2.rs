//![VK_KHR_synchronization2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_synchronization2.html) - device extension
//!# Description
//!This extension modifies the original core synchronization APIs to simplify
//!the interface and improve usability of these APIs.
//!It also adds new pipeline stage and access flag types that extend into the
//!64-bit range, as we have run out within the 32-bit range.
//!The new flags are identical to the old values within the 32-bit range, with
//!new stages and bits beyond that.Pipeline stages and access flags are now specified together in
//! memory
//!barrier structures, making the connection between the two more obvious.
//!Additionally, scoping the pipeline stages into the barrier structs allows
//!the use of the `MEMORY_READ` and `MEMORY_WRITE` flags without
//!sacrificing precision.
//!The per-stage access flags should be used to disambiguate specific accesses
//!in a given stage or set of stages - for instance, between uniform reads and
//!sampling operations.Layout transitions have been simplified as well; rather than requiring a
//!different set of layouts for depth/stencil/color attachments, there are
//!generic `VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL_KHR` and
//!`VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR` layouts which are contextually
//!applied based on the image format.
//!For example, for a depth format image,
//!`VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR` is equivalent to
//!`VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL_KHR`.
//!`VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR` also functionally replaces
//!`VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`.Events are now more efficient, because they include
//! memory dependency
//!information when you set them on the device.
//!Previously, this information was only known when waiting on an event, so the
//!dependencies could not be satisfied until the wait occurred.
//!That sometimes meant stalling the pipeline when the wait occurred.
//!The new API provides enough information for implementations to satisfy these
//!dependencies in parallel with other tasks.Queue submission has been changed to wrap command
//! buffers and semaphores in
//!extensible structures, which incorporate changes from Vulkan 1.1,
//!`[`VK_KHR_device_group`]`, and `[`VK_KHR_timeline_semaphore`]`.
//!This also adds a pipeline stage to the semaphore signal operation, mirroring
//!the existing pipeline stage specification for wait operations.Other miscellaneous changes
//! include:
//! - Events can now be specified as interacting only with the device, allowing more efficient
//!   access to the underlying object.
//! - Image memory barriers that do not perform an image layout transition can be specified by
//!   setting `oldLayout` equal to `newLayout`.  - E.g. the old and new layout can both be set to
//!   `VK_IMAGE_LAYOUT_UNDEFINED`, without discarding data in the image.
//! - Queue family ownership transfer parameters are simplified in some cases.
//! - Where two synchronization commands need to be matched up (queue transfer operations, events),
//!   the dependency information specified in each place must now match completely for consistency.
//! - Extensions with commands or functions with a [`PipelineStageFlags`] or
//!   [`PipelineStageFlagBits`] parameter have had those APIs replaced with equivalents using
//!   [`PipelineStageFlags2KHR`].
//! - The new event and barrier interfaces are now more extensible for future changes.
//! - Relevant pipeline stage masks can now be specified as empty with the new
//!   `VK_PIPELINE_STAGE_NONE_KHR` and `VK_PIPELINE_STAGE_2_NONE_KHR` values.
//! - [`MemoryBarrier2KHR`] can be chained to [`SubpassDependency2`], overriding the original 32-bit
//!   stage and access masks.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_synchronization2]
//!   @tobski%0A<<Here describe the issue or question you have about the VK_KHR_synchronization2
//!   extension>>)
//!# New functions & commands
//! - [`CmdPipelineBarrier2KHR`]
//! - [`CmdResetEvent2KHR`]
//! - [`CmdSetEvent2KHR`]
//! - [`CmdWaitEvents2KHR`]
//! - [`CmdWriteTimestamp2KHR`]
//! - [`QueueSubmit2KHR`]
//!If [`VK_AMD_buffer_marker`] is supported:
//! - [`CmdWriteBufferMarker2AMD`]
//!If [`VK_NV_device_diagnostic_checkpoints`] is supported:
//! - [`GetQueueCheckpointData2NV`]
//!# New structures
//! - [`BufferMemoryBarrier2KHR`]
//! - [`CommandBufferSubmitInfoKHR`]
//! - [`DependencyInfoKHR`]
//! - [`ImageMemoryBarrier2KHR`]
//! - [`SemaphoreSubmitInfoKHR`]
//! - [`SubmitInfo2KHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceSynchronization2FeaturesKHR`]
//! - Extending [`SubpassDependency2`]:  - [`MemoryBarrier2KHR`]
//!If [`VK_NV_device_diagnostic_checkpoints`] is supported:
//! - [`CheckpointData2NV`]
//! - Extending [`QueueFamilyProperties2`]:  - [`QueueFamilyCheckpointProperties2NV`]
//!# New enums
//! - [`AccessFlagBits2KHR`]
//! - [`PipelineStageFlagBits2KHR`]
//! - [`SubmitFlagBitsKHR`]
//!# New bitmasks
//! - [`AccessFlags2KHR`]
//! - [`PipelineStageFlags2KHR`]
//! - [`SubmitFlagsKHR`]
//!# New constants
//! - [`KHR_SYNCHRONIZATION_2_EXTENSION_NAME`]
//! - [`KHR_SYNCHRONIZATION_2_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:  - `VK_ACCESS_NONE_KHR`
//! - Extending [`EventCreateFlagBits`]:  - `VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR`
//! - Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL_KHR`  -
//!   `VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR`
//! - Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_NONE_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO_KHR`  - `VK_STRUCTURE_TYPE_DEPENDENCY_INFO_KHR`
//!   - `VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2_KHR`  - `VK_STRUCTURE_TYPE_MEMORY_BARRIER_2_KHR`
//!   - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO_KHR`  - `VK_STRUCTURE_TYPE_SUBMIT_INFO_2_KHR`
//!If [`VK_EXT_blend_operation_advanced`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT`
//!If [`VK_EXT_conditional_rendering`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT`
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
//!If [`VK_EXT_fragment_density_map`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`
//! - Extending [`PipelineStageFlagBits2`]:  -
//!   `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
//!If [`VK_EXT_transform_feedback`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT`  -
//!   `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT`  -
//!   `VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT`
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
//!If [`VK_KHR_acceleration_structure`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`  -
//!   `VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`
//! - Extending [`PipelineStageFlagBits2`]:  -
//!   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`
//!If [`VK_KHR_fragment_shading_rate`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR`
//! - Extending [`PipelineStageFlagBits2`]:  -
//!   `VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
//!If [`VK_KHR_ray_tracing_pipeline`] is supported:
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR`
//!If [`VK_NV_device_diagnostic_checkpoints`] is supported:
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV`  -
//!   `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV`
//!If [`VK_NV_device_generated_commands`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV`  -
//!   `VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV`
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV`
//!If [`VK_NV_mesh_shader`] is supported:
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`  -
//!   `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
//!If [`VK_NV_ray_tracing`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV`  -
//!   `VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV`
//! - Extending [`PipelineStageFlagBits2`]:  -
//!   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV`  -
//!   `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV`
//!If [`VK_NV_shading_rate_image`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV`
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
//!# Version History
//! - Revision 1, 2020-12-03 (Tobias Hector)  - Internal revisions
//!# Other info
//! * 2020-12-03
//! * - Promoted to Vulkan 1.3 Core  - Interacts with `[`VK_KHR_create_renderpass2`]`
//! * - Tobias Hector
//!# Related
//! - [`AccessFlagBits2KHR`]
//! - [`AccessFlags2KHR`]
//! - [`BufferMemoryBarrier2KHR`]
//! - [`CommandBufferSubmitInfoKHR`]
//! - [`DependencyInfoKHR`]
//! - [`Flags64`]
//! - [`ImageMemoryBarrier2KHR`]
//! - [`MemoryBarrier2KHR`]
//! - [`PhysicalDeviceSynchronization2FeaturesKHR`]
//! - [`PipelineStageFlagBits2KHR`]
//! - [`PipelineStageFlags2KHR`]
//! - [`SemaphoreSubmitInfoKHR`]
//! - [`SubmitFlagBitsKHR`]
//! - [`SubmitFlagsKHR`]
//! - [`SubmitInfo2KHR`]
//! - [`CmdPipelineBarrier2KHR`]
//! - [`CmdResetEvent2KHR`]
//! - [`CmdSetEvent2KHR`]
//! - [`CmdWaitEvents2KHR`]
//! - [`CmdWriteTimestamp2KHR`]
//! - [`QueueSubmit2KHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseOutStructure, StructureType},
    vulkan1_3::PipelineStageFlags2,
};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION")]
pub const KHR_SYNCHRONIZATION_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME")]
pub const KHR_SYNCHRONIZATION_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_synchronization2");
///[VkFlags64](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFlags64.html) - Vulkan 64-bit bitmasks
///# C Specifications
///A collection of 64-bit flags is represented by a bitmask using the type
///[`Flags64`]:
///```c
///// Provided by VK_KHR_synchronization2
///typedef uint64_t VkFlags64;
///```
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`Flags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFlags64")]
pub type Flags64 = u64;
///[VkQueueFamilyCheckpointProperties2NV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyCheckpointProperties2NV.html) - Return structure for queue family checkpoint information query
///# C Specifications
///The [`QueueFamilyCheckpointProperties2NV`] structure is defined as:
///```c
///// Provided by VK_KHR_synchronization2 with VK_NV_device_diagnostic_checkpoints
///typedef struct VkQueueFamilyCheckpointProperties2NV {
///    VkStructureType          sType;
///    void*                    pNext;
///    VkPipelineStageFlags2    checkpointExecutionStageMask;
///} VkQueueFamilyCheckpointProperties2NV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`checkpoint_execution_stage_mask`] is a mask indicating which pipeline stages the
///   implementation can execute checkpoint markers in.
///# Description
///Additional queue family information can be queried by setting
///[`QueueFamilyProperties2`]::[`p_next`] to point to a
///[`QueueFamilyCheckpointProperties2NV`] structure.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV`
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`VK_NV_device_diagnostic_checkpoints`]
/// - [`PipelineStageFlags2`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueueFamilyCheckpointProperties2NV")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct QueueFamilyCheckpointProperties2NV<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`checkpoint_execution_stage_mask`] is a mask indicating which pipeline
    ///stages the implementation can execute checkpoint markers in.
    pub checkpoint_execution_stage_mask: PipelineStageFlags2,
}
impl<'lt> Default for QueueFamilyCheckpointProperties2NV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            checkpoint_execution_stage_mask: Default::default(),
        }
    }
}
impl<'lt> QueueFamilyCheckpointProperties2NV<'lt> {
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
    pub fn checkpoint_execution_stage_mask(&self) -> PipelineStageFlags2 {
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
    pub fn checkpoint_execution_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
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
    pub fn set_checkpoint_execution_stage_mask(&mut self, value: crate::vulkan1_3::PipelineStageFlags2) -> &mut Self {
        self.checkpoint_execution_stage_mask = value;
        self
    }
}
///[VkCheckpointData2NV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCheckpointData2NV.html) - Return structure for command buffer checkpoint data
///# C Specifications
///The [`CheckpointData2NV`] structure is defined as:
///```c
///// Provided by VK_KHR_synchronization2 with VK_NV_device_diagnostic_checkpoints
///typedef struct VkCheckpointData2NV {
///    VkStructureType          sType;
///    void*                    pNext;
///    VkPipelineStageFlags2    stage;
///    void*                    pCheckpointMarker;
///} VkCheckpointData2NV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`stage`] indicates a single pipeline stage which the checkpoint marker data refers to.
/// - [`checkpoint_marker`] contains the value of the last checkpoint marker executed in the stage
///   that [`stage`] refers to.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV`
/// - [`p_next`] **must**  be `NULL`
///The stages at which a checkpoint marker  **can**  be executed are
///implementation-defined and  **can**  be queried by calling
///[`GetPhysicalDeviceQueueFamilyProperties2`].
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`VK_NV_device_diagnostic_checkpoints`]
/// - [`PipelineStageFlags2`]
/// - [`StructureType`]
/// - [`GetQueueCheckpointData2NV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCheckpointData2NV")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CheckpointData2NV<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`stage`] indicates a single pipeline stage which the checkpoint
    ///marker data refers to.
    pub stage: PipelineStageFlags2,
    ///[`checkpoint_marker`] contains the value of the last checkpoint marker
    ///executed in the stage that [`stage`] refers to.
    pub checkpoint_marker: *mut c_void,
}
impl<'lt> Default for CheckpointData2NV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            stage: Default::default(),
            checkpoint_marker: std::ptr::null_mut(),
        }
    }
}
impl<'lt> CheckpointData2NV<'lt> {
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
    pub fn stage(&self) -> PipelineStageFlags2 {
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
    pub fn stage_mut(&mut self) -> &mut PipelineStageFlags2 {
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
    pub fn set_stage(&mut self, value: crate::vulkan1_3::PipelineStageFlags2) -> &mut Self {
        self.stage = value;
        self
    }
    ///Sets the raw value of [`Self::checkpoint_marker`]
    pub fn set_checkpoint_marker(&mut self, value: &'lt mut std::ffi::c_void) -> &mut Self {
        self.checkpoint_marker = value as *mut _;
        self
    }
}
