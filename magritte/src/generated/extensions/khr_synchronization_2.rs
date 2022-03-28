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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct QueueFamilyCheckpointProperties2NV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`checkpoint_execution_stage_mask`] is a mask indicating which pipeline
    ///stages the implementation can execute checkpoint markers in.
    checkpoint_execution_stage_mask: PipelineStageFlags2,
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CheckpointData2NV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`stage`] indicates a single pipeline stage which the checkpoint
    ///marker data refers to.
    stage: PipelineStageFlags2,
    ///[`checkpoint_marker`] contains the value of the last checkpoint marker
    ///executed in the stage that [`stage`] refers to.
    checkpoint_marker: *mut c_void,
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
