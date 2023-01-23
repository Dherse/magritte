//!# [VK_NVX_binary_import](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NVX_binary_import.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nvx_binary_import/VK_NVX_binary_import.md")]
use crate::{
    cstr,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, CommandBuffer, Device, StructureType, VulkanResultCodes},
};
use std::ffi::{c_char, CStr};
///# [VkCuModuleCreateInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuModuleCreateInfoNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_binary_import/VkCuModuleCreateInfoNVX.md")]
#[doc(alias = "VkCuModuleCreateInfoNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CuModuleCreateInfoNVX {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "dataSize")]
    data_size: usize,
    #[doc(alias = "pData")]
    data: *const std::ffi::c_void,
}
///# [VkCuFunctionCreateInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuFunctionCreateInfoNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_binary_import/VkCuFunctionCreateInfoNVX.md")]
#[doc(alias = "VkCuFunctionCreateInfoNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CuFunctionCreateInfoNVX {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    module: CuModuleNVX,
    #[doc(alias = "pName")]
    name: *const c_char,
}
///# [VkCuLaunchInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuLaunchInfoNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_binary_import/VkCuLaunchInfoNVX.md")]
#[doc(alias = "VkCuLaunchInfoNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CuLaunchInfoNVX {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    function: CuFunctionNVX,
    #[doc(alias = "gridDimX")]
    grid_dim_x: u32,
    #[doc(alias = "gridDimY")]
    grid_dim_y: u32,
    #[doc(alias = "gridDimZ")]
    grid_dim_z: u32,
    #[doc(alias = "blockDimX")]
    block_dim_x: u32,
    #[doc(alias = "blockDimY")]
    block_dim_y: u32,
    #[doc(alias = "blockDimZ")]
    block_dim_z: u32,
    #[doc(alias = "sharedMemBytes")]
    shared_mem_bytes: u32,
    #[doc(alias = "paramCount")]
    param_count: usize,
    #[doc(alias = "pParams")]
    params: *const *const std::ffi::c_void,
    #[doc(alias = "extraCount")]
    extra_count: usize,
    #[doc(alias = "pExtras")]
    extras: *const *const std::ffi::c_void,
}
///# [VkCuModuleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuModuleNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_binary_import/VkCuModuleNVX.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkCuModuleNVX")]
#[repr(transparent)]
pub struct CuModuleNVX(u64);
impl CuModuleNVX {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for CuModuleNVX {
    fn default() -> Self {
        Self::null()
    }
}
///# [VkCuFunctionNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuFunctionNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_binary_import/VkCuFunctionNVX.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkCuFunctionNVX")]
#[repr(transparent)]
pub struct CuFunctionNVX(u64);
impl CuFunctionNVX {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for CuFunctionNVX {
    fn default() -> Self {
        Self::null()
    }
}
#[doc(alias = "VK_NVX_BINARY_IMPORT_SPEC_VERSION")]
pub const NVX_BINARY_IMPORT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NVX_BINARY_IMPORT_EXTENSION_NAME")]
pub const NVX_BINARY_IMPORT_EXTENSION_NAME: &'static CStr = cstr!("VK_NVX_binary_import");
///# [vkCreateCuModuleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCuModuleNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_binary_import/vkCreateCuModuleNVX.md")]
#[doc(alias = "vkCreateCuModuleNVX")]
pub type FNCreateCuModuleNvx = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuModuleCreateInfoNVX,
    p_allocator: *const AllocationCallbacks,
    p_module: *mut CuModuleNVX,
) -> VulkanResultCodes;
///# [vkCreateCuFunctionNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCuFunctionNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_binary_import/vkCreateCuFunctionNVX.md")]
#[doc(alias = "vkCreateCuFunctionNVX")]
pub type FNCreateCuFunctionNvx = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuFunctionCreateInfoNVX,
    p_allocator: *const AllocationCallbacks,
    p_function: *mut CuFunctionNVX,
) -> VulkanResultCodes;
///# [vkDestroyCuModuleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCuModuleNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_binary_import/vkDestroyCuModuleNVX.md")]
#[doc(alias = "vkDestroyCuModuleNVX")]
pub type FNDestroyCuModuleNvx =
    unsafe extern "system" fn(device: Device, module: CuModuleNVX, p_allocator: *const AllocationCallbacks);
///# [vkDestroyCuFunctionNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCuFunctionNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_binary_import/vkDestroyCuFunctionNVX.md")]
#[doc(alias = "vkDestroyCuFunctionNVX")]
pub type FNDestroyCuFunctionNvx =
    unsafe extern "system" fn(device: Device, function: CuFunctionNVX, p_allocator: *const AllocationCallbacks);
///# [vkCmdCuLaunchKernelNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCuLaunchKernelNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_binary_import/vkCmdCuLaunchKernelNVX.md")]
#[doc(alias = "vkCmdCuLaunchKernelNVX")]
pub type FNCmdCuLaunchKernelNvx =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_launch_info: *const CuLaunchInfoNVX);
