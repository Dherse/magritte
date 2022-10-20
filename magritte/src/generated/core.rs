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
///[VK_LUID_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_LUID_SIZE.html) - Length of a locally unique device identifier
///# C Specifications
///[`LUID_SIZE`] is the length in `uint8_t` values of an array
///containing a locally unique device identifier, as returned in
///[`PhysicalDeviceIdProperties`]::deviceLUID.
///```c
///#define VK_LUID_SIZE                      8U
///```
///or the equivalent
///```c
///#define VK_LUID_SIZE_KHR                  VK_LUID_SIZE
///```
///# Related
/// - [`khr_external_fence_capabilities`]
/// - [`khr_external_memory_capabilities`]
/// - [`khr_external_semaphore_capabilities`]
/// - [`crate::vulkan1_1`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_LUID_SIZE")]
pub const LUID_SIZE: u32 = 8;
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
///[VK_LOD_CLAMP_NONE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_LOD_CLAMP_NONE.html) - Maximum level of detail unclamped access sentinel
///# C Specifications
///[`LOD_CLAMP_NONE`] is a special constant value used for
///[`SamplerCreateInfo::max_lod`] to indicate that maximum LOD
///clamping should not be performed.
///```c
///#define VK_LOD_CLAMP_NONE                 1000.0F
///```
///# Related
/// - [`crate::vulkan1_0`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_LOD_CLAMP_NONE")]
pub const LOD_CLAMP_NONE: f32 = 1000.0;
///[VK_REMAINING_MIP_LEVELS](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_REMAINING_MIP_LEVELS.html) - Sentinel for all remaining mipmap levels
///# C Specifications
///[`REMAINING_MIP_LEVELS`] is a special constant value used for image
///views to indicate that all remaining mipmap levels in an image after the
///base level should be included in the view.
///```c
///#define VK_REMAINING_MIP_LEVELS           (~0U)
///```
///# Related
/// - [`crate::vulkan1_0`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_REMAINING_MIP_LEVELS")]
pub const REMAINING_MIP_LEVELS: u32 = !0;
///[VK_REMAINING_ARRAY_LAYERS](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_REMAINING_ARRAY_LAYERS.html) - Sentinel for all remaining array layers
///# C Specifications
///[`REMAINING_ARRAY_LAYERS`] is a special constant value used for image
///views to indicate that all remaining array layers in an image after the base
///layer should be included in the view.
///```c
///#define VK_REMAINING_ARRAY_LAYERS         (~0U)
///```
///# Related
/// - [`crate::vulkan1_0`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_REMAINING_ARRAY_LAYERS")]
pub const REMAINING_ARRAY_LAYERS: u32 = !0;
///[VK_WHOLE_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_WHOLE_SIZE.html) - Sentinel value to use entire remaining array length
///# C Specifications
///[`WHOLE_SIZE`] is a special value indicating that the entire remaining
///length of a buffer following a given `offset` should be used.
///It  **can**  be specified for [`BufferMemoryBarrier::size`] and other
///structures.
///```c
///#define VK_WHOLE_SIZE                     (~0ULL)
///```
///# Related
/// - [`crate::vulkan1_0`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_WHOLE_SIZE")]
pub const WHOLE_SIZE: u64 = !0;
///[VK_ATTACHMENT_UNUSED](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_ATTACHMENT_UNUSED.html) - Unused attachment sentinel
///# C Specifications
///[`ATTACHMENT_UNUSED`] is a constant indicating that a render pass
///attachment is not used.
///```c
///#define VK_ATTACHMENT_UNUSED              (~0U)
///```
///# Related
/// - [`crate::vulkan1_0`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_ATTACHMENT_UNUSED")]
pub const ATTACHMENT_UNUSED: u32 = !0;
///[VK_TRUE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_TRUE.html) - Boolean true value
///# C Specifications
///[`TRUE`] is a constant representing a [`Bool32`] **True**  value.
///```c
///#define VK_TRUE                           1U
///```
///# Related
/// - [`FALSE`]
/// - [`crate::vulkan1_0`]
/// - [`Bool32`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_TRUE")]
pub const TRUE: u32 = 1;
///[VK_FALSE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_FALSE.html) - Boolean false value
///# C Specifications
///[`FALSE`] is a constant representing a [`Bool32`] **False**  value.
///```c
///#define VK_FALSE                          0U
///```
///# Related
/// - [`TRUE`]
/// - [`crate::vulkan1_0`]
/// - [`Bool32`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_FALSE")]
pub const FALSE: u32 = 0;
///[VK_QUEUE_FAMILY_IGNORED](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_IGNORED.html) - Ignored queue family index sentinel
///# C Specifications
///The special queue family index [`QUEUE_FAMILY_IGNORED`] indicates that
///a queue family parameter or member is ignored.
///```c
///#define VK_QUEUE_FAMILY_IGNORED           (~0U)
///```
///# Related
/// - [`crate::vulkan1_0`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_QUEUE_FAMILY_IGNORED")]
pub const QUEUE_FAMILY_IGNORED: u32 = !0;
///[VK_QUEUE_FAMILY_EXTERNAL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_EXTERNAL.html) - External queue family index sentinel
///# C Specifications
///The special queue family index [`QUEUE_FAMILY_EXTERNAL`] represents any
///queue external to the resource’s current Vulkan instance, as long as the
///queue uses the same underlying
///device group or
///physical device, and the same driver version as the resource’s
///[`Device`], as indicated by
///[`PhysicalDeviceIdProperties::device_uuid`] and
///[`PhysicalDeviceIdProperties::driver_uuid`].
///```c
///#define VK_QUEUE_FAMILY_EXTERNAL          (~1U)
///```
///or the equivalent
///```c
///#define VK_QUEUE_FAMILY_EXTERNAL_KHR      VK_QUEUE_FAMILY_EXTERNAL
///```
///# Related
/// - [`khr_external_memory`]
/// - [`crate::vulkan1_1`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_QUEUE_FAMILY_EXTERNAL")]
pub const QUEUE_FAMILY_EXTERNAL: u32 = !1;
///[VK_QUEUE_FAMILY_FOREIGN_EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_FOREIGN_EXT.html) - Foreign queue family index sentinel
///# C Specifications
///The special queue family index [`QUEUE_FAMILY_FOREIGN_EXT`] represents
///any queue external to the resource’s current Vulkan instance, regardless of
///the queue’s underlying physical device or driver version.
///This includes, for example, queues for fixed-function image processing
///devices, media codec devices, and display devices, as well as all queues
///that use the same underlying
///device group or
///physical device, and the same driver version as the resource’s
///[`Device`].
///```c
///#define VK_QUEUE_FAMILY_FOREIGN_EXT       (~2U)
///```
///# Related
/// - [`ext_queue_family_foreign`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_QUEUE_FAMILY_FOREIGN_EXT")]
pub const QUEUE_FAMILY_FOREIGN_EXT: u32 = !2;
///[VK_SUBPASS_EXTERNAL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_SUBPASS_EXTERNAL.html) - Subpass index sentinel expanding synchronization scope outside a subpass
///# C Specifications
///[`SUBPASS_EXTERNAL`] is a special subpass index value expanding
///synchronization scope outside a subpass.
///It is described in more detail by [`SubpassDependency`].
///```c
///#define VK_SUBPASS_EXTERNAL               (~0U)
///```
///# Related
/// - [`crate::vulkan1_0`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_SUBPASS_EXTERNAL")]
pub const SUBPASS_EXTERNAL: u32 = !0;
///[VK_MAX_DEVICE_GROUP_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DEVICE_GROUP_SIZE.html) - Length of a physical device handle array
///# C Specifications
///[`MAX_DEVICE_GROUP_SIZE`] is the length of an array containing
///[`PhysicalDevice`] handle values representing all physical devices in a
///group, as returned in
///[`PhysicalDeviceGroupProperties`]::physicalDevices.
///```c
///#define VK_MAX_DEVICE_GROUP_SIZE          32U
///```
///or the equivalent
///```c
///#define VK_MAX_DEVICE_GROUP_SIZE_KHR      VK_MAX_DEVICE_GROUP_SIZE
///```
///# Related
/// - [`khr_device_group_creation`]
/// - [`crate::vulkan1_1`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_DEVICE_GROUP_SIZE")]
pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
///[VK_MAX_DRIVER_NAME_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_NAME_SIZE.html) - Maximum length of a physical device driver name string
///# C Specifications
///[`MAX_DRIVER_NAME_SIZE`] is the length in `char` values of an array
///containing a driver name string, as returned in
///[`PhysicalDeviceDriverProperties`]::driverName.
///```c
///#define VK_MAX_DRIVER_NAME_SIZE           256U
///```
///or the equivalent
///```c
///#define VK_MAX_DRIVER_NAME_SIZE_KHR       VK_MAX_DRIVER_NAME_SIZE
///```
///# Related
/// - [`khr_driver_properties`]
/// - [`crate::vulkan1_2`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_DRIVER_NAME_SIZE")]
pub const MAX_DRIVER_NAME_SIZE: u32 = 256;
///[VK_MAX_DRIVER_INFO_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_INFO_SIZE.html) - Length of a physical device driver information string
///# C Specifications
///[`MAX_DRIVER_INFO_SIZE`] is the length in `char` values of an array
///containing a driver information string, as returned in
///[`PhysicalDeviceDriverProperties`]::driverInfo.
///```c
///#define VK_MAX_DRIVER_INFO_SIZE           256U
///```
///or the equivalent
///```c
///#define VK_MAX_DRIVER_INFO_SIZE_KHR       VK_MAX_DRIVER_INFO_SIZE
///```
///# Related
/// - [`khr_driver_properties`]
/// - [`crate::vulkan1_2`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_DRIVER_INFO_SIZE")]
pub const MAX_DRIVER_INFO_SIZE: u32 = 256;
///[VK_SHADER_UNUSED_KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_SHADER_UNUSED_KHR.html) - Sentinel for an unused shader index
///# C Specifications
///[`SHADER_UNUSED_KHR`] is a special shader index used to indicate that a
///ray generation, miss, or callable shader member is not used.
///```c
///#define VK_SHADER_UNUSED_KHR              (~0U)
///```
///or the equivalent
///```c
///#define VK_SHADER_UNUSED_NV               VK_SHADER_UNUSED_KHR
///```
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`nv_ray_tracing`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_SHADER_UNUSED_KHR")]
pub const SHADER_UNUSED_KHR: u32 = !0;
///[VK_MAX_GLOBAL_PRIORITY_SIZE_KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_GLOBAL_PRIORITY_SIZE_KHR.html) - Length of an array of global queue priorities
///# C Specifications
///[`MAX_GLOBAL_PRIORITY_SIZE_KHR`] is the length of an array of
///[`QueueGlobalPriorityKHR`] enumerants representing supported queue
///priorities, as returned in
///[`QueueFamilyGlobalPriorityPropertiesKHR::priorities`].
///```c
///#define VK_MAX_GLOBAL_PRIORITY_SIZE_KHR   16U
///```
///or the equivalent
///```c
///#define VK_MAX_GLOBAL_PRIORITY_SIZE_EXT   VK_MAX_GLOBAL_PRIORITY_SIZE_KHR
///```
///# Related
/// - [`khr_global_priority`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_GLOBAL_PRIORITY_SIZE_KHR")]
pub const MAX_GLOBAL_PRIORITY_SIZE_KHR: u32 = 16;
