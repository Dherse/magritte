[VkPhysicalDeviceProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties.html) - Structure specifying physical device properties

# C Specifications
The [`PhysicalDeviceProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPhysicalDeviceProperties {
    uint32_t                            apiVersion;
    uint32_t                            driverVersion;
    uint32_t                            vendorID;
    uint32_t                            deviceID;
    VkPhysicalDeviceType                deviceType;
    char                                deviceName[VK_MAX_PHYSICAL_DEVICE_NAME_SIZE];
    uint8_t                             pipelineCacheUUID[VK_UUID_SIZE];
    VkPhysicalDeviceLimits              limits;
    VkPhysicalDeviceSparseProperties    sparseProperties;
} VkPhysicalDeviceProperties;
```

# Members
- [`api_version`] is the version of Vulkan supported by the device, encoded as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers).
- [`driver_version`] is the vendor-specified version of the driver.
- [`vendor_id`] is a unique identifier for the *vendor* (see below) of the physical device.
- [`device_id`] is a unique identifier for the physical device among devices available from the vendor.
- [`device_type`] is a [`PhysicalDeviceType`] specifying the type of device.
- [`device_name`] is an array of `VK_MAX_PHYSICAL_DEVICE_NAME_SIZE``char` containing a null-terminated UTF-8 string which is the name of the device.
- [`pipeline_cache_uuid`] is an array of `VK_UUID_SIZE``uint8_t` values representing a universally unique identifier for the device.
- [`limits`] is the [`PhysicalDeviceLimits`] structure specifying device-specific limits of the physical device. See [Limits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits) for details.
- [`sparse_properties`] is the [`PhysicalDeviceSparseProperties`] structure specifying various sparse related properties of the physical device. See [Sparse Properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory-physicalprops) for details.

# Description
On implementations that claim support for the [Roadmap 2022](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#roadmap-2022)
profile, the major and minor version expressed by [`api_version`] **must**  be
at least Vulkan 1.3.The [`vendor_id`] and [`device_id`] fields are provided to allow
applications to adapt to device characteristics that are not adequately
exposed by other Vulkan queries.The *vendor* identified by [`vendor_id`] is the entity responsible for the
most salient characteristics of the underlying implementation of the
[`PhysicalDevice`] being queried.If the vendor has a [PCI
vendor ID](https://pcisig.com/membership/member-companies), the low 16 bits of [`vendor_id`] **must**  contain that PCI vendor
ID, and the remaining bits  **must**  be set to zero.
Otherwise, the value returned  **must**  be a valid Khronos vendor ID, obtained
as described in the [Vulkan Documentation and Extensions:
Procedures and Conventions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vulkan-styleguide) document in the section “Registering a Vendor
ID with Khronos”.
Khronos vendor IDs are allocated starting at 0x10000, to distinguish them
from the PCI vendor ID namespace.
Khronos vendor IDs are symbolically defined in the [`VendorId`] type.The vendor is also responsible for the value returned in [`device_id`].
If the implementation is driven primarily by a [PCI
device](https://pcisig.com/) with a [PCI device ID](https://pcisig.com/), the low 16 bits of
[`device_id`] **must**  contain that PCI device ID, and the remaining bits
 **must**  be set to zero.
Otherwise, the choice of what values to return  **may**  be dictated by operating
system or platform policies - but  **should**  uniquely identify both the device
version and any major configuration options (for example, core count in the
case of multicore devices).

# Related
- [`crate::vulkan1_0`]
- [`PhysicalDeviceLimits`]
- [`PhysicalDeviceProperties2`]
- [`PhysicalDeviceSparseProperties`]
- [`PhysicalDeviceType`]
- [`get_physical_device_properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        