//![VK_KHR_push_descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_push_descriptor.html) - device extension
//!# Description
//!This extension allows descriptors to be written into the command buffer,
//!while the implementation is responsible for managing their memory.
//!Push descriptors may enable easier porting from older APIs and in some cases
//!can be more efficient than writing descriptors into descriptor sets.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_push_descriptor]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the VK_KHR_push_descriptor
//!   extension>>)
//!# New functions & commands
//! - [`cmd_push_descriptor_set_khr`]
//!If [`khr_descriptor_update_template`] is supported:
//! - [`cmd_push_descriptor_set_with_template_khr`]
//!If [Version 1.1]() is supported:
//! - [`cmd_push_descriptor_set_with_template_khr`]
//!# New structures
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDevicePushDescriptorPropertiesKHR`]
//!# New constants
//! - [`KHR_PUSH_DESCRIPTOR_EXTENSION_NAME`]
//! - [`KHR_PUSH_DESCRIPTOR_SPEC_VERSION`]
//! - Extending [`DescriptorSetLayoutCreateFlagBits`]:  -
//!   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR`
//!If [`khr_descriptor_update_template`] is supported:
//! - Extending [`DescriptorUpdateTemplateType`]:  -
//!   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
//!If [Version 1.1]() is supported:
//! - Extending [`DescriptorUpdateTemplateType`]:  -
//!   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
//!# Version History
//! - Revision 1, 2016-10-15 (Jeff Bolz)  - Internal revisions
//! - Revision 2, 2017-09-12 (Tobias Hector)  - Added interactions with Vulkan 1.1
//!# Other info
//! * 2017-09-12
//! * No known IP claims.
//! * - Jeff Bolz, NVIDIA  - Michael Worcester, Imagination Technologies
//!# Related
//! - [`PhysicalDevicePushDescriptorPropertiesKHR`]
//! - [`cmd_push_descriptor_set_khr`]
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
        BaseOutStructure, CommandBuffer, Device, PipelineBindPoint, PipelineLayout, StructureType, WriteDescriptorSet,
    },
    AsRaw, Unique,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION")]
pub const KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME")]
pub const KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_push_descriptor");
///[vkCmdPushDescriptorSetKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html) - Pushes descriptor updates into a command buffer
///# C Specifications
///In addition to allocating descriptor sets and binding them to a command
///buffer, an application  **can**  record descriptor updates into the command
///buffer.To push descriptor updates into a command buffer, call:
///```c
///// Provided by VK_KHR_push_descriptor
///void vkCmdPushDescriptorSetKHR(
///    VkCommandBuffer                             commandBuffer,
///    VkPipelineBindPoint                         pipelineBindPoint,
///    VkPipelineLayout                            layout,
///    uint32_t                                    set,
///    uint32_t                                    descriptorWriteCount,
///    const VkWriteDescriptorSet*                 pDescriptorWrites);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer that the descriptors will be recorded in.
/// - [`pipeline_bind_point`] is a [`PipelineBindPoint`] indicating the type of the pipeline that
///   will use the descriptors. There is a separate set of push descriptor bindings for each
///   pipeline type, so binding one does not disturb the others.
/// - [`layout`] is a [`PipelineLayout`] object used to program the bindings.
/// - [`set`] is the set number of the descriptor set in the pipeline layout that will be updated.
/// - [`descriptor_write_count`] is the number of elements in the [`p_descriptor_writes`] array.
/// - [`p_descriptor_writes`] is a pointer to an array of [`WriteDescriptorSet`] structures
///   describing the descriptors to be updated.
///# Description
///*Push descriptors* are a small bank of descriptors whose storage is
///internally managed by the command buffer rather than being written into a
///descriptor set and later bound to a command buffer.
///Push descriptors allow for incremental updates of descriptors without
///managing the lifetime of descriptor sets.When a command buffer begins recording, all push
/// descriptors are undefined.
///Push descriptors  **can**  be updated incrementally and cause shaders to use the
///updated descriptors for subsequent [bound
///pipeline commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-bindpoint-commands) with the pipeline type set by [`pipeline_bind_point`]
///until the descriptor is overwritten, or else until the set is disturbed as
///described in [Pipeline Layout
///Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-compatibility).
///When the set is disturbed or push descriptors with a different descriptor
///set layout are set, all push descriptors are undefined.Push descriptors that are [statically used](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-staticuse) by a
///pipeline  **must**  not be undefined at the time that a drawing or dispatching
///command is recorded to execute using that pipeline.
///This includes immutable sampler descriptors, which  **must**  be pushed before
///they are accessed by a pipeline (the immutable samplers are pushed, rather
///than the samplers in [`p_descriptor_writes`]).
///Push descriptors that are not statically used  **can**  remain undefined.Push descriptors do not
/// use dynamic offsets.
///Instead, the corresponding non-dynamic descriptor types  **can**  be used and the
///`offset` member of [`DescriptorBufferInfo`] **can**  be changed each
///time the descriptor is written.Each element of [`p_descriptor_writes`] is interpreted as in
///[`WriteDescriptorSet`], except the `dstSet` member is ignored.To push an immutable sampler, use
/// a [`WriteDescriptorSet`] with
///`dstBinding` and `dstArrayElement` selecting the immutable sampler’s
///binding.
///If the descriptor type is `VK_DESCRIPTOR_TYPE_SAMPLER`, the
///`pImageInfo` parameter is ignored and the immutable sampler is taken
///from the push descriptor set layout in the pipeline layout.
///If the descriptor type is `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`,
///the `sampler` member of the `pImageInfo` parameter is ignored and
///the immutable sampler is taken from the push descriptor set layout in the
///pipeline layout.
///## Valid Usage
/// - [`pipeline_bind_point`] **must**  be supported by the [`command_buffer`]’s parent
///   [`CommandPool`]’s queue family
/// - [`set`] **must**  be less than [`PipelineLayoutCreateInfo::set_layout_count`] provided when
///   [`layout`] was created
/// - [`set`] **must**  be the unique set number in the pipeline layout that uses a descriptor set
///   layout that was created with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`
/// - For each element i where [`p_descriptor_writes`][i].`descriptorType` is
///   `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, or
///   `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`, [`p_descriptor_writes`][i].`pImageInfo` **must**  be a
///   valid pointer to an array of [`p_descriptor_writes`][i].`descriptorCount` valid
///   [`DescriptorImageInfo`] structures
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
/// - [`layout`] **must**  be a valid [`PipelineLayout`] handle
/// - [`p_descriptor_writes`] **must**  be a valid pointer to an array of [`descriptor_write_count`]
///   valid [`WriteDescriptorSet`] structures
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
/// - [`descriptor_write_count`] **must**  be greater than `0`
/// - Both of [`command_buffer`], and [`layout`] **must**  have been created, allocated, or
///   retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`khr_push_descriptor`]
/// - [`CommandBuffer`]
/// - [`PipelineBindPoint`]
/// - [`PipelineLayout`]
/// - [`WriteDescriptorSet`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdPushDescriptorSetKHR")]
pub type FNCmdPushDescriptorSetKhr = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet<'lt>,
    ),
>;
///[VkPhysicalDevicePushDescriptorPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html) - Structure describing push descriptor limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDevicePushDescriptorPropertiesKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_push_descriptor
///typedef struct VkPhysicalDevicePushDescriptorPropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxPushDescriptors;
///} VkPhysicalDevicePushDescriptorPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_push_descriptors`] is the maximum number of descriptors that  **can**  be used in a
///   descriptor set created with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR` set.
///# Description
///If the [`PhysicalDevicePushDescriptorPropertiesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR`
///# Related
/// - [`khr_push_descriptor`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDevicePushDescriptorPropertiesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDevicePushDescriptorPropertiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_push_descriptors`] is the maximum
    ///number of descriptors that  **can**  be used in a descriptor set created with
    ///`VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR` set.
    pub max_push_descriptors: u32,
}
impl<'lt> Default for PhysicalDevicePushDescriptorPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            max_push_descriptors: 0,
        }
    }
}
impl<'lt> PhysicalDevicePushDescriptorPropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::max_push_descriptors`]
    pub fn max_push_descriptors(&self) -> u32 {
        self.max_push_descriptors
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
    ///Gets a mutable reference to the value of [`Self::max_push_descriptors`]
    pub fn max_push_descriptors_mut(&mut self) -> &mut u32 {
        &mut self.max_push_descriptors
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
    ///Sets the value of [`Self::max_push_descriptors`]
    pub fn set_max_push_descriptors(mut self, value: u32) -> Self {
        self.max_push_descriptors = value;
        self
    }
}
impl CommandBuffer {
    ///[vkCmdPushDescriptorSetKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html) - Pushes descriptor updates into a command buffer
    ///# C Specifications
    ///In addition to allocating descriptor sets and binding them to a command
    ///buffer, an application  **can**  record descriptor updates into the command
    ///buffer.To push descriptor updates into a command buffer, call:
    ///```c
    ///// Provided by VK_KHR_push_descriptor
    ///void vkCmdPushDescriptorSetKHR(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkPipelineBindPoint                         pipelineBindPoint,
    ///    VkPipelineLayout                            layout,
    ///    uint32_t                                    set,
    ///    uint32_t                                    descriptorWriteCount,
    ///    const VkWriteDescriptorSet*                 pDescriptorWrites);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer that the descriptors will be recorded in.
    /// - [`pipeline_bind_point`] is a [`PipelineBindPoint`] indicating the type of the pipeline
    ///   that will use the descriptors. There is a separate set of push descriptor bindings for
    ///   each pipeline type, so binding one does not disturb the others.
    /// - [`layout`] is a [`PipelineLayout`] object used to program the bindings.
    /// - [`set`] is the set number of the descriptor set in the pipeline layout that will be
    ///   updated.
    /// - [`descriptor_write_count`] is the number of elements in the [`p_descriptor_writes`] array.
    /// - [`p_descriptor_writes`] is a pointer to an array of [`WriteDescriptorSet`] structures
    ///   describing the descriptors to be updated.
    ///# Description
    ///*Push descriptors* are a small bank of descriptors whose storage is
    ///internally managed by the command buffer rather than being written into a
    ///descriptor set and later bound to a command buffer.
    ///Push descriptors allow for incremental updates of descriptors without
    ///managing the lifetime of descriptor sets.When a command buffer begins recording, all push
    /// descriptors are undefined.
    ///Push descriptors  **can**  be updated incrementally and cause shaders to use the
    ///updated descriptors for subsequent [bound
    ///pipeline commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-bindpoint-commands) with the pipeline type set by [`pipeline_bind_point`]
    ///until the descriptor is overwritten, or else until the set is disturbed as
    ///described in [Pipeline Layout
    ///Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-compatibility).
    ///When the set is disturbed or push descriptors with a different descriptor
    ///set layout are set, all push descriptors are undefined.Push descriptors that are [statically used](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-staticuse) by a
    ///pipeline  **must**  not be undefined at the time that a drawing or dispatching
    ///command is recorded to execute using that pipeline.
    ///This includes immutable sampler descriptors, which  **must**  be pushed before
    ///they are accessed by a pipeline (the immutable samplers are pushed, rather
    ///than the samplers in [`p_descriptor_writes`]).
    ///Push descriptors that are not statically used  **can**  remain undefined.Push descriptors do
    /// not use dynamic offsets.
    ///Instead, the corresponding non-dynamic descriptor types  **can**  be used and the
    ///`offset` member of [`DescriptorBufferInfo`] **can**  be changed each
    ///time the descriptor is written.Each element of [`p_descriptor_writes`] is interpreted as in
    ///[`WriteDescriptorSet`], except the `dstSet` member is ignored.To push an immutable sampler,
    /// use a [`WriteDescriptorSet`] with
    ///`dstBinding` and `dstArrayElement` selecting the immutable sampler’s
    ///binding.
    ///If the descriptor type is `VK_DESCRIPTOR_TYPE_SAMPLER`, the
    ///`pImageInfo` parameter is ignored and the immutable sampler is taken
    ///from the push descriptor set layout in the pipeline layout.
    ///If the descriptor type is `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`,
    ///the `sampler` member of the `pImageInfo` parameter is ignored and
    ///the immutable sampler is taken from the push descriptor set layout in the
    ///pipeline layout.
    ///## Valid Usage
    /// - [`pipeline_bind_point`] **must**  be supported by the [`command_buffer`]’s parent
    ///   [`CommandPool`]’s queue family
    /// - [`set`] **must**  be less than [`PipelineLayoutCreateInfo::set_layout_count`] provided
    ///   when [`layout`] was created
    /// - [`set`] **must**  be the unique set number in the pipeline layout that uses a descriptor
    ///   set layout that was created with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`
    /// - For each element i where [`p_descriptor_writes`][i].`descriptorType` is
    ///   `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, or
    ///   `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`, [`p_descriptor_writes`][i].`pImageInfo` **must**
    ///   be a valid pointer to an array of [`p_descriptor_writes`][i].`descriptorCount` valid
    ///   [`DescriptorImageInfo`] structures
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
    /// - [`layout`] **must**  be a valid [`PipelineLayout`] handle
    /// - [`p_descriptor_writes`] **must**  be a valid pointer to an array of
    ///   [`descriptor_write_count`] valid [`WriteDescriptorSet`] structures
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, or compute operations
    /// - [`descriptor_write_count`] **must**  be greater than `0`
    /// - Both of [`command_buffer`], and [`layout`] **must**  have been created, allocated, or
    ///   retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`khr_push_descriptor`]
    /// - [`CommandBuffer`]
    /// - [`PipelineBindPoint`]
    /// - [`PipelineLayout`]
    /// - [`WriteDescriptorSet`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdPushDescriptorSetKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_push_descriptor_set_khr<'lt>(
        self: &Unique<CommandBuffer>,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: Option<u32>,
        p_descriptor_writes: &[crate::vulkan1_0::WriteDescriptorSet<'lt>],
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_push_descriptor()
            .and_then(|vtable| vtable.cmd_push_descriptor_set_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_push_descriptor()
            .and_then(|vtable| vtable.cmd_push_descriptor_set_khr())
            .unwrap_unchecked();
        let descriptor_write_count = (|len: usize| len)(p_descriptor_writes.len()) as _;
        let _return = _function(
            self.as_raw(),
            pipeline_bind_point,
            layout,
            set.unwrap_or_default() as _,
            descriptor_write_count,
            p_descriptor_writes.as_ptr(),
        );
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_push_descriptor`
pub struct DeviceKhrPushDescriptorVTable {
    ///See [`FNCmdPushDescriptorSetKhr`] for more information.
    pub cmd_push_descriptor_set_khr: FNCmdPushDescriptorSetKhr,
}
impl DeviceKhrPushDescriptorVTable {
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
            cmd_push_descriptor_set_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdPushDescriptorSetKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::cmd_push_descriptor_set_khr`]. See [`FNCmdPushDescriptorSetKhr`] for more
    /// information.
    pub fn cmd_push_descriptor_set_khr(&self) -> FNCmdPushDescriptorSetKhr {
        self.cmd_push_descriptor_set_khr
    }
}
