//![VK_EXT_validation_cache](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_cache.html) - device extension
//!# Description
//!This extension provides a mechanism for caching the results of potentially
//!expensive internal validation operations across multiple runs of a Vulkan
//!application.
//!At the core is the [`ValidationCacheEXT`] object type, which is managed
//!similarly to the existing [`PipelineCache`].The new struct
//! [`ShaderModuleValidationCacheCreateInfoEXT`] can be
//!included in the `pNext` chain at [`create_shader_module`] time.
//!It contains a [`ValidationCacheEXT`] to use when validating the
//![`ShaderModule`].
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Cort Stratton [cdwfs](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_validation_cache]
//!   @cdwfs%0A<<Here describe the issue or question you have about the VK_EXT_validation_cache
//!   extension>>)
//!# New handles
//! - [`ValidationCacheEXT`]
//!# New functions & commands
//! - [`create_validation_cache_ext`]
//! - [`destroy_validation_cache_ext`]
//! - [`get_validation_cache_data_ext`]
//! - [`merge_validation_caches_ext`]
//!# New structures
//! - [`ValidationCacheCreateInfoEXT`]
//! - Extending [`ShaderModuleCreateInfo`]:  - [`ShaderModuleValidationCacheCreateInfoEXT`]
//!# New enums
//! - [`ValidationCacheHeaderVersionEXT`]
//!# New bitmasks
//! - [`ValidationCacheCreateFlagsEXT`]
//!# New constants
//! - [`EXT_VALIDATION_CACHE_EXTENSION_NAME`]
//! - [`EXT_VALIDATION_CACHE_SPEC_VERSION`]
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_VALIDATION_CACHE_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2017-08-29 (Cort Stratton)  - Initial draft
//!# Other info
//! * 2017-08-29
//! * No known IP claims.
//! * - Cort Stratton, Google  - Chris Forbes, Google
//!# Related
//! - [`ShaderModuleValidationCacheCreateInfoEXT`]
//! - [`ValidationCacheCreateFlagsEXT`]
//! - [`ValidationCacheCreateInfoEXT`]
//! - [`ValidationCacheEXT`]
//! - [`ValidationCacheHeaderVersionEXT`]
//! - [`create_validation_cache_ext`]
//! - [`destroy_validation_cache_ext`]
//! - [`get_validation_cache_data_ext`]
//! - [`merge_validation_caches_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    entry::Entry,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Device, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
    AsRaw, Handle, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VALIDATION_CACHE_SPEC_VERSION")]
pub const EXT_VALIDATION_CACHE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VALIDATION_CACHE_EXTENSION_NAME")]
pub const EXT_VALIDATION_CACHE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_validation_cache");
///[vkCreateValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateValidationCacheEXT.html) - Creates a new validation cache
///# C Specifications
///To create validation cache objects, call:
///```c
///// Provided by VK_EXT_validation_cache
///VkResult vkCreateValidationCacheEXT(
///    VkDevice                                    device,
///    const VkValidationCacheCreateInfoEXT*       pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkValidationCacheEXT*                       pValidationCache);
///```
/// # Parameters
/// - [`device`] is the logical device that creates the validation cache object.
/// - [`p_create_info`] is a pointer to a [`ValidationCacheCreateInfoEXT`] structure containing the
///   initial parameters for the validation cache object.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// - [`p_validation_cache`] is a pointer to a [`ValidationCacheEXT`] handle in which the resulting
///   validation cache object is returned.
/// # Description
/// Once created, a validation cache  **can**  be passed to the
/// [`create_shader_module`] command by adding this object to the
/// [`ShaderModuleCreateInfo`] structure’s `pNext` chain.
/// If a [`ShaderModuleValidationCacheCreateInfoEXT`] object is included in
/// the [`ShaderModuleCreateInfo::p_next`] chain, and its
/// `validationCache` field is not [`crate::Handle::null`], the implementation
/// will query it for possible reuse opportunities and update it with new
/// content.
/// The use of the validation cache object in these commands is internally
/// synchronized, and the same validation cache object  **can**  be used in multiple
/// threads simultaneously.
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`ValidationCacheCreateInfoEXT`]
///   structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_validation_cache`] **must**  be a valid pointer to a [`ValidationCacheEXT`] handle
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`VK_EXT_validation_cache`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
/// - [`ValidationCacheCreateInfoEXT`]
/// - [`ValidationCacheEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateValidationCacheEXT")]
pub type FNCreateValidationCacheExt = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ValidationCacheCreateInfoEXT<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_validation_cache: *mut ValidationCacheEXT,
    ) -> VulkanResultCodes,
>;
///[vkDestroyValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyValidationCacheEXT.html) - Destroy a validation cache object
///# C Specifications
///To destroy a validation cache, call:
///```c
///// Provided by VK_EXT_validation_cache
///void vkDestroyValidationCacheEXT(
///    VkDevice                                    device,
///    VkValidationCacheEXT                        validationCache,
///    const VkAllocationCallbacks*                pAllocator);
///```
/// # Parameters
/// - [`device`] is the logical device that destroys the validation cache object.
/// - [`validation_cache`] is the handle of the validation cache to destroy.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// # Description
/// ## Valid Usage
/// - If [`AllocationCallbacks`] were provided when [`validation_cache`] was created, a compatible
///   set of callbacks  **must**  be provided here
/// - If no [`AllocationCallbacks`] were provided when [`validation_cache`] was created,
///   [`p_allocator`] **must**  be `NULL`
///
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - If [`validation_cache`] is not [`crate::Handle::null`], [`validation_cache`] **must**  be a
///   valid [`ValidationCacheEXT`] handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - If [`validation_cache`] is a valid handle, it  **must**  have been created, allocated, or
///   retrieved from [`device`]
///
/// ## Host Synchronization
/// - Host access to [`validation_cache`] **must**  be externally synchronized
/// # Related
/// - [`VK_EXT_validation_cache`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
/// - [`ValidationCacheEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDestroyValidationCacheEXT")]
pub type FNDestroyValidationCacheExt = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_allocator: *const AllocationCallbacks<'lt>,
    ),
>;
///[vkGetValidationCacheDataEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetValidationCacheDataEXT.html) - Get the data store from a validation cache
///# C Specifications
///Data  **can**  be retrieved from a validation cache object using the command:
///```c
///// Provided by VK_EXT_validation_cache
///VkResult vkGetValidationCacheDataEXT(
///    VkDevice                                    device,
///    VkValidationCacheEXT                        validationCache,
///    size_t*                                     pDataSize,
///    void*                                       pData);
///```
/// # Parameters
/// - [`device`] is the logical device that owns the validation cache.
/// - [`validation_cache`] is the validation cache to retrieve data from.
/// - [`p_data_size`] is a pointer to a value related to the amount of data in the validation cache,
///   as described below.
/// - [`p_data`] is either `NULL` or a pointer to a buffer.
/// # Description
/// If [`p_data`] is `NULL`, then the maximum size of the data that  **can**  be
/// retrieved from the validation cache, in bytes, is returned in
/// [`p_data_size`].
/// Otherwise, [`p_data_size`] **must**  point to a variable set by the user to the
/// size of the buffer, in bytes, pointed to by [`p_data`], and on return the
/// variable is overwritten with the amount of data actually written to
/// [`p_data`].
/// If [`p_data_size`] is less than the maximum size that  **can**  be retrieved by
/// the validation cache, at most [`p_data_size`] bytes will be written to
/// [`p_data`], and [`get_validation_cache_data_ext`] will return
/// `VK_INCOMPLETE` instead of `VK_SUCCESS`, to indicate that not all of
/// the validation cache was returned.Any data written to [`p_data`] is valid and  **can**  be
/// provided as the
/// `pInitialData` member of the [`ValidationCacheCreateInfoEXT`]
/// structure passed to [`create_validation_cache_ext`].Two calls to
/// [`get_validation_cache_data_ext`] with the same parameters
/// **must**  retrieve the same data unless a command that modifies the contents of
/// the cache is called between them.Applications  **can**  store the data retrieved from the
/// validation cache, and
/// use these data, possibly in a future run of the application, to populate new
/// validation cache objects.
/// The results of validation, however,  **may**  depend on the vendor ID, device ID,
/// driver version, and other details of the device.
/// To enable applications to detect when previously retrieved data is
/// incompatible with the device, the initial bytes written to [`p_data`] **must**
/// be a header consisting of the following members:The first four bytes encode the length of the
/// entire validation cache
/// header, in bytes.
/// This value includes all fields in the header including the validation cache
/// version field and the size of the length field.The next four bytes encode the validation cache
/// version, as described for
/// [`ValidationCacheHeaderVersionEXT`].
/// A consumer of the validation cache  **should**  use the cache version to
/// interpret the remainder of the cache header.If [`p_data_size`] is less than what is necessary to
/// store this header,
/// nothing will be written to [`p_data`] and zero will be written to
/// [`p_data_size`].
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`validation_cache`] **must**  be a valid [`ValidationCacheEXT`] handle
/// - [`p_data_size`] **must**  be a valid pointer to a `size_t` value
/// - If the value referenced by [`p_data_size`] is not `0`, and [`p_data`] is not `NULL`,
///   [`p_data`] **must**  be a valid pointer to an array of [`p_data_size`] bytes
/// - [`validation_cache`] **must**  have been created, allocated, or retrieved from [`device`]
///
/// ## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_EXT_validation_cache`]
/// - [`Device`]
/// - [`ValidationCacheEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetValidationCacheDataEXT")]
pub type FNGetValidationCacheDataExt = Option<
    unsafe extern "system" fn(
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> VulkanResultCodes,
>;
///[vkMergeValidationCachesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergeValidationCachesEXT.html) - Combine the data stores of validation caches
///# C Specifications
///Validation cache objects  **can**  be merged using the command:
///```c
///// Provided by VK_EXT_validation_cache
///VkResult vkMergeValidationCachesEXT(
///    VkDevice                                    device,
///    VkValidationCacheEXT                        dstCache,
///    uint32_t                                    srcCacheCount,
///    const VkValidationCacheEXT*                 pSrcCaches);
///```
/// # Parameters
/// - [`device`] is the logical device that owns the validation cache objects.
/// - [`dst_cache`] is the handle of the validation cache to merge results into.
/// - [`src_cache_count`] is the length of the [`p_src_caches`] array.
/// - [`p_src_caches`] is a pointer to an array of validation cache handles, which will be merged
///   into [`dst_cache`]. The previous contents of [`dst_cache`] are included after the merge.
/// # Description
/// ## Valid Usage
/// - [`dst_cache`] **must**  not appear in the list of source caches
///
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`dst_cache`] **must**  be a valid [`ValidationCacheEXT`] handle
/// - [`p_src_caches`] **must**  be a valid pointer to an array of [`src_cache_count`] valid
///   [`ValidationCacheEXT`] handles
/// - [`src_cache_count`] **must**  be greater than `0`
/// - [`dst_cache`] **must**  have been created, allocated, or retrieved from [`device`]
/// - Each element of [`p_src_caches`] **must**  have been created, allocated, or retrieved from
///   [`device`]
///
/// ## Host Synchronization
/// - Host access to [`dst_cache`] **must**  be externally synchronized
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_EXT_validation_cache`]
/// - [`Device`]
/// - [`ValidationCacheEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkMergeValidationCachesEXT")]
pub type FNMergeValidationCachesExt = Option<
    unsafe extern "system" fn(
        device: Device,
        dst_cache: ValidationCacheEXT,
        src_cache_count: u32,
        p_src_caches: *const ValidationCacheEXT,
    ) -> VulkanResultCodes,
>;
///[VkValidationCacheHeaderVersionEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheHeaderVersionEXT.html) - Encode validation cache version
///# C Specifications
///Possible values of the second group of four bytes in the header returned by
///[`get_validation_cache_data_ext`], encoding the validation cache version,
///are:
///```c
///// Provided by VK_EXT_validation_cache
///typedef enum VkValidationCacheHeaderVersionEXT {
///    VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT = 1,
///} VkValidationCacheHeaderVersionEXT;
///```
/// # Description
/// - [`ONE`] specifies version one of the validation cache.
/// # Related
/// - [`VK_EXT_validation_cache`]
/// - [`create_validation_cache_ext`]
/// - [`get_validation_cache_data_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkValidationCacheHeaderVersionEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ValidationCacheHeaderVersionEXT(i32);
impl const Default for ValidationCacheHeaderVersionEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ValidationCacheHeaderVersionEXT {
    ///[`ONE`] specifies version one
    ///of the validation cache.
    pub const ONE: Self = Self(1);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for ValidationCacheHeaderVersionEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(ValidationCacheHeaderVersionEXT))
            .field(match *self {
                Self::ONE => &"ONE",
                other => unreachable!(
                    concat!(
                        "invalid value for",
                        stringify!(ValidationCacheHeaderVersionEXT),
                        ": {:?}"
                    ),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for ValidationCacheHeaderVersionEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::ONE => &"ONE",
            other => unreachable!(
                concat!(
                    "invalid value for",
                    stringify!(ValidationCacheHeaderVersionEXT),
                    ": {:?}"
                ),
                other
            ),
        })
    }
}
///[VkValidationCacheCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheCreateFlagsEXT.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_EXT_validation_cache
///typedef VkFlags VkValidationCacheCreateFlagsEXT;
///```
/// # Related
/// - [`VK_EXT_validation_cache`]
/// - [`ValidationCacheCreateInfoEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ValidationCacheCreateFlagsEXT(u32);
impl const Default for ValidationCacheCreateFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ValidationCacheCreateFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(ValidationCacheCreateFlagsEXT))
            .field(&self.0)
            .finish()
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`initial_data_size`] is the number of bytes in [`initial_data`]. If [`initial_data_size`] is
///   zero, the validation cache will initially be empty.
/// - [`initial_data`] is a pointer to previously retrieved validation cache data. If the validation
///   cache data is incompatible (as defined below) with the device, the validation cache will be
///   initially empty. If [`initial_data_size`] is zero, [`initial_data`] is ignored.
/// # Description
/// ## Valid Usage
/// - If [`initial_data_size`] is not `0`, it  **must**  be equal to the size of [`initial_data`],
///   as returned by [`get_validation_cache_data_ext`] when [`initial_data`] was originally
///   retrieved
/// - If [`initial_data_size`] is not `0`, [`initial_data`] **must**  have been retrieved from a
///   previous call to [`get_validation_cache_data_ext`]
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// - If [`initial_data_size`] is not `0`, [`initial_data`] **must**  be a valid pointer to an array
///   of [`initial_data_size`] bytes
/// # Related
/// - [`VK_EXT_validation_cache`]
/// - [`StructureType`]
/// - [`ValidationCacheCreateFlagsEXT`]
/// - [`create_validation_cache_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkValidationCacheCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ValidationCacheCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: ValidationCacheCreateFlagsEXT,
    ///[`initial_data_size`] is the number of bytes in [`initial_data`].
    ///If [`initial_data_size`] is zero, the validation cache will initially be
    ///empty.
    pub initial_data_size: usize,
    ///[`initial_data`] is a pointer to previously retrieved validation cache
    ///data.
    ///If the validation cache data is incompatible (as defined below) with the
    ///device, the validation cache will be initially empty.
    ///If [`initial_data_size`] is zero, [`initial_data`] is ignored.
    pub initial_data: *const c_void,
}
impl<'lt> Default for ValidationCacheCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VALIDATION_CACHE_CREATE_INFO_EXT,
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
    ///Gets the raw value of [`Self::initial_data`]
    pub fn initial_data_raw(&self) -> *const c_void {
        self.initial_data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::initial_data`]
    pub fn set_initial_data_raw(mut self, value: *const c_void) -> Self {
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
        &mut self.initial_data_size
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(mut self, value: crate::extensions::ext_validation_cache::ValidationCacheCreateFlagsEXT) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::initial_data_size`]
    pub fn set_initial_data_size(mut self, value: usize) -> Self {
        self.initial_data_size = value;
        self
    }
    ///Sets the value of [`Self::initial_data`]
    pub fn set_initial_data(mut self, value: &'lt [std::ffi::c_void]) -> Self {
        let len_ = value.len() as usize;
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`validation_cache`] is the validation cache object from which the results of prior validation
///   attempts will be written, and to which new validation results for this [`ShaderModule`] will
///   be written (if not already present).
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT`
/// - [`validation_cache`] **must**  be a valid [`ValidationCacheEXT`] handle
/// # Related
/// - [`VK_EXT_validation_cache`]
/// - [`StructureType`]
/// - [`ValidationCacheEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkShaderModuleValidationCacheCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ShaderModuleValidationCacheCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`validation_cache`] is the validation cache object from which the
    ///results of prior validation attempts will be written, and to which new
    ///validation results for this [`ShaderModule`] will be written (if not
    ///already present).
    pub validation_cache: ValidationCacheEXT,
}
impl<'lt> Default for ShaderModuleValidationCacheCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::validation_cache`]
    pub fn set_validation_cache(mut self, value: crate::extensions::ext_validation_cache::ValidationCacheEXT) -> Self {
        self.validation_cache = value;
        self
    }
}
impl Device {
    ///[vkCreateValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateValidationCacheEXT.html) - Creates a new validation cache
    ///# C Specifications
    ///To create validation cache objects, call:
    ///```c
    ///// Provided by VK_EXT_validation_cache
    ///VkResult vkCreateValidationCacheEXT(
    ///    VkDevice                                    device,
    ///    const VkValidationCacheCreateInfoEXT*       pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkValidationCacheEXT*                       pValidationCache);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that creates the validation cache object.
    /// - [`p_create_info`] is a pointer to a [`ValidationCacheCreateInfoEXT`] structure containing
    ///   the initial parameters for the validation cache object.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// - [`p_validation_cache`] is a pointer to a [`ValidationCacheEXT`] handle in which the
    ///   resulting validation cache object is returned.
    /// # Description
    /// Once created, a validation cache  **can**  be passed to the
    /// [`create_shader_module`] command by adding this object to the
    /// [`ShaderModuleCreateInfo`] structure’s `pNext` chain.
    /// If a [`ShaderModuleValidationCacheCreateInfoEXT`] object is included in
    /// the [`ShaderModuleCreateInfo::p_next`] chain, and its
    /// `validationCache` field is not [`crate::Handle::null`], the implementation
    /// will query it for possible reuse opportunities and update it with new
    /// content.
    /// The use of the validation cache object in these commands is internally
    /// synchronized, and the same validation cache object  **can**  be used in multiple
    /// threads simultaneously.
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`ValidationCacheCreateInfoEXT`]
    ///   structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_validation_cache`] **must**  be a valid pointer to a [`ValidationCacheEXT`] handle
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`VK_EXT_validation_cache`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    /// - [`ValidationCacheCreateInfoEXT`]
    /// - [`ValidationCacheEXT`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateValidationCacheEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_validation_cache_ext<'lt>(
        self: &Unique<Device>,
        p_create_info: &ValidationCacheCreateInfoEXT<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<ValidationCacheEXT>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_validation_cache()
            .and_then(|vtable| vtable.create_validation_cache_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_validation_cache()
            .and_then(|vtable| vtable.create_validation_cache_ext())
            .unwrap_unchecked();
        let mut p_validation_cache = MaybeUninit::<ValidationCacheEXT>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const ValidationCacheCreateInfoEXT<'lt>,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_validation_cache.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(
                _return,
                Unique::new(self, p_validation_cache.assume_init(), AtomicBool::default()),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkDestroyValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyValidationCacheEXT.html) - Destroy a validation cache object
    ///# C Specifications
    ///To destroy a validation cache, call:
    ///```c
    ///// Provided by VK_EXT_validation_cache
    ///void vkDestroyValidationCacheEXT(
    ///    VkDevice                                    device,
    ///    VkValidationCacheEXT                        validationCache,
    ///    const VkAllocationCallbacks*                pAllocator);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that destroys the validation cache object.
    /// - [`validation_cache`] is the handle of the validation cache to destroy.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// # Description
    /// ## Valid Usage
    /// - If [`AllocationCallbacks`] were provided when [`validation_cache`] was created, a
    ///   compatible set of callbacks  **must**  be provided here
    /// - If no [`AllocationCallbacks`] were provided when [`validation_cache`] was created,
    ///   [`p_allocator`] **must**  be `NULL`
    ///
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - If [`validation_cache`] is not [`crate::Handle::null`], [`validation_cache`] **must**  be
    ///   a valid [`ValidationCacheEXT`] handle
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - If [`validation_cache`] is a valid handle, it  **must**  have been created, allocated, or
    ///   retrieved from [`device`]
    ///
    /// ## Host Synchronization
    /// - Host access to [`validation_cache`] **must**  be externally synchronized
    /// # Related
    /// - [`VK_EXT_validation_cache`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    /// - [`ValidationCacheEXT`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkDestroyValidationCacheEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn destroy_validation_cache_ext<'lt>(
        self: &Unique<Device>,
        validation_cache: Option<ValidationCacheEXT>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_validation_cache()
            .and_then(|vtable| vtable.destroy_validation_cache_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_validation_cache()
            .and_then(|vtable| vtable.destroy_validation_cache_ext())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            validation_cache.unwrap_or_default(),
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl Device {
    ///[vkGetValidationCacheDataEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetValidationCacheDataEXT.html) - Get the data store from a validation cache
    ///# C Specifications
    ///Data  **can**  be retrieved from a validation cache object using the command:
    ///```c
    ///// Provided by VK_EXT_validation_cache
    ///VkResult vkGetValidationCacheDataEXT(
    ///    VkDevice                                    device,
    ///    VkValidationCacheEXT                        validationCache,
    ///    size_t*                                     pDataSize,
    ///    void*                                       pData);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that owns the validation cache.
    /// - [`validation_cache`] is the validation cache to retrieve data from.
    /// - [`p_data_size`] is a pointer to a value related to the amount of data in the validation
    ///   cache, as described below.
    /// - [`p_data`] is either `NULL` or a pointer to a buffer.
    /// # Description
    /// If [`p_data`] is `NULL`, then the maximum size of the data that  **can**  be
    /// retrieved from the validation cache, in bytes, is returned in
    /// [`p_data_size`].
    /// Otherwise, [`p_data_size`] **must**  point to a variable set by the user to the
    /// size of the buffer, in bytes, pointed to by [`p_data`], and on return the
    /// variable is overwritten with the amount of data actually written to
    /// [`p_data`].
    /// If [`p_data_size`] is less than the maximum size that  **can**  be retrieved by
    /// the validation cache, at most [`p_data_size`] bytes will be written to
    /// [`p_data`], and [`get_validation_cache_data_ext`] will return
    /// `VK_INCOMPLETE` instead of `VK_SUCCESS`, to indicate that not all of
    /// the validation cache was returned.Any data written to [`p_data`] is valid and  **can**  be
    /// provided as the
    /// `pInitialData` member of the [`ValidationCacheCreateInfoEXT`]
    /// structure passed to [`create_validation_cache_ext`].Two calls to
    /// [`get_validation_cache_data_ext`] with the same parameters
    /// **must**  retrieve the same data unless a command that modifies the contents of
    /// the cache is called between them.Applications  **can**  store the data retrieved from the
    /// validation cache, and
    /// use these data, possibly in a future run of the application, to populate new
    /// validation cache objects.
    /// The results of validation, however,  **may**  depend on the vendor ID, device ID,
    /// driver version, and other details of the device.
    /// To enable applications to detect when previously retrieved data is
    /// incompatible with the device, the initial bytes written to [`p_data`] **must**
    /// be a header consisting of the following members:The first four bytes encode the length of
    /// the entire validation cache
    /// header, in bytes.
    /// This value includes all fields in the header including the validation cache
    /// version field and the size of the length field.The next four bytes encode the validation
    /// cache version, as described for
    /// [`ValidationCacheHeaderVersionEXT`].
    /// A consumer of the validation cache  **should**  use the cache version to
    /// interpret the remainder of the cache header.If [`p_data_size`] is less than what is
    /// necessary to store this header,
    /// nothing will be written to [`p_data`] and zero will be written to
    /// [`p_data_size`].
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`validation_cache`] **must**  be a valid [`ValidationCacheEXT`] handle
    /// - [`p_data_size`] **must**  be a valid pointer to a `size_t` value
    /// - If the value referenced by [`p_data_size`] is not `0`, and [`p_data`] is not `NULL`,
    ///   [`p_data`] **must**  be a valid pointer to an array of [`p_data_size`] bytes
    /// - [`validation_cache`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`VK_EXT_validation_cache`]
    /// - [`Device`]
    /// - [`ValidationCacheEXT`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetValidationCacheDataEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_validation_cache_data_ext(
        self: &Unique<Device>,
        validation_cache: ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: Option<*mut c_void>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_validation_cache()
            .and_then(|vtable| vtable.get_validation_cache_data_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_validation_cache()
            .and_then(|vtable| vtable.get_validation_cache_data_ext())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            validation_cache,
            p_data_size,
            p_data.unwrap_or_else(std::ptr::null_mut),
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkMergeValidationCachesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergeValidationCachesEXT.html) - Combine the data stores of validation caches
    ///# C Specifications
    ///Validation cache objects  **can**  be merged using the command:
    ///```c
    ///// Provided by VK_EXT_validation_cache
    ///VkResult vkMergeValidationCachesEXT(
    ///    VkDevice                                    device,
    ///    VkValidationCacheEXT                        dstCache,
    ///    uint32_t                                    srcCacheCount,
    ///    const VkValidationCacheEXT*                 pSrcCaches);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that owns the validation cache objects.
    /// - [`dst_cache`] is the handle of the validation cache to merge results into.
    /// - [`src_cache_count`] is the length of the [`p_src_caches`] array.
    /// - [`p_src_caches`] is a pointer to an array of validation cache handles, which will be
    ///   merged into [`dst_cache`]. The previous contents of [`dst_cache`] are included after the
    ///   merge.
    /// # Description
    /// ## Valid Usage
    /// - [`dst_cache`] **must**  not appear in the list of source caches
    ///
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`dst_cache`] **must**  be a valid [`ValidationCacheEXT`] handle
    /// - [`p_src_caches`] **must**  be a valid pointer to an array of [`src_cache_count`] valid
    ///   [`ValidationCacheEXT`] handles
    /// - [`src_cache_count`] **must**  be greater than `0`
    /// - [`dst_cache`] **must**  have been created, allocated, or retrieved from [`device`]
    /// - Each element of [`p_src_caches`] **must**  have been created, allocated, or retrieved from
    ///   [`device`]
    ///
    /// ## Host Synchronization
    /// - Host access to [`dst_cache`] **must**  be externally synchronized
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`VK_EXT_validation_cache`]
    /// - [`Device`]
    /// - [`ValidationCacheEXT`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkMergeValidationCachesEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn merge_validation_caches_ext(
        self: &Unique<Device>,
        dst_cache: ValidationCacheEXT,
        p_src_caches: &[crate::extensions::ext_validation_cache::ValidationCacheEXT],
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_validation_cache()
            .and_then(|vtable| vtable.merge_validation_caches_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_validation_cache()
            .and_then(|vtable| vtable.merge_validation_caches_ext())
            .unwrap_unchecked();
        let src_cache_count = (|len: usize| len)(p_src_caches.len()) as _;
        let _return = _function(self.as_raw(), dst_cache, src_cache_count, p_src_caches.as_ptr());
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
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
///Applications  **can**  manage the host memory consumed by a validation cache
///object and control the amount of data retrieved from a validation cache
///object.Validation cache objects are represented by [`ValidationCacheEXT`]
///handles:
///```c
///// Provided by VK_EXT_validation_cache
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkValidationCacheEXT)
///```
/// # Related
/// - [`VK_EXT_validation_cache`]
/// - [`ShaderModuleValidationCacheCreateInfoEXT`]
/// - [`create_validation_cache_ext`]
/// - [`destroy_validation_cache_ext`]
/// - [`get_validation_cache_data_ext`]
/// - [`merge_validation_caches_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkValidationCacheEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
    pub fn is_null(&self) -> bool {
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
        Self::null()
    }
}
impl Handle for ValidationCacheEXT {
    type Parent = Unique<Device>;
    type VTable = ();
    type Metadata = AtomicBool;
    type Raw = u64;
    #[inline]
    fn as_raw(self) -> Self::Raw {
        self.0
    }
    #[inline]
    unsafe fn from_raw(this: Self::Raw) -> Self {
        Self(this)
    }
    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {
        if !self.metadata().load(Ordering::Acquire) {
            self.device()
                .destroy_validation_cache_ext(Some(self.as_raw().coerce()), None);
        }
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<ValidationCacheEXT> {
    ///Gets the reference to the [`Entry`]
    #[inline]
    pub fn entry(&self) -> &Arc<Entry> {
        self.parent().parent().parent().parent()
    }
    ///Gets the reference to the [`Instance`]
    #[inline]
    pub fn instance(&self) -> &Unique<Instance> {
        self.parent().parent().parent()
    }
    ///Gets the reference to the [`PhysicalDevice`]
    #[inline]
    pub fn physical_device(&self) -> &Unique<PhysicalDevice> {
        self.parent().parent()
    }
    ///Gets the reference to the [`Device`]
    #[inline]
    pub fn device(&self) -> &Unique<Device> {
        self.parent()
    }
    ///Disables the base dropping behaviour of this handle
    #[inline]
    pub fn disable_drop(&self) {
        self.metadata().store(true, Ordering::Relaxed);
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_validation_cache`
pub struct DeviceExtValidationCacheVTable {
    ///See [`FNCreateValidationCacheExt`] for more information.
    pub create_validation_cache_ext: FNCreateValidationCacheExt,
    ///See [`FNDestroyValidationCacheExt`] for more information.
    pub destroy_validation_cache_ext: FNDestroyValidationCacheExt,
    ///See [`FNGetValidationCacheDataExt`] for more information.
    pub get_validation_cache_data_ext: FNGetValidationCacheDataExt,
    ///See [`FNMergeValidationCachesExt`] for more information.
    pub merge_validation_caches_ext: FNMergeValidationCachesExt,
}
impl DeviceExtValidationCacheVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            create_validation_cache_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateValidationCacheEXT").as_ptr()))
            },
            destroy_validation_cache_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkDestroyValidationCacheEXT").as_ptr()))
            },
            get_validation_cache_data_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetValidationCacheDataEXT").as_ptr()))
            },
            merge_validation_caches_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkMergeValidationCachesEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::create_validation_cache_ext`]. See [`FNCreateValidationCacheExt`] for more
    /// information.
    pub fn create_validation_cache_ext(&self) -> FNCreateValidationCacheExt {
        self.create_validation_cache_ext
    }
    ///Gets [`Self::destroy_validation_cache_ext`]. See [`FNDestroyValidationCacheExt`] for more
    /// information.
    pub fn destroy_validation_cache_ext(&self) -> FNDestroyValidationCacheExt {
        self.destroy_validation_cache_ext
    }
    ///Gets [`Self::get_validation_cache_data_ext`]. See [`FNGetValidationCacheDataExt`] for more
    /// information.
    pub fn get_validation_cache_data_ext(&self) -> FNGetValidationCacheDataExt {
        self.get_validation_cache_data_ext
    }
    ///Gets [`Self::merge_validation_caches_ext`]. See [`FNMergeValidationCachesExt`] for more
    /// information.
    pub fn merge_validation_caches_ext(&self) -> FNMergeValidationCachesExt {
        self.merge_validation_caches_ext
    }
}
