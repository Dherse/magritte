use crate::{
    cstr,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, CommandBuffer, Device, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
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
    name: *const CStr,
}
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
#[doc(alias = "vkCreateCuModuleNVX")]
pub type FNCreateCuModuleNvx = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuModuleCreateInfoNVX,
    p_allocator: *const AllocationCallbacks,
    p_module: *mut CuModuleNVX,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateCuFunctionNVX")]
pub type FNCreateCuFunctionNvx = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuFunctionCreateInfoNVX,
    p_allocator: *const AllocationCallbacks,
    p_function: *mut CuFunctionNVX,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyCuModuleNVX")]
pub type FNDestroyCuModuleNvx =
    unsafe extern "system" fn(device: Device, module: CuModuleNVX, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkDestroyCuFunctionNVX")]
pub type FNDestroyCuFunctionNvx =
    unsafe extern "system" fn(device: Device, function: CuFunctionNVX, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkCmdCuLaunchKernelNVX")]
pub type FNCmdCuLaunchKernelNvx =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_launch_info: *const CuLaunchInfoNVX);
