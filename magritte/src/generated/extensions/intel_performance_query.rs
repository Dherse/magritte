//![VK_INTEL_performance_query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_INTEL_performance_query.html) - device extension
//!# Description
//!This extension allows an application to capture performance data to be
//!interpreted by a external application or library.Such a library is available at : [https://github.com/intel/metrics-discovery](https://github.com/intel/metrics-discovery)Performance analysis tools such as
//![Graphics
//!Performance Analyzers](https://software.intel.com/content/www/us/en/develop/tools/graphics-performance-analyzers.html) make use of this extension and the metrics-discovery
//!library to present the data in a human readable way.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Lionel Landwerlin [llandwerlin](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_INTEL_performance_query]
//!   @llandwerlin%0A<<Here describe the issue or question you have about the
//!   VK_INTEL_performance_query extension>>)
//!# New handles
//! - [`PerformanceConfigurationINTEL`]
//!# New functions & commands
//! - [`acquire_performance_configuration_intel`]
//! - [`cmd_set_performance_marker_intel`]
//! - [`cmd_set_performance_override_intel`]
//! - [`cmd_set_performance_stream_marker_intel`]
//! - [`get_performance_parameter_intel`]
//! - [`initialize_performance_api_intel`]
//! - [`queue_set_performance_configuration_intel`]
//! - [`release_performance_configuration_intel`]
//! - [`uninitialize_performance_api_intel`]
//!# New structures
//! - [`InitializePerformanceApiInfoINTEL`]
//! - [`PerformanceConfigurationAcquireInfoINTEL`]
//! - [`PerformanceMarkerInfoINTEL`]
//! - [`PerformanceOverrideInfoINTEL`]
//! - [`PerformanceStreamMarkerInfoINTEL`]
//! - [`PerformanceValueINTEL`]
//! - Extending [`QueryPoolCreateInfo`]:  - [`QueryPoolCreateInfoINTEL`]  -
//!   [`QueryPoolPerformanceQueryCreateInfoINTEL`]
//!# New enums
//! - [`PerformanceConfigurationTypeINTEL`]
//! - [`PerformanceOverrideTypeINTEL`]
//! - [`PerformanceParameterTypeINTEL`]
//! - [`PerformanceValueTypeINTEL`]
//! - [`QueryPoolSamplingModeINTEL`]
//!# New constants
//! - [`INTEL_PERFORMANCE_QUERY_EXTENSION_NAME`]
//! - [`INTEL_PERFORMANCE_QUERY_SPEC_VERSION`]
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL`
//! - Extending [`QueryType`]:  - `VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL`  -
//!   `VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL`  -
//!   `VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL`  -
//!   `VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL`  -
//!   `VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL`  -
//!   `VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO_INTEL`  -
//!   `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL`
//!# Version History
//! - Revision 2, 2020-03-06 (Lionel Landwerlin)  - Rename VkQueryPoolCreateInfoINTEL in
//!   VkQueryPoolPerformanceQueryCreateInfoINTEL
//! - Revision 1, 2018-05-16 (Lionel Landwerlin)  - Initial revision
//!# Other info
//! * 2018-05-16
//! * No known IP claims.
//! * - Lionel Landwerlin, Intel  - Piotr Maciejewski, Intel
//!# Related
//! - [`InitializePerformanceApiInfoINTEL`]
//! - [`PerformanceConfigurationAcquireInfoINTEL`]
//! - [`PerformanceConfigurationINTEL`]
//! - [`PerformanceConfigurationTypeINTEL`]
//! - [`PerformanceMarkerInfoINTEL`]
//! - [`PerformanceOverrideInfoINTEL`]
//! - [`PerformanceOverrideTypeINTEL`]
//! - [`PerformanceParameterTypeINTEL`]
//! - [`PerformanceStreamMarkerInfoINTEL`]
//! - [`PerformanceValueDataINTEL`]
//! - [`PerformanceValueINTEL`]
//! - [`PerformanceValueTypeINTEL`]
//! - [`QueryPoolCreateInfoINTEL`]
//! - [`QueryPoolPerformanceQueryCreateInfoINTEL`]
//! - [`QueryPoolSamplingModeINTEL`]
//! - [`acquire_performance_configuration_intel`]
//! - [`cmd_set_performance_marker_intel`]
//! - [`cmd_set_performance_override_intel`]
//! - [`cmd_set_performance_stream_marker_intel`]
//! - [`get_performance_parameter_intel`]
//! - [`initialize_performance_api_intel`]
//! - [`queue_set_performance_configuration_intel`]
//! - [`release_performance_configuration_intel`]
//! - [`uninitialize_performance_api_intel`]
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
        BaseInStructure, Bool32, CommandBuffer, Device, Instance, PhysicalDevice, Queue, StructureType,
        VulkanResultCodes,
    },
    AsRaw, Handle, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
    mem::MaybeUninit,
    os::raw::c_char,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_INTEL_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const INTEL_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_INTEL_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const INTEL_PERFORMANCE_QUERY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_INTEL_performance_query");
///[vkInitializePerformanceApiINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInitializePerformanceApiINTEL.html) - Initialize a device for performance queries
///# C Specifications
///Prior to creating a performance query pool, initialize the device for
///performance queries with the call:
///```c
///// Provided by VK_INTEL_performance_query
///VkResult vkInitializePerformanceApiINTEL(
///    VkDevice                                    device,
///    const VkInitializePerformanceApiInfoINTEL*  pInitializeInfo);
///```
/// # Parameters
/// - [`device`] is the logical device used for the queries.
/// - [`p_initialize_info`] is a pointer to a [`InitializePerformanceApiInfoINTEL`] structure
///   specifying initialization parameters.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_initialize_info`] **must**  be a valid pointer to a valid
///   [`InitializePerformanceApiInfoINTEL`] structure
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`Device`]
/// - [`InitializePerformanceApiInfoINTEL`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkInitializePerformanceApiINTEL")]
pub type FNInitializePerformanceApiIntel = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_initialize_info: *const InitializePerformanceApiInfoINTEL<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkUninitializePerformanceApiINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUninitializePerformanceApiINTEL.html) - Uninitialize a device for performance queries
///# C Specifications
///Once performance query operations have completed, uninitalize the device for
///performance queries with the call:
///```c
///// Provided by VK_INTEL_performance_query
///void vkUninitializePerformanceApiINTEL(
///    VkDevice                                    device);
///```
/// # Parameters
/// - [`device`] is the logical device used for the queries.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`Device`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkUninitializePerformanceApiINTEL")]
pub type FNUninitializePerformanceApiIntel = Option<unsafe extern "system" fn(device: Device)>;
///[vkAcquirePerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html) - Acquire the performance query capability
///# C Specifications
///To acquire a device performance configuration, call:
///```c
///// Provided by VK_INTEL_performance_query
///VkResult vkAcquirePerformanceConfigurationINTEL(
///    VkDevice                                    device,
///    const VkPerformanceConfigurationAcquireInfoINTEL* pAcquireInfo,
///    VkPerformanceConfigurationINTEL*            pConfiguration);
///```
/// # Parameters
/// - [`device`] is the logical device that the performance query commands will be submitted to.
/// - [`p_acquire_info`] is a pointer to a [`PerformanceConfigurationAcquireInfoINTEL`] structure,
///   specifying the performance configuration to acquire.
/// - [`p_configuration`] is a pointer to a [`PerformanceConfigurationINTEL`] handle in which the
///   resulting configuration object is returned.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_acquire_info`] **must**  be a valid pointer to a valid
///   [`PerformanceConfigurationAcquireInfoINTEL`] structure
/// - [`p_configuration`] **must**  be a valid pointer to a [`PerformanceConfigurationINTEL`] handle
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`Device`]
/// - [`PerformanceConfigurationAcquireInfoINTEL`]
/// - [`PerformanceConfigurationINTEL`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
pub type FNAcquirePerformanceConfigurationIntel = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL<'lt>,
        p_configuration: *mut PerformanceConfigurationINTEL,
    ) -> VulkanResultCodes,
>;
///[vkReleasePerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html) - Release a configuration to capture performance data
///# C Specifications
///To release a device performance configuration, call:
///```c
///// Provided by VK_INTEL_performance_query
///VkResult vkReleasePerformanceConfigurationINTEL(
///    VkDevice                                    device,
///    VkPerformanceConfigurationINTEL             configuration);
///```
/// # Parameters
/// - [`device`] is the device associated to the configuration object to release.
/// - [`configuration`] is the configuration object to release.
/// # Description
/// ## Valid Usage
/// -  [`configuration`] **must**  not be released before all command buffers submitted while the configuration was set are in [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
///
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - If [`configuration`] is not [`crate::Handle::null`], [`configuration`] **must**  be a valid
///   [`PerformanceConfigurationINTEL`] handle
/// - If [`configuration`] is a valid handle, it  **must**  have been created, allocated, or
///   retrieved from [`device`]
///
/// ## Host Synchronization
/// - Host access to [`configuration`] **must**  be externally synchronized
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`Device`]
/// - [`PerformanceConfigurationINTEL`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
pub type FNReleasePerformanceConfigurationIntel = Option<
    unsafe extern "system" fn(device: Device, configuration: PerformanceConfigurationINTEL) -> VulkanResultCodes,
>;
///[vkQueueSetPerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html) - Set a performance query
///# C Specifications
///To set a performance configuration, call:
///```c
///// Provided by VK_INTEL_performance_query
///VkResult vkQueueSetPerformanceConfigurationINTEL(
///    VkQueue                                     queue,
///    VkPerformanceConfigurationINTEL             configuration);
///```
/// # Parameters
/// - [`queue`] is the queue on which the configuration will be used.
/// - [`configuration`] is the configuration to use.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`queue`] **must**  be a valid [`Queue`] handle
/// - [`configuration`] **must**  be a valid [`PerformanceConfigurationINTEL`] handle
/// - Both of [`configuration`], and [`queue`] **must**  have been created, allocated, or retrieved
///   from the same [`Device`]
///
/// ## Command Properties
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceConfigurationINTEL`]
/// - [`Queue`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
pub type FNQueueSetPerformanceConfigurationIntel =
    Option<unsafe extern "system" fn(queue: Queue, configuration: PerformanceConfigurationINTEL) -> VulkanResultCodes>;
///[vkGetPerformanceParameterINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPerformanceParameterINTEL.html) - Query performance capabilities of the device
///# C Specifications
///Some performance query features of a device can be discovered with the call:
///```c
///// Provided by VK_INTEL_performance_query
///VkResult vkGetPerformanceParameterINTEL(
///    VkDevice                                    device,
///    VkPerformanceParameterTypeINTEL             parameter,
///    VkPerformanceValueINTEL*                    pValue);
///```
/// # Parameters
/// - [`device`] is the logical device to query.
/// - [`parameter`] is the parameter to query.
/// - [`p_value`] is a pointer to a [`PerformanceValueINTEL`] structure in which the type and value
///   of the parameter are returned.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`parameter`] **must**  be a valid [`PerformanceParameterTypeINTEL`] value
/// - [`p_value`] **must**  be a valid pointer to a [`PerformanceValueINTEL`] structure
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`Device`]
/// - [`PerformanceParameterTypeINTEL`]
/// - [`PerformanceValueINTEL`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPerformanceParameterINTEL")]
pub type FNGetPerformanceParameterIntel = Option<
    unsafe extern "system" fn(
        device: Device,
        parameter: PerformanceParameterTypeINTEL,
        p_value: *mut PerformanceValueINTEL,
    ) -> VulkanResultCodes,
>;
///[vkCmdSetPerformanceMarkerINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html) - Markers
///# C Specifications
///To help associate query results with a particular point at which an
///application emitted commands, markers can be set into the command buffers
///with the call:
///```c
///// Provided by VK_INTEL_performance_query
///VkResult vkCmdSetPerformanceMarkerINTEL(
///    VkCommandBuffer                             commandBuffer,
///    const VkPerformanceMarkerInfoINTEL*         pMarkerInfo);
///```
/// # Parameters
/// The last marker set onto a command buffer before the end of a query will be
/// part of the query result.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_marker_info`] **must**  be a valid pointer to a valid [`PerformanceMarkerInfoINTEL`]
///   structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics,
///   compute, or transfer operations
///
/// ## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
/// ## Command Properties
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`CommandBuffer`]
/// - [`PerformanceMarkerInfoINTEL`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
pub type FNCmdSetPerformanceMarkerIntel = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const PerformanceMarkerInfoINTEL<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkCmdSetPerformanceStreamMarkerINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html) - Markers
///# C Specifications
///When monitoring the behavior of an application wihtin the dataset generated
///by the entire set of applications running on the system, it is useful to
///identify draw calls within a potentially huge amount of performance data.
///To do so, application can generate stream markers that will be used to trace
///back a particular draw call with a particular performance data item.
///```c
///// Provided by VK_INTEL_performance_query
///VkResult vkCmdSetPerformanceStreamMarkerINTEL(
///    VkCommandBuffer                             commandBuffer,
///    const VkPerformanceStreamMarkerInfoINTEL*   pMarkerInfo);
///```
/// # Description
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_marker_info`] **must**  be a valid pointer to a valid [`PerformanceStreamMarkerInfoINTEL`]
///   structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics,
///   compute, or transfer operations
///
/// ## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
/// ## Command Properties
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`CommandBuffer`]
/// - [`PerformanceStreamMarkerInfoINTEL`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
pub type FNCmdSetPerformanceStreamMarkerIntel = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const PerformanceStreamMarkerInfoINTEL<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkCmdSetPerformanceOverrideINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html) - Performance override settings
///# C Specifications
///Some applications might want measure the effect of a set of commands with a
///different settings.
///It is possible to override a particular settings using :
///```c
///// Provided by VK_INTEL_performance_query
///VkResult vkCmdSetPerformanceOverrideINTEL(
///    VkCommandBuffer                             commandBuffer,
///    const VkPerformanceOverrideInfoINTEL*       pOverrideInfo);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer where the override takes place.
/// - [`p_override_info`] is a pointer to a [`PerformanceOverrideInfoINTEL`] structure selecting the
///   parameter to override.
/// # Description
/// ## Valid Usage
/// - [`p_override_info`] **must**  not be used with a [`PerformanceOverrideTypeINTEL`] that is not
///   reported available by [`get_performance_parameter_intel`]
///
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_override_info`] **must**  be a valid pointer to a valid [`PerformanceOverrideInfoINTEL`]
///   structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics,
///   compute, or transfer operations
///
/// ## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
/// ## Command Properties
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`CommandBuffer`]
/// - [`PerformanceOverrideInfoINTEL`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
pub type FNCmdSetPerformanceOverrideIntel = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_override_info: *const PerformanceOverrideInfoINTEL<'lt>,
    ) -> VulkanResultCodes,
>;
///[VkPerformanceConfigurationTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationTypeINTEL.html) - Type of performance configuration
///# C Specifications
///Possible values of
///[`PerformanceConfigurationAcquireInfoINTEL::type_`], specifying
///performance configuration types, are:
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkPerformanceConfigurationTypeINTEL {
///    VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL = 0,
///} VkPerformanceConfigurationTypeINTEL;
///```
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceConfigurationAcquireInfoINTEL`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceConfigurationTypeINTEL")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PerformanceConfigurationTypeINTEL(i32);
impl const Default for PerformanceConfigurationTypeINTEL {
    fn default() -> Self {
        Self(0)
    }
}
impl PerformanceConfigurationTypeINTEL {
    ///No documentation found
    pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED: Self = Self(0);
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
impl std::fmt::Debug for PerformanceConfigurationTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PerformanceConfigurationTypeINTEL))
            .field(match *self {
                Self::COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED => &"COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED",
                other => unreachable!(
                    concat!(
                        "invalid value for",
                        stringify!(PerformanceConfigurationTypeINTEL),
                        ": {:?}"
                    ),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for PerformanceConfigurationTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED => &"COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED",
            other => unreachable!(
                concat!(
                    "invalid value for",
                    stringify!(PerformanceConfigurationTypeINTEL),
                    ": {:?}"
                ),
                other
            ),
        })
    }
}
///[VkQueryPoolSamplingModeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolSamplingModeINTEL.html) - Enum specifying how performance queries should be captured
///# C Specifications
///Possible values of
///[`QueryPoolPerformanceQueryCreateInfoINTEL::performance_counters_sampling`]
///are:
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkQueryPoolSamplingModeINTEL {
///    VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL = 0,
///} VkQueryPoolSamplingModeINTEL;
///```
/// # Description
/// - [`MANUAL`] is the default mode in which the application calls [`cmd_begin_query`] and
///   [`cmd_end_query`] to record performance data.
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`QueryPoolPerformanceQueryCreateInfoINTEL`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueryPoolSamplingModeINTEL")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct QueryPoolSamplingModeINTEL(i32);
impl const Default for QueryPoolSamplingModeINTEL {
    fn default() -> Self {
        Self(0)
    }
}
impl QueryPoolSamplingModeINTEL {
    ///[`MANUAL`] is the default mode in
    ///which the application calls [`cmd_begin_query`] and
    ///[`cmd_end_query`] to record performance data.
    pub const MANUAL: Self = Self(0);
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
impl std::fmt::Debug for QueryPoolSamplingModeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(QueryPoolSamplingModeINTEL))
            .field(match *self {
                Self::MANUAL => &"MANUAL",
                other => unreachable!(
                    concat!("invalid value for", stringify!(QueryPoolSamplingModeINTEL), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for QueryPoolSamplingModeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::MANUAL => &"MANUAL",
            other => unreachable!(
                concat!("invalid value for", stringify!(QueryPoolSamplingModeINTEL), ": {:?}"),
                other
            ),
        })
    }
}
///[VkPerformanceOverrideTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideTypeINTEL.html) - Performance override type
///# C Specifications
///Possible values of [`PerformanceOverrideInfoINTEL::type_`],
///specifying performance override types, are:
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkPerformanceOverrideTypeINTEL {
///    VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL = 0,
///    VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL = 1,
///} VkPerformanceOverrideTypeINTEL;
///```
/// # Description
/// - [`NULL_HARDWARE`] turns all rendering operations into noop.
/// - [`FLUSH_GPU_CACHES`] stalls the stream of commands until all previously emitted commands have
///   completed and all caches been flushed and invalidated.
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceOverrideInfoINTEL`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceOverrideTypeINTEL")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PerformanceOverrideTypeINTEL(i32);
impl const Default for PerformanceOverrideTypeINTEL {
    fn default() -> Self {
        Self(0)
    }
}
impl PerformanceOverrideTypeINTEL {
    ///[`NULL_HARDWARE`] turns all
    ///rendering operations into noop.
    pub const NULL_HARDWARE: Self = Self(0);
    ///[`FLUSH_GPU_CACHES`] stalls the
    ///stream of commands until all previously emitted commands have completed
    ///and all caches been flushed and invalidated.
    pub const FLUSH_GPU_CACHES: Self = Self(1);
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
impl std::fmt::Debug for PerformanceOverrideTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PerformanceOverrideTypeINTEL))
            .field(match *self {
                Self::NULL_HARDWARE => &"NULL_HARDWARE",
                Self::FLUSH_GPU_CACHES => &"FLUSH_GPU_CACHES",
                other => unreachable!(
                    concat!("invalid value for", stringify!(PerformanceOverrideTypeINTEL), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for PerformanceOverrideTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::NULL_HARDWARE => &"NULL_HARDWARE",
            Self::FLUSH_GPU_CACHES => &"FLUSH_GPU_CACHES",
            other => unreachable!(
                concat!("invalid value for", stringify!(PerformanceOverrideTypeINTEL), ": {:?}"),
                other
            ),
        })
    }
}
///[VkPerformanceParameterTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceParameterTypeINTEL.html) - Parameters that can be queried
///# C Specifications
///Possible values of [`get_performance_parameter_intel`]`::parameter`,
///specifying a performance query feature, are:
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkPerformanceParameterTypeINTEL {
///    VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL = 0,
///    VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL = 1,
///} VkPerformanceParameterTypeINTEL;
///```
/// # Description
/// - [`HW_COUNTERS_SUPPORTED`] has a boolean result which tells whether hardware counters can be
///   captured.
/// - [`STREAM_MARKER_VALID_BITS`] has a 32 bits integer result which tells how many bits can be
///   written into the [`PerformanceValueINTEL`] value.
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`get_performance_parameter_intel`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceParameterTypeINTEL")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PerformanceParameterTypeINTEL(i32);
impl const Default for PerformanceParameterTypeINTEL {
    fn default() -> Self {
        Self(0)
    }
}
impl PerformanceParameterTypeINTEL {
    ///[`HW_COUNTERS_SUPPORTED`] has a
    ///boolean result which tells whether hardware counters can be captured.
    pub const HW_COUNTERS_SUPPORTED: Self = Self(0);
    ///[`STREAM_MARKER_VALID_BITS`] has a
    ///32 bits integer result which tells how many bits can be written into the
    ///[`PerformanceValueINTEL`] value.
    pub const STREAM_MARKER_VALID_BITS: Self = Self(1);
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
impl std::fmt::Debug for PerformanceParameterTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PerformanceParameterTypeINTEL))
            .field(match *self {
                Self::HW_COUNTERS_SUPPORTED => &"HW_COUNTERS_SUPPORTED",
                Self::STREAM_MARKER_VALID_BITS => &"STREAM_MARKER_VALID_BITS",
                other => unreachable!(
                    concat!("invalid value for", stringify!(PerformanceParameterTypeINTEL), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for PerformanceParameterTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::HW_COUNTERS_SUPPORTED => &"HW_COUNTERS_SUPPORTED",
            Self::STREAM_MARKER_VALID_BITS => &"STREAM_MARKER_VALID_BITS",
            other => unreachable!(
                concat!("invalid value for", stringify!(PerformanceParameterTypeINTEL), ": {:?}"),
                other
            ),
        })
    }
}
///[VkPerformanceValueTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueTypeINTEL.html) - Type of the parameters that can be queried
///# C Specifications
///Possible values of [`PerformanceValueINTEL::type_`], specifying the
///type of the data returned in [`PerformanceValueINTEL::data`], are:
/// - [`UINT32`] specifies that unsigned 32-bit integer data is returned in `data.value32`.
/// - [`UINT64`] specifies that unsigned 64-bit integer data is returned in `data.value64`.
/// - [`FLOAT`] specifies that floating-point data is returned in `data.valueFloat`.
/// - [`BOOL`] specifies that [`Bool32`] data is returned in `data.valueBool`.
/// - [`STRING`] specifies that a pointer to a null-terminated UTF-8 string is returned in
///   `data.valueString`. The pointer is valid for the lifetime of the `device` parameter passed to
///   [`get_performance_parameter_intel`].
///
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkPerformanceValueTypeINTEL {
///    VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL = 0,
///    VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL = 1,
///    VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL = 2,
///    VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL = 3,
///    VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL = 4,
///} VkPerformanceValueTypeINTEL;
///```
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceValueINTEL`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceValueTypeINTEL")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PerformanceValueTypeINTEL(i32);
impl const Default for PerformanceValueTypeINTEL {
    fn default() -> Self {
        Self(0)
    }
}
impl PerformanceValueTypeINTEL {
    ///No documentation found
    pub const UINT32: Self = Self(0);
    ///No documentation found
    pub const UINT64: Self = Self(1);
    ///No documentation found
    pub const FLOAT: Self = Self(2);
    ///No documentation found
    pub const BOOL: Self = Self(3);
    ///No documentation found
    pub const STRING: Self = Self(4);
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
impl std::fmt::Debug for PerformanceValueTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PerformanceValueTypeINTEL))
            .field(match *self {
                Self::UINT32 => &"UINT32",
                Self::UINT64 => &"UINT64",
                Self::FLOAT => &"FLOAT",
                Self::BOOL => &"BOOL",
                Self::STRING => &"STRING",
                other => unreachable!(
                    concat!("invalid value for", stringify!(PerformanceValueTypeINTEL), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for PerformanceValueTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::UINT32 => &"UINT32",
            Self::UINT64 => &"UINT64",
            Self::FLOAT => &"FLOAT",
            Self::BOOL => &"BOOL",
            Self::STRING => &"STRING",
            other => unreachable!(
                concat!("invalid value for", stringify!(PerformanceValueTypeINTEL), ": {:?}"),
                other
            ),
        })
    }
}
///[VkPerformanceValueINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueINTEL.html) - Container for value and types of parameters that can be queried
///# C Specifications
///The [`PerformanceValueINTEL`] structure is defined as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkPerformanceValueINTEL {
///    VkPerformanceValueTypeINTEL    type;
///    VkPerformanceValueDataINTEL    data;
///} VkPerformanceValueINTEL;
///```
/// # Members
/// - [`type_`] is a [`PerformanceValueTypeINTEL`] value specifying the type of the returned data.
/// - [`data`] is a [`PerformanceValueDataINTEL`] union specifying the value of the returned data.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`type_`] **must**  be a valid [`PerformanceValueTypeINTEL`] value
/// - If [`type_`] is `VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL`, the `valueString` member of [`data`]
///   **must**  be a null-terminated UTF-8 string
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceValueDataINTEL`]
/// - [`PerformanceValueTypeINTEL`]
/// - [`get_performance_parameter_intel`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceValueINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceValueINTEL {
    ///[`type_`] is a [`PerformanceValueTypeINTEL`] value specifying the
    ///type of the returned data.
    pub type_: PerformanceValueTypeINTEL,
    ///[`data`] is a [`PerformanceValueDataINTEL`] union specifying the
    ///value of the returned data.
    pub data: PerformanceValueDataINTEL,
}
impl Default for PerformanceValueINTEL {
    fn default() -> Self {
        Self {
            type_: Default::default(),
            data: unsafe { std::mem::zeroed() },
        }
    }
}
impl PerformanceValueINTEL {
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> PerformanceValueTypeINTEL {
        self.type_
    }
    ///Gets the value of [`Self::data`]
    pub fn data(&self) -> PerformanceValueDataINTEL {
        self.data
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type_mut(&mut self) -> &mut PerformanceValueTypeINTEL {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::data`]
    pub fn data_mut(&mut self) -> &mut PerformanceValueDataINTEL {
        &mut self.data
    }
    ///Sets the value of [`Self::type_`]
    pub fn set_type_(mut self, value: crate::extensions::intel_performance_query::PerformanceValueTypeINTEL) -> Self {
        self.type_ = value;
        self
    }
    ///Sets the value of [`Self::data`]
    pub fn set_data(mut self, value: crate::extensions::intel_performance_query::PerformanceValueDataINTEL) -> Self {
        self.data = value;
        self
    }
}
///[VkInitializePerformanceApiInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInitializePerformanceApiInfoINTEL.html) - Structure specifying parameters of initialize of the device
///# C Specifications
///The [`InitializePerformanceApiInfoINTEL`] structure is defined as :
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkInitializePerformanceApiInfoINTEL {
///    VkStructureType    sType;
///    const void*        pNext;
///    void*              pUserData;
///} VkInitializePerformanceApiInfoINTEL;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`user_data`] is a pointer for application data.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL`
/// - [`p_next`] **must**  be `NULL`
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`StructureType`]
/// - [`initialize_performance_api_intel`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkInitializePerformanceApiInfoINTEL")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct InitializePerformanceApiInfoINTEL<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`user_data`] is a pointer for application data.
    pub user_data: *mut c_void,
}
impl<'lt> Default for InitializePerformanceApiInfoINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::INITIALIZE_PERFORMANCE_API_INFO_INTEL,
            p_next: std::ptr::null(),
            user_data: std::ptr::null_mut(),
        }
    }
}
impl<'lt> InitializePerformanceApiInfoINTEL<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::user_data`]
    pub fn user_data_raw(&self) -> *mut c_void {
        self.user_data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::user_data`]
    pub fn set_user_data_raw(mut self, value: *mut c_void) -> Self {
        self.user_data = value;
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
    ///Gets the value of [`Self::user_data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn user_data(&self) -> &c_void {
        &*self.user_data
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::user_data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn user_data_mut(&mut self) -> &mut c_void {
        &mut *self.user_data
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
    ///Sets the value of [`Self::user_data`]
    pub fn set_user_data(mut self, value: &'lt mut std::ffi::c_void) -> Self {
        self.user_data = value as *mut _;
        self
    }
}
///[VkQueryPoolPerformanceQueryCreateInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html) - Structure specifying parameters to create a pool of performance queries
///# C Specifications
///The [`QueryPoolPerformanceQueryCreateInfoINTEL`] structure is defined
///as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkQueryPoolPerformanceQueryCreateInfoINTEL {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkQueryPoolSamplingModeINTEL    performanceCountersSampling;
///} VkQueryPoolPerformanceQueryCreateInfoINTEL;
///```
/// ```c
///// Provided by VK_INTEL_performance_query
///typedef VkQueryPoolPerformanceQueryCreateInfoINTEL VkQueryPoolCreateInfoINTEL;
///```
/// # Members
/// To create a pool for Intel performance queries, set
/// [`QueryPoolCreateInfo::query_type`] to
/// `VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL` and add a
/// [`QueryPoolPerformanceQueryCreateInfoINTEL`] structure to the
/// [`p_next`] chain of the [`QueryPoolCreateInfo`] structure.
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`performance_counters_sampling`] describe how performance queries should be captured.
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL`
/// - [`performance_counters_sampling`] **must**  be a valid [`QueryPoolSamplingModeINTEL`] value
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`QueryPoolSamplingModeINTEL`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueryPoolPerformanceQueryCreateInfoINTEL")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct QueryPoolPerformanceQueryCreateInfoINTEL<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`performance_counters_sampling`] describe how performance queries
    ///should be captured.
    pub performance_counters_sampling: QueryPoolSamplingModeINTEL,
}
impl<'lt> Default for QueryPoolPerformanceQueryCreateInfoINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL,
            p_next: std::ptr::null(),
            performance_counters_sampling: Default::default(),
        }
    }
}
impl<'lt> QueryPoolPerformanceQueryCreateInfoINTEL<'lt> {
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
    ///Gets the value of [`Self::performance_counters_sampling`]
    pub fn performance_counters_sampling(&self) -> QueryPoolSamplingModeINTEL {
        self.performance_counters_sampling
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::performance_counters_sampling`]
    pub fn performance_counters_sampling_mut(&mut self) -> &mut QueryPoolSamplingModeINTEL {
        &mut self.performance_counters_sampling
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
    ///Sets the value of [`Self::performance_counters_sampling`]
    pub fn set_performance_counters_sampling(
        mut self,
        value: crate::extensions::intel_performance_query::QueryPoolSamplingModeINTEL,
    ) -> Self {
        self.performance_counters_sampling = value;
        self
    }
}
///[VkPerformanceMarkerInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceMarkerInfoINTEL.html) - Structure specifying performance markers
///# C Specifications
///The [`PerformanceMarkerInfoINTEL`] structure is defined as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkPerformanceMarkerInfoINTEL {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint64_t           marker;
///} VkPerformanceMarkerInfoINTEL;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`marker`] is the marker value that will be recorded into the opaque query results.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL`
/// - [`p_next`] **must**  be `NULL`
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`StructureType`]
/// - [`cmd_set_performance_marker_intel`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceMarkerInfoINTEL")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PerformanceMarkerInfoINTEL<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`marker`] is the marker value that will be recorded into the opaque
    ///query results.
    pub marker: u64,
}
impl<'lt> Default for PerformanceMarkerInfoINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PERFORMANCE_MARKER_INFO_INTEL,
            p_next: std::ptr::null(),
            marker: 0,
        }
    }
}
impl<'lt> PerformanceMarkerInfoINTEL<'lt> {
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
    ///Gets the value of [`Self::marker`]
    pub fn marker(&self) -> u64 {
        self.marker
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::marker`]
    pub fn marker_mut(&mut self) -> &mut u64 {
        &mut self.marker
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
    ///Sets the value of [`Self::marker`]
    pub fn set_marker(mut self, value: u64) -> Self {
        self.marker = value;
        self
    }
}
///[VkPerformanceStreamMarkerInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceStreamMarkerInfoINTEL.html) - Structure specifying stream performance markers
///# C Specifications
///The [`PerformanceStreamMarkerInfoINTEL`] structure is defined as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkPerformanceStreamMarkerInfoINTEL {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           marker;
///} VkPerformanceStreamMarkerInfoINTEL;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`marker`] is the marker value that will be recorded into the reports consumed by an external
///   application.
/// # Description
/// ## Valid Usage
/// - The value written by the application into [`marker`] **must**  only used the valid bits as
///   reported by [`get_performance_parameter_intel`] with the
///   `VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL`
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL`
/// - [`p_next`] **must**  be `NULL`
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`StructureType`]
/// - [`cmd_set_performance_stream_marker_intel`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceStreamMarkerInfoINTEL")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PerformanceStreamMarkerInfoINTEL<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`marker`] is the marker value that will be recorded into the reports
    ///consumed by an external application.
    pub marker: u32,
}
impl<'lt> Default for PerformanceStreamMarkerInfoINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PERFORMANCE_STREAM_MARKER_INFO_INTEL,
            p_next: std::ptr::null(),
            marker: 0,
        }
    }
}
impl<'lt> PerformanceStreamMarkerInfoINTEL<'lt> {
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
    ///Gets the value of [`Self::marker`]
    pub fn marker(&self) -> u32 {
        self.marker
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::marker`]
    pub fn marker_mut(&mut self) -> &mut u32 {
        &mut self.marker
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
    ///Sets the value of [`Self::marker`]
    pub fn set_marker(mut self, value: u32) -> Self {
        self.marker = value;
        self
    }
}
///[VkPerformanceOverrideInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideInfoINTEL.html) - Performance override information
///# C Specifications
///The [`PerformanceOverrideInfoINTEL`] structure is defined as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkPerformanceOverrideInfoINTEL {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkPerformanceOverrideTypeINTEL    type;
///    VkBool32                          enable;
///    uint64_t                          parameter;
///} VkPerformanceOverrideInfoINTEL;
///```
/// # Members
/// - [`type_`] is the particular [`PerformanceOverrideTypeINTEL`] to set.
/// - [`enable`] defines whether the override is enabled.
/// - [`parameter`] is a potential required parameter for the override.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL`
/// - [`p_next`] **must**  be `NULL`
/// - [`type_`] **must**  be a valid [`PerformanceOverrideTypeINTEL`] value
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`Bool32`]
/// - [`PerformanceOverrideTypeINTEL`]
/// - [`StructureType`]
/// - [`cmd_set_performance_override_intel`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceOverrideInfoINTEL")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PerformanceOverrideInfoINTEL<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///No documentation found
    pub s_type: StructureType,
    ///No documentation found
    pub p_next: *const BaseInStructure<'lt>,
    ///[`type_`] is the particular [`PerformanceOverrideTypeINTEL`] to
    ///set.
    pub type_: PerformanceOverrideTypeINTEL,
    ///[`enable`] defines whether the override is enabled.
    pub enable: Bool32,
    ///[`parameter`] is a potential required parameter for the override.
    pub parameter: u64,
}
impl<'lt> Default for PerformanceOverrideInfoINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PERFORMANCE_OVERRIDE_INFO_INTEL,
            p_next: std::ptr::null(),
            type_: Default::default(),
            enable: 0,
            parameter: 0,
        }
    }
}
impl<'lt> PerformanceOverrideInfoINTEL<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::enable`]
    pub fn enable_raw(&self) -> Bool32 {
        self.enable
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::enable`]
    pub fn set_enable_raw(mut self, value: Bool32) -> Self {
        self.enable = value;
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
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> PerformanceOverrideTypeINTEL {
        self.type_
    }
    ///Gets the value of [`Self::enable`]
    pub fn enable(&self) -> bool {
        unsafe { std::mem::transmute(self.enable as u8) }
    }
    ///Gets the value of [`Self::parameter`]
    pub fn parameter(&self) -> u64 {
        self.parameter
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type_mut(&mut self) -> &mut PerformanceOverrideTypeINTEL {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::enable`]
    pub fn enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.enable as *mut Bool32).cast::<u32>().cast::<u8>().cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::parameter`]
    pub fn parameter_mut(&mut self) -> &mut u64 {
        &mut self.parameter
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
    ///Sets the value of [`Self::type_`]
    pub fn set_type_(
        mut self,
        value: crate::extensions::intel_performance_query::PerformanceOverrideTypeINTEL,
    ) -> Self {
        self.type_ = value;
        self
    }
    ///Sets the value of [`Self::enable`]
    pub fn set_enable(mut self, value: bool) -> Self {
        self.enable = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::parameter`]
    pub fn set_parameter(mut self, value: u64) -> Self {
        self.parameter = value;
        self
    }
}
///[VkPerformanceConfigurationAcquireInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html) - Acquire a configuration to capture performance data
///# C Specifications
///The [`PerformanceConfigurationAcquireInfoINTEL`] structure is defined
///as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkPerformanceConfigurationAcquireInfoINTEL {
///    VkStructureType                        sType;
///    const void*                            pNext;
///    VkPerformanceConfigurationTypeINTEL    type;
///} VkPerformanceConfigurationAcquireInfoINTEL;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`type_`] is one of the [`PerformanceConfigurationTypeINTEL`] type of performance
///   configuration that will be acquired.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL`
/// - [`p_next`] **must**  be `NULL`
/// - [`type_`] **must**  be a valid [`PerformanceConfigurationTypeINTEL`] value
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceConfigurationTypeINTEL`]
/// - [`StructureType`]
/// - [`acquire_performance_configuration_intel`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceConfigurationAcquireInfoINTEL")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PerformanceConfigurationAcquireInfoINTEL<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`type_`] is one of the [`PerformanceConfigurationTypeINTEL`] type
    ///of performance configuration that will be acquired.
    pub type_: PerformanceConfigurationTypeINTEL,
}
impl<'lt> Default for PerformanceConfigurationAcquireInfoINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL,
            p_next: std::ptr::null(),
            type_: Default::default(),
        }
    }
}
impl<'lt> PerformanceConfigurationAcquireInfoINTEL<'lt> {
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
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> PerformanceConfigurationTypeINTEL {
        self.type_
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type_mut(&mut self) -> &mut PerformanceConfigurationTypeINTEL {
        &mut self.type_
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
    ///Sets the value of [`Self::type_`]
    pub fn set_type_(
        mut self,
        value: crate::extensions::intel_performance_query::PerformanceConfigurationTypeINTEL,
    ) -> Self {
        self.type_ = value;
        self
    }
}
///[VkPerformanceValueDataINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueDataINTEL.html) - Values returned for the parameters
///# C Specifications
///The [`PerformanceValueDataINTEL`] union is defined as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef union VkPerformanceValueDataINTEL {
///    uint32_t       value32;
///    uint64_t       value64;
///    float          valueFloat;
///    VkBool32       valueBool;
///    const char*    valueString;
///} VkPerformanceValueDataINTEL;
///```
/// # Members
/// - `data.value32` represents 32-bit integer data.
/// - `data.value64` represents 64-bit integer data.
/// - `data.valueFloat` represents floating-point data.
/// - `data.valueBool` represents [`Bool32`] data.
/// - `data.valueString` represents a pointer to a null-terminated UTF-8 string.
/// # Description
/// The correct member of the union is determined by the associated
/// [`PerformanceValueTypeINTEL`] value.
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`Bool32`]
/// - [`PerformanceValueINTEL`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceValueDataINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub union PerformanceValueDataINTEL {
    ///No documentation found
    pub value32: u32,
    ///No documentation found
    pub value64: u64,
    ///No documentation found
    pub value_float: f32,
    ///No documentation found
    pub value_bool: Bool32,
    ///No documentation found
    pub value_string: *const c_char,
}
impl Default for PerformanceValueDataINTEL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Device {
    ///[vkInitializePerformanceApiINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInitializePerformanceApiINTEL.html) - Initialize a device for performance queries
    ///# C Specifications
    ///Prior to creating a performance query pool, initialize the device for
    ///performance queries with the call:
    ///```c
    ///// Provided by VK_INTEL_performance_query
    ///VkResult vkInitializePerformanceApiINTEL(
    ///    VkDevice                                    device,
    ///    const VkInitializePerformanceApiInfoINTEL*  pInitializeInfo);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device used for the queries.
    /// - [`p_initialize_info`] is a pointer to a [`InitializePerformanceApiInfoINTEL`] structure
    ///   specifying initialization parameters.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_initialize_info`] **must**  be a valid pointer to a valid
    ///   [`InitializePerformanceApiInfoINTEL`] structure
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`VK_INTEL_performance_query`]
    /// - [`Device`]
    /// - [`InitializePerformanceApiInfoINTEL`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkInitializePerformanceApiINTEL")]
    #[track_caller]
    #[inline]
    pub unsafe fn initialize_performance_api_intel<'lt>(
        self: &Unique<Device>,
        p_initialize_info: &InitializePerformanceApiInfoINTEL<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.initialize_performance_api_intel())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.initialize_performance_api_intel())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_initialize_info as *const InitializePerformanceApiInfoINTEL<'lt>,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkUninitializePerformanceApiINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUninitializePerformanceApiINTEL.html) - Uninitialize a device for performance queries
    ///# C Specifications
    ///Once performance query operations have completed, uninitalize the device for
    ///performance queries with the call:
    ///```c
    ///// Provided by VK_INTEL_performance_query
    ///void vkUninitializePerformanceApiINTEL(
    ///    VkDevice                                    device);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device used for the queries.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// # Related
    /// - [`VK_INTEL_performance_query`]
    /// - [`Device`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkUninitializePerformanceApiINTEL")]
    #[track_caller]
    #[inline]
    pub unsafe fn uninitialize_performance_api_intel(self: &Unique<Device>) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.uninitialize_performance_api_intel())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.uninitialize_performance_api_intel())
            .unwrap_unchecked();
        let _return = _function(self.as_raw());
        ()
    }
}
impl Device {
    ///[vkAcquirePerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html) - Acquire the performance query capability
    ///# C Specifications
    ///To acquire a device performance configuration, call:
    ///```c
    ///// Provided by VK_INTEL_performance_query
    ///VkResult vkAcquirePerformanceConfigurationINTEL(
    ///    VkDevice                                    device,
    ///    const VkPerformanceConfigurationAcquireInfoINTEL* pAcquireInfo,
    ///    VkPerformanceConfigurationINTEL*            pConfiguration);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that the performance query commands will be submitted to.
    /// - [`p_acquire_info`] is a pointer to a [`PerformanceConfigurationAcquireInfoINTEL`]
    ///   structure, specifying the performance configuration to acquire.
    /// - [`p_configuration`] is a pointer to a [`PerformanceConfigurationINTEL`] handle in which
    ///   the resulting configuration object is returned.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_acquire_info`] **must**  be a valid pointer to a valid
    ///   [`PerformanceConfigurationAcquireInfoINTEL`] structure
    /// - [`p_configuration`] **must**  be a valid pointer to a [`PerformanceConfigurationINTEL`]
    ///   handle
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`VK_INTEL_performance_query`]
    /// - [`Device`]
    /// - [`PerformanceConfigurationAcquireInfoINTEL`]
    /// - [`PerformanceConfigurationINTEL`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
    #[track_caller]
    #[inline]
    pub unsafe fn acquire_performance_configuration_intel<'lt>(
        self: &Unique<Device>,
        p_acquire_info: &PerformanceConfigurationAcquireInfoINTEL<'lt>,
    ) -> VulkanResult<Unique<PerformanceConfigurationINTEL>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.acquire_performance_configuration_intel())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.acquire_performance_configuration_intel())
            .unwrap_unchecked();
        let mut p_configuration = MaybeUninit::<PerformanceConfigurationINTEL>::uninit();
        let _return = _function(
            self.as_raw(),
            p_acquire_info as *const PerformanceConfigurationAcquireInfoINTEL<'lt>,
            p_configuration.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(
                _return,
                Unique::new(self, p_configuration.assume_init(), AtomicBool::default()),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkReleasePerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html) - Release a configuration to capture performance data
    ///# C Specifications
    ///To release a device performance configuration, call:
    ///```c
    ///// Provided by VK_INTEL_performance_query
    ///VkResult vkReleasePerformanceConfigurationINTEL(
    ///    VkDevice                                    device,
    ///    VkPerformanceConfigurationINTEL             configuration);
    ///```
    /// # Parameters
    /// - [`device`] is the device associated to the configuration object to release.
    /// - [`configuration`] is the configuration object to release.
    /// # Description
    /// ## Valid Usage
    /// - [`configuration`] **must**  not be released before all command buffers submitted while the
    ///   configuration was set are in [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
    ///
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - If [`configuration`] is not [`crate::Handle::null`], [`configuration`] **must**  be a
    ///   valid [`PerformanceConfigurationINTEL`] handle
    /// - If [`configuration`] is a valid handle, it  **must**  have been created, allocated, or
    ///   retrieved from [`device`]
    ///
    /// ## Host Synchronization
    /// - Host access to [`configuration`] **must**  be externally synchronized
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`VK_INTEL_performance_query`]
    /// - [`Device`]
    /// - [`PerformanceConfigurationINTEL`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
    #[track_caller]
    #[inline]
    pub unsafe fn release_performance_configuration_intel(
        self: &Unique<Device>,
        configuration: Option<PerformanceConfigurationINTEL>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.release_performance_configuration_intel())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.release_performance_configuration_intel())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), configuration.unwrap_or_default());
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetPerformanceParameterINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPerformanceParameterINTEL.html) - Query performance capabilities of the device
    ///# C Specifications
    ///Some performance query features of a device can be discovered with the call:
    ///```c
    ///// Provided by VK_INTEL_performance_query
    ///VkResult vkGetPerformanceParameterINTEL(
    ///    VkDevice                                    device,
    ///    VkPerformanceParameterTypeINTEL             parameter,
    ///    VkPerformanceValueINTEL*                    pValue);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device to query.
    /// - [`parameter`] is the parameter to query.
    /// - [`p_value`] is a pointer to a [`PerformanceValueINTEL`] structure in which the type and
    ///   value of the parameter are returned.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`parameter`] **must**  be a valid [`PerformanceParameterTypeINTEL`] value
    /// - [`p_value`] **must**  be a valid pointer to a [`PerformanceValueINTEL`] structure
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`VK_INTEL_performance_query`]
    /// - [`Device`]
    /// - [`PerformanceParameterTypeINTEL`]
    /// - [`PerformanceValueINTEL`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPerformanceParameterINTEL")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_performance_parameter_intel(
        self: &Unique<Device>,
        parameter: PerformanceParameterTypeINTEL,
    ) -> VulkanResult<PerformanceValueINTEL> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.get_performance_parameter_intel())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.get_performance_parameter_intel())
            .unwrap_unchecked();
        let mut p_value = MaybeUninit::<PerformanceValueINTEL>::uninit();
        let _return = _function(self.as_raw(), parameter, p_value.as_mut_ptr());
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_value.assume_init()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Queue {
    ///[vkQueueSetPerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html) - Set a performance query
    ///# C Specifications
    ///To set a performance configuration, call:
    ///```c
    ///// Provided by VK_INTEL_performance_query
    ///VkResult vkQueueSetPerformanceConfigurationINTEL(
    ///    VkQueue                                     queue,
    ///    VkPerformanceConfigurationINTEL             configuration);
    ///```
    /// # Parameters
    /// - [`queue`] is the queue on which the configuration will be used.
    /// - [`configuration`] is the configuration to use.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`queue`] **must**  be a valid [`Queue`] handle
    /// - [`configuration`] **must**  be a valid [`PerformanceConfigurationINTEL`] handle
    /// - Both of [`configuration`], and [`queue`] **must**  have been created, allocated, or
    ///   retrieved from the same [`Device`]
    ///
    /// ## Command Properties
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`VK_INTEL_performance_query`]
    /// - [`PerformanceConfigurationINTEL`]
    /// - [`Queue`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
    #[track_caller]
    #[inline]
    pub unsafe fn queue_set_performance_configuration_intel(
        self: &Unique<Queue>,
        configuration: PerformanceConfigurationINTEL,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.queue_set_performance_configuration_intel())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.queue_set_performance_configuration_intel())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), configuration);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl CommandBuffer {
    ///[vkCmdSetPerformanceMarkerINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html) - Markers
    ///# C Specifications
    ///To help associate query results with a particular point at which an
    ///application emitted commands, markers can be set into the command buffers
    ///with the call:
    ///```c
    ///// Provided by VK_INTEL_performance_query
    ///VkResult vkCmdSetPerformanceMarkerINTEL(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkPerformanceMarkerInfoINTEL*         pMarkerInfo);
    ///```
    /// # Parameters
    /// The last marker set onto a command buffer before the end of a query will be
    /// part of the query result.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_marker_info`] **must**  be a valid pointer to a valid [`PerformanceMarkerInfoINTEL`]
    ///   structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, compute, or transfer operations
    ///
    /// ## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    /// ## Command Properties
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`VK_INTEL_performance_query`]
    /// - [`CommandBuffer`]
    /// - [`PerformanceMarkerInfoINTEL`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_set_performance_marker_intel<'lt>(
        self: &Unique<CommandBuffer>,
        p_marker_info: &PerformanceMarkerInfoINTEL<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.cmd_set_performance_marker_intel())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.cmd_set_performance_marker_intel())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_marker_info as *const PerformanceMarkerInfoINTEL<'lt>);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl CommandBuffer {
    ///[vkCmdSetPerformanceStreamMarkerINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html) - Markers
    ///# C Specifications
    ///When monitoring the behavior of an application wihtin the dataset generated
    ///by the entire set of applications running on the system, it is useful to
    ///identify draw calls within a potentially huge amount of performance data.
    ///To do so, application can generate stream markers that will be used to trace
    ///back a particular draw call with a particular performance data item.
    ///```c
    ///// Provided by VK_INTEL_performance_query
    ///VkResult vkCmdSetPerformanceStreamMarkerINTEL(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkPerformanceStreamMarkerInfoINTEL*   pMarkerInfo);
    ///```
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_marker_info`] **must**  be a valid pointer to a valid
    ///   [`PerformanceStreamMarkerInfoINTEL`] structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, compute, or transfer operations
    ///
    /// ## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    /// ## Command Properties
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`VK_INTEL_performance_query`]
    /// - [`CommandBuffer`]
    /// - [`PerformanceStreamMarkerInfoINTEL`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_set_performance_stream_marker_intel<'lt>(
        self: &Unique<CommandBuffer>,
        p_marker_info: &PerformanceStreamMarkerInfoINTEL<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.cmd_set_performance_stream_marker_intel())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.cmd_set_performance_stream_marker_intel())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_marker_info as *const PerformanceStreamMarkerInfoINTEL<'lt>,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl CommandBuffer {
    ///[vkCmdSetPerformanceOverrideINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html) - Performance override settings
    ///# C Specifications
    ///Some applications might want measure the effect of a set of commands with a
    ///different settings.
    ///It is possible to override a particular settings using :
    ///```c
    ///// Provided by VK_INTEL_performance_query
    ///VkResult vkCmdSetPerformanceOverrideINTEL(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkPerformanceOverrideInfoINTEL*       pOverrideInfo);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer where the override takes place.
    /// - [`p_override_info`] is a pointer to a [`PerformanceOverrideInfoINTEL`] structure selecting
    ///   the parameter to override.
    /// # Description
    /// ## Valid Usage
    /// - [`p_override_info`] **must**  not be used with a [`PerformanceOverrideTypeINTEL`] that is
    ///   not reported available by [`get_performance_parameter_intel`]
    ///
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_override_info`] **must**  be a valid pointer to a valid
    ///   [`PerformanceOverrideInfoINTEL`] structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, compute, or transfer operations
    ///
    /// ## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    /// ## Command Properties
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`VK_INTEL_performance_query`]
    /// - [`CommandBuffer`]
    /// - [`PerformanceOverrideInfoINTEL`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_set_performance_override_intel<'lt>(
        self: &Unique<CommandBuffer>,
        p_override_info: &PerformanceOverrideInfoINTEL<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.cmd_set_performance_override_intel())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .intel_performance_query()
            .and_then(|vtable| vtable.cmd_set_performance_override_intel())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_override_info as *const PerformanceOverrideInfoINTEL<'lt>,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
///[VkPerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationINTEL.html) - Device configuration for performance queries
///# C Specifications
///Before submitting command buffers containing performance queries commands to
///a device queue, the application must acquire and set a performance query
///configuration.
///The configuration can be released once all command buffers containing
///performance query commands are not in a pending state.
///```c
///// Provided by VK_INTEL_performance_query
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPerformanceConfigurationINTEL)
///```
/// # Related
/// - [`VK_INTEL_performance_query`]
/// - [`acquire_performance_configuration_intel`]
/// - [`queue_set_performance_configuration_intel`]
/// - [`release_performance_configuration_intel`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceConfigurationINTEL")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct PerformanceConfigurationINTEL(pub u64);
impl PerformanceConfigurationINTEL {
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
unsafe impl Send for PerformanceConfigurationINTEL {}
impl Default for PerformanceConfigurationINTEL {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for PerformanceConfigurationINTEL {
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
                .release_performance_configuration_intel(Some(self.as_raw().coerce()));
        }
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<PerformanceConfigurationINTEL> {
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
///The V-table of [`Device`] for functions from `VK_INTEL_performance_query`
pub struct DeviceIntelPerformanceQueryVTable {
    ///See [`FNInitializePerformanceApiIntel`] for more information.
    pub initialize_performance_api_intel: FNInitializePerformanceApiIntel,
    ///See [`FNUninitializePerformanceApiIntel`] for more information.
    pub uninitialize_performance_api_intel: FNUninitializePerformanceApiIntel,
    ///See [`FNAcquirePerformanceConfigurationIntel`] for more information.
    pub acquire_performance_configuration_intel: FNAcquirePerformanceConfigurationIntel,
    ///See [`FNReleasePerformanceConfigurationIntel`] for more information.
    pub release_performance_configuration_intel: FNReleasePerformanceConfigurationIntel,
    ///See [`FNQueueSetPerformanceConfigurationIntel`] for more information.
    pub queue_set_performance_configuration_intel: FNQueueSetPerformanceConfigurationIntel,
    ///See [`FNGetPerformanceParameterIntel`] for more information.
    pub get_performance_parameter_intel: FNGetPerformanceParameterIntel,
    ///See [`FNCmdSetPerformanceMarkerIntel`] for more information.
    pub cmd_set_performance_marker_intel: FNCmdSetPerformanceMarkerIntel,
    ///See [`FNCmdSetPerformanceStreamMarkerIntel`] for more information.
    pub cmd_set_performance_stream_marker_intel: FNCmdSetPerformanceStreamMarkerIntel,
    ///See [`FNCmdSetPerformanceOverrideIntel`] for more information.
    pub cmd_set_performance_override_intel: FNCmdSetPerformanceOverrideIntel,
}
impl DeviceIntelPerformanceQueryVTable {
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
            initialize_performance_api_intel: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkInitializePerformanceApiINTEL").as_ptr(),
                ))
            },
            uninitialize_performance_api_intel: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkUninitializePerformanceApiINTEL").as_ptr(),
                ))
            },
            acquire_performance_configuration_intel: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkAcquirePerformanceConfigurationINTEL").as_ptr(),
                ))
            },
            release_performance_configuration_intel: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkReleasePerformanceConfigurationINTEL").as_ptr(),
                ))
            },
            queue_set_performance_configuration_intel: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkQueueSetPerformanceConfigurationINTEL").as_ptr(),
                ))
            },
            get_performance_parameter_intel: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPerformanceParameterINTEL").as_ptr(),
                ))
            },
            cmd_set_performance_marker_intel: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdSetPerformanceMarkerINTEL").as_ptr(),
                ))
            },
            cmd_set_performance_stream_marker_intel: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdSetPerformanceStreamMarkerINTEL").as_ptr(),
                ))
            },
            cmd_set_performance_override_intel: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdSetPerformanceOverrideINTEL").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::initialize_performance_api_intel`]. See [`FNInitializePerformanceApiIntel`] for
    /// more information.
    pub fn initialize_performance_api_intel(&self) -> FNInitializePerformanceApiIntel {
        self.initialize_performance_api_intel
    }
    ///Gets [`Self::uninitialize_performance_api_intel`]. See [`FNUninitializePerformanceApiIntel`]
    /// for more information.
    pub fn uninitialize_performance_api_intel(&self) -> FNUninitializePerformanceApiIntel {
        self.uninitialize_performance_api_intel
    }
    ///Gets [`Self::acquire_performance_configuration_intel`]. See
    /// [`FNAcquirePerformanceConfigurationIntel`] for more information.
    pub fn acquire_performance_configuration_intel(&self) -> FNAcquirePerformanceConfigurationIntel {
        self.acquire_performance_configuration_intel
    }
    ///Gets [`Self::release_performance_configuration_intel`]. See
    /// [`FNReleasePerformanceConfigurationIntel`] for more information.
    pub fn release_performance_configuration_intel(&self) -> FNReleasePerformanceConfigurationIntel {
        self.release_performance_configuration_intel
    }
    ///Gets [`Self::queue_set_performance_configuration_intel`]. See
    /// [`FNQueueSetPerformanceConfigurationIntel`] for more information.
    pub fn queue_set_performance_configuration_intel(&self) -> FNQueueSetPerformanceConfigurationIntel {
        self.queue_set_performance_configuration_intel
    }
    ///Gets [`Self::get_performance_parameter_intel`]. See [`FNGetPerformanceParameterIntel`] for
    /// more information.
    pub fn get_performance_parameter_intel(&self) -> FNGetPerformanceParameterIntel {
        self.get_performance_parameter_intel
    }
    ///Gets [`Self::cmd_set_performance_marker_intel`]. See [`FNCmdSetPerformanceMarkerIntel`] for
    /// more information.
    pub fn cmd_set_performance_marker_intel(&self) -> FNCmdSetPerformanceMarkerIntel {
        self.cmd_set_performance_marker_intel
    }
    ///Gets [`Self::cmd_set_performance_stream_marker_intel`]. See
    /// [`FNCmdSetPerformanceStreamMarkerIntel`] for more information.
    pub fn cmd_set_performance_stream_marker_intel(&self) -> FNCmdSetPerformanceStreamMarkerIntel {
        self.cmd_set_performance_stream_marker_intel
    }
    ///Gets [`Self::cmd_set_performance_override_intel`]. See [`FNCmdSetPerformanceOverrideIntel`]
    /// for more information.
    pub fn cmd_set_performance_override_intel(&self) -> FNCmdSetPerformanceOverrideIntel {
        self.cmd_set_performance_override_intel
    }
}
