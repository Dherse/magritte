use crate::native::vulkan1_0::{
    AllocationCallbacks, BaseInStructure, CommandBuffer, Device, StructureType, VulkanResultCodes,
};
use std::ffi::c_char;
#[doc(alias = "VkCuModuleCreateInfoNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CuModuleCreateInfoNVX {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "dataSize")]
    pub data_size: usize,
    #[doc(alias = "pData")]
    pub data: *const std::ffi::c_void,
}
impl Default for CuModuleCreateInfoNVX {
    fn default() -> Self {
        Self {
            s_type: StructureType::CuModuleCreateInfoNvx,
            p_next: unsafe { std::mem::zeroed() },
            data_size: unsafe { std::mem::zeroed() },
            data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCuFunctionCreateInfoNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CuFunctionCreateInfoNVX {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub module: CuModuleNVX,
    #[doc(alias = "pName")]
    pub name: *const c_char,
}
impl Default for CuFunctionCreateInfoNVX {
    fn default() -> Self {
        Self {
            s_type: StructureType::CuFunctionCreateInfoNvx,
            p_next: unsafe { std::mem::zeroed() },
            module: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCuLaunchInfoNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CuLaunchInfoNVX {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub function: CuFunctionNVX,
    #[doc(alias = "gridDimX")]
    pub grid_dim_x: u32,
    #[doc(alias = "gridDimY")]
    pub grid_dim_y: u32,
    #[doc(alias = "gridDimZ")]
    pub grid_dim_z: u32,
    #[doc(alias = "blockDimX")]
    pub block_dim_x: u32,
    #[doc(alias = "blockDimY")]
    pub block_dim_y: u32,
    #[doc(alias = "blockDimZ")]
    pub block_dim_z: u32,
    #[doc(alias = "sharedMemBytes")]
    pub shared_mem_bytes: u32,
    #[doc(alias = "paramCount")]
    pub param_count: usize,
    #[doc(alias = "pParams")]
    pub params: *const *const std::ffi::c_void,
    #[doc(alias = "extraCount")]
    pub extra_count: usize,
    #[doc(alias = "pExtras")]
    pub extras: *const *const std::ffi::c_void,
}
impl Default for CuLaunchInfoNVX {
    fn default() -> Self {
        Self {
            s_type: StructureType::CuLaunchInfoNvx,
            p_next: unsafe { std::mem::zeroed() },
            function: unsafe { std::mem::zeroed() },
            grid_dim_x: unsafe { std::mem::zeroed() },
            grid_dim_y: unsafe { std::mem::zeroed() },
            grid_dim_z: unsafe { std::mem::zeroed() },
            block_dim_x: unsafe { std::mem::zeroed() },
            block_dim_y: unsafe { std::mem::zeroed() },
            block_dim_z: unsafe { std::mem::zeroed() },
            shared_mem_bytes: unsafe { std::mem::zeroed() },
            param_count: unsafe { std::mem::zeroed() },
            params: unsafe { std::mem::zeroed() },
            extra_count: unsafe { std::mem::zeroed() },
            extras: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkCuModuleNVX")]
#[repr(transparent)]
pub struct CuModuleNVX(u64);
impl CuModuleNVX {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for CuModuleNVX {
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
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for CuFunctionNVX {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::extensions::nvx_binary_import::{
    NVX_BINARY_IMPORT_EXTENSION_NAME, NVX_BINARY_IMPORT_SPEC_VERSION,
};
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
