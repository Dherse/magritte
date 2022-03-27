use crate::vulkan1_0::{BaseInStructure, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VALIDATION_CACHE_SPEC_VERSION")]
pub const EXT_VALIDATION_CACHE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VALIDATION_CACHE_EXTENSION_NAME")]
pub const EXT_VALIDATION_CACHE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_validation_cache");
///[VkValidationCacheHeaderVersionEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheHeaderVersionEXT.html) - Encode validation cache version
///# C Specifications
///Possible values of the second group of four bytes in the header returned by
///[`GetValidationCacheDataEXT`], encoding the validation cache version,
///are:
///```c
///// Provided by VK_EXT_validation_cache
///typedef enum VkValidationCacheHeaderVersionEXT {
///    VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT = 1,
///} VkValidationCacheHeaderVersionEXT;
///```
///# Description
/// - [`ValidationCacheHeaderVersionOneExt`] specifies version one of the validation cache.
///# Related
/// - [`VK_EXT_validation_cache`]
/// - [`CreateValidationCacheEXT`]
/// - [`GetValidationCacheDataEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkValidationCacheHeaderVersionEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ValidationCacheHeaderVersionEXT {
    #[doc(hidden)]
    Empty = 0,
    ///[`ValidationCacheHeaderVersionOneExt`] specifies version one
    ///of the validation cache.
    ValidationCacheHeaderVersionOneExt = 1,
}
impl const Default for ValidationCacheHeaderVersionEXT {
    fn default() -> Self {
        Empty
    }
}
impl ValidationCacheHeaderVersionEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkValidationCacheCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheCreateInfoEXT.html) - Structure specifying parameters of a newly created validation cache
///# C Specifications
///The [`ValidationCacheCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_validation_cache
///typedef struct VkValidationCacheCreateInfoEXT {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkValidationCacheCreateFlagsEXT    flags;
///    size_t                             initialDataSize;
///    const void*                        pInitialData;
///} VkValidationCacheCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`initial_data_size`] is the number of bytes in [`initial_data`]. If [`initial_data_size`] is
///   zero, the validation cache will initially be empty.
/// - [`initial_data`] is a pointer to previously retrieved validation cache data. If the validation
///   cache data is incompatible (as defined below) with the device, the validation cache will be
///   initially empty. If [`initial_data_size`] is zero, [`initial_data`] is ignored.
///# Description
///Valid Usage
/// - If [`initial_data_size`] is not `0`, it **must** be equal to the size of [`initial_data`], as
///   returned by [`GetValidationCacheDataEXT`] when [`initial_data`] was originally retrieved
/// - If [`initial_data_size`] is not `0`, [`initial_data`]**must** have been retrieved from a
///   previous call to [`GetValidationCacheDataEXT`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
/// - If [`initial_data_size`] is not `0`, [`initial_data`]**must** be a valid pointer to an array
///   of [`initial_data_size`] bytes
///# Related
/// - [`VK_EXT_validation_cache`]
/// - [`StructureType`]
/// - [`ValidationCacheCreateFlagsEXT`]
/// - [`CreateValidationCacheEXT`]
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
pub struct ValidationCacheCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: ValidationCacheCreateFlagsEXT,
    ///[`initial_data_size`] is the number of bytes in [`initial_data`].
    ///If [`initial_data_size`] is zero, the validation cache will initially be
    ///empty.
    initial_data_size: usize,
    ///[`initial_data`] is a pointer to previously retrieved validation cache
    ///data.
    ///If the validation cache data is incompatible (as defined below) with the
    ///device, the validation cache will be initially empty.
    ///If [`initial_data_size`] is zero, [`initial_data`] is ignored.
    initial_data: *const c_void,
}
impl<'lt> Default for ValidationCacheCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            initial_data_size: 0,
            initial_data: std::ptr::null(),
        }
    }
}
impl<'lt> ValidationCacheCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::initial_data_size`]
    pub fn initial_data_size_raw(&self) -> usize {
        self.initial_data_size
    }
    ///Gets the raw value of [`Self::initial_data`]
    pub fn initial_data_raw(&self) -> *const c_void {
        self.initial_data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::initial_data_size`]
    pub fn set_initial_data_size_raw(&mut self, value: usize) -> &mut Self {
        self.initial_data_size = value;
        self
    }
    ///Sets the raw value of [`Self::initial_data`]
    pub fn set_initial_data_raw(&mut self, value: *const c_void) -> &mut Self {
        self.initial_data = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> ValidationCacheCreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::initial_data_size`]
    pub fn initial_data_size(&self) -> usize {
        self.initial_data_size
    }
    ///Gets the value of [`Self::initial_data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn initial_data(&self) -> &[c_void] {
        std::slice::from_raw_parts(self.initial_data, self.initial_data_size as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ValidationCacheCreateFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::initial_data_size`]
    pub fn initial_data_size_mut(&mut self) -> &mut usize {
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
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ext_validation_cache::ValidationCacheCreateFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::initial_data_size`]
    pub fn set_initial_data_size(&mut self, value: usize) -> &mut Self {
        self.initial_data_size = value;
        self
    }
    ///Sets the raw value of [`Self::initial_data`]
    pub fn set_initial_data(&mut self, value: &'lt [std::ffi::c_void]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.initial_data = value.as_ptr();
        self.initial_data_size = len_;
        self
    }
}
///[VkShaderModuleValidationCacheCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderModuleValidationCacheCreateInfoEXT.html) - Specify validation cache to use during shader module creation
///# C Specifications
///To use a [`ValidationCacheEXT`] to cache shader validation results, add
///a [`ShaderModuleValidationCacheCreateInfoEXT`] structure to the
///[`p_next`] chain of the [`ShaderModuleCreateInfo`] structure,
///specifying the cache object to use.The [`ShaderModuleValidationCacheCreateInfoEXT`] struct is
/// defined as:
///```c
///// Provided by VK_EXT_validation_cache
///typedef struct VkShaderModuleValidationCacheCreateInfoEXT {
///    VkStructureType         sType;
///    const void*             pNext;
///    VkValidationCacheEXT    validationCache;
///} VkShaderModuleValidationCacheCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`validation_cache`] is the validation cache object from which the results of prior validation
///   attempts will be written, and to which new validation results for this [`ShaderModule`] will
///   be written (if not already present).
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT`
/// - [`validation_cache`]**must** be a valid [`ValidationCacheEXT`] handle
///# Related
/// - [`VK_EXT_validation_cache`]
/// - [`StructureType`]
/// - [`ValidationCacheEXT`]
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
pub struct ShaderModuleValidationCacheCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`validation_cache`] is the validation cache object from which the
    ///results of prior validation attempts will be written, and to which new
    ///validation results for this [`ShaderModule`] will be written (if not
    ///already present).
    validation_cache: ValidationCacheEXT,
}
impl<'lt> Default for ShaderModuleValidationCacheCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            validation_cache: Default::default(),
        }
    }
}
impl<'lt> ShaderModuleValidationCacheCreateInfoEXT<'lt> {
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
    ///Gets the value of [`Self::validation_cache`]
    pub fn validation_cache(&self) -> ValidationCacheEXT {
        self.validation_cache
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::validation_cache`]
    pub fn validation_cache_mut(&mut self) -> &mut ValidationCacheEXT {
        &mut self.validation_cache
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
    ///Sets the raw value of [`Self::validation_cache`]
    pub fn set_validation_cache(
        &mut self,
        value: crate::extensions::ext_validation_cache::ValidationCacheEXT,
    ) -> &mut Self {
        self.validation_cache = value;
        self
    }
}
///[VkValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheEXT.html) - Opaque handle to a validation cache object
///# C Specifications
///Validation cache objects allow the result of internal validation to be
///reused, both within a single application run and between multiple runs.
///Reuse within a single run is achieved by passing the same validation cache
///object when creating supported Vulkan objects.
///Reuse across runs of an application is achieved by retrieving validation
///cache contents in one run of an application, saving the contents, and using
///them to preinitialize a validation cache on a subsequent run.
///The contents of the validation cache objects are managed by the validation
///layers.
///Applications **can** manage the host memory consumed by a validation cache
///object and control the amount of data retrieved from a validation cache
///object.Validation cache objects are represented by [`ValidationCacheEXT`]
///handles:
///```c
///// Provided by VK_EXT_validation_cache
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkValidationCacheEXT)
///```
///# Related
/// - [`VK_EXT_validation_cache`]
/// - [`ShaderModuleValidationCacheCreateInfoEXT`]
/// - [`CreateValidationCacheEXT`]
/// - [`DestroyValidationCacheEXT`]
/// - [`GetValidationCacheDataEXT`]
/// - [`MergeValidationCachesEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct ValidationCacheEXT(pub u64);
impl ValidationCacheEXT {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub const fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for ValidationCacheEXT {}
impl Default for ValidationCacheEXT {
    fn default() -> Self {
        Self::default()
    }
}
impl std::fmt::Pointer for ValidationCacheEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
impl std::fmt::Debug for ValidationCacheEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
