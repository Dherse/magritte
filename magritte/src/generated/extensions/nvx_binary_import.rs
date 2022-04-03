//![VK_NVX_binary_import](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NVX_binary_import.html) - device extension
//!# Description
//!This extension allows applications to import CuBIN binaries and execute
//!them.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Eric Werness [ewerness-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NVX_binary_import]
//!   @ewerness-nv%0A<<Here describe the issue or question you have about the VK_NVX_binary_import
//!   extension>>)
//! - Liam Middlebrook [liam-middlebrook](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NVX_binary_import]
//!   @liam-middlebrook%0A<<Here describe the issue or question you have about the
//!   VK_NVX_binary_import extension>>)
//!# New handles
//! - [`CuFunctionNVX`]
//! - [`CuModuleNVX`]
//!# New functions & commands
//! - [`cmd_cu_launch_kernel_nvx`]
//! - [`create_cu_function_nvx`]
//! - [`create_cu_module_nvx`]
//! - [`destroy_cu_function_nvx`]
//! - [`destroy_cu_module_nvx`]
//!# New structures
//! - [`CuFunctionCreateInfoNVX`]
//! - [`CuLaunchInfoNVX`]
//! - [`CuModuleCreateInfoNVX`]
//!# New constants
//! - [`NVX_BINARY_IMPORT_EXTENSION_NAME`]
//! - [`NVX_BINARY_IMPORT_SPEC_VERSION`]
//! - Extending [`DebugReportObjectTypeEXT`]:  - `VK_DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX_EXT`
//!   - `VK_DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX_EXT`
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_CU_FUNCTION_NVX`  -
//!   `VK_OBJECT_TYPE_CU_MODULE_NVX`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX`  -
//!   `VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX`  - `VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX`
//!# Version History
//! - Revision 1, 2021-04-09 (Eric Werness)  - Internal revisions
//!# Other info
//! * 2021-04-09
//! * - Eric Werness, NVIDIA  - Liam Middlebrook, NVIDIA
//!# Related
//! - [`CuFunctionCreateInfoNVX`]
//! - [`CuFunctionNVX`]
//! - [`CuLaunchInfoNVX`]
//! - [`CuModuleCreateInfoNVX`]
//! - [`CuModuleNVX`]
//! - [`cmd_cu_launch_kernel_nvx`]
//! - [`create_cu_function_nvx`]
//! - [`create_cu_module_nvx`]
//! - [`destroy_cu_function_nvx`]
//! - [`destroy_cu_module_nvx`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{AllocationCallbacks, BaseInStructure, CommandBuffer, Device, StructureType, VulkanResultCodes};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
    os::raw::c_char,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NVX_BINARY_IMPORT_SPEC_VERSION")]
pub const NVX_BINARY_IMPORT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NVX_BINARY_IMPORT_EXTENSION_NAME")]
pub const NVX_BINARY_IMPORT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NVX_binary_import");
///[vkCreateCuModuleNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuModuleNVX.html) - Stub description of vkCreateCuModuleNVX
///# C Specifications
///There is currently no specification language written for this command.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_NVX_binary_import
///VkResult vkCreateCuModuleNVX(
///    VkDevice                                    device,
///    const VkCuModuleCreateInfoNVX*              pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkCuModuleNVX*                              pModule);
///```
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`CuModuleCreateInfoNVX`] structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_module`] **must**  be a valid pointer to a [`CuModuleNVX`] handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`AllocationCallbacks`]
/// - [`CuModuleCreateInfoNVX`]
/// - [`CuModuleNVX`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateCuModuleNVX")]
pub type FNCreateCuModuleNvx = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_create_info: *const CuModuleCreateInfoNVX<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_module: *mut CuModuleNVX,
    ) -> VulkanResultCodes,
>;
///[vkCreateCuFunctionNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuFunctionNVX.html) - Stub description of vkCreateCuFunctionNVX
///# C Specifications
///There is currently no specification language written for this command.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_NVX_binary_import
///VkResult vkCreateCuFunctionNVX(
///    VkDevice                                    device,
///    const VkCuFunctionCreateInfoNVX*            pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkCuFunctionNVX*                            pFunction);
///```
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`CuFunctionCreateInfoNVX`]
///   structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_function`] **must**  be a valid pointer to a [`CuFunctionNVX`] handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`AllocationCallbacks`]
/// - [`CuFunctionCreateInfoNVX`]
/// - [`CuFunctionNVX`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateCuFunctionNVX")]
pub type FNCreateCuFunctionNvx = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_create_info: *const CuFunctionCreateInfoNVX<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_function: *mut CuFunctionNVX,
    ) -> VulkanResultCodes,
>;
///[vkDestroyCuModuleNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuModuleNVX.html) - Stub description of vkDestroyCuModuleNVX
///# C Specifications
///There is currently no specification language written for this command.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_NVX_binary_import
///void vkDestroyCuModuleNVX(
///    VkDevice                                    device,
///    VkCuModuleNVX                               module,
///    const VkAllocationCallbacks*                pAllocator);
///```
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`module`] **must**  be a valid [`CuModuleNVX`] handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`module`] **must**  have been created, allocated, or retrieved from [`device`]
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`AllocationCallbacks`]
/// - [`CuModuleNVX`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDestroyCuModuleNVX")]
pub type FNDestroyCuModuleNvx = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        module: CuModuleNVX,
        p_allocator: *const AllocationCallbacks<'lt>,
    ),
>;
///[vkDestroyCuFunctionNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuFunctionNVX.html) - Stub description of vkDestroyCuFunctionNVX
///# C Specifications
///There is currently no specification language written for this command.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_NVX_binary_import
///void vkDestroyCuFunctionNVX(
///    VkDevice                                    device,
///    VkCuFunctionNVX                             function,
///    const VkAllocationCallbacks*                pAllocator);
///```
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`function`] **must**  be a valid [`CuFunctionNVX`] handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`function`] **must**  have been created, allocated, or retrieved from [`device`]
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`AllocationCallbacks`]
/// - [`CuFunctionNVX`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDestroyCuFunctionNVX")]
pub type FNDestroyCuFunctionNvx = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        function: CuFunctionNVX,
        p_allocator: *const AllocationCallbacks<'lt>,
    ),
>;
///[vkCmdCuLaunchKernelNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCuLaunchKernelNVX.html) - Stub description of vkCmdCuLaunchKernelNVX
///# C Specifications
///There is currently no specification language written for this command.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_NVX_binary_import
///void vkCmdCuLaunchKernelNVX(
///    VkCommandBuffer                             commandBuffer,
///    const VkCuLaunchInfoNVX*                    pLaunchInfo);
///```
///# Description
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_launch_info`] **must**  be a valid pointer to a valid [`CuLaunchInfoNVX`] structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
///
///## Host Synchronization
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`CommandBuffer`]
/// - [`CuLaunchInfoNVX`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdCuLaunchKernelNVX")]
pub type FNCmdCuLaunchKernelNvx = Option<
    for<'lt> unsafe extern "system" fn(command_buffer: CommandBuffer, p_launch_info: *const CuLaunchInfoNVX<'lt>),
>;
///[VkCuModuleCreateInfoNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuModuleCreateInfoNVX.html) - Stub description of VkCuModuleCreateInfoNVX
///# C Specifications
///There is currently no specification language written for this type.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_NVX_binary_import
///typedef struct VkCuModuleCreateInfoNVX {
///    VkStructureType    sType;
///    const void*        pNext;
///    size_t             dataSize;
///    const void*        pData;
///} VkCuModuleCreateInfoNVX;
///```
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX`
/// - [`p_next`] **must**  be `NULL`
/// - [`data`] **must**  be a valid pointer to an array of [`data_size`] bytes
/// - [`data_size`] **must**  be greater than `0`
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`StructureType`]
/// - [`create_cu_module_nvx`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCuModuleCreateInfoNVX")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CuModuleCreateInfoNVX<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] **must**  be `VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX`
    pub s_type: StructureType,
    ///[`p_next`] **must**  be `NULL`
    pub p_next: *const BaseInStructure<'lt>,
    ///[`data_size`] **must**  be greater than `0`
    pub data_size: usize,
    ///[`data`] **must**  be a valid pointer to an array of [`data_size`] bytes
    pub data: *const c_void,
}
impl<'lt> Default for CuModuleCreateInfoNVX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::CuModuleCreateInfoNvx,
            p_next: std::ptr::null(),
            data_size: 0,
            data: std::ptr::null(),
        }
    }
}
impl<'lt> CuModuleCreateInfoNVX<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::data`]
    pub fn data_raw(&self) -> *const c_void {
        self.data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::data`]
    pub fn set_data_raw(&mut self, value: *const c_void) -> &mut Self {
        self.data = value;
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
    ///Gets the value of [`Self::data_size`]
    pub fn data_size(&self) -> usize {
        self.data_size
    }
    ///Gets the value of [`Self::data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn data(&self) -> &[c_void] {
        std::slice::from_raw_parts(self.data, self.data_size as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::data_size`]
    pub fn data_size_mut(&mut self) -> &mut usize {
        &mut self.data_size
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
    ///Sets the raw value of [`Self::data_size`]
    pub fn set_data_size(&mut self, value: usize) -> &mut Self {
        self.data_size = value;
        self
    }
    ///Sets the raw value of [`Self::data`]
    pub fn set_data(&mut self, value: &'lt [std::ffi::c_void]) -> &mut Self {
        let len_ = value.len() as usize;
        let len_ = len_;
        self.data = value.as_ptr();
        self.data_size = len_;
        self
    }
}
///[VkCuFunctionCreateInfoNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuFunctionCreateInfoNVX.html) - Stub description of VkCuFunctionCreateInfoNVX
///# C Specifications
///There is currently no specification language written for this type.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_NVX_binary_import
///typedef struct VkCuFunctionCreateInfoNVX {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkCuModuleNVX      module;
///    const char*        pName;
///} VkCuFunctionCreateInfoNVX;
///```
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX`
/// - [`p_next`] **must**  be `NULL`
/// - [`module`] **must**  be a valid [`CuModuleNVX`] handle
/// - [`name`] **must**  be a null-terminated UTF-8 string
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`CuModuleNVX`]
/// - [`StructureType`]
/// - [`create_cu_function_nvx`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCuFunctionCreateInfoNVX")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CuFunctionCreateInfoNVX<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] **must**  be `VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX`
    pub s_type: StructureType,
    ///[`p_next`] **must**  be `NULL`
    pub p_next: *const BaseInStructure<'lt>,
    ///[`module`] **must**  be a valid [`CuModuleNVX`] handle
    pub module: CuModuleNVX,
    ///[`name`] **must**  be a null-terminated UTF-8 string
    pub name: *const c_char,
}
impl<'lt> Default for CuFunctionCreateInfoNVX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::CuFunctionCreateInfoNvx,
            p_next: std::ptr::null(),
            module: Default::default(),
            name: std::ptr::null(),
        }
    }
}
impl<'lt> CuFunctionCreateInfoNVX<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::name`]
    pub fn name_raw(&self) -> *const c_char {
        self.name
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name_raw(&mut self, value: *const c_char) -> &mut Self {
        self.name = value;
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
    ///Gets the value of [`Self::module`]
    pub fn module(&self) -> CuModuleNVX {
        self.module
    }
    ///Gets the value of [`Self::name`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn name(&self) -> &'lt CStr {
        CStr::from_ptr(self.name)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::module`]
    pub fn module_mut(&mut self) -> &mut CuModuleNVX {
        &mut self.module
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
    ///Sets the raw value of [`Self::module`]
    pub fn set_module(&mut self, value: crate::extensions::nvx_binary_import::CuModuleNVX) -> &mut Self {
        self.module = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name(&mut self, value: *const std::os::raw::c_char) -> &mut Self {
        self.name = value;
        self
    }
}
///[VkCuLaunchInfoNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuLaunchInfoNVX.html) - Stub description of VkCuLaunchInfoNVX
///# C Specifications
///There is currently no specification language written for this type.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_NVX_binary_import
///typedef struct VkCuLaunchInfoNVX {
///    VkStructureType        sType;
///    const void*            pNext;
///    VkCuFunctionNVX        function;
///    uint32_t               gridDimX;
///    uint32_t               gridDimY;
///    uint32_t               gridDimZ;
///    uint32_t               blockDimX;
///    uint32_t               blockDimY;
///    uint32_t               blockDimZ;
///    uint32_t               sharedMemBytes;
///    size_t                 paramCount;
///    const void* const *    pParams;
///    size_t                 extraCount;
///    const void* const *    pExtras;
///} VkCuLaunchInfoNVX;
///```
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX`
/// - [`p_next`] **must**  be `NULL`
/// - [`function`] **must**  be a valid [`CuFunctionNVX`] handle
/// - If [`param_count`] is not `0`, [`params`] **must**  be a valid pointer to an array of
///   [`param_count`] bytes
/// - If [`extra_count`] is not `0`, [`extras`] **must**  be a valid pointer to an array of
///   [`extra_count`] bytes
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`CuFunctionNVX`]
/// - [`StructureType`]
/// - [`cmd_cu_launch_kernel_nvx`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCuLaunchInfoNVX")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CuLaunchInfoNVX<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] **must**  be `VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX`
    pub s_type: StructureType,
    ///[`p_next`] **must**  be `NULL`
    pub p_next: *const BaseInStructure<'lt>,
    ///[`function`] **must**  be a valid [`CuFunctionNVX`] handle
    pub function: CuFunctionNVX,
    ///No documentation found
    pub grid_dim_x: u32,
    ///No documentation found
    pub grid_dim_y: u32,
    ///No documentation found
    pub grid_dim_z: u32,
    ///No documentation found
    pub block_dim_x: u32,
    ///No documentation found
    pub block_dim_y: u32,
    ///No documentation found
    pub block_dim_z: u32,
    ///No documentation found
    pub shared_mem_bytes: u32,
    ///If [`param_count`] is not `0`, [`params`] **must**  be a valid pointer to an array of
    /// [`param_count`] bytes
    pub param_count: usize,
    ///No documentation found
    pub params: *const *const c_void,
    ///If [`extra_count`] is not `0`, [`extras`] **must**  be a valid pointer to an array of
    /// [`extra_count`] bytes
    pub extra_count: usize,
    ///No documentation found
    pub extras: *const *const c_void,
}
impl<'lt> Default for CuLaunchInfoNVX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::CuLaunchInfoNvx,
            p_next: std::ptr::null(),
            function: Default::default(),
            grid_dim_x: 0,
            grid_dim_y: 0,
            grid_dim_z: 0,
            block_dim_x: 0,
            block_dim_y: 0,
            block_dim_z: 0,
            shared_mem_bytes: 0,
            param_count: 0,
            params: std::ptr::null(),
            extra_count: 0,
            extras: std::ptr::null(),
        }
    }
}
impl<'lt> CuLaunchInfoNVX<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::params`]
    pub fn params_raw(&self) -> *const *const c_void {
        self.params
    }
    ///Gets the raw value of [`Self::extras`]
    pub fn extras_raw(&self) -> *const *const c_void {
        self.extras
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::params`]
    pub fn set_params_raw(&mut self, value: *const *const c_void) -> &mut Self {
        self.params = value;
        self
    }
    ///Sets the raw value of [`Self::extras`]
    pub fn set_extras_raw(&mut self, value: *const *const c_void) -> &mut Self {
        self.extras = value;
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
    ///Gets the value of [`Self::function`]
    pub fn function(&self) -> CuFunctionNVX {
        self.function
    }
    ///Gets the value of [`Self::grid_dim_x`]
    pub fn grid_dim_x(&self) -> u32 {
        self.grid_dim_x
    }
    ///Gets the value of [`Self::grid_dim_y`]
    pub fn grid_dim_y(&self) -> u32 {
        self.grid_dim_y
    }
    ///Gets the value of [`Self::grid_dim_z`]
    pub fn grid_dim_z(&self) -> u32 {
        self.grid_dim_z
    }
    ///Gets the value of [`Self::block_dim_x`]
    pub fn block_dim_x(&self) -> u32 {
        self.block_dim_x
    }
    ///Gets the value of [`Self::block_dim_y`]
    pub fn block_dim_y(&self) -> u32 {
        self.block_dim_y
    }
    ///Gets the value of [`Self::block_dim_z`]
    pub fn block_dim_z(&self) -> u32 {
        self.block_dim_z
    }
    ///Gets the value of [`Self::shared_mem_bytes`]
    pub fn shared_mem_bytes(&self) -> u32 {
        self.shared_mem_bytes
    }
    ///Gets the value of [`Self::param_count`]
    pub fn param_count(&self) -> usize {
        self.param_count
    }
    ///Gets the value of [`Self::params`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn params(&self) -> &[*const c_void] {
        std::slice::from_raw_parts(self.params, self.param_count as usize)
    }
    ///Gets the value of [`Self::extra_count`]
    pub fn extra_count(&self) -> usize {
        self.extra_count
    }
    ///Gets the value of [`Self::extras`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn extras(&self) -> &[*const c_void] {
        std::slice::from_raw_parts(self.extras, self.extra_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::function`]
    pub fn function_mut(&mut self) -> &mut CuFunctionNVX {
        &mut self.function
    }
    ///Gets a mutable reference to the value of [`Self::grid_dim_x`]
    pub fn grid_dim_x_mut(&mut self) -> &mut u32 {
        &mut self.grid_dim_x
    }
    ///Gets a mutable reference to the value of [`Self::grid_dim_y`]
    pub fn grid_dim_y_mut(&mut self) -> &mut u32 {
        &mut self.grid_dim_y
    }
    ///Gets a mutable reference to the value of [`Self::grid_dim_z`]
    pub fn grid_dim_z_mut(&mut self) -> &mut u32 {
        &mut self.grid_dim_z
    }
    ///Gets a mutable reference to the value of [`Self::block_dim_x`]
    pub fn block_dim_x_mut(&mut self) -> &mut u32 {
        &mut self.block_dim_x
    }
    ///Gets a mutable reference to the value of [`Self::block_dim_y`]
    pub fn block_dim_y_mut(&mut self) -> &mut u32 {
        &mut self.block_dim_y
    }
    ///Gets a mutable reference to the value of [`Self::block_dim_z`]
    pub fn block_dim_z_mut(&mut self) -> &mut u32 {
        &mut self.block_dim_z
    }
    ///Gets a mutable reference to the value of [`Self::shared_mem_bytes`]
    pub fn shared_mem_bytes_mut(&mut self) -> &mut u32 {
        &mut self.shared_mem_bytes
    }
    ///Gets a mutable reference to the value of [`Self::param_count`]
    pub fn param_count_mut(&mut self) -> &mut usize {
        &mut self.param_count
    }
    ///Gets a mutable reference to the value of [`Self::extra_count`]
    pub fn extra_count_mut(&mut self) -> &mut usize {
        &mut self.extra_count
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
    ///Sets the raw value of [`Self::function`]
    pub fn set_function(&mut self, value: crate::extensions::nvx_binary_import::CuFunctionNVX) -> &mut Self {
        self.function = value;
        self
    }
    ///Sets the raw value of [`Self::grid_dim_x`]
    pub fn set_grid_dim_x(&mut self, value: u32) -> &mut Self {
        self.grid_dim_x = value;
        self
    }
    ///Sets the raw value of [`Self::grid_dim_y`]
    pub fn set_grid_dim_y(&mut self, value: u32) -> &mut Self {
        self.grid_dim_y = value;
        self
    }
    ///Sets the raw value of [`Self::grid_dim_z`]
    pub fn set_grid_dim_z(&mut self, value: u32) -> &mut Self {
        self.grid_dim_z = value;
        self
    }
    ///Sets the raw value of [`Self::block_dim_x`]
    pub fn set_block_dim_x(&mut self, value: u32) -> &mut Self {
        self.block_dim_x = value;
        self
    }
    ///Sets the raw value of [`Self::block_dim_y`]
    pub fn set_block_dim_y(&mut self, value: u32) -> &mut Self {
        self.block_dim_y = value;
        self
    }
    ///Sets the raw value of [`Self::block_dim_z`]
    pub fn set_block_dim_z(&mut self, value: u32) -> &mut Self {
        self.block_dim_z = value;
        self
    }
    ///Sets the raw value of [`Self::shared_mem_bytes`]
    pub fn set_shared_mem_bytes(&mut self, value: u32) -> &mut Self {
        self.shared_mem_bytes = value;
        self
    }
    ///Sets the raw value of [`Self::param_count`]
    pub fn set_param_count(&mut self, value: usize) -> &mut Self {
        self.param_count = value;
        self
    }
    ///Sets the raw value of [`Self::params`]
    pub fn set_params(&mut self, value: &'lt [*const std::ffi::c_void]) -> &mut Self {
        let len_ = value.len() as usize;
        let len_ = len_;
        self.params = value.as_ptr();
        self.param_count = len_;
        self
    }
    ///Sets the raw value of [`Self::extra_count`]
    pub fn set_extra_count(&mut self, value: usize) -> &mut Self {
        self.extra_count = value;
        self
    }
    ///Sets the raw value of [`Self::extras`]
    pub fn set_extras(&mut self, value: &'lt [*const std::ffi::c_void]) -> &mut Self {
        let len_ = value.len() as usize;
        let len_ = len_;
        self.extras = value.as_ptr();
        self.extra_count = len_;
        self
    }
}
///[VkCuModuleNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuModuleNVX.html) - Stub description of VkCuModuleNVX
///# C Specifications
///There is currently no specification language written for this type.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_NVX_binary_import
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkCuModuleNVX)
///```
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`CuFunctionCreateInfoNVX`]
/// - [`create_cu_module_nvx`]
/// - [`destroy_cu_module_nvx`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCuModuleNVX")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct CuModuleNVX(pub u64);
impl CuModuleNVX {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for CuModuleNVX {}
impl Default for CuModuleNVX {
    fn default() -> Self {
        Self::null()
    }
}
///[VkCuFunctionNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuFunctionNVX.html) - Stub description of VkCuFunctionNVX
///# C Specifications
///There is currently no specification language written for this type.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_NVX_binary_import
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkCuFunctionNVX)
///```
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`CuLaunchInfoNVX`]
/// - [`create_cu_function_nvx`]
/// - [`destroy_cu_function_nvx`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCuFunctionNVX")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct CuFunctionNVX(pub u64);
impl CuFunctionNVX {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for CuFunctionNVX {}
impl Default for CuFunctionNVX {
    fn default() -> Self {
        Self::null()
    }
}
///The V-table of [`Device`] for functions from VK_NVX_binary_import
pub struct DeviceNvxBinaryImportVTable {
    ///See [`FNCreateCuModuleNvx`] for more information.
    pub create_cu_module_nvx: FNCreateCuModuleNvx,
    ///See [`FNCreateCuFunctionNvx`] for more information.
    pub create_cu_function_nvx: FNCreateCuFunctionNvx,
    ///See [`FNDestroyCuModuleNvx`] for more information.
    pub destroy_cu_module_nvx: FNDestroyCuModuleNvx,
    ///See [`FNDestroyCuFunctionNvx`] for more information.
    pub destroy_cu_function_nvx: FNDestroyCuFunctionNvx,
    ///See [`FNCmdCuLaunchKernelNvx`] for more information.
    pub cmd_cu_launch_kernel_nvx: FNCmdCuLaunchKernelNvx,
}
impl DeviceNvxBinaryImportVTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Device) -> Self
    where
        F: Fn(Device, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            create_cu_module_nvx: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateCuModuleNVX")))
            },
            create_cu_function_nvx: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateCuFunctionNVX")))
            },
            destroy_cu_module_nvx: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkDestroyCuModuleNVX")))
            },
            destroy_cu_function_nvx: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkDestroyCuFunctionNVX")))
            },
            cmd_cu_launch_kernel_nvx: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdCuLaunchKernelNVX")))
            },
        }
    }
    ///Gets [`Self::create_cu_module_nvx`]. See [`FNCreateCuModuleNvx`] for more information.
    pub fn create_cu_module_nvx(&self) -> FNCreateCuModuleNvx {
        self.create_cu_module_nvx
    }
    ///Gets [`Self::create_cu_function_nvx`]. See [`FNCreateCuFunctionNvx`] for more information.
    pub fn create_cu_function_nvx(&self) -> FNCreateCuFunctionNvx {
        self.create_cu_function_nvx
    }
    ///Gets [`Self::destroy_cu_module_nvx`]. See [`FNDestroyCuModuleNvx`] for more information.
    pub fn destroy_cu_module_nvx(&self) -> FNDestroyCuModuleNvx {
        self.destroy_cu_module_nvx
    }
    ///Gets [`Self::destroy_cu_function_nvx`]. See [`FNDestroyCuFunctionNvx`] for more information.
    pub fn destroy_cu_function_nvx(&self) -> FNDestroyCuFunctionNvx {
        self.destroy_cu_function_nvx
    }
    ///Gets [`Self::cmd_cu_launch_kernel_nvx`]. See [`FNCmdCuLaunchKernelNvx`] for more
    /// information.
    pub fn cmd_cu_launch_kernel_nvx(&self) -> FNCmdCuLaunchKernelNvx {
        self.cmd_cu_launch_kernel_nvx
    }
}
