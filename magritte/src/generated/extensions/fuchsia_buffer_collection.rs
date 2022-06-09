//![VK_FUCHSIA_buffer_collection](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_buffer_collection.html) - device extension
//!# Description
//!A buffer collection is a collection of one or more buffers which were
//!allocated together as a group and which all have the same properties.
//!These properties describe the buffers' internal representation such as its
//!dimensions and memory layout.
//!This ensures that all of the buffers can be used interchangeably by tasks
//!that require swapping among multiple buffers, such as double-buffered
//!graphics rendering.By sharing such a collection of buffers between components, communication
//!about buffer lifecycle can be made much simpler and more efficient.
//!For example, when a content producer finishes writing to a buffer, it can
//!message the consumer of the buffer with the buffer index, rather than
//!passing a handle to the shared memory.On Fuchsia, the Sysmem service uses buffer collections as
//! a core construct
//!in its design.
//!VK_FUCHSIA_buffer_collection is the Vulkan extension that allows Vulkan
//!applications to interoperate with the Sysmem service on Fuchsia.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`fuchsia_external_memory`]`
//! - Requires `[`khr_sampler_ycbcr_conversion`]`
//!# Contacts
//! - John Rosasco [rosasco](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_FUCHSIA_buffer_collection]
//!   @rosasco%0A<<Here describe the issue or question you have about the
//!   VK_FUCHSIA_buffer_collection extension>>)
//!# New handles
//! - [`BufferCollectionFUCHSIA`]
//!# New functions & commands
//! - [`create_buffer_collection_fuchsia`]
//! - [`destroy_buffer_collection_fuchsia`]
//! - [`get_buffer_collection_properties_fuchsia`]
//! - [`set_buffer_collection_buffer_constraints_fuchsia`]
//! - [`set_buffer_collection_image_constraints_fuchsia`]
//!# New structures
//! - [`BufferCollectionConstraintsInfoFUCHSIA`]
//! - [`BufferCollectionCreateInfoFUCHSIA`]
//! - [`BufferCollectionPropertiesFUCHSIA`]
//! - [`BufferConstraintsInfoFUCHSIA`]
//! - [`ImageConstraintsInfoFUCHSIA`]
//! - [`ImageFormatConstraintsInfoFUCHSIA`]
//! - [`SysmemColorSpaceFUCHSIA`]
//! - Extending [`BufferCreateInfo`]:  - [`BufferCollectionBufferCreateInfoFUCHSIA`]
//! - Extending [`ImageCreateInfo`]:  - [`BufferCollectionImageCreateInfoFUCHSIA`]
//! - Extending [`MemoryAllocateInfo`]:  - [`ImportMemoryBufferCollectionFUCHSIA`]
//!# New enums
//! - [`ImageConstraintsInfoFlagBitsFUCHSIA`]
//!# New bitmasks
//! - [`ImageConstraintsInfoFlagsFUCHSIA`]
//! - [`ImageFormatConstraintsFlagsFUCHSIA`]
//!# New constants
//! - [`FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME`]
//! - [`FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION`]
//! - Extending [`DebugReportObjectTypeEXT`]:  -
//!   `VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA_EXT`
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA`  -
//!   `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA`  -
//!   `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA`  -
//!   `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA`  -
//!   `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA`  -
//!   `VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA`  -
//!   `VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA`  -
//!   `VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA`  -
//!   `VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA`  -
//!   `VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA`
//!# Known issues & F.A.Q
//!1) When configuring a [`ImageConstraintsInfoFUCHSIA`] structure for
//!constraint setting, should a NULL `pFormatConstraints` parameter be
//!allowed ? **RESOLVED** : No.
//!Specifying a NULL `pFormatConstraints` results in logical complexity of
//!interpreting the relationship between the
//![`ImageCreateInfo::usage`] settings of the elements of the
//!`pImageCreateInfos` array and the implied or desired
//![`FormatFeatureFlags`].The explicit requirement for `pFormatConstraints` to be non-NULL
//!simplifies the implied logic of the implementation and expectations for the
//!Vulkan application.
//!# Version History
//! - Revision 2, 2021-09-23 (John Rosasco)  - Review passes
//! - Revision 1, 2021-03-09 (John Rosasco)  - Initial revision
//!# Other info
//! * 2021-09-23
//! * No known IP claims.
//! * - Craig Stout, Google  - John Bauman, Google  - John Rosasco, Google
//!# Related
//! - [`BufferCollectionBufferCreateInfoFUCHSIA`]
//! - [`BufferCollectionConstraintsInfoFUCHSIA`]
//! - [`BufferCollectionCreateInfoFUCHSIA`]
//! - [`BufferCollectionFUCHSIA`]
//! - [`BufferCollectionImageCreateInfoFUCHSIA`]
//! - [`BufferCollectionPropertiesFUCHSIA`]
//! - [`BufferConstraintsInfoFUCHSIA`]
//! - [`ImageConstraintsInfoFUCHSIA`]
//! - [`ImageConstraintsInfoFlagBitsFUCHSIA`]
//! - [`ImageConstraintsInfoFlagsFUCHSIA`]
//! - [`ImageFormatConstraintsFlagsFUCHSIA`]
//! - [`ImageFormatConstraintsInfoFUCHSIA`]
//! - [`ImportMemoryBufferCollectionFUCHSIA`]
//! - [`SysmemColorSpaceFUCHSIA`]
//! - [`create_buffer_collection_fuchsia`]
//! - [`destroy_buffer_collection_fuchsia`]
//! - [`get_buffer_collection_properties_fuchsia`]
//! - [`set_buffer_collection_buffer_constraints_fuchsia`]
//! - [`set_buffer_collection_image_constraints_fuchsia`]
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
    native::zx_handle_t,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, BufferCreateInfo, ComponentMapping, Device,
        FormatFeatureFlags, ImageCreateInfo, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
    vulkan1_1::{ChromaLocation, SamplerYcbcrModelConversion, SamplerYcbcrRange},
    AsRaw, Handle, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION")]
pub const FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME")]
pub const FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_FUCHSIA_buffer_collection");
///[vkCreateBufferCollectionFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html) - Create a new buffer collection
///# C Specifications
///To create an [`BufferCollectionFUCHSIA`] for Vulkan to participate in
///the buffer collection:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///VkResult vkCreateBufferCollectionFUCHSIA(
///    VkDevice                                    device,
///    const VkBufferCollectionCreateInfoFUCHSIA*  pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkBufferCollectionFUCHSIA*                  pCollection);
///```
///# Parameters
/// - [`device`] is the logical device that creates the [`BufferCollectionFUCHSIA`]
/// - [`p_create_info`] is a pointer to a [`BufferCollectionCreateInfoFUCHSIA`] structure containing
///   parameters affecting creation of the buffer collection
/// - [`p_allocator`] is a pointer to a [`AllocationCallbacks`] structure controlling host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter
/// - `pBufferCollection` is a pointer to a [`BufferCollectionFUCHSIA`] handle in which the
///   resulting buffer collection object is returned
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid
///   [`BufferCollectionCreateInfoFUCHSIA`] structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_collection`] **must**  be a valid pointer to a [`BufferCollectionFUCHSIA`] handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`  -
///   `VK_ERROR_INITIALIZATION_FAILED`
///
///## Host AccessAll functions referencing a [`BufferCollectionFUCHSIA`] **must**  be
///externally synchronized with the exception of
///[`create_buffer_collection_fuchsia`].
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`AllocationCallbacks`]
/// - [`BufferCollectionCreateInfoFUCHSIA`]
/// - [`BufferCollectionFUCHSIA`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
pub type FNCreateBufferCollectionFuchsia = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_create_info: *const BufferCollectionCreateInfoFUCHSIA<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_collection: *mut BufferCollectionFUCHSIA,
    ) -> VulkanResultCodes,
>;
///[vkSetBufferCollectionBufferConstraintsFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html) - Set buffer-based constraints for a buffer collection
///# C Specifications
///To set the constraints on a [`Buffer`] buffer collection, call:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///VkResult vkSetBufferCollectionBufferConstraintsFUCHSIA(
///    VkDevice                                    device,
///    VkBufferCollectionFUCHSIA                   collection,
///    const VkBufferConstraintsInfoFUCHSIA*       pBufferConstraintsInfo);
///```
///# Parameters
/// - [`device`] is the logical device
/// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
/// - [`p_buffer_constraints_info`] is a pointer to a [`BufferConstraintsInfoFUCHSIA`] structure
///# Description
///[`set_buffer_collection_buffer_constraints_fuchsia`] **may**  fail if the
///implementation does not support the constraints specified in the
///`bufferCollectionConstraints` structure.
///If that occurs, [`set_buffer_collection_buffer_constraints_fuchsia`] will
///return `VK_ERROR_FORMAT_NOT_SUPPORTED`.
///## Valid Usage
/// - [`set_buffer_collection_image_constraints_fuchsia`] or
///   [`set_buffer_collection_buffer_constraints_fuchsia`] **must**  not have already been called on
///   [`collection`]
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
/// - [`p_buffer_constraints_info`] **must**  be a valid pointer to a valid
///   [`BufferConstraintsInfoFUCHSIA`] structure
/// - [`collection`] **must**  have been created, allocated, or retrieved from [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_OUT_OF_HOST_MEMORY`  -
///   `VK_ERROR_FORMAT_NOT_SUPPORTED`
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`BufferCollectionFUCHSIA`]
/// - [`BufferConstraintsInfoFUCHSIA`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
pub type FNSetBufferCollectionBufferConstraintsFuchsia = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        collection: BufferCollectionFUCHSIA,
        p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkSetBufferCollectionImageConstraintsFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html) - Set image-based constraints for a buffer collection
///# C Specifications
///Setting the constraints on the buffer collection initiates the format
///negotiation and allocation of the buffer collection.
///To set the constraints on a [`Image`] buffer collection, call:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///VkResult vkSetBufferCollectionImageConstraintsFUCHSIA(
///    VkDevice                                    device,
///    VkBufferCollectionFUCHSIA                   collection,
///    const VkImageConstraintsInfoFUCHSIA*        pImageConstraintsInfo);
///```
///# Parameters
/// - [`device`] is the logical device
/// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
/// - [`p_image_constraints_info`] is a pointer to a [`ImageConstraintsInfoFUCHSIA`] structure
///# Description
///[`set_buffer_collection_image_constraints_fuchsia`] **may**  fail if
///[`p_image_constraints_info`]`::formatConstraintsCount` is larger than the
///implementation-defined limit.
///If that occurs, [`set_buffer_collection_image_constraints_fuchsia`] will
///return VK_ERROR_INITIALIZATION_FAILED.[`set_buffer_collection_image_constraints_fuchsia`]
/// **may**  fail if the
///implementation does not support any of the formats described by the
///[`p_image_constraints_info`] structure.
///If that occurs, [`set_buffer_collection_image_constraints_fuchsia`] will
///return `VK_ERROR_FORMAT_NOT_SUPPORTED`.
///## Valid Usage
/// - [`set_buffer_collection_image_constraints_fuchsia`] or
///   [`set_buffer_collection_buffer_constraints_fuchsia`] **must**  not have already been called on
///   [`collection`]
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
/// - [`p_image_constraints_info`] **must**  be a valid pointer to a valid
///   [`ImageConstraintsInfoFUCHSIA`] structure
/// - [`collection`] **must**  have been created, allocated, or retrieved from [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_OUT_OF_HOST_MEMORY`  -
///   `VK_ERROR_FORMAT_NOT_SUPPORTED`
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`BufferCollectionFUCHSIA`]
/// - [`Device`]
/// - [`ImageConstraintsInfoFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
pub type FNSetBufferCollectionImageConstraintsFuchsia = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        collection: BufferCollectionFUCHSIA,
        p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkDestroyBufferCollectionFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html) - Destroy a buffer collection
///# C Specifications
///To release a [`BufferCollectionFUCHSIA`]:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///void vkDestroyBufferCollectionFUCHSIA(
///    VkDevice                                    device,
///    VkBufferCollectionFUCHSIA                   collection,
///    const VkAllocationCallbacks*                pAllocator);
///```
///# Parameters
/// - [`device`] is the logical device that creates the [`BufferCollectionFUCHSIA`]
/// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
/// - [`p_allocator`] is a pointer to a [`AllocationCallbacks`] structure controlling host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter
///# Description
///## Valid Usage
/// - [`Image`] and [`Buffer`] objects that referenced [`collection`] upon creation by inclusion of
///   a [`BufferCollectionImageCreateInfoFUCHSIA`] or [`BufferCollectionBufferCreateInfoFUCHSIA`]
///   chained to their [`ImageCreateInfo`] or [`BufferCreateInfo`] structures respectively,  **may**
///   outlive [`collection`].
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`collection`] **must**  have been created, allocated, or retrieved from [`device`]
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`AllocationCallbacks`]
/// - [`BufferCollectionFUCHSIA`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
pub type FNDestroyBufferCollectionFuchsia = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        collection: BufferCollectionFUCHSIA,
        p_allocator: *const AllocationCallbacks<'lt>,
    ),
>;
///[vkGetBufferCollectionPropertiesFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html) - Retrieve properties from a buffer collection
///# C Specifications
///After constraints have been set on the buffer collection by calling
///[`set_buffer_collection_image_constraints_fuchsia`] or
///[`set_buffer_collection_buffer_constraints_fuchsia`], call
///[`get_buffer_collection_properties_fuchsia`] to retrieve the negotiated and
///finalized properties of the buffer collection.The call to
/// [`get_buffer_collection_properties_fuchsia`] is synchronous.
///It waits for the Sysmem format negotiation and buffer collection allocation
///to complete before returning.
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///VkResult vkGetBufferCollectionPropertiesFUCHSIA(
///    VkDevice                                    device,
///    VkBufferCollectionFUCHSIA                   collection,
///    VkBufferCollectionPropertiesFUCHSIA*        pProperties);
///```
///# Parameters
/// - [`device`] is the logical device handle
/// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
/// - [`p_properties`] is a pointer to the retrieved [`BufferCollectionPropertiesFUCHSIA`] struct
///# Description
///For image-based buffer collections, upon calling
///[`get_buffer_collection_properties_fuchsia`], Sysmem will choose an element
///of the [`ImageConstraintsInfoFUCHSIA`]`::pImageCreateInfos`
///established by the preceding call to
///[`set_buffer_collection_image_constraints_fuchsia`].
///The index of the element chosen is stored in and can be retrieved from
///[`BufferCollectionPropertiesFUCHSIA::create_info_index`].For buffer-based buffer collections, a
/// single [`BufferCreateInfo`] is
///specified as [`BufferConstraintsInfoFUCHSIA::create_info`].
///[`BufferCollectionPropertiesFUCHSIA::create_info_index`] will
///therefore always be zero.[`get_buffer_collection_properties_fuchsia`] **may**  fail if Sysmem is
/// unable
///to resolve the constraints of all of the participants in the buffer
///collection.
///If that occurs, [`get_buffer_collection_properties_fuchsia`] will return
///`VK_ERROR_INITIALIZATION_FAILED`.
///## Valid Usage
/// - Prior to calling [`get_buffer_collection_properties_fuchsia`], the constraints on the buffer
///   collection  **must**  have been set by either
///   [`set_buffer_collection_image_constraints_fuchsia`] or
///   [`set_buffer_collection_buffer_constraints_fuchsia`].
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
/// - [`p_properties`] **must**  be a valid pointer to a [`BufferCollectionPropertiesFUCHSIA`]
///   structure
/// - [`collection`] **must**  have been created, allocated, or retrieved from [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`BufferCollectionFUCHSIA`]
/// - [`BufferCollectionPropertiesFUCHSIA`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
pub type FNGetBufferCollectionPropertiesFuchsia = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        collection: BufferCollectionFUCHSIA,
        p_properties: *mut BufferCollectionPropertiesFUCHSIA<'lt>,
    ) -> VulkanResultCodes,
>;
///[VkImageConstraintsInfoFlagBitsFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFlagBitsFUCHSIA.html) - Bitmask specifying image constraints flags
///# C Specifications
///Bits which  **can**  be set in
///[`ImageConstraintsInfoFlagBitsFUCHSIA`]`::flags` include:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef enum VkImageConstraintsInfoFlagBitsFUCHSIA {
///    VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA = 0x00000001,
///    VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA = 0x00000002,
///    VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA = 0x00000004,
///    VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA = 0x00000008,
///    VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA = 0x00000010,
///} VkImageConstraintsInfoFlagBitsFUCHSIA;
///```
///# Description
///General hints about the type of memory that should be allocated by Sysmem
///based on the expected usage of the images in the buffer collection include:
/// - [`CPU_READ_RARELY`]
/// - [`CPU_READ_OFTEN`]
/// - [`CPU_WRITE_RARELY`]
/// - [`CPU_WRITE_OFTEN`]
///For protected memory:
/// - [`PROTECTED_OPTIONAL`] specifies that protected memory is optional for the buffer collection.
///Note that if all participants in the buffer collection (Vulkan or otherwise)
///specify that protected memory is optional, Sysmem will not allocate
///protected memory.
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`ImageConstraintsInfoFlagsFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageConstraintsInfoFlagBitsFUCHSIA")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ImageConstraintsInfoFlagBitsFUCHSIA(u32);
impl const Default for ImageConstraintsInfoFlagBitsFUCHSIA {
    fn default() -> Self {
        Self(0)
    }
}
impl ImageConstraintsInfoFlagBitsFUCHSIA {
    ///[`CPU_READ_RARELY`]
    pub const CPU_READ_RARELY: Self = Self(1);
    ///[`CPU_READ_OFTEN`]
    pub const CPU_READ_OFTEN: Self = Self(2);
    ///[`CPU_WRITE_RARELY`]
    pub const CPU_WRITE_RARELY: Self = Self(4);
    ///[`CPU_WRITE_OFTEN`]
    pub const CPU_WRITE_OFTEN: Self = Self(8);
    ///[`PROTECTED_OPTIONAL`] specifies
    ///that protected memory is optional for the buffer collection.
    pub const PROTECTED_OPTIONAL: Self = Self(16);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for ImageConstraintsInfoFlagBitsFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ImageConstraintsInfoFlagBitsFUCHSIA);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ImageConstraintsInfoFlagBitsFUCHSIA::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        ImageConstraintsInfoFlagBitsFUCHSIA::CPU_READ_RARELY => f.write_str("CPU_READ_RARELY")?,
                        ImageConstraintsInfoFlagBitsFUCHSIA::CPU_READ_OFTEN => f.write_str("CPU_READ_OFTEN")?,
                        ImageConstraintsInfoFlagBitsFUCHSIA::CPU_WRITE_RARELY => f.write_str("CPU_WRITE_RARELY")?,
                        ImageConstraintsInfoFlagBitsFUCHSIA::CPU_WRITE_OFTEN => f.write_str("CPU_WRITE_OFTEN")?,
                        ImageConstraintsInfoFlagBitsFUCHSIA::PROTECTED_OPTIONAL => f.write_str("PROTECTED_OPTIONAL")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ImageConstraintsInfoFlagBitsFUCHSIA))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkImageFormatConstraintsFlagsFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsFlagsFUCHSIA.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef VkFlags VkImageFormatConstraintsFlagsFUCHSIA;
///```
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`ImageFormatConstraintsInfoFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ImageFormatConstraintsFlagsFUCHSIA(u32);
impl const Default for ImageFormatConstraintsFlagsFUCHSIA {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ImageFormatConstraintsFlagsFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(ImageFormatConstraintsFlagsFUCHSIA))
            .field(&self.0)
            .finish()
    }
}
///[VkImageConstraintsInfoFlagBitsFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFlagBitsFUCHSIA.html) - Bitmask specifying image constraints flags
///# C Specifications
///Bits which  **can**  be set in
///[`ImageConstraintsInfoFlagBitsFUCHSIA`]`::flags` include:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef enum VkImageConstraintsInfoFlagBitsFUCHSIA {
///    VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA = 0x00000001,
///    VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA = 0x00000002,
///    VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA = 0x00000004,
///    VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA = 0x00000008,
///    VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA = 0x00000010,
///} VkImageConstraintsInfoFlagBitsFUCHSIA;
///```
///# Description
///General hints about the type of memory that should be allocated by Sysmem
///based on the expected usage of the images in the buffer collection include:
/// - [`CPU_READ_RARELY`]
/// - [`CPU_READ_OFTEN`]
/// - [`CPU_WRITE_RARELY`]
/// - [`CPU_WRITE_OFTEN`]
///For protected memory:
/// - [`PROTECTED_OPTIONAL`] specifies that protected memory is optional for the buffer collection.
///Note that if all participants in the buffer collection (Vulkan or otherwise)
///specify that protected memory is optional, Sysmem will not allocate
///protected memory.
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`ImageConstraintsInfoFlagsFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageConstraintsInfoFlagsFUCHSIA")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ImageConstraintsInfoFlagsFUCHSIA(u32);
impl const Default for ImageConstraintsInfoFlagsFUCHSIA {
    fn default() -> Self {
        Self(0)
    }
}
impl From<ImageConstraintsInfoFlagBitsFUCHSIA> for ImageConstraintsInfoFlagsFUCHSIA {
    fn from(from: ImageConstraintsInfoFlagBitsFUCHSIA) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl ImageConstraintsInfoFlagsFUCHSIA {
    ///[`CPU_READ_RARELY`]
    pub const CPU_READ_RARELY: Self = Self(1);
    ///[`CPU_READ_OFTEN`]
    pub const CPU_READ_OFTEN: Self = Self(2);
    ///[`CPU_WRITE_RARELY`]
    pub const CPU_WRITE_RARELY: Self = Self(4);
    ///[`CPU_WRITE_OFTEN`]
    pub const CPU_WRITE_OFTEN: Self = Self(8);
    ///[`PROTECTED_OPTIONAL`] specifies
    ///that protected memory is optional for the buffer collection.
    pub const PROTECTED_OPTIONAL: Self = Self(16);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::CPU_READ_RARELY;
        }
        {
            all |= Self::CPU_READ_OFTEN;
        }
        {
            all |= Self::CPU_WRITE_RARELY;
        }
        {
            all |= Self::CPU_WRITE_OFTEN;
        }
        {
            all |= Self::PROTECTED_OPTIONAL;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ImageConstraintsInfoFlagsFUCHSIA {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ImageConstraintsInfoFlagsFUCHSIA {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ImageConstraintsInfoFlagsFUCHSIA {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ImageConstraintsInfoFlagsFUCHSIA {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ImageConstraintsInfoFlagsFUCHSIA> for ImageConstraintsInfoFlagsFUCHSIA {
    fn extend<T: IntoIterator<Item = ImageConstraintsInfoFlagsFUCHSIA>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<ImageConstraintsInfoFlagBitsFUCHSIA> for ImageConstraintsInfoFlagsFUCHSIA {
    fn extend<T: IntoIterator<Item = ImageConstraintsInfoFlagBitsFUCHSIA>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<ImageConstraintsInfoFlagBitsFUCHSIA>>::from(i));
        }
    }
}
impl FromIterator<ImageConstraintsInfoFlagsFUCHSIA> for ImageConstraintsInfoFlagsFUCHSIA {
    fn from_iter<T: IntoIterator<Item = ImageConstraintsInfoFlagsFUCHSIA>>(
        iterator: T,
    ) -> ImageConstraintsInfoFlagsFUCHSIA {
        let mut out = Self::empty();
        <Self as Extend<ImageConstraintsInfoFlagsFUCHSIA>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<ImageConstraintsInfoFlagBitsFUCHSIA> for ImageConstraintsInfoFlagsFUCHSIA {
    fn from_iter<T: IntoIterator<Item = ImageConstraintsInfoFlagBitsFUCHSIA>>(
        iterator: T,
    ) -> ImageConstraintsInfoFlagsFUCHSIA {
        let mut out = Self::empty();
        <Self as Extend<ImageConstraintsInfoFlagBitsFUCHSIA>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ImageConstraintsInfoFlagsFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ImageConstraintsInfoFlagsFUCHSIA);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ImageConstraintsInfoFlagsFUCHSIA::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ImageConstraintsInfoFlagsFUCHSIA::CPU_READ_RARELY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CPU_READ_RARELY))?;
                    }
                    if self.0.contains(ImageConstraintsInfoFlagsFUCHSIA::CPU_READ_OFTEN) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CPU_READ_OFTEN))?;
                    }
                    if self.0.contains(ImageConstraintsInfoFlagsFUCHSIA::CPU_WRITE_RARELY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CPU_WRITE_RARELY))?;
                    }
                    if self.0.contains(ImageConstraintsInfoFlagsFUCHSIA::CPU_WRITE_OFTEN) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CPU_WRITE_OFTEN))?;
                    }
                    if self.0.contains(ImageConstraintsInfoFlagsFUCHSIA::PROTECTED_OPTIONAL) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PROTECTED_OPTIONAL))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ImageConstraintsInfoFlagsFUCHSIA))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkImportMemoryBufferCollectionFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryBufferCollectionFUCHSIA.html) - Structure to specify the Sysmem buffer to import
///# C Specifications
///The [`ImportMemoryBufferCollectionFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkImportMemoryBufferCollectionFUCHSIA {
///    VkStructureType              sType;
///    const void*                  pNext;
///    VkBufferCollectionFUCHSIA    collection;
///    uint32_t                     index;
///} VkImportMemoryBufferCollectionFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
/// - [`index`] the index of the buffer to import from [`collection`]
///# Description
///## Valid Usage
/// - [`index`] **must**  be less than the value retrieved as
///   [`BufferCollectionPropertiesFUCHSIA`]:bufferCount
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA`
/// - [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`BufferCollectionFUCHSIA`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImportMemoryBufferCollectionFUCHSIA")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ImportMemoryBufferCollectionFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    pub p_next: *const BaseInStructure<'lt>,
    ///[`collection`] is the [`BufferCollectionFUCHSIA`] handle
    pub collection: BufferCollectionFUCHSIA,
    ///[`index`] the index of the buffer to import from [`collection`]
    pub index: u32,
}
impl<'lt> Default for ImportMemoryBufferCollectionFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA,
            p_next: std::ptr::null(),
            collection: Default::default(),
            index: 0,
        }
    }
}
impl<'lt> ImportMemoryBufferCollectionFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::collection`]
    pub fn collection(&self) -> BufferCollectionFUCHSIA {
        self.collection
    }
    ///Gets the value of [`Self::index`]
    pub fn index(&self) -> u32 {
        self.index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::collection`]
    pub fn collection_mut(&mut self) -> &mut BufferCollectionFUCHSIA {
        &mut self.collection
    }
    ///Gets a mutable reference to the value of [`Self::index`]
    pub fn index_mut(&mut self) -> &mut u32 {
        &mut self.index
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::collection`]
    pub fn set_collection(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    ) -> &mut Self {
        self.collection = value;
        self
    }
    ///Sets the value of [`Self::index`]
    pub fn set_index(&mut self, value: u32) -> &mut Self {
        self.index = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::collection`]
    pub fn with_collection(
        mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    ) -> Self {
        self.collection = value;
        self
    }
    ///Sets the value of [`Self::index`]
    pub fn with_index(mut self, value: u32) -> Self {
        self.index = value;
        self
    }
}
///[VkBufferCollectionImageCreateInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionImageCreateInfoFUCHSIA.html) - Create a VkBufferCollectionFUCHSIA-compatible VkImage
///# C Specifications
///The [`BufferCollectionImageCreateInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkBufferCollectionImageCreateInfoFUCHSIA {
///    VkStructureType              sType;
///    const void*                  pNext;
///    VkBufferCollectionFUCHSIA    collection;
///    uint32_t                     index;
///} VkBufferCollectionImageCreateInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
/// - [`index`] is the index of the buffer in the buffer collection from which the memory will be
///   imported
///# Description
///## Valid Usage
/// - [`index`] **must**  be less than [`BufferCollectionPropertiesFUCHSIA::buffer_count`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA`
/// - [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`BufferCollectionFUCHSIA`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBufferCollectionImageCreateInfoFUCHSIA")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct BufferCollectionImageCreateInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    pub p_next: *const BaseInStructure<'lt>,
    ///[`collection`] is the [`BufferCollectionFUCHSIA`] handle
    pub collection: BufferCollectionFUCHSIA,
    ///[`index`] is the index of the buffer in the buffer collection from
    ///which the memory will be imported
    pub index: u32,
}
impl<'lt> Default for BufferCollectionImageCreateInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            collection: Default::default(),
            index: 0,
        }
    }
}
impl<'lt> BufferCollectionImageCreateInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::collection`]
    pub fn collection(&self) -> BufferCollectionFUCHSIA {
        self.collection
    }
    ///Gets the value of [`Self::index`]
    pub fn index(&self) -> u32 {
        self.index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::collection`]
    pub fn collection_mut(&mut self) -> &mut BufferCollectionFUCHSIA {
        &mut self.collection
    }
    ///Gets a mutable reference to the value of [`Self::index`]
    pub fn index_mut(&mut self) -> &mut u32 {
        &mut self.index
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::collection`]
    pub fn set_collection(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    ) -> &mut Self {
        self.collection = value;
        self
    }
    ///Sets the value of [`Self::index`]
    pub fn set_index(&mut self, value: u32) -> &mut Self {
        self.index = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::collection`]
    pub fn with_collection(
        mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    ) -> Self {
        self.collection = value;
        self
    }
    ///Sets the value of [`Self::index`]
    pub fn with_index(mut self, value: u32) -> Self {
        self.index = value;
        self
    }
}
///[VkBufferCollectionBufferCreateInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionBufferCreateInfoFUCHSIA.html) - Create a VkBufferCollectionFUCHSIA-compatible VkBuffer
///# C Specifications
///The [`BufferCollectionBufferCreateInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkBufferCollectionBufferCreateInfoFUCHSIA {
///    VkStructureType              sType;
///    const void*                  pNext;
///    VkBufferCollectionFUCHSIA    collection;
///    uint32_t                     index;
///} VkBufferCollectionBufferCreateInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
/// - [`index`] is the index of the buffer in the buffer collection from which the memory will be
///   imported
///# Description
///## Valid Usage
/// - [`index`] **must**  be less than [`BufferCollectionPropertiesFUCHSIA::buffer_count`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA`
/// - [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`BufferCollectionFUCHSIA`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBufferCollectionBufferCreateInfoFUCHSIA")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct BufferCollectionBufferCreateInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    pub p_next: *const BaseInStructure<'lt>,
    ///[`collection`] is the [`BufferCollectionFUCHSIA`] handle
    pub collection: BufferCollectionFUCHSIA,
    ///[`index`] is the index of the buffer in the buffer collection from
    ///which the memory will be imported
    pub index: u32,
}
impl<'lt> Default for BufferCollectionBufferCreateInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            collection: Default::default(),
            index: 0,
        }
    }
}
impl<'lt> BufferCollectionBufferCreateInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::collection`]
    pub fn collection(&self) -> BufferCollectionFUCHSIA {
        self.collection
    }
    ///Gets the value of [`Self::index`]
    pub fn index(&self) -> u32 {
        self.index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::collection`]
    pub fn collection_mut(&mut self) -> &mut BufferCollectionFUCHSIA {
        &mut self.collection
    }
    ///Gets a mutable reference to the value of [`Self::index`]
    pub fn index_mut(&mut self) -> &mut u32 {
        &mut self.index
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::collection`]
    pub fn set_collection(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    ) -> &mut Self {
        self.collection = value;
        self
    }
    ///Sets the value of [`Self::index`]
    pub fn set_index(&mut self, value: u32) -> &mut Self {
        self.index = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::collection`]
    pub fn with_collection(
        mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    ) -> Self {
        self.collection = value;
        self
    }
    ///Sets the value of [`Self::index`]
    pub fn with_index(mut self, value: u32) -> Self {
        self.index = value;
        self
    }
}
///[VkBufferCollectionCreateInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionCreateInfoFUCHSIA.html) - Structure specifying desired parameters to create the buffer collection
///# C Specifications
///The [`BufferCollectionCreateInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkBufferCollectionCreateInfoFUCHSIA {
///    VkStructureType    sType;
///    const void*        pNext;
///    zx_handle_t        collectionToken;
///} VkBufferCollectionCreateInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`collection_token`] is a [`zx_handle_t`] containing the Sysmem client’s buffer collection
///   token
///# Description
///## Valid Usage
/// - [`collection_token`] **must**  be a valid [`zx_handle_t`] to a Zircon channel allocated from
///   Sysmem (`fuchsia.sysmem.Allocator`/AllocateSharedCollection) with `ZX_DEFAULT_CHANNEL_RIGHTS`
///   rights
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`StructureType`]
/// - [`create_buffer_collection_fuchsia`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBufferCollectionCreateInfoFUCHSIA")]
#[repr(C)]
pub struct BufferCollectionCreateInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    pub p_next: *const BaseInStructure<'lt>,
    ///[`collection_token`] is a [`zx_handle_t`] containing the Sysmem
    ///client’s buffer collection token
    pub collection_token: zx_handle_t,
}
impl<'lt> Default for BufferCollectionCreateInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BUFFER_COLLECTION_CREATE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            collection_token: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> BufferCollectionCreateInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::collection_token`]
    pub fn collection_token_raw(&self) -> &zx_handle_t {
        &self.collection_token
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::collection_token`]
    pub fn set_collection_token_raw(&mut self, value: zx_handle_t) -> &mut Self {
        self.collection_token = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::collection_token`]
    pub fn with_collection_token_raw(mut self, value: zx_handle_t) -> Self {
        self.collection_token = value;
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
    ///Gets the value of [`Self::collection_token`]
    pub fn collection_token(&self) -> zx_handle_t {
        self.collection_token
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::collection_token`]
    pub fn collection_token_mut(&mut self) -> &mut zx_handle_t {
        &mut self.collection_token
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::collection_token`]
    pub fn set_collection_token(&mut self, value: crate::native::zx_handle_t) -> &mut Self {
        self.collection_token = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::collection_token`]
    pub fn with_collection_token(mut self, value: crate::native::zx_handle_t) -> Self {
        self.collection_token = value;
        self
    }
}
///[VkBufferCollectionPropertiesFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionPropertiesFUCHSIA.html) - Structure specifying the negotiated format chosen by Sysmem
///# C Specifications
///The [`BufferCollectionPropertiesFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkBufferCollectionPropertiesFUCHSIA {
///    VkStructureType                  sType;
///    void*                            pNext;
///    uint32_t                         memoryTypeBits;
///    uint32_t                         bufferCount;
///    uint32_t                         createInfoIndex;
///    uint64_t                         sysmemPixelFormat;
///    VkFormatFeatureFlags             formatFeatures;
///    VkSysmemColorSpaceFUCHSIA        sysmemColorSpaceIndex;
///    VkComponentMapping               samplerYcbcrConversionComponents;
///    VkSamplerYcbcrModelConversion    suggestedYcbcrModel;
///    VkSamplerYcbcrRange              suggestedYcbcrRange;
///    VkChromaLocation                 suggestedXChromaOffset;
///    VkChromaLocation                 suggestedYChromaOffset;
///} VkBufferCollectionPropertiesFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`memory_type_bits`] is a bitmask containing one bit set for every memory type which the
///   buffer collection can be imported as buffer collection
/// - [`buffer_count`] is the number of buffers in the collection
/// - [`create_info_index`] as described in [Sysmem chosen create infos](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sysmem-chosen-create-infos)
/// - [`sysmem_pixel_format`] is the Sysmem `PixelFormatType` as defined in
///   `fuchsia.sysmem/image_formats.fidl`
/// - [`format_features`] is a bitmask of [`FormatFeatureFlagBits`] shared by the buffer collection
/// - [`sysmem_color_space_index`] is a [`SysmemColorSpaceFUCHSIA`] struct specifying the color
///   space
/// - [`sampler_ycbcr_conversion_components`] is a [`ComponentMapping`] struct specifying the
///   component mapping
/// - [`suggested_ycbcr_model`] is a [`SamplerYcbcrModelConversion`] value specifying the suggested
///   Y′C<sub>B</sub>C<sub>R</sub> model
/// - [`suggested_ycbcr_range`] is a [`SamplerYcbcrRange`] value specifying the suggested
///   Y′C<sub>B</sub>C<sub>R</sub> range
/// - [`suggested_x_chroma_offset`] is a [`ChromaLocation`] value specifying the suggested X chroma
///   offset
/// - [`suggested_y_chroma_offset`] is a [`ChromaLocation`] value specifying the suggested Y chroma
///   offset
///# Description
///`sysmemColorSpace` is only set for image-based buffer collections where
///the constraints were specified using [`ImageConstraintsInfoFUCHSIA`] in
///a call to [`set_buffer_collection_image_constraints_fuchsia`].For image-based buffer
/// collections, [`create_info_index`] will identify both
///the [`ImageConstraintsInfoFUCHSIA`]`::pImageCreateInfos` element and
///the [`ImageConstraintsInfoFUCHSIA::format_constraints`] element
///chosen by Sysmem when [`set_buffer_collection_image_constraints_fuchsia`] was
///called.
///The value of [`sysmem_color_space_index`] will be an index to one of the
///color spaces provided in the
///[`ImageFormatConstraintsInfoFUCHSIA::color_spaces`] array.The implementation must have
/// [`format_features`] with all bits set that
///were set in
///[`ImageFormatConstraintsInfoFUCHSIA::required_format_features`], by
///the call to [`set_buffer_collection_image_constraints_fuchsia`], at
///[`create_info_index`] (other bits could be set as well).
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
/// - [`format_features`] **must**  be a valid combination of [`FormatFeatureFlagBits`] values
/// - [`format_features`] **must**  not be `0`
/// - [`sysmem_color_space_index`] **must**  be a valid [`SysmemColorSpaceFUCHSIA`] structure
/// - [`sampler_ycbcr_conversion_components`] **must**  be a valid [`ComponentMapping`] structure
/// - [`suggested_ycbcr_model`] **must**  be a valid [`SamplerYcbcrModelConversion`] value
/// - [`suggested_ycbcr_range`] **must**  be a valid [`SamplerYcbcrRange`] value
/// - [`suggested_x_chroma_offset`] **must**  be a valid [`ChromaLocation`] value
/// - [`suggested_y_chroma_offset`] **must**  be a valid [`ChromaLocation`] value
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`ChromaLocation`]
/// - [`ComponentMapping`]
/// - [`FormatFeatureFlags`]
/// - [`SamplerYcbcrModelConversion`]
/// - [`SamplerYcbcrRange`]
/// - [`StructureType`]
/// - [`SysmemColorSpaceFUCHSIA`]
/// - [`get_buffer_collection_properties_fuchsia`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBufferCollectionPropertiesFUCHSIA")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct BufferCollectionPropertiesFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`memory_type_bits`] is a bitmask containing one bit set for every
    ///memory type which the buffer collection can be imported as buffer
    ///collection
    pub memory_type_bits: u32,
    ///[`buffer_count`] is the number of buffers in the collection
    pub buffer_count: u32,
    ///[`create_info_index`] as described in [Sysmem chosen create infos](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sysmem-chosen-create-infos)
    pub create_info_index: u32,
    ///[`sysmem_pixel_format`] is the Sysmem `PixelFormatType` as defined in
    ///`fuchsia.sysmem/image_formats.fidl`
    pub sysmem_pixel_format: u64,
    ///[`format_features`] is a bitmask of [`FormatFeatureFlagBits`]
    ///shared by the buffer collection
    pub format_features: FormatFeatureFlags,
    ///[`sysmem_color_space_index`] is a [`SysmemColorSpaceFUCHSIA`] struct
    ///specifying the color space
    pub sysmem_color_space_index: SysmemColorSpaceFUCHSIA<'lt>,
    ///[`sampler_ycbcr_conversion_components`] is a [`ComponentMapping`]
    ///struct specifying the component mapping
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    ///[`suggested_ycbcr_model`] is a [`SamplerYcbcrModelConversion`] value
    ///specifying the suggested Y′C<sub>B</sub>C<sub>R</sub> model
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    ///[`suggested_ycbcr_range`] is a [`SamplerYcbcrRange`] value
    ///specifying the suggested Y′C<sub>B</sub>C<sub>R</sub> range
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    ///[`suggested_x_chroma_offset`] is a [`ChromaLocation`] value
    ///specifying the suggested X chroma offset
    pub suggested_x_chroma_offset: ChromaLocation,
    ///[`suggested_y_chroma_offset`] is a [`ChromaLocation`] value
    ///specifying the suggested Y chroma offset
    pub suggested_y_chroma_offset: ChromaLocation,
}
impl<'lt> Default for BufferCollectionPropertiesFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BUFFER_COLLECTION_PROPERTIES_FUCHSIA,
            p_next: std::ptr::null_mut(),
            memory_type_bits: 0,
            buffer_count: 0,
            create_info_index: 0,
            sysmem_pixel_format: 0,
            format_features: Default::default(),
            sysmem_color_space_index: Default::default(),
            sampler_ycbcr_conversion_components: Default::default(),
            suggested_ycbcr_model: Default::default(),
            suggested_ycbcr_range: Default::default(),
            suggested_x_chroma_offset: Default::default(),
            suggested_y_chroma_offset: Default::default(),
        }
    }
}
impl<'lt> BufferCollectionPropertiesFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::memory_type_bits`]
    pub fn memory_type_bits(&self) -> u32 {
        self.memory_type_bits
    }
    ///Gets the value of [`Self::buffer_count`]
    pub fn buffer_count(&self) -> u32 {
        self.buffer_count
    }
    ///Gets the value of [`Self::create_info_index`]
    pub fn create_info_index(&self) -> u32 {
        self.create_info_index
    }
    ///Gets the value of [`Self::sysmem_pixel_format`]
    pub fn sysmem_pixel_format(&self) -> u64 {
        self.sysmem_pixel_format
    }
    ///Gets the value of [`Self::format_features`]
    pub fn format_features(&self) -> FormatFeatureFlags {
        self.format_features
    }
    ///Gets the value of [`Self::sysmem_color_space_index`]
    pub fn sysmem_color_space_index(&self) -> &SysmemColorSpaceFUCHSIA<'lt> {
        &self.sysmem_color_space_index
    }
    ///Gets the value of [`Self::sampler_ycbcr_conversion_components`]
    pub fn sampler_ycbcr_conversion_components(&self) -> ComponentMapping {
        self.sampler_ycbcr_conversion_components
    }
    ///Gets the value of [`Self::suggested_ycbcr_model`]
    pub fn suggested_ycbcr_model(&self) -> SamplerYcbcrModelConversion {
        self.suggested_ycbcr_model
    }
    ///Gets the value of [`Self::suggested_ycbcr_range`]
    pub fn suggested_ycbcr_range(&self) -> SamplerYcbcrRange {
        self.suggested_ycbcr_range
    }
    ///Gets the value of [`Self::suggested_x_chroma_offset`]
    pub fn suggested_x_chroma_offset(&self) -> ChromaLocation {
        self.suggested_x_chroma_offset
    }
    ///Gets the value of [`Self::suggested_y_chroma_offset`]
    pub fn suggested_y_chroma_offset(&self) -> ChromaLocation {
        self.suggested_y_chroma_offset
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::memory_type_bits`]
    pub fn memory_type_bits_mut(&mut self) -> &mut u32 {
        &mut self.memory_type_bits
    }
    ///Gets a mutable reference to the value of [`Self::buffer_count`]
    pub fn buffer_count_mut(&mut self) -> &mut u32 {
        &mut self.buffer_count
    }
    ///Gets a mutable reference to the value of [`Self::create_info_index`]
    pub fn create_info_index_mut(&mut self) -> &mut u32 {
        &mut self.create_info_index
    }
    ///Gets a mutable reference to the value of [`Self::sysmem_pixel_format`]
    pub fn sysmem_pixel_format_mut(&mut self) -> &mut u64 {
        &mut self.sysmem_pixel_format
    }
    ///Gets a mutable reference to the value of [`Self::format_features`]
    pub fn format_features_mut(&mut self) -> &mut FormatFeatureFlags {
        &mut self.format_features
    }
    ///Gets a mutable reference to the value of [`Self::sysmem_color_space_index`]
    pub fn sysmem_color_space_index_mut(&mut self) -> &mut SysmemColorSpaceFUCHSIA<'lt> {
        &mut self.sysmem_color_space_index
    }
    ///Gets a mutable reference to the value of [`Self::sampler_ycbcr_conversion_components`]
    pub fn sampler_ycbcr_conversion_components_mut(&mut self) -> &mut ComponentMapping {
        &mut self.sampler_ycbcr_conversion_components
    }
    ///Gets a mutable reference to the value of [`Self::suggested_ycbcr_model`]
    pub fn suggested_ycbcr_model_mut(&mut self) -> &mut SamplerYcbcrModelConversion {
        &mut self.suggested_ycbcr_model
    }
    ///Gets a mutable reference to the value of [`Self::suggested_ycbcr_range`]
    pub fn suggested_ycbcr_range_mut(&mut self) -> &mut SamplerYcbcrRange {
        &mut self.suggested_ycbcr_range
    }
    ///Gets a mutable reference to the value of [`Self::suggested_x_chroma_offset`]
    pub fn suggested_x_chroma_offset_mut(&mut self) -> &mut ChromaLocation {
        &mut self.suggested_x_chroma_offset
    }
    ///Gets a mutable reference to the value of [`Self::suggested_y_chroma_offset`]
    pub fn suggested_y_chroma_offset_mut(&mut self) -> &mut ChromaLocation {
        &mut self.suggested_y_chroma_offset
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::memory_type_bits`]
    pub fn set_memory_type_bits(&mut self, value: u32) -> &mut Self {
        self.memory_type_bits = value;
        self
    }
    ///Sets the value of [`Self::buffer_count`]
    pub fn set_buffer_count(&mut self, value: u32) -> &mut Self {
        self.buffer_count = value;
        self
    }
    ///Sets the value of [`Self::create_info_index`]
    pub fn set_create_info_index(&mut self, value: u32) -> &mut Self {
        self.create_info_index = value;
        self
    }
    ///Sets the value of [`Self::sysmem_pixel_format`]
    pub fn set_sysmem_pixel_format(&mut self, value: u64) -> &mut Self {
        self.sysmem_pixel_format = value;
        self
    }
    ///Sets the value of [`Self::format_features`]
    pub fn set_format_features(&mut self, value: crate::vulkan1_0::FormatFeatureFlags) -> &mut Self {
        self.format_features = value;
        self
    }
    ///Sets the value of [`Self::sysmem_color_space_index`]
    pub fn set_sysmem_color_space_index(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA<'lt>,
    ) -> &mut Self {
        self.sysmem_color_space_index = value;
        self
    }
    ///Sets the value of [`Self::sampler_ycbcr_conversion_components`]
    pub fn set_sampler_ycbcr_conversion_components(&mut self, value: crate::vulkan1_0::ComponentMapping) -> &mut Self {
        self.sampler_ycbcr_conversion_components = value;
        self
    }
    ///Sets the value of [`Self::suggested_ycbcr_model`]
    pub fn set_suggested_ycbcr_model(&mut self, value: crate::vulkan1_1::SamplerYcbcrModelConversion) -> &mut Self {
        self.suggested_ycbcr_model = value;
        self
    }
    ///Sets the value of [`Self::suggested_ycbcr_range`]
    pub fn set_suggested_ycbcr_range(&mut self, value: crate::vulkan1_1::SamplerYcbcrRange) -> &mut Self {
        self.suggested_ycbcr_range = value;
        self
    }
    ///Sets the value of [`Self::suggested_x_chroma_offset`]
    pub fn set_suggested_x_chroma_offset(&mut self, value: crate::vulkan1_1::ChromaLocation) -> &mut Self {
        self.suggested_x_chroma_offset = value;
        self
    }
    ///Sets the value of [`Self::suggested_y_chroma_offset`]
    pub fn set_suggested_y_chroma_offset(&mut self, value: crate::vulkan1_1::ChromaLocation) -> &mut Self {
        self.suggested_y_chroma_offset = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::memory_type_bits`]
    pub fn with_memory_type_bits(mut self, value: u32) -> Self {
        self.memory_type_bits = value;
        self
    }
    ///Sets the value of [`Self::buffer_count`]
    pub fn with_buffer_count(mut self, value: u32) -> Self {
        self.buffer_count = value;
        self
    }
    ///Sets the value of [`Self::create_info_index`]
    pub fn with_create_info_index(mut self, value: u32) -> Self {
        self.create_info_index = value;
        self
    }
    ///Sets the value of [`Self::sysmem_pixel_format`]
    pub fn with_sysmem_pixel_format(mut self, value: u64) -> Self {
        self.sysmem_pixel_format = value;
        self
    }
    ///Sets the value of [`Self::format_features`]
    pub fn with_format_features(mut self, value: crate::vulkan1_0::FormatFeatureFlags) -> Self {
        self.format_features = value;
        self
    }
    ///Sets the value of [`Self::sysmem_color_space_index`]
    pub fn with_sysmem_color_space_index(
        mut self,
        value: crate::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA<'lt>,
    ) -> Self {
        self.sysmem_color_space_index = value;
        self
    }
    ///Sets the value of [`Self::sampler_ycbcr_conversion_components`]
    pub fn with_sampler_ycbcr_conversion_components(mut self, value: crate::vulkan1_0::ComponentMapping) -> Self {
        self.sampler_ycbcr_conversion_components = value;
        self
    }
    ///Sets the value of [`Self::suggested_ycbcr_model`]
    pub fn with_suggested_ycbcr_model(mut self, value: crate::vulkan1_1::SamplerYcbcrModelConversion) -> Self {
        self.suggested_ycbcr_model = value;
        self
    }
    ///Sets the value of [`Self::suggested_ycbcr_range`]
    pub fn with_suggested_ycbcr_range(mut self, value: crate::vulkan1_1::SamplerYcbcrRange) -> Self {
        self.suggested_ycbcr_range = value;
        self
    }
    ///Sets the value of [`Self::suggested_x_chroma_offset`]
    pub fn with_suggested_x_chroma_offset(mut self, value: crate::vulkan1_1::ChromaLocation) -> Self {
        self.suggested_x_chroma_offset = value;
        self
    }
    ///Sets the value of [`Self::suggested_y_chroma_offset`]
    pub fn with_suggested_y_chroma_offset(mut self, value: crate::vulkan1_1::ChromaLocation) -> Self {
        self.suggested_y_chroma_offset = value;
        self
    }
}
///[VkBufferConstraintsInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferConstraintsInfoFUCHSIA.html) - Structure buffer-based buffer collection constraints
///# C Specifications
///The [`BufferConstraintsInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkBufferConstraintsInfoFUCHSIA {
///    VkStructureType                             sType;
///    const void*                                 pNext;
///    VkBufferCreateInfo                          createInfo;
///    VkFormatFeatureFlags                        requiredFormatFeatures;
///    VkBufferCollectionConstraintsInfoFUCHSIA    bufferCollectionConstraints;
///} VkBufferConstraintsInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - `pBufferCreateInfo` a pointer to a [`BufferCreateInfo`] struct describing the buffer
///   attributes for the buffer collection
/// - [`required_format_features`] bitmask of [`FormatFeatureFlagBits`] required features of the
///   buffers in the buffer collection
/// - [`buffer_collection_constraints`] is used to supply parameters for the negotiation and
///   allocation of the buffer collection
///# Description
///## Valid Usage
/// - The [`required_format_features`] bitmask of [`FormatFeatureFlagBits`] **must**  be chosen from
///   among the buffer compatible format features listed in [buffer compatible format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#buffer-compatible-format-features)
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
/// - [`create_info`] **must**  be a valid [`BufferCreateInfo`] structure
/// - [`required_format_features`] **must**  be a valid combination of [`FormatFeatureFlagBits`]
///   values
/// - [`buffer_collection_constraints`] **must**  be a valid
///   [`BufferCollectionConstraintsInfoFUCHSIA`] structure
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`BufferCollectionConstraintsInfoFUCHSIA`]
/// - [`BufferCreateInfo`]
/// - [`FormatFeatureFlags`]
/// - [`StructureType`]
/// - [`set_buffer_collection_buffer_constraints_fuchsia`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBufferConstraintsInfoFUCHSIA")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct BufferConstraintsInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    pub p_next: *const BaseInStructure<'lt>,
    ///No documentation found
    pub create_info: BufferCreateInfo<'lt>,
    ///[`required_format_features`] bitmask of [`FormatFeatureFlagBits`]
    ///required features of the buffers in the buffer collection
    pub required_format_features: FormatFeatureFlags,
    ///[`buffer_collection_constraints`] is used to supply parameters for the
    ///negotiation and allocation of the buffer collection
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'lt>,
}
impl<'lt> Default for BufferConstraintsInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BUFFER_CONSTRAINTS_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            create_info: Default::default(),
            required_format_features: Default::default(),
            buffer_collection_constraints: Default::default(),
        }
    }
}
impl<'lt> BufferConstraintsInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::create_info`]
    pub fn create_info(&self) -> &BufferCreateInfo<'lt> {
        &self.create_info
    }
    ///Gets the value of [`Self::required_format_features`]
    pub fn required_format_features(&self) -> FormatFeatureFlags {
        self.required_format_features
    }
    ///Gets the value of [`Self::buffer_collection_constraints`]
    pub fn buffer_collection_constraints(&self) -> &BufferCollectionConstraintsInfoFUCHSIA<'lt> {
        &self.buffer_collection_constraints
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::create_info`]
    pub fn create_info_mut(&mut self) -> &mut BufferCreateInfo<'lt> {
        &mut self.create_info
    }
    ///Gets a mutable reference to the value of [`Self::required_format_features`]
    pub fn required_format_features_mut(&mut self) -> &mut FormatFeatureFlags {
        &mut self.required_format_features
    }
    ///Gets a mutable reference to the value of [`Self::buffer_collection_constraints`]
    pub fn buffer_collection_constraints_mut(&mut self) -> &mut BufferCollectionConstraintsInfoFUCHSIA<'lt> {
        &mut self.buffer_collection_constraints
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::create_info`]
    pub fn set_create_info(&mut self, value: crate::vulkan1_0::BufferCreateInfo<'lt>) -> &mut Self {
        self.create_info = value;
        self
    }
    ///Sets the value of [`Self::required_format_features`]
    pub fn set_required_format_features(&mut self, value: crate::vulkan1_0::FormatFeatureFlags) -> &mut Self {
        self.required_format_features = value;
        self
    }
    ///Sets the value of [`Self::buffer_collection_constraints`]
    pub fn set_buffer_collection_constraints(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA<'lt>,
    ) -> &mut Self {
        self.buffer_collection_constraints = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::create_info`]
    pub fn with_create_info(mut self, value: crate::vulkan1_0::BufferCreateInfo<'lt>) -> Self {
        self.create_info = value;
        self
    }
    ///Sets the value of [`Self::required_format_features`]
    pub fn with_required_format_features(mut self, value: crate::vulkan1_0::FormatFeatureFlags) -> Self {
        self.required_format_features = value;
        self
    }
    ///Sets the value of [`Self::buffer_collection_constraints`]
    pub fn with_buffer_collection_constraints(
        mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA<'lt>,
    ) -> Self {
        self.buffer_collection_constraints = value;
        self
    }
}
///[VkSysmemColorSpaceFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSysmemColorSpaceFUCHSIA.html) - Structure describing the buffer collections color space
///# C Specifications
///The [`SysmemColorSpaceFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkSysmemColorSpaceFUCHSIA {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           colorSpace;
///} VkSysmemColorSpaceFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`color_space`] value of the Sysmem `ColorSpaceType`
///# Description
///## Valid Usage
/// - [`color_space`] **must**  be a `ColorSpaceType` as defined in
///   `fuchsia.sysmem/image_formats.fidl`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`BufferCollectionPropertiesFUCHSIA`]
/// - [`ImageFormatConstraintsInfoFUCHSIA`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSysmemColorSpaceFUCHSIA")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct SysmemColorSpaceFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    pub p_next: *const BaseInStructure<'lt>,
    ///[`color_space`] value of the Sysmem `ColorSpaceType`
    pub color_space: u32,
}
impl<'lt> Default for SysmemColorSpaceFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SYSMEM_COLOR_SPACE_FUCHSIA,
            p_next: std::ptr::null(),
            color_space: 0,
        }
    }
}
impl<'lt> SysmemColorSpaceFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::color_space`]
    pub fn color_space(&self) -> u32 {
        self.color_space
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::color_space`]
    pub fn color_space_mut(&mut self) -> &mut u32 {
        &mut self.color_space
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::color_space`]
    pub fn set_color_space(&mut self, value: u32) -> &mut Self {
        self.color_space = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::color_space`]
    pub fn with_color_space(mut self, value: u32) -> Self {
        self.color_space = value;
        self
    }
}
///[VkImageFormatConstraintsInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsInfoFUCHSIA.html) - Structure image-based buffer collection constraints
///# C Specifications
///The [`ImageFormatConstraintsInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkImageFormatConstraintsInfoFUCHSIA {
///    VkStructureType                         sType;
///    const void*                             pNext;
///    VkImageCreateInfo                       imageCreateInfo;
///    VkFormatFeatureFlags                    requiredFormatFeatures;
///    VkImageFormatConstraintsFlagsFUCHSIA    flags;
///    uint64_t                                sysmemPixelFormat;
///    uint32_t                                colorSpaceCount;
///    const VkSysmemColorSpaceFUCHSIA*        pColorSpaces;
///} VkImageFormatConstraintsInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`image_create_info`] is the [`ImageCreateInfo`] used to create a [`Image`] that is to use
///   memory from the [`BufferCollectionFUCHSIA`]
/// - [`required_format_features`] is a bitmask of [`FormatFeatureFlagBits`] specifying required
///   features of the buffers in the buffer collection
/// - [`flags`] is reserved for future use
/// - [`sysmem_pixel_format`] is a `PixelFormatType` value from the
///   `fuchsia.sysmem/image_formats.fidl` FIDL interface
/// - [`color_space_count`] the element count of [`color_spaces`]
/// - [`color_spaces`] is a pointer to an array of [`SysmemColorSpaceFUCHSIA`] structs of size
///   [`color_space_count`]
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
/// - [`image_create_info`] **must**  be a valid [`ImageCreateInfo`] structure
/// - [`required_format_features`] **must**  be a valid combination of [`FormatFeatureFlagBits`]
///   values
/// - [`required_format_features`] **must**  not be `0`
/// - [`flags`] **must**  be `0`
/// - [`color_spaces`] **must**  be a valid pointer to an array of [`color_space_count`] valid
///   [`SysmemColorSpaceFUCHSIA`] structures
/// - [`color_space_count`] **must**  be greater than `0`
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`FormatFeatureFlags`]
/// - [`ImageConstraintsInfoFUCHSIA`]
/// - [`ImageCreateInfo`]
/// - [`ImageFormatConstraintsFlagsFUCHSIA`]
/// - [`StructureType`]
/// - [`SysmemColorSpaceFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageFormatConstraintsInfoFUCHSIA")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ImageFormatConstraintsInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    pub p_next: *const BaseInStructure<'lt>,
    ///[`image_create_info`] is the [`ImageCreateInfo`] used to create a
    ///[`Image`] that is to use memory from the
    ///[`BufferCollectionFUCHSIA`]
    pub image_create_info: ImageCreateInfo<'lt>,
    ///[`required_format_features`] is a bitmask of
    ///[`FormatFeatureFlagBits`] specifying required features of the
    ///buffers in the buffer collection
    pub required_format_features: FormatFeatureFlags,
    ///[`flags`] is reserved for future use
    pub flags: ImageFormatConstraintsFlagsFUCHSIA,
    ///[`sysmem_pixel_format`] is a `PixelFormatType` value from the
    ///`fuchsia.sysmem/image_formats.fidl` FIDL interface
    pub sysmem_pixel_format: u64,
    ///[`color_space_count`] the element count of [`color_spaces`]
    pub color_space_count: u32,
    ///[`color_spaces`] is a pointer to an array of
    ///[`SysmemColorSpaceFUCHSIA`] structs of size [`color_space_count`]
    pub color_spaces: *const SysmemColorSpaceFUCHSIA<'lt>,
}
impl<'lt> Default for ImageFormatConstraintsInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            image_create_info: Default::default(),
            required_format_features: Default::default(),
            flags: Default::default(),
            sysmem_pixel_format: 0,
            color_space_count: 0,
            color_spaces: std::ptr::null(),
        }
    }
}
impl<'lt> ImageFormatConstraintsInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::color_spaces`]
    pub fn color_spaces_raw(&self) -> *const SysmemColorSpaceFUCHSIA<'lt> {
        self.color_spaces
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::color_spaces`]
    pub fn set_color_spaces_raw(&mut self, value: *const SysmemColorSpaceFUCHSIA<'lt>) -> &mut Self {
        self.color_spaces = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::color_spaces`]
    pub fn with_color_spaces_raw(mut self, value: *const SysmemColorSpaceFUCHSIA<'lt>) -> Self {
        self.color_spaces = value;
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
    ///Gets the value of [`Self::image_create_info`]
    pub fn image_create_info(&self) -> &ImageCreateInfo<'lt> {
        &self.image_create_info
    }
    ///Gets the value of [`Self::required_format_features`]
    pub fn required_format_features(&self) -> FormatFeatureFlags {
        self.required_format_features
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> ImageFormatConstraintsFlagsFUCHSIA {
        self.flags
    }
    ///Gets the value of [`Self::sysmem_pixel_format`]
    pub fn sysmem_pixel_format(&self) -> u64 {
        self.sysmem_pixel_format
    }
    ///Gets the value of [`Self::color_space_count`]
    pub fn color_space_count(&self) -> u32 {
        self.color_space_count
    }
    ///Gets the value of [`Self::color_spaces`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn color_spaces(&self) -> &[SysmemColorSpaceFUCHSIA<'lt>] {
        std::slice::from_raw_parts(self.color_spaces, self.color_space_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::image_create_info`]
    pub fn image_create_info_mut(&mut self) -> &mut ImageCreateInfo<'lt> {
        &mut self.image_create_info
    }
    ///Gets a mutable reference to the value of [`Self::required_format_features`]
    pub fn required_format_features_mut(&mut self) -> &mut FormatFeatureFlags {
        &mut self.required_format_features
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ImageFormatConstraintsFlagsFUCHSIA {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::sysmem_pixel_format`]
    pub fn sysmem_pixel_format_mut(&mut self) -> &mut u64 {
        &mut self.sysmem_pixel_format
    }
    ///Gets a mutable reference to the value of [`Self::color_space_count`]
    pub fn color_space_count_mut(&mut self) -> &mut u32 {
        &mut self.color_space_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::image_create_info`]
    pub fn set_image_create_info(&mut self, value: crate::vulkan1_0::ImageCreateInfo<'lt>) -> &mut Self {
        self.image_create_info = value;
        self
    }
    ///Sets the value of [`Self::required_format_features`]
    pub fn set_required_format_features(&mut self, value: crate::vulkan1_0::FormatFeatureFlags) -> &mut Self {
        self.required_format_features = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::ImageFormatConstraintsFlagsFUCHSIA,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::sysmem_pixel_format`]
    pub fn set_sysmem_pixel_format(&mut self, value: u64) -> &mut Self {
        self.sysmem_pixel_format = value;
        self
    }
    ///Sets the value of [`Self::color_space_count`]
    pub fn set_color_space_count(&mut self, value: u32) -> &mut Self {
        self.color_space_count = value;
        self
    }
    ///Sets the value of [`Self::color_spaces`]
    pub fn set_color_spaces(
        &mut self,
        value: &'lt [crate::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.color_spaces = value.as_ptr();
        self.color_space_count = len_;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::image_create_info`]
    pub fn with_image_create_info(mut self, value: crate::vulkan1_0::ImageCreateInfo<'lt>) -> Self {
        self.image_create_info = value;
        self
    }
    ///Sets the value of [`Self::required_format_features`]
    pub fn with_required_format_features(mut self, value: crate::vulkan1_0::FormatFeatureFlags) -> Self {
        self.required_format_features = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(
        mut self,
        value: crate::extensions::fuchsia_buffer_collection::ImageFormatConstraintsFlagsFUCHSIA,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::sysmem_pixel_format`]
    pub fn with_sysmem_pixel_format(mut self, value: u64) -> Self {
        self.sysmem_pixel_format = value;
        self
    }
    ///Sets the value of [`Self::color_space_count`]
    pub fn with_color_space_count(mut self, value: u32) -> Self {
        self.color_space_count = value;
        self
    }
    ///Sets the value of [`Self::color_spaces`]
    pub fn with_color_spaces(
        mut self,
        value: &'lt [crate::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.color_spaces = value.as_ptr();
        self.color_space_count = len_;
        self
    }
}
///[VkImageConstraintsInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFUCHSIA.html) - Structure of image-based buffer collection constraints
///# C Specifications
///The [`ImageConstraintsInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkImageConstraintsInfoFUCHSIA {
///    VkStructureType                               sType;
///    const void*                                   pNext;
///    uint32_t                                      formatConstraintsCount;
///    const VkImageFormatConstraintsInfoFUCHSIA*    pFormatConstraints;
///    VkBufferCollectionConstraintsInfoFUCHSIA      bufferCollectionConstraints;
///    VkImageConstraintsInfoFlagsFUCHSIA            flags;
///} VkImageConstraintsInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format_constraints_count`] is the number of elements in [`format_constraints`].
/// - [`format_constraints`] is a pointer to an array of [`ImageFormatConstraintsInfoFUCHSIA`]
///   structures of size [`format_constraints_count`] that is used to further constrain buffer
///   collection format selection for image-based buffer collections.
/// - [`buffer_collection_constraints`] is a [`BufferCollectionConstraintsInfoFUCHSIA`] structure
///   used to supply parameters for the negotiation and allocation for buffer-based buffer
///   collections.
/// - [`flags`] is a [`ImageConstraintsInfoFlagBitsFUCHSIA`] value specifying hints about the type
///   of memory Sysmem should allocate for the buffer collection.
///# Description
///## Valid Usage
/// - All elements of [`format_constraints`] **must**  have at least one bit set in its
///   [`ImageFormatConstraintsInfoFUCHSIA::required_format_features`]
/// - If [`format_constraints`]`::imageCreateInfo`::`usage` contains `VK_IMAGE_USAGE_SAMPLED_BIT`,
///   then [`format_constraints`]`::requiredFormatFeatures` **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`
/// - If [`format_constraints`]`::imageCreateInfo`::`usage` contains `VK_IMAGE_USAGE_STORAGE_BIT`,
///   then [`format_constraints`]`::requiredFormatFeatures` **must**  contain
///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT`
/// - If [`format_constraints`]`::imageCreateInfo`::`usage` contains
///   `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`, then [`format_constraints`]`::requiredFormatFeatures`
///   **must**  contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
/// - If [`format_constraints`]`::imageCreateInfo`::`usage` contains
///   `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, then
///   [`format_constraints`]`::requiredFormatFeatures` **must**  contain
///   `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`format_constraints`]`::imageCreateInfo`::`usage` contains
///   `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`, then [`format_constraints`]`::requiredFormatFeatures`
///   **must**  contain at least one of `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT` or
///   `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If the [`attachmentFragmentShadingRate` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate)
///   is enabled, and [`format_constraints`]`::imageCreateInfo`::`usage` contains
///   `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`, then
///   [`format_constraints`]`::requiredFormatFeatures` **must**  contain
///   `VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
/// - [`format_constraints`] **must**  be a valid pointer to an array of
///   [`format_constraints_count`] valid [`ImageFormatConstraintsInfoFUCHSIA`] structures
/// - [`buffer_collection_constraints`] **must**  be a valid
///   [`BufferCollectionConstraintsInfoFUCHSIA`] structure
/// - [`flags`] **must**  be a valid combination of [`ImageConstraintsInfoFlagBitsFUCHSIA`] values
/// - [`format_constraints_count`] **must**  be greater than `0`
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`BufferCollectionConstraintsInfoFUCHSIA`]
/// - [`ImageConstraintsInfoFlagsFUCHSIA`]
/// - [`ImageFormatConstraintsInfoFUCHSIA`]
/// - [`StructureType`]
/// - [`set_buffer_collection_image_constraints_fuchsia`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageConstraintsInfoFUCHSIA")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ImageConstraintsInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`format_constraints_count`] is the number of elements in
    ///[`format_constraints`].
    pub format_constraints_count: u32,
    ///[`format_constraints`] is a pointer to an array of
    ///[`ImageFormatConstraintsInfoFUCHSIA`] structures of size
    ///[`format_constraints_count`] that is used to further constrain buffer
    ///collection format selection for image-based buffer collections.
    pub format_constraints: *const ImageFormatConstraintsInfoFUCHSIA<'lt>,
    ///[`buffer_collection_constraints`] is a
    ///[`BufferCollectionConstraintsInfoFUCHSIA`] structure used to supply
    ///parameters for the negotiation and allocation for buffer-based buffer
    ///collections.
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'lt>,
    ///[`flags`] is a [`ImageConstraintsInfoFlagBitsFUCHSIA`] value
    ///specifying hints about the type of memory Sysmem should allocate for the
    ///buffer collection.
    pub flags: ImageConstraintsInfoFlagsFUCHSIA,
}
impl<'lt> Default for ImageConstraintsInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMAGE_CONSTRAINTS_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            format_constraints_count: 0,
            format_constraints: std::ptr::null(),
            buffer_collection_constraints: Default::default(),
            flags: Default::default(),
        }
    }
}
impl<'lt> ImageConstraintsInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::format_constraints`]
    pub fn format_constraints_raw(&self) -> *const ImageFormatConstraintsInfoFUCHSIA<'lt> {
        self.format_constraints
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::format_constraints`]
    pub fn set_format_constraints_raw(&mut self, value: *const ImageFormatConstraintsInfoFUCHSIA<'lt>) -> &mut Self {
        self.format_constraints = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::format_constraints`]
    pub fn with_format_constraints_raw(mut self, value: *const ImageFormatConstraintsInfoFUCHSIA<'lt>) -> Self {
        self.format_constraints = value;
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
    ///Gets the value of [`Self::format_constraints_count`]
    pub fn format_constraints_count(&self) -> u32 {
        self.format_constraints_count
    }
    ///Gets the value of [`Self::format_constraints`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn format_constraints(&self) -> &[ImageFormatConstraintsInfoFUCHSIA<'lt>] {
        std::slice::from_raw_parts(self.format_constraints, self.format_constraints_count as usize)
    }
    ///Gets the value of [`Self::buffer_collection_constraints`]
    pub fn buffer_collection_constraints(&self) -> &BufferCollectionConstraintsInfoFUCHSIA<'lt> {
        &self.buffer_collection_constraints
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> ImageConstraintsInfoFlagsFUCHSIA {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::format_constraints_count`]
    pub fn format_constraints_count_mut(&mut self) -> &mut u32 {
        &mut self.format_constraints_count
    }
    ///Gets a mutable reference to the value of [`Self::buffer_collection_constraints`]
    pub fn buffer_collection_constraints_mut(&mut self) -> &mut BufferCollectionConstraintsInfoFUCHSIA<'lt> {
        &mut self.buffer_collection_constraints
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ImageConstraintsInfoFlagsFUCHSIA {
        &mut self.flags
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::format_constraints_count`]
    pub fn set_format_constraints_count(&mut self, value: u32) -> &mut Self {
        self.format_constraints_count = value;
        self
    }
    ///Sets the value of [`Self::format_constraints`]
    pub fn set_format_constraints(
        &mut self,
        value: &'lt [crate::extensions::fuchsia_buffer_collection::ImageFormatConstraintsInfoFUCHSIA<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.format_constraints = value.as_ptr();
        self.format_constraints_count = len_;
        self
    }
    ///Sets the value of [`Self::buffer_collection_constraints`]
    pub fn set_buffer_collection_constraints(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA<'lt>,
    ) -> &mut Self {
        self.buffer_collection_constraints = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFlagsFUCHSIA,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::format_constraints_count`]
    pub fn with_format_constraints_count(mut self, value: u32) -> Self {
        self.format_constraints_count = value;
        self
    }
    ///Sets the value of [`Self::format_constraints`]
    pub fn with_format_constraints(
        mut self,
        value: &'lt [crate::extensions::fuchsia_buffer_collection::ImageFormatConstraintsInfoFUCHSIA<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.format_constraints = value.as_ptr();
        self.format_constraints_count = len_;
        self
    }
    ///Sets the value of [`Self::buffer_collection_constraints`]
    pub fn with_buffer_collection_constraints(
        mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA<'lt>,
    ) -> Self {
        self.buffer_collection_constraints = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(
        mut self,
        value: crate::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFlagsFUCHSIA,
    ) -> Self {
        self.flags = value;
        self
    }
}
///[VkBufferCollectionConstraintsInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionConstraintsInfoFUCHSIA.html) - Structure of general buffer collection constraints
///# C Specifications
///The [`BufferCollectionConstraintsInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkBufferCollectionConstraintsInfoFUCHSIA {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           minBufferCount;
///    uint32_t           maxBufferCount;
///    uint32_t           minBufferCountForCamping;
///    uint32_t           minBufferCountForDedicatedSlack;
///    uint32_t           minBufferCountForSharedSlack;
///} VkBufferCollectionConstraintsInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`min_buffer_count`] is the minimum number of buffers available in the collection
/// - [`max_buffer_count`] is the maximum number of buffers allowed in the collection
/// - [`min_buffer_count_for_camping`] is the per-participant minimum buffers for camping
/// - [`min_buffer_count_for_dedicated_slack`] is the per-participant minimum buffers for dedicated
///   slack
/// - [`min_buffer_count_for_shared_slack`] is the per-participant minimum buffers for shared slack
///# Description
///Sysmem uses all buffer count parameters in combination to determine the
///number of buffers it will allocate.
///Sysmem defines buffer count constraints in
///`fuchsia.sysmem/constraints.fidl`.*Camping* as referred to by [`min_buffer_count_for_camping`],
/// is the number of
///buffers that should be available for the participant that are not for
///transient use.
///This number of buffers is required for the participant to logically operate.*Slack* as referred
/// to by [`min_buffer_count_for_dedicated_slack`] and
///[`min_buffer_count_for_shared_slack`], refers to the number of buffers desired
///by participants for optimal performance.
///[`min_buffer_count_for_dedicated_slack`] refers to the current participant.
///[`min_buffer_count_for_shared_slack`] refers to buffer slack for all
///participants in the collection.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`BufferConstraintsInfoFUCHSIA`]
/// - [`ImageConstraintsInfoFUCHSIA`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBufferCollectionConstraintsInfoFUCHSIA")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct BufferCollectionConstraintsInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    pub p_next: *const BaseInStructure<'lt>,
    ///[`min_buffer_count`] is the minimum number of buffers available in the
    ///collection
    pub min_buffer_count: u32,
    ///[`max_buffer_count`] is the maximum number of buffers allowed in the
    ///collection
    pub max_buffer_count: u32,
    ///[`min_buffer_count_for_camping`] is the per-participant minimum buffers
    ///for camping
    pub min_buffer_count_for_camping: u32,
    ///[`min_buffer_count_for_dedicated_slack`] is the per-participant minimum
    ///buffers for dedicated slack
    pub min_buffer_count_for_dedicated_slack: u32,
    ///[`min_buffer_count_for_shared_slack`] is the per-participant minimum
    ///buffers for shared slack
    pub min_buffer_count_for_shared_slack: u32,
}
impl<'lt> Default for BufferCollectionConstraintsInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            min_buffer_count: 0,
            max_buffer_count: 0,
            min_buffer_count_for_camping: 0,
            min_buffer_count_for_dedicated_slack: 0,
            min_buffer_count_for_shared_slack: 0,
        }
    }
}
impl<'lt> BufferCollectionConstraintsInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::min_buffer_count`]
    pub fn min_buffer_count(&self) -> u32 {
        self.min_buffer_count
    }
    ///Gets the value of [`Self::max_buffer_count`]
    pub fn max_buffer_count(&self) -> u32 {
        self.max_buffer_count
    }
    ///Gets the value of [`Self::min_buffer_count_for_camping`]
    pub fn min_buffer_count_for_camping(&self) -> u32 {
        self.min_buffer_count_for_camping
    }
    ///Gets the value of [`Self::min_buffer_count_for_dedicated_slack`]
    pub fn min_buffer_count_for_dedicated_slack(&self) -> u32 {
        self.min_buffer_count_for_dedicated_slack
    }
    ///Gets the value of [`Self::min_buffer_count_for_shared_slack`]
    pub fn min_buffer_count_for_shared_slack(&self) -> u32 {
        self.min_buffer_count_for_shared_slack
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::min_buffer_count`]
    pub fn min_buffer_count_mut(&mut self) -> &mut u32 {
        &mut self.min_buffer_count
    }
    ///Gets a mutable reference to the value of [`Self::max_buffer_count`]
    pub fn max_buffer_count_mut(&mut self) -> &mut u32 {
        &mut self.max_buffer_count
    }
    ///Gets a mutable reference to the value of [`Self::min_buffer_count_for_camping`]
    pub fn min_buffer_count_for_camping_mut(&mut self) -> &mut u32 {
        &mut self.min_buffer_count_for_camping
    }
    ///Gets a mutable reference to the value of [`Self::min_buffer_count_for_dedicated_slack`]
    pub fn min_buffer_count_for_dedicated_slack_mut(&mut self) -> &mut u32 {
        &mut self.min_buffer_count_for_dedicated_slack
    }
    ///Gets a mutable reference to the value of [`Self::min_buffer_count_for_shared_slack`]
    pub fn min_buffer_count_for_shared_slack_mut(&mut self) -> &mut u32 {
        &mut self.min_buffer_count_for_shared_slack
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::min_buffer_count`]
    pub fn set_min_buffer_count(&mut self, value: u32) -> &mut Self {
        self.min_buffer_count = value;
        self
    }
    ///Sets the value of [`Self::max_buffer_count`]
    pub fn set_max_buffer_count(&mut self, value: u32) -> &mut Self {
        self.max_buffer_count = value;
        self
    }
    ///Sets the value of [`Self::min_buffer_count_for_camping`]
    pub fn set_min_buffer_count_for_camping(&mut self, value: u32) -> &mut Self {
        self.min_buffer_count_for_camping = value;
        self
    }
    ///Sets the value of [`Self::min_buffer_count_for_dedicated_slack`]
    pub fn set_min_buffer_count_for_dedicated_slack(&mut self, value: u32) -> &mut Self {
        self.min_buffer_count_for_dedicated_slack = value;
        self
    }
    ///Sets the value of [`Self::min_buffer_count_for_shared_slack`]
    pub fn set_min_buffer_count_for_shared_slack(&mut self, value: u32) -> &mut Self {
        self.min_buffer_count_for_shared_slack = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::min_buffer_count`]
    pub fn with_min_buffer_count(mut self, value: u32) -> Self {
        self.min_buffer_count = value;
        self
    }
    ///Sets the value of [`Self::max_buffer_count`]
    pub fn with_max_buffer_count(mut self, value: u32) -> Self {
        self.max_buffer_count = value;
        self
    }
    ///Sets the value of [`Self::min_buffer_count_for_camping`]
    pub fn with_min_buffer_count_for_camping(mut self, value: u32) -> Self {
        self.min_buffer_count_for_camping = value;
        self
    }
    ///Sets the value of [`Self::min_buffer_count_for_dedicated_slack`]
    pub fn with_min_buffer_count_for_dedicated_slack(mut self, value: u32) -> Self {
        self.min_buffer_count_for_dedicated_slack = value;
        self
    }
    ///Sets the value of [`Self::min_buffer_count_for_shared_slack`]
    pub fn with_min_buffer_count_for_shared_slack(mut self, value: u32) -> Self {
        self.min_buffer_count_for_shared_slack = value;
        self
    }
}
impl Device {
    ///[vkCreateBufferCollectionFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html) - Create a new buffer collection
    ///# C Specifications
    ///To create an [`BufferCollectionFUCHSIA`] for Vulkan to participate in
    ///the buffer collection:
    ///```c
    ///// Provided by VK_FUCHSIA_buffer_collection
    ///VkResult vkCreateBufferCollectionFUCHSIA(
    ///    VkDevice                                    device,
    ///    const VkBufferCollectionCreateInfoFUCHSIA*  pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkBufferCollectionFUCHSIA*                  pCollection);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that creates the [`BufferCollectionFUCHSIA`]
    /// - [`p_create_info`] is a pointer to a [`BufferCollectionCreateInfoFUCHSIA`] structure
    ///   containing parameters affecting creation of the buffer collection
    /// - [`p_allocator`] is a pointer to a [`AllocationCallbacks`] structure controlling host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter
    /// - `pBufferCollection` is a pointer to a [`BufferCollectionFUCHSIA`] handle in which the
    ///   resulting buffer collection object is returned
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid
    ///   [`BufferCollectionCreateInfoFUCHSIA`] structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_collection`] **must**  be a valid pointer to a [`BufferCollectionFUCHSIA`] handle
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`  -
    ///   `VK_ERROR_INITIALIZATION_FAILED`
    ///
    ///## Host AccessAll functions referencing a [`BufferCollectionFUCHSIA`] **must**  be
    ///externally synchronized with the exception of
    ///[`create_buffer_collection_fuchsia`].
    ///# Related
    /// - [`fuchsia_buffer_collection`]
    /// - [`AllocationCallbacks`]
    /// - [`BufferCollectionCreateInfoFUCHSIA`]
    /// - [`BufferCollectionFUCHSIA`]
    /// - [`Device`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_buffer_collection_fuchsia<'lt>(
        self: &Unique<Device>,
        p_create_info: &BufferCollectionCreateInfoFUCHSIA<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<BufferCollectionFUCHSIA>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .fuchsia_buffer_collection()
            .and_then(|vtable| vtable.create_buffer_collection_fuchsia())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .fuchsia_buffer_collection()
            .and_then(|vtable| vtable.create_buffer_collection_fuchsia())
            .unwrap_unchecked();
        let mut p_collection = MaybeUninit::<BufferCollectionFUCHSIA>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const BufferCollectionCreateInfoFUCHSIA<'lt>,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_collection.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(
                _return,
                Unique::new(self, p_collection.assume_init(), AtomicBool::default()),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkSetBufferCollectionBufferConstraintsFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html) - Set buffer-based constraints for a buffer collection
    ///# C Specifications
    ///To set the constraints on a [`Buffer`] buffer collection, call:
    ///```c
    ///// Provided by VK_FUCHSIA_buffer_collection
    ///VkResult vkSetBufferCollectionBufferConstraintsFUCHSIA(
    ///    VkDevice                                    device,
    ///    VkBufferCollectionFUCHSIA                   collection,
    ///    const VkBufferConstraintsInfoFUCHSIA*       pBufferConstraintsInfo);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device
    /// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
    /// - [`p_buffer_constraints_info`] is a pointer to a [`BufferConstraintsInfoFUCHSIA`] structure
    ///# Description
    ///[`set_buffer_collection_buffer_constraints_fuchsia`] **may**  fail if the
    ///implementation does not support the constraints specified in the
    ///`bufferCollectionConstraints` structure.
    ///If that occurs, [`set_buffer_collection_buffer_constraints_fuchsia`] will
    ///return `VK_ERROR_FORMAT_NOT_SUPPORTED`.
    ///## Valid Usage
    /// - [`set_buffer_collection_image_constraints_fuchsia`] or
    ///   [`set_buffer_collection_buffer_constraints_fuchsia`] **must**  not have already been
    ///   called on [`collection`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
    /// - [`p_buffer_constraints_info`] **must**  be a valid pointer to a valid
    ///   [`BufferConstraintsInfoFUCHSIA`] structure
    /// - [`collection`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_OUT_OF_HOST_MEMORY`  -
    ///   `VK_ERROR_FORMAT_NOT_SUPPORTED`
    ///# Related
    /// - [`fuchsia_buffer_collection`]
    /// - [`BufferCollectionFUCHSIA`]
    /// - [`BufferConstraintsInfoFUCHSIA`]
    /// - [`Device`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
    #[track_caller]
    #[inline]
    pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia<'lt>(
        self: &Unique<Device>,
        collection: BufferCollectionFUCHSIA,
        p_buffer_constraints_info: &BufferConstraintsInfoFUCHSIA<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .fuchsia_buffer_collection()
            .and_then(|vtable| vtable.set_buffer_collection_buffer_constraints_fuchsia())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .fuchsia_buffer_collection()
            .and_then(|vtable| vtable.set_buffer_collection_buffer_constraints_fuchsia())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            collection,
            p_buffer_constraints_info as *const BufferConstraintsInfoFUCHSIA<'lt>,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkSetBufferCollectionImageConstraintsFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html) - Set image-based constraints for a buffer collection
    ///# C Specifications
    ///Setting the constraints on the buffer collection initiates the format
    ///negotiation and allocation of the buffer collection.
    ///To set the constraints on a [`Image`] buffer collection, call:
    ///```c
    ///// Provided by VK_FUCHSIA_buffer_collection
    ///VkResult vkSetBufferCollectionImageConstraintsFUCHSIA(
    ///    VkDevice                                    device,
    ///    VkBufferCollectionFUCHSIA                   collection,
    ///    const VkImageConstraintsInfoFUCHSIA*        pImageConstraintsInfo);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device
    /// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
    /// - [`p_image_constraints_info`] is a pointer to a [`ImageConstraintsInfoFUCHSIA`] structure
    ///# Description
    ///[`set_buffer_collection_image_constraints_fuchsia`] **may**  fail if
    ///[`p_image_constraints_info`]`::formatConstraintsCount` is larger than the
    ///implementation-defined limit.
    ///If that occurs, [`set_buffer_collection_image_constraints_fuchsia`] will
    ///return VK_ERROR_INITIALIZATION_FAILED.[`set_buffer_collection_image_constraints_fuchsia`]
    /// **may**  fail if the
    ///implementation does not support any of the formats described by the
    ///[`p_image_constraints_info`] structure.
    ///If that occurs, [`set_buffer_collection_image_constraints_fuchsia`] will
    ///return `VK_ERROR_FORMAT_NOT_SUPPORTED`.
    ///## Valid Usage
    /// - [`set_buffer_collection_image_constraints_fuchsia`] or
    ///   [`set_buffer_collection_buffer_constraints_fuchsia`] **must**  not have already been
    ///   called on [`collection`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
    /// - [`p_image_constraints_info`] **must**  be a valid pointer to a valid
    ///   [`ImageConstraintsInfoFUCHSIA`] structure
    /// - [`collection`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_OUT_OF_HOST_MEMORY`  -
    ///   `VK_ERROR_FORMAT_NOT_SUPPORTED`
    ///# Related
    /// - [`fuchsia_buffer_collection`]
    /// - [`BufferCollectionFUCHSIA`]
    /// - [`Device`]
    /// - [`ImageConstraintsInfoFUCHSIA`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
    #[track_caller]
    #[inline]
    pub unsafe fn set_buffer_collection_image_constraints_fuchsia<'lt>(
        self: &Unique<Device>,
        collection: BufferCollectionFUCHSIA,
        p_image_constraints_info: &ImageConstraintsInfoFUCHSIA<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .fuchsia_buffer_collection()
            .and_then(|vtable| vtable.set_buffer_collection_image_constraints_fuchsia())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .fuchsia_buffer_collection()
            .and_then(|vtable| vtable.set_buffer_collection_image_constraints_fuchsia())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            collection,
            p_image_constraints_info as *const ImageConstraintsInfoFUCHSIA<'lt>,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkDestroyBufferCollectionFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html) - Destroy a buffer collection
    ///# C Specifications
    ///To release a [`BufferCollectionFUCHSIA`]:
    ///```c
    ///// Provided by VK_FUCHSIA_buffer_collection
    ///void vkDestroyBufferCollectionFUCHSIA(
    ///    VkDevice                                    device,
    ///    VkBufferCollectionFUCHSIA                   collection,
    ///    const VkAllocationCallbacks*                pAllocator);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that creates the [`BufferCollectionFUCHSIA`]
    /// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
    /// - [`p_allocator`] is a pointer to a [`AllocationCallbacks`] structure controlling host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter
    ///# Description
    ///## Valid Usage
    /// - [`Image`] and [`Buffer`] objects that referenced [`collection`] upon creation by inclusion
    ///   of a [`BufferCollectionImageCreateInfoFUCHSIA`] or
    ///   [`BufferCollectionBufferCreateInfoFUCHSIA`] chained to their [`ImageCreateInfo`] or
    ///   [`BufferCreateInfo`] structures respectively,  **may**  outlive [`collection`].
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`collection`] **must**  have been created, allocated, or retrieved from [`device`]
    ///# Related
    /// - [`fuchsia_buffer_collection`]
    /// - [`AllocationCallbacks`]
    /// - [`BufferCollectionFUCHSIA`]
    /// - [`Device`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
    #[track_caller]
    #[inline]
    pub unsafe fn destroy_buffer_collection_fuchsia<'lt>(
        self: &Unique<Device>,
        collection: BufferCollectionFUCHSIA,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .fuchsia_buffer_collection()
            .and_then(|vtable| vtable.destroy_buffer_collection_fuchsia())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .fuchsia_buffer_collection()
            .and_then(|vtable| vtable.destroy_buffer_collection_fuchsia())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            collection,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl Device {
    ///[vkGetBufferCollectionPropertiesFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html) - Retrieve properties from a buffer collection
    ///# C Specifications
    ///After constraints have been set on the buffer collection by calling
    ///[`set_buffer_collection_image_constraints_fuchsia`] or
    ///[`set_buffer_collection_buffer_constraints_fuchsia`], call
    ///[`get_buffer_collection_properties_fuchsia`] to retrieve the negotiated and
    ///finalized properties of the buffer collection.The call to
    /// [`get_buffer_collection_properties_fuchsia`] is synchronous.
    ///It waits for the Sysmem format negotiation and buffer collection allocation
    ///to complete before returning.
    ///```c
    ///// Provided by VK_FUCHSIA_buffer_collection
    ///VkResult vkGetBufferCollectionPropertiesFUCHSIA(
    ///    VkDevice                                    device,
    ///    VkBufferCollectionFUCHSIA                   collection,
    ///    VkBufferCollectionPropertiesFUCHSIA*        pProperties);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device handle
    /// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
    /// - [`p_properties`] is a pointer to the retrieved [`BufferCollectionPropertiesFUCHSIA`]
    ///   struct
    ///# Description
    ///For image-based buffer collections, upon calling
    ///[`get_buffer_collection_properties_fuchsia`], Sysmem will choose an element
    ///of the [`ImageConstraintsInfoFUCHSIA`]`::pImageCreateInfos`
    ///established by the preceding call to
    ///[`set_buffer_collection_image_constraints_fuchsia`].
    ///The index of the element chosen is stored in and can be retrieved from
    ///[`BufferCollectionPropertiesFUCHSIA::create_info_index`].For buffer-based buffer
    /// collections, a single [`BufferCreateInfo`] is
    ///specified as [`BufferConstraintsInfoFUCHSIA::create_info`].
    ///[`BufferCollectionPropertiesFUCHSIA::create_info_index`] will
    ///therefore always be zero.[`get_buffer_collection_properties_fuchsia`] **may**  fail if
    /// Sysmem is unable
    ///to resolve the constraints of all of the participants in the buffer
    ///collection.
    ///If that occurs, [`get_buffer_collection_properties_fuchsia`] will return
    ///`VK_ERROR_INITIALIZATION_FAILED`.
    ///## Valid Usage
    /// - Prior to calling [`get_buffer_collection_properties_fuchsia`], the constraints on the
    ///   buffer collection  **must**  have been set by either
    ///   [`set_buffer_collection_image_constraints_fuchsia`] or
    ///   [`set_buffer_collection_buffer_constraints_fuchsia`].
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
    /// - [`p_properties`] **must**  be a valid pointer to a [`BufferCollectionPropertiesFUCHSIA`]
    ///   structure
    /// - [`collection`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`
    ///# Related
    /// - [`fuchsia_buffer_collection`]
    /// - [`BufferCollectionFUCHSIA`]
    /// - [`BufferCollectionPropertiesFUCHSIA`]
    /// - [`Device`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_buffer_collection_properties_fuchsia<'lt>(
        self: &Unique<Device>,
        collection: BufferCollectionFUCHSIA,
        p_properties: Option<BufferCollectionPropertiesFUCHSIA<'lt>>,
    ) -> VulkanResult<BufferCollectionPropertiesFUCHSIA<'lt>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .fuchsia_buffer_collection()
            .and_then(|vtable| vtable.get_buffer_collection_properties_fuchsia())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .fuchsia_buffer_collection()
            .and_then(|vtable| vtable.get_buffer_collection_properties_fuchsia())
            .unwrap_unchecked();
        let mut p_properties = p_properties.unwrap_or_default();
        let _return = _function(self.as_raw(), collection, &mut p_properties);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, {
                p_properties.p_next = std::ptr::null_mut();
                p_properties
            }),
            e => VulkanResult::Err(e),
        }
    }
}
///[VkBufferCollectionFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionFUCHSIA.html) - Opaque handle to a buffer collection object
///# C Specifications
///Fuchsia’s FIDL-based Sysmem service interoperates with Vulkan via the
///`[`fuchsia_buffer_collection`]` extension.A buffer collection is a set of one or more buffers
/// which were allocated
///together as a group and which all have the same properties.
///These properties describe the buffers' internal representation, such as its
///dimensions and memory layout.
///This ensures that all of the buffers can be used interchangeably by tasks
///that require swapping among multiple buffers, such as double-buffered
///graphics rendering.On Fuchsia, the Sysmem service uses buffer collections as a core construct
///in its design.Buffer collections are represented by [`BufferCollectionFUCHSIA`]
///handles:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkBufferCollectionFUCHSIA)
///```
///# Related
/// - [`fuchsia_buffer_collection`]
/// - [`BufferCollectionBufferCreateInfoFUCHSIA`]
/// - [`BufferCollectionImageCreateInfoFUCHSIA`]
/// - [`ImportMemoryBufferCollectionFUCHSIA`]
/// - [`create_buffer_collection_fuchsia`]
/// - [`destroy_buffer_collection_fuchsia`]
/// - [`get_buffer_collection_properties_fuchsia`]
/// - [`set_buffer_collection_buffer_constraints_fuchsia`]
/// - [`set_buffer_collection_image_constraints_fuchsia`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBufferCollectionFUCHSIA")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct BufferCollectionFUCHSIA(pub u64);
impl BufferCollectionFUCHSIA {
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
unsafe impl Send for BufferCollectionFUCHSIA {}
impl Default for BufferCollectionFUCHSIA {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for BufferCollectionFUCHSIA {
    type Parent = Unique<Device>;
    type VTable = ();
    type Metadata = AtomicBool;
    type Storage = u64;
    #[inline]
    fn as_stored(self) -> Self::Storage {
        self.0
    }
    #[inline]
    unsafe fn from_stored(this: Self::Storage) -> Self {
        Self(this)
    }
    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {
        if !self.metadata().load(Ordering::Acquire) {
            self.device()
                .destroy_buffer_collection_fuchsia(self.as_raw().coerce(), None);
        }
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<BufferCollectionFUCHSIA> {
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
#[cfg(feature = "VK_EXT_debug_utils")]
impl BufferCollectionFUCHSIA {
    ///Give a user-friendly name to an object
    pub fn set_name(self: &Unique<Self>, name: &std::ffi::CStr) {
        assert!(
            self.strong_count() == 1,
            "`set_name` requires that the object be synchronized"
        );
        if !self.device().instance().metadata().ext_debug_utils() {
            return;
        }
        let info = crate::generated::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT::default()
            .with_object_type(crate::generated::vulkan1_0::ObjectType::BUFFER_COLLECTION_FUCHSIA)
            .with_object_handle(self.as_stored() as u64)
            .with_object_name(name.as_ptr());
        unsafe {
            self.device().set_debug_utils_object_name_ext(&info).unwrap();
        }
    }
    ///Attach arbitrary data to an object
    ///In addition to setting a name for an object, debugging and validation layers may have uses
    /// for additional
    ///binary data on a per-object basis that has no other place in the Vulkan API. For example, a
    /// VkShaderModule
    ///could have additional debugging data attached to it to aid in offline shader tracing.
    pub fn set_tag(self: &Unique<Self>, tag: u64, data: &[u8]) {
        assert!(
            self.strong_count() == 1,
            "`set_name` requires that the object be synchronized"
        );
        if !self.device().instance().metadata().ext_debug_utils() {
            return;
        }
        let info = crate::generated::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT::default()
            .with_object_type(crate::generated::vulkan1_0::ObjectType::BUFFER_COLLECTION_FUCHSIA)
            .with_object_handle(self.as_stored() as u64)
            .with_tag_name(tag)
            .with_tag_size(data.len() as _)
            .with_tag_raw(data.as_ptr().cast());
        unsafe {
            self.device().set_debug_utils_object_tag_ext(&info).unwrap();
        }
    }
}
///The V-table of [`Device`] for functions from `VK_FUCHSIA_buffer_collection`
pub struct DeviceFuchsiaBufferCollectionVTable {
    ///See [`FNCreateBufferCollectionFuchsia`] for more information.
    pub create_buffer_collection_fuchsia: FNCreateBufferCollectionFuchsia,
    ///See [`FNSetBufferCollectionBufferConstraintsFuchsia`] for more information.
    pub set_buffer_collection_buffer_constraints_fuchsia: FNSetBufferCollectionBufferConstraintsFuchsia,
    ///See [`FNSetBufferCollectionImageConstraintsFuchsia`] for more information.
    pub set_buffer_collection_image_constraints_fuchsia: FNSetBufferCollectionImageConstraintsFuchsia,
    ///See [`FNDestroyBufferCollectionFuchsia`] for more information.
    pub destroy_buffer_collection_fuchsia: FNDestroyBufferCollectionFuchsia,
    ///See [`FNGetBufferCollectionPropertiesFuchsia`] for more information.
    pub get_buffer_collection_properties_fuchsia: FNGetBufferCollectionPropertiesFuchsia,
}
impl DeviceFuchsiaBufferCollectionVTable {
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
            create_buffer_collection_fuchsia: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCreateBufferCollectionFUCHSIA").as_ptr(),
                ))
            },
            set_buffer_collection_buffer_constraints_fuchsia: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkSetBufferCollectionBufferConstraintsFUCHSIA").as_ptr(),
                ))
            },
            set_buffer_collection_image_constraints_fuchsia: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkSetBufferCollectionImageConstraintsFUCHSIA").as_ptr(),
                ))
            },
            destroy_buffer_collection_fuchsia: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkDestroyBufferCollectionFUCHSIA").as_ptr(),
                ))
            },
            get_buffer_collection_properties_fuchsia: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetBufferCollectionPropertiesFUCHSIA").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::create_buffer_collection_fuchsia`]. See [`FNCreateBufferCollectionFuchsia`] for
    /// more information.
    pub fn create_buffer_collection_fuchsia(&self) -> FNCreateBufferCollectionFuchsia {
        self.create_buffer_collection_fuchsia
    }
    ///Gets [`Self::set_buffer_collection_buffer_constraints_fuchsia`]. See
    /// [`FNSetBufferCollectionBufferConstraintsFuchsia`] for more information.
    pub fn set_buffer_collection_buffer_constraints_fuchsia(&self) -> FNSetBufferCollectionBufferConstraintsFuchsia {
        self.set_buffer_collection_buffer_constraints_fuchsia
    }
    ///Gets [`Self::set_buffer_collection_image_constraints_fuchsia`]. See
    /// [`FNSetBufferCollectionImageConstraintsFuchsia`] for more information.
    pub fn set_buffer_collection_image_constraints_fuchsia(&self) -> FNSetBufferCollectionImageConstraintsFuchsia {
        self.set_buffer_collection_image_constraints_fuchsia
    }
    ///Gets [`Self::destroy_buffer_collection_fuchsia`]. See [`FNDestroyBufferCollectionFuchsia`]
    /// for more information.
    pub fn destroy_buffer_collection_fuchsia(&self) -> FNDestroyBufferCollectionFuchsia {
        self.destroy_buffer_collection_fuchsia
    }
    ///Gets [`Self::get_buffer_collection_properties_fuchsia`]. See
    /// [`FNGetBufferCollectionPropertiesFuchsia`] for more information.
    pub fn get_buffer_collection_properties_fuchsia(&self) -> FNGetBufferCollectionPropertiesFuchsia {
        self.get_buffer_collection_properties_fuchsia
    }
}
