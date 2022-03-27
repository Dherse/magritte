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
/// - [`data`]**must** be a valid pointer to an array of [`data_size`] bytes
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CuModuleCreateInfoNVX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX`
    s_type: StructureType,
    ///[`p_next`]**must** be `NULL`
    p_next: *const BaseInStructure<'lt>,
    ///[`data_size`]**must** be greater than `0`
    data_size: usize,
    ///[`data`]**must** be a valid pointer to an array of [`data_size`] bytes
    data: *const c_void,
}
impl<'lt> Default for CuModuleCreateInfoNVX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    ///Gets the raw value of [`Self::data_size`]
    pub fn data_size_raw(&self) -> usize {
        self.data_size
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
    ///Sets the raw value of [`Self::data_size`]
    pub fn set_data_size_raw(&mut self, value: usize) -> &mut Self {
        self.data_size = value;
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
        &mut getter
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
        let len_ = value.len() as u32;
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX`
/// - [`p_next`]**must** be `NULL`
/// - [`module`]**must** be a valid [`CuModuleNVX`] handle
/// - [`name`]**must** be a null-terminated UTF-8 string
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CuFunctionCreateInfoNVX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX`
    s_type: StructureType,
    ///[`p_next`]**must** be `NULL`
    p_next: *const BaseInStructure<'lt>,
    ///[`module`]**must** be a valid [`CuModuleNVX`] handle
    module: CuModuleNVX,
    ///[`name`]**must** be a null-terminated UTF-8 string
    name: &'lt CStr,
}
impl<'lt> Default for CuFunctionCreateInfoNVX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    pub fn name_raw(&self) -> &'lt CStr {
        self.name
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name_raw(&mut self, value: &'lt CStr) -> &mut Self {
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
        self.name
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
    pub fn set_name(&mut self, value: &'lt std::ffi::CStr) -> &mut Self {
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX`
/// - [`p_next`]**must** be `NULL`
/// - [`function`]**must** be a valid [`CuFunctionNVX`] handle
/// - If [`param_count`] is not `0`, [`params`]**must** be a valid pointer to an array of
///   [`param_count`] bytes
/// - If [`extra_count`] is not `0`, [`extras`]**must** be a valid pointer to an array of
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CuLaunchInfoNVX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX`
    s_type: StructureType,
    ///[`p_next`]**must** be `NULL`
    p_next: *const BaseInStructure<'lt>,
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
    ///If [`param_count`] is not `0`, [`params`]**must** be a valid pointer to an array of
    /// [`param_count`] bytes
    param_count: usize,
    ///No documentation found
    params: *const *const c_void,
    ///If [`extra_count`] is not `0`, [`extras`]**must** be a valid pointer to an array of
    /// [`extra_count`] bytes
    extra_count: usize,
    ///No documentation found
    extras: *const *const c_void,
}
impl<'lt> Default for CuLaunchInfoNVX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    ///Gets the raw value of [`Self::grid_dim_x`]
    pub fn grid_dim_x_raw(&self) -> u32 {
        self.grid_dim_x
    }
    ///Gets the raw value of [`Self::grid_dim_y`]
    pub fn grid_dim_y_raw(&self) -> u32 {
        self.grid_dim_y
    }
    ///Gets the raw value of [`Self::grid_dim_z`]
    pub fn grid_dim_z_raw(&self) -> u32 {
        self.grid_dim_z
    }
    ///Gets the raw value of [`Self::block_dim_x`]
    pub fn block_dim_x_raw(&self) -> u32 {
        self.block_dim_x
    }
    ///Gets the raw value of [`Self::block_dim_y`]
    pub fn block_dim_y_raw(&self) -> u32 {
        self.block_dim_y
    }
    ///Gets the raw value of [`Self::block_dim_z`]
    pub fn block_dim_z_raw(&self) -> u32 {
        self.block_dim_z
    }
    ///Gets the raw value of [`Self::shared_mem_bytes`]
    pub fn shared_mem_bytes_raw(&self) -> u32 {
        self.shared_mem_bytes
    }
    ///Gets the raw value of [`Self::param_count`]
    pub fn param_count_raw(&self) -> usize {
        self.param_count
    }
    ///Gets the raw value of [`Self::params`]
    pub fn params_raw(&self) -> *const *const c_void {
        self.params
    }
    ///Gets the raw value of [`Self::extra_count`]
    pub fn extra_count_raw(&self) -> usize {
        self.extra_count
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
    ///Sets the raw value of [`Self::grid_dim_x`]
    pub fn set_grid_dim_x_raw(&mut self, value: u32) -> &mut Self {
        self.grid_dim_x = value;
        self
    }
    ///Sets the raw value of [`Self::grid_dim_y`]
    pub fn set_grid_dim_y_raw(&mut self, value: u32) -> &mut Self {
        self.grid_dim_y = value;
        self
    }
    ///Sets the raw value of [`Self::grid_dim_z`]
    pub fn set_grid_dim_z_raw(&mut self, value: u32) -> &mut Self {
        self.grid_dim_z = value;
        self
    }
    ///Sets the raw value of [`Self::block_dim_x`]
    pub fn set_block_dim_x_raw(&mut self, value: u32) -> &mut Self {
        self.block_dim_x = value;
        self
    }
    ///Sets the raw value of [`Self::block_dim_y`]
    pub fn set_block_dim_y_raw(&mut self, value: u32) -> &mut Self {
        self.block_dim_y = value;
        self
    }
    ///Sets the raw value of [`Self::block_dim_z`]
    pub fn set_block_dim_z_raw(&mut self, value: u32) -> &mut Self {
        self.block_dim_z = value;
        self
    }
    ///Sets the raw value of [`Self::shared_mem_bytes`]
    pub fn set_shared_mem_bytes_raw(&mut self, value: u32) -> &mut Self {
        self.shared_mem_bytes = value;
        self
    }
    ///Sets the raw value of [`Self::param_count`]
    pub fn set_param_count_raw(&mut self, value: usize) -> &mut Self {
        self.param_count = value;
        self
    }
    ///Sets the raw value of [`Self::params`]
    pub fn set_params_raw(&mut self, value: *const *const c_void) -> &mut Self {
        self.params = value;
        self
    }
    ///Sets the raw value of [`Self::extra_count`]
    pub fn set_extra_count_raw(&mut self, value: usize) -> &mut Self {
        self.extra_count = value;
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
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::grid_dim_y`]
    pub fn grid_dim_y_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::grid_dim_z`]
    pub fn grid_dim_z_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::block_dim_x`]
    pub fn block_dim_x_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::block_dim_y`]
    pub fn block_dim_y_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::block_dim_z`]
    pub fn block_dim_z_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::shared_mem_bytes`]
    pub fn shared_mem_bytes_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::param_count`]
    pub fn param_count_mut(&mut self) -> &mut usize {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::extra_count`]
    pub fn extra_count_mut(&mut self) -> &mut usize {
        &mut getter
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
        let len_ = value.len() as u32;
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
        let len_ = value.len() as u32;
        let len_ = len_;
        self.extras = value.as_ptr();
        self.extra_count = len_;
        self
    }
}
