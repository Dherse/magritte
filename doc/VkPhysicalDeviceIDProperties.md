[VkPhysicalDeviceIDProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIDProperties.html) - Structure specifying IDs related to the physical device

# C Specifications
The [`PhysicalDeviceIdProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDeviceIDProperties {
    VkStructureType    sType;
    void*              pNext;
    uint8_t            deviceUUID[VK_UUID_SIZE];
    uint8_t            driverUUID[VK_UUID_SIZE];
    uint8_t            deviceLUID[VK_LUID_SIZE];
    uint32_t           deviceNodeMask;
    VkBool32           deviceLUIDValid;
} VkPhysicalDeviceIDProperties;
```
or the equivalent
```c
// Provided by VK_KHR_external_fence_capabilities, VK_KHR_external_memory_capabilities, VK_KHR_external_semaphore_capabilities
typedef VkPhysicalDeviceIDProperties VkPhysicalDeviceIDPropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

# Description
- [`device_uuid`] is an array of `VK_UUID_SIZE``uint8_t` values representing a universally unique identifier for the device.
- [`driver_uuid`] is an array of `VK_UUID_SIZE``uint8_t` values representing a universally unique identifier for the driver build in use by the device.
- [`device_luid`] is an array of `VK_LUID_SIZE``uint8_t` values representing a locally unique identifier for the device.
- [`device_node_mask`] is a `uint32_t` bitfield identifying the node within a linked device adapter corresponding to the device.
- [`device_luid_valid`] is a boolean value that will be `VK_TRUE` if [`device_luid`] contains a valid LUID and [`device_node_mask`] contains a valid node mask, and `VK_FALSE` if they do not.
If the [`PhysicalDeviceIdProperties`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.[`device_uuid`] **must**  be immutable for a given device across instances,
processes, driver APIs, driver versions, and system reboots.Applications  **can**  compare the [`driver_uuid`] value across instance and
process boundaries, and  **can**  make similar queries in external APIs to
determine whether they are capable of sharing memory objects and resources
using them with the device.[`device_uuid`] and/or [`driver_uuid`] **must**  be used to determine whether
a particular external object can be shared between driver components, where
such a restriction exists as defined in the compatibility table for the
particular object type:
- [External memory handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)
- [External semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
- [External fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)
If [`device_luid_valid`] is `VK_FALSE`, the values of [`device_luid`]
and [`device_node_mask`] are undefined.
If [`device_luid_valid`] is `VK_TRUE` and Vulkan is running on the
Windows operating system, the contents of [`device_luid`] **can**  be cast to
an `LUID` object and  **must**  be equal to the locally unique identifier of a
`IDXGIAdapter1` object that corresponds to `physicalDevice`.
If [`device_luid_valid`] is `VK_TRUE`, [`device_node_mask`] **must** 
contain exactly one bit.
If Vulkan is running on an operating system that supports the Direct3D 12
API and `physicalDevice` corresponds to an individual device in a linked
device adapter, [`device_node_mask`] identifies the Direct3D 12 node
corresponding to `physicalDevice`.
Otherwise, [`device_node_mask`] **must**  be `1`.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES`

# Related
- [`crate::vulkan1_1`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        