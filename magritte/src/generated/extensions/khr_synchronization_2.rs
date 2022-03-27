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
///[`QueueFamilyCheckpointProperties2NV`] structure.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct QueueFamilyCheckpointProperties2NV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`checkpoint_execution_stage_mask`] is a mask indicating which pipeline
    ///stages the implementation can execute checkpoint markers in.
    checkpoint_execution_stage_mask: PipelineStageFlags2,
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
/// - [`p_checkpoint_marker`] contains the value of the last checkpoint marker executed in the stage
///   that [`stage`] refers to.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV`
/// - [`p_next`]**must** be `NULL`
///The stages at which a checkpoint marker **can** be executed are
///implementation-defined and **can** be queried by calling
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CheckpointData2NV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`stage`] indicates a single pipeline stage which the checkpoint
    ///marker data refers to.
    stage: PipelineStageFlags2,
    ///[`p_checkpoint_marker`] contains the value of the last checkpoint marker
    ///executed in the stage that [`stage`] refers to.
    p_checkpoint_marker: *const c_void,
}
