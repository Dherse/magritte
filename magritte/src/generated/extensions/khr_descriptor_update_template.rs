//![VK_KHR_descriptor_update_template](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_descriptor_update_template.html) - device extension
//!# Description
//!Applications may wish to update a fixed set of descriptors in a large number
//!of descriptor sets very frequently, i.e. during initializaton phase or if it
//!is required to rebuild descriptor sets for each frame.
//!For those cases it is also not unlikely that all information required to
//!update a single descriptor set is stored in a single struct.
//!This extension provides a way to update a fixed set of descriptors in a
//!single [`DescriptorSet`] with a pointer to a user defined data structure
//!describing the new descriptors.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Markus Tavenrath [mtavenrath](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_descriptor_update_template]
//!   @mtavenrath%0A<<Here describe the issue or question you have about the
//!   VK_KHR_descriptor_update_template extension>>)
//!# New handles
//! - [`DescriptorUpdateTemplateKHR`]
//!# New functions & commands
//! - [`create_descriptor_update_template_khr`]
//! - [`destroy_descriptor_update_template_khr`]
//! - [`update_descriptor_set_with_template_khr`]
//!If [`VK_KHR_push_descriptor`] is supported:
//! - [`cmd_push_descriptor_set_with_template_khr`]
//!# New structures
//! - [`DescriptorUpdateTemplateCreateInfoKHR`]
//! - [`DescriptorUpdateTemplateEntryKHR`]
//!# New enums
//! - [`DescriptorUpdateTemplateTypeKHR`]
//!# New bitmasks
//! - [`DescriptorUpdateTemplateCreateFlagsKHR`]
//!# New constants
//! - [`KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME`]
//! - [`KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION`]
//! - Extending [`DescriptorUpdateTemplateType`]:  -
//!   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR`
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR`
//!If [`VK_EXT_debug_report`] is supported:
//! - Extending [`DebugReportObjectTypeEXT`]:  -
//!   `VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT`
//!If [`VK_KHR_push_descriptor`] is supported:
//! - Extending [`DescriptorUpdateTemplateType`]:  -
//!   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
//!# Version History
//! - Revision 1, 2016-01-11 (Markus Tavenrath)  - Initial draft
//!# Other info
//! * 2017-09-05
//! * No known IP claims.
//! * - Interacts with `[`VK_KHR_push_descriptor`]`  - Promoted to Vulkan 1.1 Core
//! * - Jeff Bolz, NVIDIA  - Michael Worcester, Imagination Technologies
//!# Related
//! - [`DescriptorUpdateTemplateCreateFlagsKHR`]
//! - [`DescriptorUpdateTemplateCreateInfoKHR`]
//! - [`DescriptorUpdateTemplateEntryKHR`]
//! - [`DescriptorUpdateTemplateKHR`]
//! - [`DescriptorUpdateTemplateTypeKHR`]
//! - [`create_descriptor_update_template_khr`]
//! - [`destroy_descriptor_update_template_khr`]
//! - [`update_descriptor_set_with_template_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{CommandBuffer, Device, PipelineLayout},
    vulkan1_1::{
        DescriptorUpdateTemplate, FNCreateDescriptorUpdateTemplate, FNDestroyDescriptorUpdateTemplate,
        FNUpdateDescriptorSetWithTemplate,
    },
    AsRaw, Unique,
};
use std::ffi::{c_void, CStr};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION")]
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME")]
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_descriptor_update_template");
///[vkCmdPushDescriptorSetWithTemplateKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) - Pushes descriptor updates into a command buffer using a descriptor update template
///# C Specifications
///It is also possible to use a descriptor update template to specify the push
///descriptors to update.
///To do so, call:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_push_descriptor, VK_KHR_descriptor_update_template with
///// VK_KHR_push_descriptor
///void vkCmdPushDescriptorSetWithTemplateKHR(
///    VkCommandBuffer                             commandBuffer,
///    VkDescriptorUpdateTemplate                  descriptorUpdateTemplate,
///    VkPipelineLayout                            layout,
///    uint32_t                                    set,
///    const void*                                 pData);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer that the descriptors will be recorded in.
/// - [`descriptor_update_template`] is a descriptor update template defining how to interpret the
///   descriptor information in [`p_data`].
/// - [`layout`] is a [`PipelineLayout`] object used to program the bindings. It  **must**  be
///   compatible with the layout used to create the [`descriptor_update_template`] handle.
/// - [`set`] is the set number of the descriptor set in the pipeline layout that will be updated.
///   This  **must**  be the same number used to create the [`descriptor_update_template`] handle.
/// - [`p_data`] is a pointer to memory containing descriptors for the templated update.
/// # Description
/// ## Valid Usage
/// - The `pipelineBindPoint` specified during the creation of the descriptor update template
///   **must**  be supported by the [`command_buffer`]’s parent [`CommandPool`]’s queue family
/// - [`p_data`] **must**  be a valid pointer to a memory containing one or more valid instances of
///   [`DescriptorImageInfo`], [`DescriptorBufferInfo`], or [`BufferView`] in a layout defined by
///   [`descriptor_update_template`] when it was created with [`create_descriptor_update_template`]
///
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`descriptor_update_template`] **must**  be a valid [`DescriptorUpdateTemplate`] handle
/// - [`layout`] **must**  be a valid [`PipelineLayout`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
/// - Each of [`command_buffer`], [`descriptor_update_template`], and [`layout`] **must**  have been
///   created, allocated, or retrieved from the same [`Device`]
///
/// ## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
/// ## Command Properties
/// ## API example
/// ```c
///struct AppDataStructure
///{
///    VkDescriptorImageInfo  imageInfo;          // a single image info
///    // ... some more application related data
///};
///
///const VkDescriptorUpdateTemplateEntry descriptorUpdateTemplateEntries[] =
///{
///    // binding to a single image descriptor
///    {
///        0,                                           // binding
///        0,                                           // dstArrayElement
///        1,                                           // descriptorCount
///        VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,   // descriptorType
///        offsetof(AppDataStructure, imageInfo),       // offset
///        0                                            // stride is not required if descriptorCount
/// is 1
///    }
///};
///// create a descriptor update template for push descriptor set updates
///const VkDescriptorUpdateTemplateCreateInfo createInfo =
///{
///    VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,  // sType
///    NULL,                                                      // pNext
///    0,                                                         // flags
///    1,                                                         // descriptorUpdateEntryCount
///    descriptorUpdateTemplateEntries,                           // pDescriptorUpdateEntries
///    VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR,   // templateType
///    0,                                                         // descriptorSetLayout, ignored by
/// given templateType
///    VK_PIPELINE_BIND_POINT_GRAPHICS,                           // pipelineBindPoint
///    myPipelineLayout,                                          // pipelineLayout
///    0,                                                         // set
///};
///
///VkDescriptorUpdateTemplate myDescriptorUpdateTemplate;
///myResult = vkCreateDescriptorUpdateTemplate(
///    myDevice,
///    &createInfo,
///    NULL,
///    &myDescriptorUpdateTemplate);
///
///AppDataStructure appData;
///// fill appData here or cache it in your engine
///vkCmdPushDescriptorSetWithTemplateKHR(myCmdBuffer, myDescriptorUpdateTemplate, myPipelineLayout,
/// 0,&appData);
///```
/// # Related
/// - [`VK_KHR_descriptor_update_template`]
/// - [`VK_KHR_push_descriptor`]
/// - [`crate::vulkan1_1`]
/// - [`CommandBuffer`]
/// - [`DescriptorUpdateTemplate`]
/// - [`PipelineLayout`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
pub type FNCmdPushDescriptorSetWithTemplateKhr = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        p_data: *const c_void,
    ),
>;
impl CommandBuffer {
    ///[vkCmdPushDescriptorSetWithTemplateKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) - Pushes descriptor updates into a command buffer using a descriptor update template
    ///# C Specifications
    ///It is also possible to use a descriptor update template to specify the push
    ///descriptors to update.
    ///To do so, call:
    ///```c
    ///// Provided by VK_VERSION_1_1 with VK_KHR_push_descriptor, VK_KHR_descriptor_update_template
    ///// with VK_KHR_push_descriptor
    ///void vkCmdPushDescriptorSetWithTemplateKHR(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkDescriptorUpdateTemplate                  descriptorUpdateTemplate,
    ///    VkPipelineLayout                            layout,
    ///    uint32_t                                    set,
    ///    const void*                                 pData);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer that the descriptors will be recorded in.
    /// - [`descriptor_update_template`] is a descriptor update template defining how to interpret
    ///   the descriptor information in [`p_data`].
    /// - [`layout`] is a [`PipelineLayout`] object used to program the bindings. It  **must**  be
    ///   compatible with the layout used to create the [`descriptor_update_template`] handle.
    /// - [`set`] is the set number of the descriptor set in the pipeline layout that will be
    ///   updated. This  **must**  be the same number used to create the
    ///   [`descriptor_update_template`] handle.
    /// - [`p_data`] is a pointer to memory containing descriptors for the templated update.
    /// # Description
    /// ## Valid Usage
    /// - The `pipelineBindPoint` specified during the creation of the descriptor update template
    ///   **must**  be supported by the [`command_buffer`]’s parent [`CommandPool`]’s queue family
    /// - [`p_data`] **must**  be a valid pointer to a memory containing one or more valid instances
    ///   of [`DescriptorImageInfo`], [`DescriptorBufferInfo`], or [`BufferView`] in a layout
    ///   defined by [`descriptor_update_template`] when it was created with
    ///   [`create_descriptor_update_template`]
    ///
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`descriptor_update_template`] **must**  be a valid [`DescriptorUpdateTemplate`] handle
    /// - [`layout`] **must**  be a valid [`PipelineLayout`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, or compute operations
    /// - Each of [`command_buffer`], [`descriptor_update_template`], and [`layout`] **must**  have
    ///   been created, allocated, or retrieved from the same [`Device`]
    ///
    /// ## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    /// ## Command Properties
    /// ## API example
    /// ```c
    ///struct AppDataStructure
    ///{
    ///    VkDescriptorImageInfo  imageInfo;          // a single image info
    ///    // ... some more application related data
    ///};
    ///
    ///const VkDescriptorUpdateTemplateEntry descriptorUpdateTemplateEntries[] =
    ///{
    ///    // binding to a single image descriptor
    ///    {
    ///        0,                                           // binding
    ///        0,                                           // dstArrayElement
    ///        1,                                           // descriptorCount
    ///        VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,   // descriptorType
    ///        offsetof(AppDataStructure, imageInfo),       // offset
    ///        0                                            // stride is not required if
    /// descriptorCount is 1
    ///    }
    ///};
    ///// create a descriptor update template for push descriptor set updates
    ///const VkDescriptorUpdateTemplateCreateInfo createInfo =
    ///{
    ///    VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,  // sType
    ///    NULL,                                                      // pNext
    ///    0,                                                         // flags
    ///    1,                                                         // descriptorUpdateEntryCount
    ///    descriptorUpdateTemplateEntries,                           // pDescriptorUpdateEntries
    ///    VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR,   // templateType
    ///    0,                                                         // descriptorSetLayout,
    /// ignored by given templateType
    ///    VK_PIPELINE_BIND_POINT_GRAPHICS,                           // pipelineBindPoint
    ///    myPipelineLayout,                                          // pipelineLayout
    ///    0,                                                         // set
    ///};
    ///
    ///VkDescriptorUpdateTemplate myDescriptorUpdateTemplate;
    ///myResult = vkCreateDescriptorUpdateTemplate(
    ///    myDevice,
    ///    &createInfo,
    ///    NULL,
    ///    &myDescriptorUpdateTemplate);
    ///
    ///AppDataStructure appData;
    ///// fill appData here or cache it in your engine
    ///vkCmdPushDescriptorSetWithTemplateKHR(myCmdBuffer, myDescriptorUpdateTemplate,
    /// myPipelineLayout, 0,&appData);
    ///```
    /// # Related
    /// - [`VK_KHR_descriptor_update_template`]
    /// - [`VK_KHR_push_descriptor`]
    /// - [`crate::vulkan1_1`]
    /// - [`CommandBuffer`]
    /// - [`DescriptorUpdateTemplate`]
    /// - [`PipelineLayout`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(
        self: &Unique<CommandBuffer>,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: Option<u32>,
        p_data: *const c_void,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_descriptor_update_template()
            .and_then(|vtable| vtable.cmd_push_descriptor_set_with_template_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_descriptor_update_template()
            .and_then(|vtable| vtable.cmd_push_descriptor_set_with_template_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            descriptor_update_template,
            layout,
            set.unwrap_or_default() as _,
            p_data,
        );
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_descriptor_update_template`
pub struct DeviceKhrDescriptorUpdateTemplateVTable {
    ///See [`FNCmdPushDescriptorSetWithTemplateKhr`] for more information.
    pub cmd_push_descriptor_set_with_template_khr: FNCmdPushDescriptorSetWithTemplateKhr,
    ///See [`FNCreateDescriptorUpdateTemplate`] for more information.
    pub create_descriptor_update_template_khr: FNCreateDescriptorUpdateTemplate,
    ///See [`FNDestroyDescriptorUpdateTemplate`] for more information.
    pub destroy_descriptor_update_template_khr: FNDestroyDescriptorUpdateTemplate,
    ///See [`FNUpdateDescriptorSetWithTemplate`] for more information.
    pub update_descriptor_set_with_template_khr: FNUpdateDescriptorSetWithTemplate,
}
impl DeviceKhrDescriptorUpdateTemplateVTable {
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
            cmd_push_descriptor_set_with_template_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdPushDescriptorSetWithTemplateKHR").as_ptr(),
                ))
            },
            create_descriptor_update_template_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCreateDescriptorUpdateTemplateKHR").as_ptr(),
                ))
            },
            destroy_descriptor_update_template_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkDestroyDescriptorUpdateTemplateKHR").as_ptr(),
                ))
            },
            update_descriptor_set_with_template_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkUpdateDescriptorSetWithTemplateKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::cmd_push_descriptor_set_with_template_khr`]. See
    /// [`FNCmdPushDescriptorSetWithTemplateKhr`] for more information.
    pub fn cmd_push_descriptor_set_with_template_khr(&self) -> FNCmdPushDescriptorSetWithTemplateKhr {
        self.cmd_push_descriptor_set_with_template_khr
    }
    ///Gets [`Self::create_descriptor_update_template_khr`]. See
    /// [`FNCreateDescriptorUpdateTemplate`] for more information.
    pub fn create_descriptor_update_template_khr(&self) -> FNCreateDescriptorUpdateTemplate {
        self.create_descriptor_update_template_khr
    }
    ///Gets [`Self::destroy_descriptor_update_template_khr`]. See
    /// [`FNDestroyDescriptorUpdateTemplate`] for more information.
    pub fn destroy_descriptor_update_template_khr(&self) -> FNDestroyDescriptorUpdateTemplate {
        self.destroy_descriptor_update_template_khr
    }
    ///Gets [`Self::update_descriptor_set_with_template_khr`]. See
    /// [`FNUpdateDescriptorSetWithTemplate`] for more information.
    pub fn update_descriptor_set_with_template_khr(&self) -> FNUpdateDescriptorSetWithTemplate {
        self.update_descriptor_set_with_template_khr
    }
}
