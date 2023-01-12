[VK_KHR_global_priority](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_global_priority.html) - device extension

# Description
In Vulkan, users can specify device-scope queue priorities.
In some cases it may be useful to extend this concept to a system-wide
scope.
This device extension allows applications to query the global queue
priorities supported by a queue family, and then set a priority when
creating queues.
The default queue priority is `VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT`.Implementations can report which global priority levels are treated
differently by the implementation.
It is intended primarily for use in system integration along with certain
platform-specific priority enforcement rules.The driver implementation will attempt to skew hardware resource allocation
in favour of the higher-priority task.
Therefore, higher-priority work may retain similar latency and throughput
characteristics even if the system is congested with lower priority work.The global priority level of a queue shall take precedence over the
per-process queue priority
([`DeviceQueueCreateInfo::queue_priorities`]).Abuse of this feature may result in starving the rest of the system from
hardware resources.
Therefore, the driver implementation may deny requests to acquire a priority
above the default priority (`VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT`) if
the caller does not have sufficient privileges.
In this scenario `VK_ERROR_NOT_PERMITTED_EXT` is returned.The driver implementation may fail the queue allocation request if resources
required to complete the operation have been exhausted (either by the same
process or a different process).
In this scenario `VK_ERROR_INITIALIZATION_FAILED` is returned.

# Registered extension number
189

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_global_priority] @tobski%0A<<Here describe the issue or question you have about the VK_KHR_global_priority extension>>)

# New structures
- Extending [`DeviceQueueCreateInfo`]:  - [`DeviceQueueGlobalPriorityCreateInfoKHR`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`] 
- Extending [`QueueFamilyProperties2`]:  - [`QueueFamilyGlobalPriorityPropertiesKHR`]

# New enums
- [`QueueGlobalPriorityKHR`]

# New constants
- `VK_KHR_GLOBAL_PRIORITY_EXTENSION_NAME`
- `VK_KHR_GLOBAL_PRIORITY_SPEC_VERSION`
- `VK_MAX_GLOBAL_PRIORITY_SIZE_KHR`
- Extending [`VulkanResultCodes`]:  - `VK_ERROR_NOT_PERMITTED_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR`  - `VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR`

# Known issues & F.A.Q.
1) Can we additionally query whether a caller is permitted to acquire a
specific global queue priority in this extension? **RESOLVED** : No.
Whether a caller has enough privilege goes with the OS, and the Vulkan
driver cannot really guarantee that the privilege will not change in between
this query and the actual queue creation call.2) If more than 1 queue using global priority is requested, is there a good
way to know which queue is failing the device creation? **RESOLVED** : No.
There is not a good way at this moment, and it is also not quite actionable
for the applications to know that because the information may not be
accurate.
Queue creation can fail because of runtime constraints like insufficient
privilege or lack of resource, and the failure is not necessarily tied to
that particular queue configuration requested.

# Version history
- Revision 1, 2021-10-22 (Tobias Hector)  - Initial draft

# Other information
* 2021-10-22
*   - Tobias Hector, AMD  - Contributors to [`ext_global_priority`]  - Contributors to [`ext_global_priority_query`]

# Related
- [VK_MAX_GLOBAL_PRIORITY_SIZE_KHR]()
- [`DeviceQueueGlobalPriorityCreateInfoKHR`]
- [`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`]
- [`QueueFamilyGlobalPriorityPropertiesKHR`]
- [`QueueGlobalPriorityKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        