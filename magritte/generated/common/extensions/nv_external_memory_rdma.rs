use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VkRemoteAddressNV")]
pub type RemoteAddressNV = std::ffi::c_void;
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_external_memory_rdma");
