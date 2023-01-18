[VK_EXT_global_priority](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_global_priority.html) - device extension

# Description
In Vulkan, users can specify device-scope queue priorities.
In some cases it may be useful to extend this concept to a system-wide
scope.
This extension provides a mechanism for callers to set their system-wide
priority.
The default queue priority is `VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT`.The driver implementation will attempt to skew hardware resource allocation
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
175

# Revision
2

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Promoted* to `[`VK_KHR_global_priority`]` extension

# Contacts
- Andres Rodriguez [lostgoat](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_global_priority] @lostgoat%0A<<Here describe the issue or question you have about the VK_EXT_global_priority extension>>)

# New structures
- Extending [`DeviceQueueCreateInfo`]:  - [`DeviceQueueGlobalPriorityCreateInfoEXT`]

# New enums
- [`QueueGlobalPriorityEXT`]

# New constants
- [`EXT_GLOBAL_PRIORITY_EXTENSION_NAME`]
- [`EXT_GLOBAL_PRIORITY_SPEC_VERSION`]
- Extending [`VulkanResultCodes`]:  - `VK_ERROR_NOT_PERMITTED_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT`

# Version history
- Revision 2, 2017-11-03 (Andres Rodriguez)  - Fixed VkQueueGlobalPriorityEXT missing _EXT suffix 
- Revision 1, 2017-10-06 (Andres Rodriguez)  - First version.

# Other information
* 2017-10-06
* No known IP claims.
*   - Andres Rodriguez, Valve  - Pierre-Loup Griffais, Valve  - Dan Ginsburg, Valve  - Mitch Singer, AMD

# Related
- [`DeviceQueueGlobalPriorityCreateInfoEXT`]
- [`QueueGlobalPriorityEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        