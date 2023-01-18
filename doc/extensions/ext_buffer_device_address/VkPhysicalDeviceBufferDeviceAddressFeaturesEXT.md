[VkPhysicalDeviceBufferDeviceAddressFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html) - Structure describing buffer address features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_buffer_device_address
typedef struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           bufferDeviceAddress;
    VkBool32           bufferDeviceAddressCaptureReplay;
    VkBool32           bufferDeviceAddressMultiDevice;
} VkPhysicalDeviceBufferDeviceAddressFeaturesEXT;
```

```c
// Provided by VK_EXT_buffer_device_address
typedef VkPhysicalDeviceBufferDeviceAddressFeaturesEXT VkPhysicalDeviceBufferAddressFeaturesEXT;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`buffer_device_address`] indicates that the implementation supports accessing buffer memory in shaders as storage buffers via an address queried from [`get_buffer_device_address_ext`].
- [`buffer_device_address_capture_replay`] indicates that the implementation supports saving and reusing buffer addresses, e.g. for trace capture and replay.
- [`buffer_device_address_multi_device`] indicates that the implementation supports the [`buffer_device_address`] feature for logical devices created with multiple physical devices. If this feature is not supported, buffer addresses  **must**  not be queried on a logical device created with more than one physical device.
If the [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceBufferDeviceAddressFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT`

# Related
- [`VK_EXT_buffer_device_address`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        