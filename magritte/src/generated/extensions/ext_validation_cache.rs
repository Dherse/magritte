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
/// - [`initial_data_size`] is the number of bytes in [`p_initial_data`]. If [`initial_data_size`]
///   is zero, the validation cache will initially be empty.
/// - [`p_initial_data`] is a pointer to previously retrieved validation cache data. If the
///   validation cache data is incompatible (as defined below) with the device, the validation cache
///   will be initially empty. If [`initial_data_size`] is zero, [`p_initial_data`] is ignored.
///# Description
///Valid Usage
/// - If [`initial_data_size`] is not `0`, it **must** be equal to the size of [`p_initial_data`],
///   as returned by [`GetValidationCacheDataEXT`] when [`p_initial_data`] was originally retrieved
/// - If [`initial_data_size`] is not `0`, [`p_initial_data`]**must** have been retrieved from a
///   previous call to [`GetValidationCacheDataEXT`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
/// - If [`initial_data_size`] is not `0`, [`p_initial_data`]**must** be a valid pointer to an array
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ValidationCacheCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: ValidationCacheCreateFlagsEXT,
    ///[`initial_data_size`] is the number of bytes in [`p_initial_data`].
    ///If [`initial_data_size`] is zero, the validation cache will initially be
    ///empty.
    initial_data_size: usize,
    ///[`p_initial_data`] is a pointer to previously retrieved validation cache
    ///data.
    ///If the validation cache data is incompatible (as defined below) with the
    ///device, the validation cache will be initially empty.
    ///If [`initial_data_size`] is zero, [`p_initial_data`] is ignored.
    p_initial_data: *mut c_void,
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ShaderModuleValidationCacheCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`validation_cache`] is the validation cache object from which the
    ///results of prior validation attempts will be written, and to which new
    ///validation results for this [`ShaderModule`] will be written (if not
    ///already present).
    validation_cache: ValidationCacheEXT,
}
