//![VK_EXT_color_write_enable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_color_write_enable.html) - device extension
//!# Description
//!This extension allows for selectively enabling and disabling writes to
//!output color attachments via a pipeline dynamic state.The intended use cases for this new state
//! are mostly identical to those of
//!colorWriteMask, such as selectively disabling writes to avoid feedback loops
//!between subpasses or bandwidth savings for unused outputs.
//!By making the state dynamic, one additional benefit is the ability to reduce
//!pipeline counts and pipeline switching via shaders that write a superset of
//!the desired data of which subsets are selected dynamically.
//!The reason for a new state, colorWriteEnable, rather than making
//!colorWriteMask dynamic is that, on many implementations, the more flexible
//!per-component semantics of the colorWriteMask state cannot be made dynamic
//!in a performant manner.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Sharif Elcott [selcott](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_color_write_enable]
//!   @selcott%0A<<Here describe the issue or question you have about the VK_EXT_color_write_enable
//!   extension>>)
//!# New functions & commands
//! - [`CmdSetColorWriteEnableEXT`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceColorWriteEnableFeaturesEXT`]
//! - Extending [`PipelineColorBlendStateCreateInfo`]:  - [`PipelineColorWriteCreateInfoEXT`]
//!# New constants
//! - [`EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME`]
//! - [`EXT_COLOR_WRITE_ENABLE_SPEC_VERSION`]
//! - Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2020-01-25 (Sharif Elcott)  - Internal revisions
//!# Other info
//! * 2020-02-25
//! * No known IP claims.
//! * - Sharif Elcott, Google  - Tobias Hector, AMD  - Piers Daniell, NVIDIA
//!# Related
//! - [`PhysicalDeviceColorWriteEnableFeaturesEXT`]
//! - [`PipelineColorWriteCreateInfoEXT`]
//! - [`CmdSetColorWriteEnableEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_COLOR_WRITE_ENABLE_SPEC_VERSION")]
pub const EXT_COLOR_WRITE_ENABLE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME")]
pub const EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_color_write_enable");
///[VkPhysicalDeviceColorWriteEnableFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceColorWriteEnableFeaturesEXT.html) - Structure describing whether writes to color attachments can be enabled and disabled dynamically
///# C Specifications
///The [`PhysicalDeviceColorWriteEnableFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_color_write_enable
///typedef struct VkPhysicalDeviceColorWriteEnableFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           colorWriteEnable;
///} VkPhysicalDeviceColorWriteEnableFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`color_write_enable`] indicates that the implementation supports the dynamic state
///   `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT`.
///If the [`PhysicalDeviceColorWriteEnableFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceColorWriteEnableFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT`
///# Related
/// - [`VK_EXT_color_write_enable`]
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
#[doc(alias = "VkPhysicalDeviceColorWriteEnableFeaturesEXT")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceColorWriteEnableFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`color_write_enable`] indicates that the
    ///implementation supports the dynamic state
    ///`VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT`.
    pub color_write_enable: Bool32,
}
impl<'lt> Default for PhysicalDeviceColorWriteEnableFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceColorWriteEnableFeaturesExt,
            p_next: std::ptr::null_mut(),
            color_write_enable: 0,
        }
    }
}
impl<'lt> PhysicalDeviceColorWriteEnableFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::color_write_enable`]
    pub fn color_write_enable_raw(&self) -> Bool32 {
        self.color_write_enable
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::color_write_enable`]
    pub fn set_color_write_enable_raw(&mut self, value: Bool32) -> &mut Self {
        self.color_write_enable = value;
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
    ///Gets the value of [`Self::color_write_enable`]
    pub fn color_write_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.color_write_enable as u8) }
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
    ///Gets a mutable reference to the value of [`Self::color_write_enable`]
    pub fn color_write_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.color_write_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.color_write_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the raw value of [`Self::color_write_enable`]
    pub fn set_color_write_enable(&mut self, value: bool) -> &mut Self {
        self.color_write_enable = value as u8 as u32;
        self
    }
}
///[VkPipelineColorWriteCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorWriteCreateInfoEXT.html) - Structure specifying color write state of a newly created pipeline
///# C Specifications
///The [`PipelineColorWriteCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_color_write_enable
///typedef struct VkPipelineColorWriteCreateInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           attachmentCount;
///    const VkBool32*    pColorWriteEnables;
///} VkPipelineColorWriteCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`attachment_count`] is the number of [`Bool32`] elements in [`color_write_enables`].
/// - [`color_write_enables`] is a pointer to an array of per target attachment boolean values
///   specifying whether color writes are enabled for the given attachment.
///# Description
///When this structure is included in the [`p_next`] chain of
///[`PipelineColorBlendStateCreateInfo`], it defines per-attachment color
///write state.
///If this structure is not included in the [`p_next`] chain, it is equivalent
///to specifying this structure with [`attachment_count`] equal to the
///[`attachment_count`] member of [`PipelineColorBlendStateCreateInfo`],
///and [`color_write_enables`] pointing to an array of as many [`TRUE`]
///values.If the [colorWriteEnable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-colorWriteEnable) feature is not enabled
///on the device, all [`Bool32`] elements in the
///[`color_write_enables`] array  **must**  be [`TRUE`].Color Write Enable interacts with the
/// [Color
///Write Mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-color-write-mask) as follows:
/// - If `colorWriteEnable` is [`TRUE`], writes to the attachment are determined by the
///   `colorWriteMask`.
/// - If `colorWriteEnable` is [`FALSE`], the `colorWriteMask` is ignored and writes to all
///   components of the attachment are disabled. This is equivalent to specifying a `colorWriteMask`
///   of 0.
///
///## Valid Usage
/// - If the [colorWriteEnable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-colorWriteEnable)
///   feature is not enabled, all elements of [`color_write_enables`] **must**  be [`TRUE`]
/// - [`attachment_count`] **must**  be equal to the [`attachment_count`] member of the
///   [`PipelineColorBlendStateCreateInfo`] structure specified during pipeline creation
/// - [`attachment_count`] **must**  be less than or equal to the `maxColorAttachments` member of
///   [`PhysicalDeviceLimits`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT`
/// - If [`attachment_count`] is not `0`, [`color_write_enables`] **must**  be a valid pointer to an
///   array of [`attachment_count`][`Bool32`] values
///# Related
/// - [`VK_EXT_color_write_enable`]
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
#[doc(alias = "VkPipelineColorWriteCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineColorWriteCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`attachment_count`] is the number of [`Bool32`] elements in
    ///[`color_write_enables`].
    pub attachment_count: u32,
    ///[`color_write_enables`] is a pointer to an array of per target
    ///attachment boolean values specifying whether color writes are enabled
    ///for the given attachment.
    pub color_write_enables: *const Bool32,
}
impl<'lt> Default for PipelineColorWriteCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PipelineColorWriteCreateInfoExt,
            p_next: std::ptr::null(),
            attachment_count: 0,
            color_write_enables: std::ptr::null(),
        }
    }
}
impl<'lt> PipelineColorWriteCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::color_write_enables`]
    pub fn color_write_enables_raw(&self) -> *const Bool32 {
        self.color_write_enables
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::color_write_enables`]
    pub fn set_color_write_enables_raw(&mut self, value: *const Bool32) -> &mut Self {
        self.color_write_enables = value;
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::attachment_count`]
    pub fn attachment_count(&self) -> u32 {
        self.attachment_count
    }
    ///Gets the value of [`Self::color_write_enables`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn color_write_enables(&self) -> &[Bool32] {
        std::slice::from_raw_parts(self.color_write_enables, self.attachment_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::attachment_count`]
    pub fn attachment_count_mut(&mut self) -> &mut u32 {
        &mut self.attachment_count
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::attachment_count`]
    pub fn set_attachment_count(&mut self, value: u32) -> &mut Self {
        self.attachment_count = value;
        self
    }
    ///Sets the raw value of [`Self::color_write_enables`]
    pub fn set_color_write_enables(&mut self, value: &'lt [crate::vulkan1_0::Bool32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.color_write_enables = value.as_ptr();
        self.attachment_count = len_;
        self
    }
}
