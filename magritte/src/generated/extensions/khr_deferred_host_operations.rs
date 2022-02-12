//![VK_KHR_deferred_host_operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_deferred_host_operations.html) - device extension
//!# Description
//!The `[`VK_KHR_deferred_host_operations`]` extension defines the
//!infrastructure and usage patterns for deferrable commands, but does not
//!specify any commands as deferrable.
//!This is left to additional dependent extensions.
//!Commands **must** not be deferred unless the deferral is specifically allowed
//!by another extension which depends on
//!`[`VK_KHR_deferred_host_operations`]`.
//!# Revision
//!4
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Josh Barczak [jbarczak](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_deferred_host_operations]
//!   @jbarczak%0A<<Here describe the issue or question you have about the
//!   VK_KHR_deferred_host_operations extension>>)
//!# New handles
//! - [`DeferredOperationKHR`]
//!# New functions & commands
//! - [`CreateDeferredOperationKHR`]
//! - [`DeferredOperationJoinKHR`]
//! - [`DestroyDeferredOperationKHR`]
//! - [`GetDeferredOperationMaxConcurrencyKHR`]
//! - [`GetDeferredOperationResultKHR`]
//!# New constants
//! - [`KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME`]
//! - [`KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION`]
//! - Extending [`ObjectType`]:
//! - `VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR`
//!
//! - Extending [`VulkanResultCodes`]:
//! - `VK_OPERATION_DEFERRED_KHR`
//! - `VK_OPERATION_NOT_DEFERRED_KHR`
//! - `VK_THREAD_DONE_KHR`
//! - `VK_THREAD_IDLE_KHR`
//!# Known issues & F.A.Q
//!0. Should this extension have a VkPhysicalDevice*FeaturesKHR structure?
//!**RESOLVED**: No.
//!This extension does not add any functionality on its own and requires a
//!dependent extension to actually enable functionality and thus there is no
//!value in adding a feature structure.
//!If necessary, any dependent extension could add a feature boolean if it
//!wanted to indicate that it is adding optional deferral support.
//!# Version History
//! - Revision 1, 2019-12-05 (Josh Barczak, Daniel Koch)
//! - Initial draft.
//!
//! - Revision 2, 2020-03-06 (Daniel Koch, Tobias Hector)
//! - Add missing VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR enum
//! - fix sample code
//! - Clarified deferred operation parameter lifetimes (#2018,!3647)
//!
//! - Revision 3, 2020-05-15 (Josh Barczak)
//! - Clarify behavior of vkGetDeferredOperationMaxConcurrencyKHR, allowing
//!it to return 0 if the operation is complete (#2036,!3850)
//!
//! - Revision 4, 2020-11-12 (Tobias Hector, Daniel Koch)
//! - Remove VkDeferredOperationInfoKHR and change return value semantics
//!when deferred host operations are in use (#2067,3813)
//! - clarify return value of vkGetDeferredOperationResultKHR (#2339,!4110)
//!# Other info
//! * 2020-11-12
//! * No known IP claims.
//!*
//! - Joshua Barczak, Intel
//! - Jeff Bolz, NVIDIA
//! - Daniel Koch, NVIDIA
//! - Slawek Grajewski, Intel
//! - Tobias Hector, AMD
//! - Yuriy O’Donnell, Epic
//! - Eric Werness, NVIDIA
//! - Baldur Karlsson, Valve
//! - Jesse Barker, Unity
//! - Contributors to VK_KHR_acceleration_structure,
//!VK_KHR_ray_tracing_pipeline
//!# Related
//! - [`DeferredOperationKHR`]
//! - [`CreateDeferredOperationKHR`]
//! - [`DeferredOperationJoinKHR`]
//! - [`DestroyDeferredOperationKHR`]
//! - [`GetDeferredOperationMaxConcurrencyKHR`]
//! - [`GetDeferredOperationResultKHR`]
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
#[doc(alias = "VK_KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION")]
pub const KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME")]
pub const KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_deferred_host_operations");
