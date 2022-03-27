use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NVX_BINARY_IMPORT_SPEC_VERSION")]
pub const NVX_BINARY_IMPORT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NVX_BINARY_IMPORT_EXTENSION_NAME")]
pub const NVX_BINARY_IMPORT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NVX_binary_import");
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX`
/// - [`p_next`]**must** be `NULL`
/// - [`p_data`]**must** be a valid pointer to an array of [`data_size`] bytes
/// - [`data_size`]**must** be greater than `0`
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`StructureType`]
/// - [`CreateCuModuleNVX`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CuModuleCreateInfoNVX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX`
    s_type: StructureType,
    ///[`p_next`]**must** be `NULL`
    p_next: *mut BaseInStructure<'lt>,
    ///[`data_size`]**must** be greater than `0`
    data_size: usize,
    ///[`p_data`]**must** be a valid pointer to an array of [`data_size`] bytes
    p_data: *mut c_void,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX`
/// - [`p_next`]**must** be `NULL`
/// - [`module`]**must** be a valid [`CuModuleNVX`] handle
/// - [`p_name`]**must** be a null-terminated UTF-8 string
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`CuModuleNVX`]
/// - [`StructureType`]
/// - [`CreateCuFunctionNVX`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CuFunctionCreateInfoNVX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX`
    s_type: StructureType,
    ///[`p_next`]**must** be `NULL`
    p_next: *mut BaseInStructure<'lt>,
    ///[`module`]**must** be a valid [`CuModuleNVX`] handle
    module: CuModuleNVX,
    ///[`p_name`]**must** be a null-terminated UTF-8 string
    p_name: &'lt CStr,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX`
/// - [`p_next`]**must** be `NULL`
/// - [`function`]**must** be a valid [`CuFunctionNVX`] handle
/// - If [`param_count`] is not `0`, [`p_params`]**must** be a valid pointer to an array of
///   [`param_count`] bytes
/// - If [`extra_count`] is not `0`, [`p_extras`]**must** be a valid pointer to an array of
///   [`extra_count`] bytes
///# Related
/// - [`VK_NVX_binary_import`]
/// - [`CuFunctionNVX`]
/// - [`StructureType`]
/// - [`CmdCuLaunchKernelNVX`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CuLaunchInfoNVX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX`
    s_type: StructureType,
    ///[`p_next`]**must** be `NULL`
    p_next: *mut BaseInStructure<'lt>,
    ///[`function`]**must** be a valid [`CuFunctionNVX`] handle
    function: CuFunctionNVX,
    ///No documentation found
    grid_dim_x: u32,
    ///No documentation found
    grid_dim_y: u32,
    ///No documentation found
    grid_dim_z: u32,
    ///No documentation found
    block_dim_x: u32,
    ///No documentation found
    block_dim_y: u32,
    ///No documentation found
    block_dim_z: u32,
    ///No documentation found
    shared_mem_bytes: u32,
    ///If [`param_count`] is not `0`, [`p_params`]**must** be a valid pointer to an array of
    /// [`param_count`] bytes
    param_count: usize,
    ///No documentation found
    p_params: *mut *mut c_void,
    ///If [`extra_count`] is not `0`, [`p_extras`]**must** be a valid pointer to an array of
    /// [`extra_count`] bytes
    extra_count: usize,
    ///No documentation found
    p_extras: *mut *mut c_void,
}
