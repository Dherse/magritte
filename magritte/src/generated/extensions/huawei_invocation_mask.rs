//![VK_HUAWEI_invocation_mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_HUAWEI_invocation_mask.html) - device extension
//!# Description
//!The rays to trace may be sparse in some use cases.
//!For example, the scene only have a few regions to reflect.
//!Providing an invocation mask image to the ray tracing commands could
//!potentially give the hardware the hint to do certain optimization without
//!invoking an additional pass to compact the ray buffer.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_ray_tracing_pipeline`]`
//! - Requires `[`khr_synchronization2`]`
//!# Contacts
//! - Yunpeng Zhu [yunxingzhu](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_HUAWEI_invocation_mask]
//!   @yunxingzhu%0A<<Here describe the issue or question you have about the
//!   VK_HUAWEI_invocation_mask extension>>)
//!# New functions & commands
//! - [`cmd_bind_invocation_mask_huawei`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceInvocationMaskFeaturesHUAWEI`]
//!# New constants
//! - [`HUAWEI_INVOCATION_MASK_EXTENSION_NAME`]
//! - [`HUAWEI_INVOCATION_MASK_SPEC_VERSION`]
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI`
//! - Extending [`ImageUsageFlagBits`]:  - `VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI`
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI`
//!# Version History
//! - Revision 1, 2021-05-27 (Yunpeng Zhu)  - Initial draft.
//!# Other info
//! * 2021-05-27
//! * - This extension requires `[`khr_ray_tracing_pipeline`]`, which allow to bind an invocation
//!   mask image before the ray tracing command  - This extension requires
//!   `[`khr_synchronization2`]`, which allows new pipeline stage for the invocation mask image
//! * - Yunpeng Zhu, HuaWei
//!# Related
//! - [`PhysicalDeviceInvocationMaskFeaturesHUAWEI`]
//! - [`cmd_bind_invocation_mask_huawei`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, Device, ImageLayout, ImageView, StructureType},
    AsRaw, Unique,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_HUAWEI_INVOCATION_MASK_SPEC_VERSION")]
pub const HUAWEI_INVOCATION_MASK_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_HUAWEI_INVOCATION_MASK_EXTENSION_NAME")]
pub const HUAWEI_INVOCATION_MASK_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_HUAWEI_invocation_mask");
///[vkCmdBindInvocationMaskHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html) - Bind an invocation mask image on a command buffer
///# C Specifications
///When invocation mask image usage is enabled in the bound ray tracing
///pipeline, the pipeline uses an invocation mask image specified by the
///command:
///```c
///// Provided by VK_HUAWEI_invocation_mask
///void vkCmdBindInvocationMaskHUAWEI(
///    VkCommandBuffer                             commandBuffer,
///    VkImageView                                 imageView,
///    VkImageLayout                               imageLayout);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded
/// - [`image_view`] is an image view handle specifying the invocation mask image [`image_view`]
///   **may**  be set to [`crate::Handle::null`], which is equivalent to specifying a view of an
///   image filled with ones value.
/// - [`image_layout`] is the layout that the image subresources accessible from [`image_view`] will
///   be in when the invocation mask image is accessed
///# Description
///## Valid Usage
/// - The [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-invocationMask)
///   feature  **must**  be enabled
/// - If [`image_view`] is not [`crate::Handle::null`], it  **must**  be a valid [`ImageView`]
///   handle of type `VK_IMAGE_VIEW_TYPE_2D`
/// - If [`image_view`] is not [`crate::Handle::null`], it  **must**  have a format of
///   `VK_FORMAT_R8_UINT`
/// - If [`image_view`] is not [`crate::Handle::null`], it  **must**  have been created with
///   `VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI` set
/// - If [`image_view`] is not [`crate::Handle::null`], [`image_layout`] **must**  be
///   `VK_IMAGE_LAYOUT_GENERAL`
/// - Thread mask image resolution must match the `width` and `height` in [`cmd_trace_rays_khr`]
/// - Each element in the invocation mask image  **must**  have the value `0` or `1`. The value 1
///   means the invocation is active
/// - `width` in [`cmd_trace_rays_khr`] should be 1
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - If [`image_view`] is not [`crate::Handle::null`], [`image_view`] **must**  be a valid
///   [`ImageView`] handle
/// - [`image_layout`] **must**  be a valid [`ImageLayout`] value
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
///   operations
/// - This command  **must**  only be called outside of a render pass instance
/// - Both of [`command_buffer`], and [`image_view`] that are valid handles of non-ignored
///   parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`huawei_invocation_mask`]
/// - [`CommandBuffer`]
/// - [`ImageLayout`]
/// - [`ImageView`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
pub type FNCmdBindInvocationMaskHuawei =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, image_view: ImageView, image_layout: ImageLayout)>;
///[VkPhysicalDeviceInvocationMaskFeaturesHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInvocationMaskFeaturesHUAWEI.html) - Structure describing invocation mask features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceInvocationMaskFeaturesHUAWEI`] structure is defined
///as:
///```c
///// Provided by VK_HUAWEI_invocation_mask
///typedef struct VkPhysicalDeviceInvocationMaskFeaturesHUAWEI {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           invocationMask;
///} VkPhysicalDeviceInvocationMaskFeaturesHUAWEI;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`invocation_mask`] indicates that the implementation supports the use of an invocation mask
///   image to optimize the ray dispatch.
///If the [`PhysicalDeviceInvocationMaskFeaturesHUAWEI`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceInvocationMaskFeaturesHUAWEI`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI`
///# Related
/// - [`huawei_invocation_mask`]
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
#[doc(alias = "VkPhysicalDeviceInvocationMaskFeaturesHUAWEI")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceInvocationMaskFeaturesHUAWEI<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`invocation_mask`] indicates that the
    ///implementation supports the use of an invocation mask image to optimize
    ///the ray dispatch.
    pub invocation_mask: Bool32,
}
impl<'lt> Default for PhysicalDeviceInvocationMaskFeaturesHUAWEI<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI,
            p_next: std::ptr::null_mut(),
            invocation_mask: 0,
        }
    }
}
impl<'lt> PhysicalDeviceInvocationMaskFeaturesHUAWEI<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::invocation_mask`]
    pub fn invocation_mask_raw(&self) -> Bool32 {
        self.invocation_mask
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::invocation_mask`]
    pub fn set_invocation_mask_raw(&mut self, value: Bool32) -> &mut Self {
        self.invocation_mask = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::invocation_mask`]
    pub fn with_invocation_mask_raw(mut self, value: Bool32) -> Self {
        self.invocation_mask = value;
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
    ///Gets the value of [`Self::invocation_mask`]
    pub fn invocation_mask(&self) -> bool {
        unsafe { std::mem::transmute(self.invocation_mask as u8) }
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
    ///Gets a mutable reference to the value of [`Self::invocation_mask`]
    pub fn invocation_mask_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.invocation_mask as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.invocation_mask as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the value of [`Self::invocation_mask`]
    pub fn set_invocation_mask(&mut self, value: bool) -> &mut Self {
        self.invocation_mask = value as u8 as u32;
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
    ///Sets the value of [`Self::invocation_mask`]
    pub fn with_invocation_mask(mut self, value: bool) -> Self {
        self.invocation_mask = value as u8 as u32;
        self
    }
}
impl CommandBuffer {
    ///[vkCmdBindInvocationMaskHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html) - Bind an invocation mask image on a command buffer
    ///# C Specifications
    ///When invocation mask image usage is enabled in the bound ray tracing
    ///pipeline, the pipeline uses an invocation mask image specified by the
    ///command:
    ///```c
    ///// Provided by VK_HUAWEI_invocation_mask
    ///void vkCmdBindInvocationMaskHUAWEI(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkImageView                                 imageView,
    ///    VkImageLayout                               imageLayout);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded
    /// - [`image_view`] is an image view handle specifying the invocation mask image [`image_view`]
    ///   **may**  be set to [`crate::Handle::null`], which is equivalent to specifying a view of an
    ///   image filled with ones value.
    /// - [`image_layout`] is the layout that the image subresources accessible from [`image_view`]
    ///   will be in when the invocation mask image is accessed
    ///# Description
    ///## Valid Usage
    /// - The [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-invocationMask)
    ///   feature  **must**  be enabled
    /// - If [`image_view`] is not [`crate::Handle::null`], it  **must**  be a valid [`ImageView`]
    ///   handle of type `VK_IMAGE_VIEW_TYPE_2D`
    /// - If [`image_view`] is not [`crate::Handle::null`], it  **must**  have a format of
    ///   `VK_FORMAT_R8_UINT`
    /// - If [`image_view`] is not [`crate::Handle::null`], it  **must**  have been created with
    ///   `VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI` set
    /// - If [`image_view`] is not [`crate::Handle::null`], [`image_layout`] **must**  be
    ///   `VK_IMAGE_LAYOUT_GENERAL`
    /// - Thread mask image resolution must match the `width` and `height` in [`cmd_trace_rays_khr`]
    /// - Each element in the invocation mask image  **must**  have the value `0` or `1`. The value
    ///   1 means the invocation is active
    /// - `width` in [`cmd_trace_rays_khr`] should be 1
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - If [`image_view`] is not [`crate::Handle::null`], [`image_view`] **must**  be a valid
    ///   [`ImageView`] handle
    /// - [`image_layout`] **must**  be a valid [`ImageLayout`] value
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
    ///   operations
    /// - This command  **must**  only be called outside of a render pass instance
    /// - Both of [`command_buffer`], and [`image_view`] that are valid handles of non-ignored
    ///   parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`huawei_invocation_mask`]
    /// - [`CommandBuffer`]
    /// - [`ImageLayout`]
    /// - [`ImageView`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_bind_invocation_mask_huawei(
        self: &Unique<CommandBuffer>,
        image_view: Option<ImageView>,
        image_layout: ImageLayout,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .huawei_invocation_mask()
            .and_then(|vtable| vtable.cmd_bind_invocation_mask_huawei())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .huawei_invocation_mask()
            .and_then(|vtable| vtable.cmd_bind_invocation_mask_huawei())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), image_view.unwrap_or_default(), image_layout);
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_HUAWEI_invocation_mask`
pub struct DeviceHuaweiInvocationMaskVTable {
    ///See [`FNCmdBindInvocationMaskHuawei`] for more information.
    pub cmd_bind_invocation_mask_huawei: FNCmdBindInvocationMaskHuawei,
}
impl DeviceHuaweiInvocationMaskVTable {
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
            cmd_bind_invocation_mask_huawei: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdBindInvocationMaskHUAWEI").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::cmd_bind_invocation_mask_huawei`]. See [`FNCmdBindInvocationMaskHuawei`] for
    /// more information.
    pub fn cmd_bind_invocation_mask_huawei(&self) -> FNCmdBindInvocationMaskHuawei {
        self.cmd_bind_invocation_mask_huawei
    }
}
