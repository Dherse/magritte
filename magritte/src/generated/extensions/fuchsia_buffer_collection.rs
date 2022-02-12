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
//! - Requires `[`VK_FUCHSIA_external_memory`]`
//! - Requires `[`VK_KHR_sampler_ycbcr_conversion`]`
//!# Contacts
//! - John Rosasco [rosasco](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_FUCHSIA_buffer_collection]
//!   @rosasco%0A<<Here describe the issue or question you have about the
//!   VK_FUCHSIA_buffer_collection extension>>)
//!# New handles
//! - [`BufferCollectionFUCHSIA`]
//!# New functions & commands
//! - [`CreateBufferCollectionFUCHSIA`]
//! - [`DestroyBufferCollectionFUCHSIA`]
//! - [`GetBufferCollectionPropertiesFUCHSIA`]
//! - [`SetBufferCollectionBufferConstraintsFUCHSIA`]
//! - [`SetBufferCollectionImageConstraintsFUCHSIA`]
//!# New structures
//! - [`BufferCollectionConstraintsInfoFUCHSIA`]
//! - [`BufferCollectionCreateInfoFUCHSIA`]
//! - [`BufferCollectionPropertiesFUCHSIA`]
//! - [`BufferConstraintsInfoFUCHSIA`]
//! - [`ImageConstraintsInfoFUCHSIA`]
//! - [`ImageFormatConstraintsInfoFUCHSIA`]
//! - [`SysmemColorSpaceFUCHSIA`]
//! - Extending [`BufferCreateInfo`]:
//! - [`BufferCollectionBufferCreateInfoFUCHSIA`]
//! - Extending [`ImageCreateInfo`]:
//! - [`BufferCollectionImageCreateInfoFUCHSIA`]
//! - Extending [`MemoryAllocateInfo`]:
//! - [`ImportMemoryBufferCollectionFUCHSIA`]
//!# New enums
//! - [`ImageConstraintsInfoFlagBitsFUCHSIA`]
//!# New bitmasks
//! - [`ImageConstraintsInfoFlagsFUCHSIA`]
//! - [`ImageFormatConstraintsFlagsFUCHSIA`]
//!# New constants
//! - [`FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME`]
//! - [`FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION`]
//! - Extending [`DebugReportObjectTypeEXT`]:
//! - `VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA_EXT`
//! - Extending [`ObjectType`]:
//! - `VK_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA`
//! - `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA`
//! - `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA`
//! - `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA`
//! - `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA`
//! - `VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA`
//! - `VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA`
//! - `VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA`
//! - `VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA`
//! - `VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA`
//!# Known issues & F.A.Q
//!1) When configuring a [`ImageConstraintsInfoFUCHSIA`] structure for
//!constraint setting, should a NULL `pFormatConstraints` parameter be
//!allowed ?**RESOLVED**: No.
//!Specifying a NULL `pFormatConstraints` results in logical complexity of
//!interpreting the relationship between the
//![`ImageCreateInfo::usage`] settings of the elements of the
//!`pImageCreateInfos` array and the implied or desired
//![`FormatFeatureFlags`].The explicit requirement for `pFormatConstraints` to be non-NULL
//!simplifies the implied logic of the implementation and expectations for the
//!Vulkan application.
//!# Version History
//! - Revision 2, 2021-09-23 (John Rosasco)
//! - Review passes
//! - Revision 1, 2021-03-09 (John Rosasco)
//! - Initial revision
//!# Other info
//! * 2021-09-23
//! * No known IP claims.
//!*
//! - Craig Stout, Google
//! - John Bauman, Google
//! - John Rosasco, Google
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
//! - [`CreateBufferCollectionFUCHSIA`]
//! - [`DestroyBufferCollectionFUCHSIA`]
//! - [`GetBufferCollectionPropertiesFUCHSIA`]
//! - [`SetBufferCollectionBufferConstraintsFUCHSIA`]
//! - [`SetBufferCollectionImageConstraintsFUCHSIA`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION")]
pub const FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME")]
pub const FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_FUCHSIA_buffer_collection");
