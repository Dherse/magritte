///[VK_MAX_PHYSICAL_DEVICE_NAME_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_PHYSICAL_DEVICE_NAME_SIZE.html) - Length of a physical device name string
///# C Specifications
///[`MAX_PHYSICAL_DEVICE_NAME_SIZE`] is the length in `char` values of
///an array containing a physical device name string, as returned in
///[`PhysicalDeviceProperties`]::deviceName.
///```c
///#define VK_MAX_PHYSICAL_DEVICE_NAME_SIZE  256U
///```
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_PHYSICAL_DEVICE_NAME_SIZE")]
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
///[VK_UUID_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_UUID_SIZE.html) - Length of a universally unique device or driver build identifier
///# C Specifications
///[`UUID_SIZE`] is the length in `uint8_t` values of an array
///containing a universally unique device or driver build identifier, as
///returned in [`PhysicalDeviceIdProperties`]::deviceUUID and
///[`PhysicalDeviceIdProperties`]::driverUUID.
///```c
///#define VK_UUID_SIZE                      16U
///```
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_UUID_SIZE")]
pub const UUID_SIZE: u32 = 16;
///[VK_MAX_EXTENSION_NAME_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_EXTENSION_NAME_SIZE.html) - Maximum length of a layer of extension name string
///# C Specifications
///[`MAX_EXTENSION_NAME_SIZE`] is the length in `char` values of an
///array containing a layer or extension name string, as returned in
///[`LayerProperties`]::layerName,
///[`ExtensionProperties`]::extensionName, and other queries.
///```c
///#define VK_MAX_EXTENSION_NAME_SIZE        256U
///```
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_EXTENSION_NAME_SIZE")]
pub const MAX_EXTENSION_NAME_SIZE: u32 = 256;
///[VK_MAX_DESCRIPTION_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DESCRIPTION_SIZE.html) - Length of a driver name string
///# C Specifications
///[`MAX_DESCRIPTION_SIZE`] is the length in `char` values of an array
///containing a string with additional descriptive information about a query,
///as returned in [`LayerProperties`]::description and other queries.
///```c
///#define VK_MAX_DESCRIPTION_SIZE           256U
///```
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_DESCRIPTION_SIZE")]
pub const MAX_DESCRIPTION_SIZE: u32 = 256;
///[VK_MAX_MEMORY_TYPES](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_MEMORY_TYPES.html) - Length of an array of memory types
///# C Specifications
///[`MAX_MEMORY_TYPES`] is the length of an array of [`MemoryType`]
///structures describing memory types, as returned in
///[`PhysicalDeviceMemoryProperties`]::memoryTypes.
///```c
///#define VK_MAX_MEMORY_TYPES               32U
///```
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_MEMORY_TYPES")]
pub const MAX_MEMORY_TYPES: u32 = 32;
///[VK_MAX_MEMORY_HEAPS](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_MEMORY_HEAPS.html) - Length of an array of memory heaps
///# C Specifications
///[`MAX_MEMORY_HEAPS`] is the length of an array of [`MemoryHeap`]
///structures describing memory heaps, as returned in
///[`PhysicalDeviceMemoryProperties`]::memoryHeaps.
///```c
///#define VK_MAX_MEMORY_HEAPS               16U
///```
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_MEMORY_HEAPS")]
pub const MAX_MEMORY_HEAPS: u32 = 16;
