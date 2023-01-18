[VK_KHR_performance_query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_performance_query.html) - device extension

# Description
The [`VK_KHR_performance_query`] extension adds a mechanism to allow querying
of performance counters for use in applications and by profiling tools.Each queue family  **may**  expose counters that  **can**  be enabled on a queue of
that family.
We extend [`QueryType`] to add a new query type for performance queries,
and chain a structure on [`QueryPoolCreateInfo`] to specify the
performance queries to enable.

# Registered extension number
117

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Alon Or-bach [alonorbach](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_performance_query] @alonorbach%0A<<Here describe the issue or question you have about the VK_KHR_performance_query extension>>)

# New commands
- [`acquire_profiling_lock_khr`]
- [`enumerate_physical_device_queue_family_performance_query_counters_khr`]
- [`get_physical_device_queue_family_performance_query_passes_khr`]
- [`release_profiling_lock_khr`]

# New structures
- [`AcquireProfilingLockInfoKHR`]
- [`PerformanceCounterDescriptionKHR`]
- [`PerformanceCounterKHR`]
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDevicePerformanceQueryFeaturesKHR`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDevicePerformanceQueryPropertiesKHR`] 
- Extending [`QueryPoolCreateInfo`]:  - [`QueryPoolPerformanceCreateInfoKHR`] 
- Extending [`SubmitInfo`], [`SubmitInfo2`]:  - [`PerformanceQuerySubmitInfoKHR`]

# New unions
- [`PerformanceCounterResultKHR`]

# New enums
- [`AcquireProfilingLockFlagBitsKHR`]
- [`PerformanceCounterDescriptionFlagBitsKHR`]
- [`PerformanceCounterScopeKHR`]
- [`PerformanceCounterStorageKHR`]
- [`PerformanceCounterUnitKHR`]

# New bitmasks
- [`AcquireProfilingLockFlagsKHR`]
- [`PerformanceCounterDescriptionFlagsKHR`]

# New constants
- [`KHR_PERFORMANCE_QUERY_EXTENSION_NAME`]
- [`KHR_PERFORMANCE_QUERY_SPEC_VERSION`]
- Extending [`QueryType`]:  - `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR`  - `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR`  - `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR`  - `VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR`  - `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR`

# Known issues & F.A.Q.
1) Should this extension include a mechanism to begin a query in command
buffer *A* and end the query in command buffer *B*? **RESOLVED**  No - queries are tied to command buffer creation and thus have to
be encapsulated within a single command buffer.2) Should this extension include a mechanism to begin and end queries
globally on the queue, not using the existing command buffer commands? **RESOLVED**  No - for the same reasoning as the resolution of 1).3) Should this extension expose counters that require multiple passes? **RESOLVED**  Yes - users should re-submit a command buffer with the same
commands in it multiple times, specifying the pass to count as the query
parameter in VkPerformanceQuerySubmitInfoKHR.4) How to handle counters across parallel workloads? **RESOLVED**  In the spirit of Vulkan, a counter description flag
`VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR`
denotes that the accuracy of a counter result is affected by parallel
workloads.5) How to handle secondary command buffers? **RESOLVED**  Secondary command buffers inherit any counter pass index
specified in the parent primary command buffer.
Note: this is no longer an issue after change from issue 10 resolution6) What commands does the profiling lock have to be held for? **RESOLVED**  For any command buffer that is being queried with a performance
query pool, the profiling lock  **must**  be held while that command buffer is in
the *recording*, *executable*, or *pending state*.7) Should we support [`cmd_copy_query_pool_results`]? **RESOLVED**  Yes.8) Should we allow performance queries to interact with multiview? **RESOLVED**  Yes, but the performance queries must be performed once for each
pass per view.9) Should a `queryCount > 1` be usable for performance queries? **RESOLVED**  Yes.
Some vendors will have costly performance counter query pool creation, and
would rather if a certain set of counters were to be used multiple times
that a `queryCount > 1` can be used to amortize the instantiation cost.10) Should we introduce an indirect mechanism to set the counter pass index? **RESOLVED**  Specify the counter pass index at submit time instead, to avoid
requiring re-recording of command buffers when multiple counter passes are
needed.

# Version history
- Revision 1, 2019-10-08

# Other information
* 2019-10-08
* No known IP claims.
*   - Jesse Barker, Unity Technologies  - Kenneth Benzie, Codeplay  - Jan-Harald Fredriksen, ARM  - Jeff Leger, Qualcomm  - Jesse Hall, Google  - Tobias Hector, AMD  - Neil Henning, Codeplay  - Baldur Karlsson  - Lionel Landwerlin, Intel  - Peter Lohrmann, AMD  - Alon Or-bach, Samsung  - Daniel Rakos, AMD  - Niklas Smedberg, Unity Technologies  - Igor Ostrowski, Intel

# Related
- [`AcquireProfilingLockFlagBitsKHR`]
- [`AcquireProfilingLockFlagsKHR`]
- [`AcquireProfilingLockInfoKHR`]
- [`PerformanceCounterDescriptionFlagBitsKHR`]
- [`PerformanceCounterDescriptionFlagsKHR`]
- [`PerformanceCounterDescriptionKHR`]
- [`PerformanceCounterKHR`]
- [`PerformanceCounterResultKHR`]
- [`PerformanceCounterScopeKHR`]
- [`PerformanceCounterStorageKHR`]
- [`PerformanceCounterUnitKHR`]
- [`PerformanceQuerySubmitInfoKHR`]
- [`PhysicalDevicePerformanceQueryFeaturesKHR`]
- [`PhysicalDevicePerformanceQueryPropertiesKHR`]
- [`QueryPoolPerformanceCreateInfoKHR`]
- [`acquire_profiling_lock_khr`]
- [`enumerate_physical_device_queue_family_performance_query_counters_khr`]
- [`get_physical_device_queue_family_performance_query_passes_khr`]
- [`release_profiling_lock_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        