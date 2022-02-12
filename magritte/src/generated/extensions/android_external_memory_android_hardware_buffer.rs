//![VK_ANDROID_external_memory_android_hardware_buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_ANDROID_external_memory_android_hardware_buffer.html) - device extension
//!# Description
//!This extension enables an application to import Android
//![`AHardwareBuffer`] objects created outside of the Vulkan device into
//!Vulkan memory objects, where they **can** be bound to images and buffers.
//!It also allows exporting an [`AHardwareBuffer`] from a Vulkan memory
//!object for symmetry with other operating systems.
//!But since not all [`AHardwareBuffer`] usages and formats have Vulkan
//!equivalents, exporting from Vulkan provides strictly less functionality than
//!creating the [`AHardwareBuffer`] externally and importing it.Some [`AHardwareBuffer`] images
//! have implementation-defined *external
//!formats* that **may** not correspond to Vulkan formats.
//!Sampler Y′C<sub>B</sub>C<sub>R</sub> conversion **can** be used to sample from these images and
//!convert them to a known color space.
//!# Revision
//!4
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_sampler_ycbcr_conversion`]`
//! - Requires `[`VK_KHR_external_memory`]`
//! - Requires `[`VK_EXT_queue_family_foreign`]`
//! - Requires `[`VK_KHR_dedicated_allocation`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_ANDROID_external_memory_android_hardware_buffer]
//!   @critsec%0A<<Here describe the issue or question you have about the
//!   VK_ANDROID_external_memory_android_hardware_buffer extension>>)
//!# New functions & commands
//! - [`GetAndroidHardwareBufferPropertiesANDROID`]
//! - [`GetMemoryAndroidHardwareBufferANDROID`]
//!# New structures
//! - [`AndroidHardwareBufferPropertiesANDROID`]
//! - [`MemoryGetAndroidHardwareBufferInfoANDROID`]
//! - Extending [`AndroidHardwareBufferPropertiesANDROID`]:
//! - [`AndroidHardwareBufferFormatPropertiesANDROID`]
//!
//! - Extending [`ImageCreateInfo`], [`SamplerYcbcrConversionCreateInfo`]:
//! - [`ExternalFormatANDROID`]
//!
//! - Extending [`ImageFormatProperties2`]:
//! - [`AndroidHardwareBufferUsageANDROID`]
//!
//! - Extending [`MemoryAllocateInfo`]:
//! - [`ImportAndroidHardwareBufferInfoANDROID`]
//!
//!If [`VK_KHR_format_feature_flags2`] is supported:
//! - Extending [`AndroidHardwareBufferPropertiesANDROID`]:
//! - [`AndroidHardwareBufferFormatProperties2ANDROID`]
//!# New constants
//! - [`ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME`]
//! - [`ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION`]
//! - Extending [`ExternalMemoryHandleTypeFlagBits`]:
//! - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID`
//! - `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID`
//! - `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID`
//! - `VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID`
//! - `VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
//! - `VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
//!
//!If [`VK_KHR_format_feature_flags2`] is supported:
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID`
//!# Known issues & F.A.Q
//!1) Other external memory objects are represented as weakly-typed handles
//!(e.g. Win32 [`HANDLE`] or POSIX file descriptor), and require a handle type
//!parameter along with handles.
//![`AHardwareBuffer`] is strongly typed, so naming the handle type is
//!redundant.
//!Does symmetry justify adding handle type parameters/fields anyway?**RESOLVED**: No.
//!The handle type is already provided in places that treat external memory
//!objects generically.
//!In the places we would add it, the application code that would have to
//!provide the handle type value is already dealing with
//![`AHardwareBuffer`]-specific commands/structures; the extra symmetry
//!would not be enough to make that code generic.2) The internal layout and therefore size of a
//! [`AHardwareBuffer`]
//!image may depend on native usage flags that do not have corresponding Vulkan
//!counterparts.
//!Do we provide this information to [`CreateImage`] somehow, or allow the
//!allocation size reported by [`GetImageMemoryRequirements`] to be
//!approximate?**RESOLVED**: Allow the allocation size to be unspecified when allocating the
//!memory.
//!It has to work this way for exported image memory anyway, since
//![`AHardwareBuffer`] allocation happens in [`AllocateMemory`], and
//!internally is performed by a separate HAL, not the Vulkan implementation
//!itself.
//!There is a similar issue with [`GetImageSubresourceLayout`]: the layout
//!is determined by the allocator HAL, so it is not known until the image is
//!bound to memory.3) Should the result of sampling an external-format image with the suggested
//!Y′C<sub>B</sub>C<sub>R</sub> conversion parameters yield the same results as using a
//!`samplerExternalOES` in OpenGL ES?**RESOLVED**: This would be desirable, so that apps converting
//! from OpenGL ES
//!to Vulkan could get the same output given the same input.
//!But since sampling and conversion from Y′C<sub>B</sub>C<sub>R</sub> images is so loosely defined
//!in OpenGL ES, multiple implementations do it in a way that does not conform
//!to Vulkan’s requirements.
//!Modifying the OpenGL ES implementation would be difficult, and would change
//!the output of existing unmodified applications.
//!Changing the output only for applications that are being modified gives
//!developers the chance to notice and mitigate any problems.
//!Implementations are encouraged to minimize differences as much as possible
//!without causing compatibility problems for existing OpenGL ES applications
//!or violating Vulkan requirements.4) Should an [`AHardwareBuffer`] with
//! `AHARDWAREBUFFER_USAGE_CPU_*`
//!usage be mappable in Vulkan? Should it be possible to export an
//!`AHardwareBuffers` with such usage?**RESOLVED**: Optional, and mapping in Vulkan is not the same
//! as
//!`AHardwareBuffer_lock`.
//!The semantics of these are different: mapping in memory is persistent, just
//!gives a raw view of the memory contents, and does not involve ownership.
//!`AHardwareBuffer_lock` gives the host exclusive access to the buffer, is
//!temporary, and allows for reformatting copy-in/copy-out.
//!Implementations are not required to support host-visible memory types for
//!imported Android hardware buffers or resources backed by them.
//!If a host-visible memory type is supported and used, the memory can be
//!mapped in Vulkan, but doing so follows Vulkan semantics: it is just a raw
//!view of the data and does not imply ownership (this means implementations
//!must not internally call `AHardwareBuffer_lock` to implement
//![`MapMemory`], or assume the application has done so).
//!Implementations are not required to support linear-tiled images backed by
//!Android hardware buffers, even if the [`AHardwareBuffer`] has CPU
//!usage.
//!There is no reliable way to allocate memory in Vulkan that can be exported
//!to a [`AHardwareBuffer`] with CPU usage.5) Android may add new [`AHardwareBuffer`] formats and
//! usage flags over
//!time.
//!Can reference to them be added to this extension, or do they need a new
//!extension?**RESOLVED**: This extension can document the interaction between the new AHB
//!formats/usages and existing Vulkan features.
//!No new Vulkan features or implementation requirements can be added.
//!The extension version number will be incremented when this additional
//!documentation is added, but the version number does not indicate that an
//!implementaiton supports Vulkan memory or resources that map to the new
//![`AHardwareBuffer`] features: support for that must be queried with
//![`GetPhysicalDeviceImageFormatProperties2`] or is implied by
//!successfully allocating a [`AHardwareBuffer`] outside of Vulkan that
//!uses the new feature and has a GPU usage flag.In essence, these are new features added to a new
//! Android API level, rather
//!than new Vulkan features.
//!The extension will only document how existing Vulkan features map to that
//!new Android feature.
//!# Version History
//! - Revision 4, 2021-09-30 (Jon Leech)
//! - Add interaction with `[`VK_KHR_format_feature_flags2`]` to `vk.xml`
//!
//! - Revision 3, 2019-08-27 (Jon Leech)
//! - Update revision history to correspond to XML version number
//!
//! - Revision 2, 2018-04-09 (Petr Kraus)
//! - Markup fixes and remove incorrect Draft status
//!
//! - Revision 1, 2018-03-04 (Jesse Hall)
//! - Initial version
//!# Other info
//! * 2021-09-30
//! * No known IP claims.
//!*
//! - Ray Smith, ARM
//! - Chad Versace, Google
//! - Jesse Hall, Google
//! - Tobias Hector, Imagination
//! - James Jones, NVIDIA
//! - Tony Zlatinski, NVIDIA
//! - Matthew Netsch, Qualcomm
//! - Andrew Garrard, Samsung
//!# Related
//! - [`AHardwareBuffer`]
//! - [`AndroidHardwareBufferFormatPropertiesANDROID`]
//! - [`AndroidHardwareBufferPropertiesANDROID`]
//! - [`AndroidHardwareBufferUsageANDROID`]
//! - [`ExternalFormatANDROID`]
//! - [`ImportAndroidHardwareBufferInfoANDROID`]
//! - [`MemoryGetAndroidHardwareBufferInfoANDROID`]
//! - [`GetAndroidHardwareBufferPropertiesANDROID`]
//! - [`GetMemoryAndroidHardwareBufferANDROID`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::{c_void, CStr};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_ANDROID_external_memory_android_hardware_buffer");
///[AHardwareBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/AHardwareBuffer.html) - Android hardware buffer type
///# C Specifications
///To remove an unnecessary compile-time dependency, an incomplete type
///definition of [`AHardwareBuffer`] is provided in the Vulkan headers:
///```c
///// Provided by VK_ANDROID_external_memory_android_hardware_buffer
///struct AHardwareBuffer;
///```
///# Related
/// - [`VK_ANDROID_external_memory_android_hardware_buffer`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
pub type AHardwareBuffer = c_void;
