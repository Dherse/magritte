//![VK_NV_external_memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory.html) - device extension
//!# Description
//!Applications may wish to export memory to other Vulkan instances or other
//!APIs, or import memory from other Vulkan instances or other APIs to enable
//!Vulkan workloads to be split up across application module, process, or API
//!boundaries.
//!This extension enables applications to create exportable Vulkan memory
//!objects such that the underlying resources can be referenced outside the
//!Vulkan instance that created them.
//!# Revision
//!1
//!# Dependencies
//! - *Deprecated* by `[`VK_KHR_external_memory`]` extension  - Which in turn was *promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_NV_external_memory_capabilities`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_external_memory]
//!   @cubanismo%0A<<Here describe the issue or question you have about the VK_NV_external_memory
//!   extension>>)
//!# New structures
//! - Extending [`ImageCreateInfo`]:  - [`ExternalMemoryImageCreateInfoNV`]
//! - Extending [`MemoryAllocateInfo`]:  - [`ExportMemoryAllocateInfoNV`]
//!# New constants
//! - [`NV_EXTERNAL_MEMORY_EXTENSION_NAME`]
//! - [`NV_EXTERNAL_MEMORY_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV`
//!# Known issues & F.A.Q
//!1) If memory objects are shared between processes and APIs, is this
//!considered aliasing according to the rules outlined in the
//![Memory Aliasing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-memory-aliasing) section? **RESOLVED** : Yes, but strict exceptions to the rules are added to allow some
//!forms of aliasing in these cases.
//!Further, other extensions may build upon these new aliasing rules to define
//!specific support usage within Vulkan for imported native memory objects, or
//!memory objects from other APIs.2) Are new image layouts or metadata required to specify image
//! layouts and
//!layout transitions compatible with non-Vulkan APIs, or with other instances
//!of the same Vulkan driver? **RESOLVED** : No.
//!Separate instances of the same Vulkan driver running on the same GPU should
//!have identical internal layout semantics, so applictions have the tools they
//!need to ensure views of images are consistent between the two instances.
//!Other APIs will fall into two categories: Those that are Vulkan compatible
//!(a term to be defined by subsequent interopability extensions), or Vulkan
//!incompatible.
//!When sharing images with Vulkan incompatible APIs, the Vulkan image must be
//!transitioned to the `VK_IMAGE_LAYOUT_GENERAL` layout before handing it
//!off to the external API.Note this does not attempt to address cross-device transitions, nor
//!transitions to engines on the same device which are not visible within the
//!Vulkan API.
//!Both of these are beyond the scope of this extension.
//!# Version History
//! - Revision 1, 2016-08-19 (James Jones)  - Initial draft
//!# Other info
//! * 2016-08-19
//! * No known IP claims.
//! * - James Jones, NVIDIA  - Carsten Rohde, NVIDIA
//!# Related
//! - [`ExportMemoryAllocateInfoNV`]
//! - [`ExternalMemoryImageCreateInfoNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_external_memory");
///[VkExternalMemoryImageCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfoNV.html) - Specify that an image may be backed by external memory
///# C Specifications
///If the [`p_next`] chain includes a [`ExternalMemoryImageCreateInfoNV`]
///structure, then that structure defines a set of external memory handle types
///that  **may**  be used as backing store for the image.The [`ExternalMemoryImageCreateInfoNV`]
/// structure is defined as:
///```c
///// Provided by VK_NV_external_memory
///typedef struct VkExternalMemoryImageCreateInfoNV {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkExternalMemoryHandleTypeFlagsNV    handleTypes;
///} VkExternalMemoryImageCreateInfoNV;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_types`] is zero, or a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] specifying
///   one or more external memory handle types.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV`
/// - [`handle_types`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`]
///   values
/// # Related
/// - [`VK_NV_external_memory`]
/// - [`ExternalMemoryHandleTypeFlagsNV`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalMemoryImageCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_types`] is zero, or a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBitsNV`] specifying one or more
    ///external memory handle types.
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}
impl<'lt> Default for ExternalMemoryImageCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ExternalMemoryImageCreateInfoNv,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl<'lt> ExternalMemoryImageCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::handle_types`]
    pub fn handle_types(&self) -> ExternalMemoryHandleTypeFlagsNV {
        self.handle_types
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_types`]
    pub fn handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagsNV {
        &mut self.handle_types
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
    ///Sets the raw value of [`Self::handle_types`]
    pub fn set_handle_types(
        &mut self,
        value: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    ) -> &mut Self {
        self.handle_types = value;
        self
    }
}
///[VkExportMemoryAllocateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfoNV.html) - Specify memory handle types that may be exported
///# C Specifications
///The [`ExportMemoryAllocateInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_external_memory
///typedef struct VkExportMemoryAllocateInfoNV {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkExternalMemoryHandleTypeFlagsNV    handleTypes;
///} VkExportMemoryAllocateInfoNV;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] specifying one or more
///   memory handle types that  **may**  be exported. Multiple handle types  **may**  be requested
///   for the same allocation as long as they are compatible, as reported by
///   [`get_physical_device_external_image_format_properties_nv`].
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV`
/// - [`handle_types`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`]
///   values
/// # Related
/// - [`VK_NV_external_memory`]
/// - [`ExternalMemoryHandleTypeFlagsNV`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExportMemoryAllocateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExportMemoryAllocateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBitsNV`] specifying one or more memory
    ///handle types that  **may**  be exported.
    ///Multiple handle types  **may**  be requested for the same allocation as long
    ///as they are compatible, as reported by
    ///[`get_physical_device_external_image_format_properties_nv`].
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}
impl<'lt> Default for ExportMemoryAllocateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ExportMemoryAllocateInfoNv,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl<'lt> ExportMemoryAllocateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::handle_types`]
    pub fn handle_types(&self) -> ExternalMemoryHandleTypeFlagsNV {
        self.handle_types
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_types`]
    pub fn handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagsNV {
        &mut self.handle_types
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
    ///Sets the raw value of [`Self::handle_types`]
    pub fn set_handle_types(
        &mut self,
        value: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    ) -> &mut Self {
        self.handle_types = value;
        self
    }
}
