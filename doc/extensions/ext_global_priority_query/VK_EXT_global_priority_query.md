[VK_EXT_global_priority_query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_global_priority_query.html) - device extension

# Description
This device extension allows applications to query the global queue
priorities supported by a queue family.
It allows implementations to report which global priority levels are treated
differently by the implementation, instead of silently mapping multiple
requested global priority levels to the same internal priority, or using
device creation failure to signal that a requested priority is not
supported.
It is intended primarily for use by system integration along with certain
platform-specific priority enforcement rules.

# Registered extension number
389

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_EXT_global_priority`]`
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Deprecation state
- *Promoted* to `[`VK_KHR_global_priority`]` extension

# Contacts
- Yiwei Zhang [zhangyiwei](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_global_priority_query] @zhangyiwei%0A<<Here describe the issue or question you have about the VK_EXT_global_priority_query extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceGlobalPriorityQueryFeaturesEXT`] 
- Extending [`QueueFamilyProperties2`]:  - [`QueueFamilyGlobalPriorityPropertiesEXT`]

# New constants
- [`EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME`]
- [`EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION`]
- [`MAX_GLOBAL_PRIORITY_SIZE_EXT`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT`

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
- Revision 1, 2021-03-29 (Yiwei Zhang)

# Other information
* 2021-03-29
* No known IP claims.
*   - Yiwei Zhang, Google

# Related
- [`MAX_GLOBAL_PRIORITY_SIZE_EXT`]
- [`PhysicalDeviceGlobalPriorityQueryFeaturesEXT`]
- [`QueueFamilyGlobalPriorityPropertiesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        