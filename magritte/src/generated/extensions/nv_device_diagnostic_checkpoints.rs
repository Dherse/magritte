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
///[`QueueFamilyCheckpointPropertiesNV`] structure.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct QueueFamilyCheckpointPropertiesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`checkpoint_execution_stage_mask`] is a mask indicating which pipeline
    ///stages the implementation can execute checkpoint markers in.
    checkpoint_execution_stage_mask: PipelineStageFlags,
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
/// - [`p_checkpoint_marker`] contains the value of the last checkpoint marker executed in the stage
///   that [`stage`] refers to.
///# Description
///The stages at which a checkpoint marker **can** be executed are
///implementation-defined and **can** be queried by calling
///[`GetPhysicalDeviceQueueFamilyProperties2`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CheckpointDataNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`stage`] is a [`PipelineStageFlagBits`] value specifying which
    ///pipeline stage the checkpoint marker data refers to.
    stage: PipelineStageFlagBits,
    ///[`p_checkpoint_marker`] contains the value of the last checkpoint marker
    ///executed in the stage that [`stage`] refers to.
    p_checkpoint_marker: *const c_void,
}
