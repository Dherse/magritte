//![VK_ANDROID_external_memory_android_hardware_buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_ANDROID_external_memory_android_hardware_buffer.html) - device extension
//!# Description
//!This extension enables an application to import Android
//![`AHardwareBuffer`] objects created outside of the Vulkan device into
//!Vulkan memory objects, where they  **can**  be bound to images and buffers.
//!It also allows exporting an [`AHardwareBuffer`] from a Vulkan memory
//!object for symmetry with other operating systems.
//!But since not all [`AHardwareBuffer`] usages and formats have Vulkan
//!equivalents, exporting from Vulkan provides strictly less functionality than
//!creating the [`AHardwareBuffer`] externally and importing it.Some [`AHardwareBuffer`] images
//! have implementation-defined *external
//!formats* that  **may**  not correspond to Vulkan formats.
//!Sampler Y′C<sub>B</sub>C<sub>R</sub> conversion  **can**  be used to sample from these images
//! and
//!convert them to a known color space.
//!# Revision
//!5
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_sampler_ycbcr_conversion`]`
//! - Requires `[`khr_external_memory`]`
//! - Requires `[`ext_queue_family_foreign`]`
//! - Requires `[`khr_dedicated_allocation`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_ANDROID_external_memory_android_hardware_buffer]
//!   @critsec%0A<<Here describe the issue or question you have about the
//!   VK_ANDROID_external_memory_android_hardware_buffer extension>>)
//!# New functions & commands
//! - [`get_android_hardware_buffer_properties_android`]
//! - [`get_memory_android_hardware_buffer_android`]
//!# New structures
//! - [`AndroidHardwareBufferPropertiesANDROID`]
//! - [`MemoryGetAndroidHardwareBufferInfoANDROID`]
//! - Extending [`AndroidHardwareBufferPropertiesANDROID`]:  -
//!   [`AndroidHardwareBufferFormatPropertiesANDROID`]
//! - Extending [`ImageCreateInfo`], [`SamplerYcbcrConversionCreateInfo`]:  -
//!   [`ExternalFormatANDROID`]
//! - Extending [`ImageFormatProperties2`]:  - [`AndroidHardwareBufferUsageANDROID`]
//! - Extending [`MemoryAllocateInfo`]:  - [`ImportAndroidHardwareBufferInfoANDROID`]
//!If [`khr_format_feature_flags2`] is supported:
//! - Extending [`AndroidHardwareBufferPropertiesANDROID`]:  -
//!   [`AndroidHardwareBufferFormatProperties2ANDROID`]
//!# New constants
//! - [`ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME`]
//! - [`ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION`]
//! - Extending [`ExternalMemoryHandleTypeFlagBits`]:  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID`  -
//!   `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID`  -
//!   `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID`  -
//!   `VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID`  -
//!   `VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`  -
//!   `VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
//!If [`khr_format_feature_flags2`] is supported:
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID`
//!# Known issues & F.A.Q
//!1) Other external memory objects are represented as weakly-typed handles
//!(e.g. Win32 [`HANDLE`] or POSIX file descriptor), and require a handle type
//!parameter along with handles.
//![`AHardwareBuffer`] is strongly typed, so naming the handle type is
//!redundant.
//!Does symmetry justify adding handle type parameters/fields anyway? **RESOLVED** : No.
//!The handle type is already provided in places that treat external memory
//!objects generically.
//!In the places we would add it, the application code that would have to
//!provide the handle type value is already dealing with
//![`AHardwareBuffer`]-specific commands/structures; the extra symmetry
//!would not be enough to make that code generic.2) The internal layout and therefore size of a
//! [`AHardwareBuffer`]
//!image may depend on native usage flags that do not have corresponding Vulkan
//!counterparts.
//!Do we provide this information to [`create_image`] somehow, or allow the
//!allocation size reported by [`get_image_memory_requirements`] to be
//!approximate? **RESOLVED** : Allow the allocation size to be unspecified when allocating the
//!memory.
//!It has to work this way for exported image memory anyway, since
//![`AHardwareBuffer`] allocation happens in [`allocate_memory`], and
//!internally is performed by a separate HAL, not the Vulkan implementation
//!itself.
//!There is a similar issue with [`get_image_subresource_layout`]: the layout
//!is determined by the allocator HAL, so it is not known until the image is
//!bound to memory.3) Should the result of sampling an external-format image with the suggested
//!Y′C<sub>B</sub>C<sub>R</sub> conversion parameters yield the same results as using a
//!`samplerExternalOES` in OpenGL ES? **RESOLVED** : This would be desirable, so that apps
//! converting from OpenGL ES
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
//!`AHardwareBuffers` with such usage? **RESOLVED** : Optional, and mapping in Vulkan is not the
//! same as
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
//![`map_memory`], or assume the application has done so).
//!Implementations are not required to support linear-tiled images backed by
//!Android hardware buffers, even if the [`AHardwareBuffer`] has CPU
//!usage.
//!There is no reliable way to allocate memory in Vulkan that can be exported
//!to a [`AHardwareBuffer`] with CPU usage.5) Android may add new [`AHardwareBuffer`] formats and
//! usage flags over
//!time.
//!Can reference to them be added to this extension, or do they need a new
//!extension? **RESOLVED** : This extension can document the interaction between the new AHB
//!formats/usages and existing Vulkan features.
//!No new Vulkan features or implementation requirements can be added.
//!The extension version number will be incremented when this additional
//!documentation is added, but the version number does not indicate that an
//!implementation supports Vulkan memory or resources that map to the new
//![`AHardwareBuffer`] features: support for that must be queried with
//![`get_physical_device_image_format_properties2`] or is implied by
//!successfully allocating a [`AHardwareBuffer`] outside of Vulkan that
//!uses the new feature and has a GPU usage flag.In essence, these are new features added to a new
//! Android API level, rather
//!than new Vulkan features.
//!The extension will only document how existing Vulkan features map to that
//!new Android feature.
//!# Version History
//! - Revision 5, 2022-02-04 (Chris Forbes)  - Describe mapping of flags for storage image support
//! - Revision 4, 2021-09-30 (Jon Leech)  - Add interaction with `[`khr_format_feature_flags2`]` to
//!   `vk.xml`
//! - Revision 3, 2019-08-27 (Jon Leech)  - Update revision history to correspond to XML version
//!   number
//! - Revision 2, 2018-04-09 (Petr Kraus)  - Markup fixes and remove incorrect Draft status
//! - Revision 1, 2018-03-04 (Jesse Hall)  - Initial version
//!# Other info
//! * 2021-09-30
//! * No known IP claims.
//! * - Ray Smith, ARM  - Chad Versace, Google  - Jesse Hall, Google  - Tobias Hector, Imagination
//!   - James Jones, NVIDIA  - Tony Zlatinski, NVIDIA  - Matthew Netsch, Qualcomm  - Andrew Garrard,
//!   Samsung
//!# Related
//! - [`AHardwareBuffer`]
//! - [`AndroidHardwareBufferFormatPropertiesANDROID`]
//! - [`AndroidHardwareBufferPropertiesANDROID`]
//! - [`AndroidHardwareBufferUsageANDROID`]
//! - [`ExternalFormatANDROID`]
//! - [`ImportAndroidHardwareBufferInfoANDROID`]
//! - [`MemoryGetAndroidHardwareBufferInfoANDROID`]
//! - [`get_android_hardware_buffer_properties_android`]
//! - [`get_memory_android_hardware_buffer_android`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, ComponentMapping, Device, DeviceMemory, DeviceSize, Format,
        FormatFeatureFlags, StructureType, VulkanResultCodes,
    },
    vulkan1_1::{ChromaLocation, SamplerYcbcrModelConversion, SamplerYcbcrRange},
    vulkan1_3::FormatFeatureFlags2,
    AsRaw, Unique, VulkanResult,
};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION: u32 = 5;
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
/// - [`android_external_memory_android_hardware_buffer`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
pub type AHardwareBuffer = c_void;
///[vkGetAndroidHardwareBufferPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html) - Get Properties of External Memory Android Hardware Buffers
///# C Specifications
///To determine the memory parameters to use when importing an Android hardware
///buffer, call:
///```c
///// Provided by VK_ANDROID_external_memory_android_hardware_buffer
///VkResult vkGetAndroidHardwareBufferPropertiesANDROID(
///    VkDevice                                    device,
///    const struct AHardwareBuffer*               buffer,
///    VkAndroidHardwareBufferPropertiesANDROID*   pProperties);
///```
///# Parameters
/// - [`device`] is the logical device that will be importing [`buffer`].
/// - [`buffer`] is the Android hardware buffer which will be imported.
/// - [`p_properties`] is a pointer to a [`AndroidHardwareBufferPropertiesANDROID`] structure in
///   which the properties of [`buffer`] are returned.
///# Description
///## Valid Usage
/// - [`buffer`] **must**  be a valid Android hardware buffer object with at least one of the
///   `AHARDWAREBUFFER_USAGE_GPU_*` flags in its `AHardwareBuffer_Desc`::`usage`
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`buffer`] **must**  be a valid pointer to a valid [`AHardwareBuffer`] value
/// - [`p_properties`] **must**  be a valid pointer to a [`AndroidHardwareBufferPropertiesANDROID`]
///   structure
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`
///# Related
/// - [`android_external_memory_android_hardware_buffer`]
/// - [`AndroidHardwareBufferPropertiesANDROID`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
pub type FNGetAndroidHardwareBufferPropertiesAndroid = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        buffer: *const AHardwareBuffer,
        p_properties: *mut AndroidHardwareBufferPropertiesANDROID<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkGetMemoryAndroidHardwareBufferANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html) - Get an Android hardware buffer for a memory object
///# C Specifications
///To export an Android hardware buffer referencing the payload of a Vulkan
///device memory object, call:
///```c
///// Provided by VK_ANDROID_external_memory_android_hardware_buffer
///VkResult vkGetMemoryAndroidHardwareBufferANDROID(
///    VkDevice                                    device,
///    const VkMemoryGetAndroidHardwareBufferInfoANDROID* pInfo,
///    struct AHardwareBuffer**                    pBuffer);
///```
///# Parameters
/// - [`device`] is the logical device that created the device memory being exported.
/// - [`p_info`] is a pointer to a [`MemoryGetAndroidHardwareBufferInfoANDROID`] structure
///   containing parameters of the export operation.
/// - [`p_buffer`] will return an Android hardware buffer referencing the payload of the device
///   memory object.
///# Description
///Each call to [`get_memory_android_hardware_buffer_android`] **must**  return an
///Android hardware buffer with a new reference acquired in addition to the
///reference held by the [`DeviceMemory`].
///To avoid leaking resources, the application  **must**  release the reference by
///calling `AHardwareBuffer_release` when it is no longer needed.
///When called with the same handle in
///[`MemoryGetAndroidHardwareBufferInfoANDROID::memory`],
///[`get_memory_android_hardware_buffer_android`] **must**  return the same Android
///hardware buffer object.
///If the device memory was created by importing an Android hardware buffer,
///[`get_memory_android_hardware_buffer_android`] **must**  return that same Android
///hardware buffer object.
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_info`] **must**  be a valid pointer to a valid
///   [`MemoryGetAndroidHardwareBufferInfoANDROID`] structure
/// - [`p_buffer`] **must**  be a valid pointer to a valid pointer to an [`AHardwareBuffer`] value
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
///# Related
/// - [`android_external_memory_android_hardware_buffer`]
/// - [`Device`]
/// - [`MemoryGetAndroidHardwareBufferInfoANDROID`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
pub type FNGetMemoryAndroidHardwareBufferAndroid = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID<'lt>,
        p_buffer: *mut *mut AHardwareBuffer,
    ) -> VulkanResultCodes,
>;
///[VkImportAndroidHardwareBufferInfoANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportAndroidHardwareBufferInfoANDROID.html) - Import memory from an Android hardware buffer
///# C Specifications
///To import memory created outside of the current Vulkan instance from an
///Android hardware buffer, add a
///[`ImportAndroidHardwareBufferInfoANDROID`] structure to the [`p_next`]
///chain of the [`MemoryAllocateInfo`] structure.
///The [`ImportAndroidHardwareBufferInfoANDROID`] structure is defined as:
///```c
///// Provided by VK_ANDROID_external_memory_android_hardware_buffer
///typedef struct VkImportAndroidHardwareBufferInfoANDROID {
///    VkStructureType            sType;
///    const void*                pNext;
///    struct AHardwareBuffer*    buffer;
///} VkImportAndroidHardwareBufferInfoANDROID;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`buffer`] is the Android hardware buffer to import.
///# Description
///If the [`allocate_memory`] command succeeds, the implementation  **must**
///acquire a reference to the imported hardware buffer, which it  **must**  release
///when the device memory object is freed.
///If the command fails, the implementation  **must**  not retain a reference.
///## Valid Usage
/// - If [`buffer`] is not `NULL`, Android hardware buffers  **must**  be supported for import, as
///   reported by [`ExternalImageFormatProperties`] or [`ExternalBufferProperties`]
/// -    If [`buffer`] is not `NULL`, it  **must**  be a valid Android hardware buffer object with `AHardwareBuffer_Desc`::`usage` compatible with Vulkan as described in [Android Hardware Buffers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer)
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
/// - [`buffer`] **must**  be a valid pointer to an [`AHardwareBuffer`] value
///# Related
/// - [`android_external_memory_android_hardware_buffer`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImportAndroidHardwareBufferInfoANDROID")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ImportAndroidHardwareBufferInfoANDROID<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`buffer`] is the Android hardware buffer to import.
    pub buffer: *mut AHardwareBuffer,
}
impl<'lt> Default for ImportAndroidHardwareBufferInfoANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
            p_next: std::ptr::null(),
            buffer: std::ptr::null_mut(),
        }
    }
}
impl<'lt> ImportAndroidHardwareBufferInfoANDROID<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::buffer`]
    pub fn buffer_raw(&self) -> *mut AHardwareBuffer {
        self.buffer
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::buffer`]
    pub fn set_buffer_raw(mut self, value: *mut AHardwareBuffer) -> Self {
        self.buffer = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::buffer`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn buffer(&self) -> &AHardwareBuffer {
        &*self.buffer
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::buffer`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn buffer_mut(&mut self) -> &mut AHardwareBuffer {
        &mut *self.buffer
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::buffer`]
    pub fn set_buffer(
        mut self,
        value: &'lt mut crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer,
    ) -> Self {
        self.buffer = value as *mut _;
        self
    }
}
///[VkAndroidHardwareBufferUsageANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferUsageANDROID.html) - Struct containing Android hardware buffer usage flags
///# C Specifications
///To obtain optimal Android hardware buffer usage flags for specific image
///creation parameters, add a [`AndroidHardwareBufferUsageANDROID`]
///structure to the [`p_next`] chain of a [`ImageFormatProperties2`]
///structure passed to [`get_physical_device_image_format_properties2`].
///This structure is defined as:
///```c
///// Provided by VK_ANDROID_external_memory_android_hardware_buffer
///typedef struct VkAndroidHardwareBufferUsageANDROID {
///    VkStructureType    sType;
///    void*              pNext;
///    uint64_t           androidHardwareBufferUsage;
///} VkAndroidHardwareBufferUsageANDROID;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`android_hardware_buffer_usage`] returns the Android hardware buffer usage flags.
///# Description
///The [`android_hardware_buffer_usage`] field  **must**  include Android hardware
///buffer usage flags listed in the
///[AHardwareBuffer Usage
///Equivalence](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-usage) table when the corresponding Vulkan image usage or image
///creation flags are included in the `usage` or `flags` fields of
///[`PhysicalDeviceImageFormatInfo2`].
///It  **must**  include at least one GPU usage flag
///(`AHARDWAREBUFFER_USAGE_GPU_*`), even if none of the corresponding Vulkan
///usages or flags are requested.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID`
///# Related
/// - [`android_external_memory_android_hardware_buffer`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAndroidHardwareBufferUsageANDROID")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct AndroidHardwareBufferUsageANDROID<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`android_hardware_buffer_usage`] returns the Android hardware buffer
    ///usage flags.
    pub android_hardware_buffer_usage: u64,
}
impl<'lt> Default for AndroidHardwareBufferUsageANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID,
            p_next: std::ptr::null_mut(),
            android_hardware_buffer_usage: 0,
        }
    }
}
impl<'lt> AndroidHardwareBufferUsageANDROID<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::android_hardware_buffer_usage`]
    pub fn android_hardware_buffer_usage(&self) -> u64 {
        self.android_hardware_buffer_usage
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::android_hardware_buffer_usage`]
    pub fn android_hardware_buffer_usage_mut(&mut self) -> &mut u64 {
        &mut self.android_hardware_buffer_usage
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::android_hardware_buffer_usage`]
    pub fn set_android_hardware_buffer_usage(mut self, value: u64) -> Self {
        self.android_hardware_buffer_usage = value;
        self
    }
}
///[VkAndroidHardwareBufferPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferPropertiesANDROID.html) - Properties of External Memory Android Hardware Buffers
///# C Specifications
///The [`AndroidHardwareBufferPropertiesANDROID`] structure returned is
///defined as:
///```c
///// Provided by VK_ANDROID_external_memory_android_hardware_buffer
///typedef struct VkAndroidHardwareBufferPropertiesANDROID {
///    VkStructureType    sType;
///    void*              pNext;
///    VkDeviceSize       allocationSize;
///    uint32_t           memoryTypeBits;
///} VkAndroidHardwareBufferPropertiesANDROID;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`allocation_size`] is the size of the external memory
/// - [`memory_type_bits`] is a bitmask containing one bit set for every memory type which the
///   specified Android hardware buffer  **can**  be imported as.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`AndroidHardwareBufferFormatProperties2ANDROID`] or
///   [`AndroidHardwareBufferFormatPropertiesANDROID`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
///# Related
/// - [`android_external_memory_android_hardware_buffer`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`get_android_hardware_buffer_properties_android`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAndroidHardwareBufferPropertiesANDROID")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct AndroidHardwareBufferPropertiesANDROID<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`allocation_size`] is the size of the external memory
    pub allocation_size: DeviceSize,
    ///[`memory_type_bits`] is a bitmask containing one bit set for every
    ///memory type which the specified Android hardware buffer  **can**  be imported
    ///as.
    pub memory_type_bits: u32,
}
impl<'lt> Default for AndroidHardwareBufferPropertiesANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID,
            p_next: std::ptr::null_mut(),
            allocation_size: Default::default(),
            memory_type_bits: 0,
        }
    }
}
impl<'lt> AndroidHardwareBufferPropertiesANDROID<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::allocation_size`]
    pub fn allocation_size(&self) -> DeviceSize {
        self.allocation_size
    }
    ///Gets the value of [`Self::memory_type_bits`]
    pub fn memory_type_bits(&self) -> u32 {
        self.memory_type_bits
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::allocation_size`]
    pub fn allocation_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.allocation_size
    }
    ///Gets a mutable reference to the value of [`Self::memory_type_bits`]
    pub fn memory_type_bits_mut(&mut self) -> &mut u32 {
        &mut self.memory_type_bits
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::allocation_size`]
    pub fn set_allocation_size(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.allocation_size = value;
        self
    }
    ///Sets the value of [`Self::memory_type_bits`]
    pub fn set_memory_type_bits(mut self, value: u32) -> Self {
        self.memory_type_bits = value;
        self
    }
}
unsafe impl<'lt> crate::Chain<'lt, AndroidHardwareBufferFormatPropertiesANDROID<'lt>>
    for AndroidHardwareBufferPropertiesANDROID<'lt>
{
}
unsafe impl<'lt> crate::Chain<'lt, AndroidHardwareBufferFormatProperties2ANDROID<'lt>>
    for AndroidHardwareBufferPropertiesANDROID<'lt>
{
}
///[VkMemoryGetAndroidHardwareBufferInfoANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetAndroidHardwareBufferInfoANDROID.html) - Structure describing an Android hardware buffer memory export operation
///# C Specifications
///The [`MemoryGetAndroidHardwareBufferInfoANDROID`] structure is defined
///as:
///```c
///// Provided by VK_ANDROID_external_memory_android_hardware_buffer
///typedef struct VkMemoryGetAndroidHardwareBufferInfoANDROID {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkDeviceMemory     memory;
///} VkMemoryGetAndroidHardwareBufferInfoANDROID;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory`] is the memory object from which the Android hardware buffer will be exported.
///# Description
///## Valid Usage
/// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` **must**  have been
///   included in [`ExportMemoryAllocateInfo::handle_types`] when [`memory`] was created
/// - If the [`p_next`] chain of the [`MemoryAllocateInfo`] used to allocate [`memory`] included a
///   [`MemoryDedicatedAllocateInfo`] with non-`NULL``image` member, then that `image` **must**
///   already be bound to [`memory`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
/// - [`p_next`] **must**  be `NULL`
/// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
///# Related
/// - [`android_external_memory_android_hardware_buffer`]
/// - [`DeviceMemory`]
/// - [`StructureType`]
/// - [`get_memory_android_hardware_buffer_android`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryGetAndroidHardwareBufferInfoANDROID")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`memory`] is the memory object from which the Android hardware buffer
    ///will be exported.
    pub memory: DeviceMemory,
}
impl<'lt> Default for MemoryGetAndroidHardwareBufferInfoANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
            p_next: std::ptr::null(),
            memory: Default::default(),
        }
    }
}
impl<'lt> MemoryGetAndroidHardwareBufferInfoANDROID<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::memory`]
    pub fn memory(&self) -> DeviceMemory {
        self.memory
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::memory`]
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::memory`]
    pub fn set_memory(mut self, value: crate::vulkan1_0::DeviceMemory) -> Self {
        self.memory = value;
        self
    }
}
///[VkAndroidHardwareBufferFormatPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferFormatPropertiesANDROID.html) - Structure describing the image format properties of an Android hardware buffer
///# C Specifications
///To obtain format properties of an Android hardware buffer, include a
///[`AndroidHardwareBufferFormatPropertiesANDROID`] structure in the
///[`p_next`] chain of the [`AndroidHardwareBufferPropertiesANDROID`]
///structure passed to [`get_android_hardware_buffer_properties_android`].
///This structure is defined as:
///```c
///// Provided by VK_ANDROID_external_memory_android_hardware_buffer
///typedef struct VkAndroidHardwareBufferFormatPropertiesANDROID {
///    VkStructureType                  sType;
///    void*                            pNext;
///    VkFormat                         format;
///    uint64_t                         externalFormat;
///    VkFormatFeatureFlags             formatFeatures;
///    VkComponentMapping               samplerYcbcrConversionComponents;
///    VkSamplerYcbcrModelConversion    suggestedYcbcrModel;
///    VkSamplerYcbcrRange              suggestedYcbcrRange;
///    VkChromaLocation                 suggestedXChromaOffset;
///    VkChromaLocation                 suggestedYChromaOffset;
///} VkAndroidHardwareBufferFormatPropertiesANDROID;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format`] is the Vulkan format corresponding to the Android hardware buffer’s format, or
///   `VK_FORMAT_UNDEFINED` if there is not an equivalent Vulkan format.
/// - [`external_format`] is an implementation-defined external format identifier for use with
///   [`ExternalFormatANDROID`]. It  **must**  not be zero.
/// - [`format_features`] describes the capabilities of this external format when used with an image
///   bound to memory imported from `buffer`.
/// - [`sampler_ycbcr_conversion_components`] is the component swizzle that  **should**  be used in
///   [`SamplerYcbcrConversionCreateInfo`].
/// - [`suggested_ycbcr_model`] is a suggested color model to use in the
///   [`SamplerYcbcrConversionCreateInfo`].
/// - [`suggested_ycbcr_range`] is a suggested numerical value range to use in
///   [`SamplerYcbcrConversionCreateInfo`].
/// - [`suggested_x_chroma_offset`] is a suggested X chroma offset to use in
///   [`SamplerYcbcrConversionCreateInfo`].
/// - [`suggested_y_chroma_offset`] is a suggested Y chroma offset to use in
///   [`SamplerYcbcrConversionCreateInfo`].
///# Description
///If the Android hardware buffer has one of the formats listed in the
///[Format Equivalence
///table](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-formats), then [`format`] **must**  have the equivalent Vulkan format listed in
///the table.
///Otherwise, [`format`] **may**  be `VK_FORMAT_UNDEFINED`, indicating the
///Android hardware buffer  **can**  only be used with an external format.The [`format_features`]
/// member  **must**  include
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT` and at least one of
///`VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT` or
///`VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`, and  **should**  include
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` and
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`.Android hardware buffers
/// with the same external format  **must**  have the same
///support for `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`,
///`VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT`,
///`VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`,
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`,
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT`,
///and
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT`.
///in [`format_features`].
///Other format features  **may**  differ between Android hardware buffers that have
///the same external format.
///This allows applications to use the same [`SamplerYcbcrConversion`]
///object (and samplers and pipelines created from them) for any Android
///hardware buffers that have the same external format.If [`format`] is not `VK_FORMAT_UNDEFINED`,
/// then the value of
///[`sampler_ycbcr_conversion_components`] **must**  be valid when used as the
///`components` member of [`SamplerYcbcrConversionCreateInfo`] with
///that format.
///If [`format`] is `VK_FORMAT_UNDEFINED`, all members of
///[`sampler_ycbcr_conversion_components`] **must**  be the
///[identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings).Implementations  **may**  not always be able to determine the color model,
///numerical range, or chroma offsets of the image contents, so the values in
///[`AndroidHardwareBufferFormatPropertiesANDROID`] are only suggestions.
///Applications  **should**  treat these values as sensible defaults to use in the
///absence of more reliable information obtained through some other means.
///If the underlying physical device is also usable via OpenGL ES with the
///[`GL_OES_EGL_image_external`](https://www.khronos.org/registry/OpenGL/extensions/OES/OES_EGL_image_external.txt)
///extension, the implementation  **should**  suggest values that will produce
///similar sampled values as would be obtained by sampling the same external
///image via `samplerExternalOES` in OpenGL ES using equivalent sampler
///parameters.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID`
///# Related
/// - [`android_external_memory_android_hardware_buffer`]
/// - [`ChromaLocation`]
/// - [`ComponentMapping`]
/// - [`Format`]
/// - [`FormatFeatureFlags`]
/// - [`SamplerYcbcrModelConversion`]
/// - [`SamplerYcbcrRange`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAndroidHardwareBufferFormatPropertiesANDROID")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`format`] is the Vulkan format corresponding to the Android hardware
    ///buffer’s format, or `VK_FORMAT_UNDEFINED` if there is not an
    ///equivalent Vulkan format.
    pub format: Format,
    ///[`external_format`] is an implementation-defined external format
    ///identifier for use with [`ExternalFormatANDROID`].
    ///It  **must**  not be zero.
    pub external_format: u64,
    ///[`format_features`] describes the capabilities of this external format
    ///when used with an image bound to memory imported from `buffer`.
    pub format_features: FormatFeatureFlags,
    ///[`sampler_ycbcr_conversion_components`] is the component swizzle that
    /// **should**  be used in [`SamplerYcbcrConversionCreateInfo`].
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    ///[`suggested_ycbcr_model`] is a suggested color model to use in the
    ///[`SamplerYcbcrConversionCreateInfo`].
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    ///[`suggested_ycbcr_range`] is a suggested numerical value range to use in
    ///[`SamplerYcbcrConversionCreateInfo`].
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    ///[`suggested_x_chroma_offset`] is a suggested X chroma offset to use in
    ///[`SamplerYcbcrConversionCreateInfo`].
    pub suggested_x_chroma_offset: ChromaLocation,
    ///[`suggested_y_chroma_offset`] is a suggested Y chroma offset to use in
    ///[`SamplerYcbcrConversionCreateInfo`].
    pub suggested_y_chroma_offset: ChromaLocation,
}
impl<'lt> Default for AndroidHardwareBufferFormatPropertiesANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID,
            p_next: std::ptr::null_mut(),
            format: Default::default(),
            external_format: 0,
            format_features: Default::default(),
            sampler_ycbcr_conversion_components: Default::default(),
            suggested_ycbcr_model: Default::default(),
            suggested_ycbcr_range: Default::default(),
            suggested_x_chroma_offset: Default::default(),
            suggested_y_chroma_offset: Default::default(),
        }
    }
}
impl<'lt> AndroidHardwareBufferFormatPropertiesANDROID<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> Format {
        self.format
    }
    ///Gets the value of [`Self::external_format`]
    pub fn external_format(&self) -> u64 {
        self.external_format
    }
    ///Gets the value of [`Self::format_features`]
    pub fn format_features(&self) -> FormatFeatureFlags {
        self.format_features
    }
    ///Gets the value of [`Self::sampler_ycbcr_conversion_components`]
    pub fn sampler_ycbcr_conversion_components(&self) -> ComponentMapping {
        self.sampler_ycbcr_conversion_components
    }
    ///Gets the value of [`Self::suggested_ycbcr_model`]
    pub fn suggested_ycbcr_model(&self) -> SamplerYcbcrModelConversion {
        self.suggested_ycbcr_model
    }
    ///Gets the value of [`Self::suggested_ycbcr_range`]
    pub fn suggested_ycbcr_range(&self) -> SamplerYcbcrRange {
        self.suggested_ycbcr_range
    }
    ///Gets the value of [`Self::suggested_x_chroma_offset`]
    pub fn suggested_x_chroma_offset(&self) -> ChromaLocation {
        self.suggested_x_chroma_offset
    }
    ///Gets the value of [`Self::suggested_y_chroma_offset`]
    pub fn suggested_y_chroma_offset(&self) -> ChromaLocation {
        self.suggested_y_chroma_offset
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Gets a mutable reference to the value of [`Self::external_format`]
    pub fn external_format_mut(&mut self) -> &mut u64 {
        &mut self.external_format
    }
    ///Gets a mutable reference to the value of [`Self::format_features`]
    pub fn format_features_mut(&mut self) -> &mut FormatFeatureFlags {
        &mut self.format_features
    }
    ///Gets a mutable reference to the value of [`Self::sampler_ycbcr_conversion_components`]
    pub fn sampler_ycbcr_conversion_components_mut(&mut self) -> &mut ComponentMapping {
        &mut self.sampler_ycbcr_conversion_components
    }
    ///Gets a mutable reference to the value of [`Self::suggested_ycbcr_model`]
    pub fn suggested_ycbcr_model_mut(&mut self) -> &mut SamplerYcbcrModelConversion {
        &mut self.suggested_ycbcr_model
    }
    ///Gets a mutable reference to the value of [`Self::suggested_ycbcr_range`]
    pub fn suggested_ycbcr_range_mut(&mut self) -> &mut SamplerYcbcrRange {
        &mut self.suggested_ycbcr_range
    }
    ///Gets a mutable reference to the value of [`Self::suggested_x_chroma_offset`]
    pub fn suggested_x_chroma_offset_mut(&mut self) -> &mut ChromaLocation {
        &mut self.suggested_x_chroma_offset
    }
    ///Gets a mutable reference to the value of [`Self::suggested_y_chroma_offset`]
    pub fn suggested_y_chroma_offset_mut(&mut self) -> &mut ChromaLocation {
        &mut self.suggested_y_chroma_offset
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::format`]
    pub fn set_format(mut self, value: crate::vulkan1_0::Format) -> Self {
        self.format = value;
        self
    }
    ///Sets the value of [`Self::external_format`]
    pub fn set_external_format(mut self, value: u64) -> Self {
        self.external_format = value;
        self
    }
    ///Sets the value of [`Self::format_features`]
    pub fn set_format_features(mut self, value: crate::vulkan1_0::FormatFeatureFlags) -> Self {
        self.format_features = value;
        self
    }
    ///Sets the value of [`Self::sampler_ycbcr_conversion_components`]
    pub fn set_sampler_ycbcr_conversion_components(mut self, value: crate::vulkan1_0::ComponentMapping) -> Self {
        self.sampler_ycbcr_conversion_components = value;
        self
    }
    ///Sets the value of [`Self::suggested_ycbcr_model`]
    pub fn set_suggested_ycbcr_model(mut self, value: crate::vulkan1_1::SamplerYcbcrModelConversion) -> Self {
        self.suggested_ycbcr_model = value;
        self
    }
    ///Sets the value of [`Self::suggested_ycbcr_range`]
    pub fn set_suggested_ycbcr_range(mut self, value: crate::vulkan1_1::SamplerYcbcrRange) -> Self {
        self.suggested_ycbcr_range = value;
        self
    }
    ///Sets the value of [`Self::suggested_x_chroma_offset`]
    pub fn set_suggested_x_chroma_offset(mut self, value: crate::vulkan1_1::ChromaLocation) -> Self {
        self.suggested_x_chroma_offset = value;
        self
    }
    ///Sets the value of [`Self::suggested_y_chroma_offset`]
    pub fn set_suggested_y_chroma_offset(mut self, value: crate::vulkan1_1::ChromaLocation) -> Self {
        self.suggested_y_chroma_offset = value;
        self
    }
}
///[VkExternalFormatANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFormatANDROID.html) - Structure containing an Android hardware buffer external format
///# C Specifications
///To create an image with an
///[external
///format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-external-formats), add a [`ExternalFormatANDROID`] structure in the [`p_next`]
///chain of [`ImageCreateInfo`].
///[`ExternalFormatANDROID`] is defined as:
///```c
///// Provided by VK_ANDROID_external_memory_android_hardware_buffer
///typedef struct VkExternalFormatANDROID {
///    VkStructureType    sType;
///    void*              pNext;
///    uint64_t           externalFormat;
///} VkExternalFormatANDROID;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`external_format`] is an implementation-defined identifier for the external format
///# Description
///If [`external_format`] is zero, the effect is as if the
///[`ExternalFormatANDROID`] structure was not present.
///Otherwise, the `image` will have the specified external format.
///## Valid Usage
/// - [`external_format`] **must**  be `0` or a value returned in the [`external_format`] member of
///   [`AndroidHardwareBufferFormatPropertiesANDROID`] by an earlier call to
///   [`get_android_hardware_buffer_properties_android`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID`
///# Related
/// - [`android_external_memory_android_hardware_buffer`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalFormatANDROID")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ExternalFormatANDROID<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`external_format`] is an implementation-defined identifier for the
    ///external format
    pub external_format: u64,
}
impl<'lt> Default for ExternalFormatANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::EXTERNAL_FORMAT_ANDROID,
            p_next: std::ptr::null_mut(),
            external_format: 0,
        }
    }
}
impl<'lt> ExternalFormatANDROID<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::external_format`]
    pub fn external_format(&self) -> u64 {
        self.external_format
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::external_format`]
    pub fn external_format_mut(&mut self) -> &mut u64 {
        &mut self.external_format
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::external_format`]
    pub fn set_external_format(mut self, value: u64) -> Self {
        self.external_format = value;
        self
    }
}
///[VkAndroidHardwareBufferFormatProperties2ANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferFormatProperties2ANDROID.html) - Structure describing the image format properties of an Android hardware buffer
///# C Specifications
///The format properties of an Android hardware buffer  **can**  be obtained by
///including a [`AndroidHardwareBufferFormatProperties2ANDROID`] structure
///in the [`p_next`] chain of the
///[`AndroidHardwareBufferPropertiesANDROID`] structure passed to
///[`get_android_hardware_buffer_properties_android`].
///This structure is defined as:
///```c
///// Provided by VK_KHR_format_feature_flags2 with
///// VK_ANDROID_external_memory_android_hardware_buffer
///typedef struct VkAndroidHardwareBufferFormatProperties2ANDROID {
///    VkStructureType                  sType;
///    void*                            pNext;
///    VkFormat                         format;
///    uint64_t                         externalFormat;
///    VkFormatFeatureFlags2            formatFeatures;
///    VkComponentMapping               samplerYcbcrConversionComponents;
///    VkSamplerYcbcrModelConversion    suggestedYcbcrModel;
///    VkSamplerYcbcrRange              suggestedYcbcrRange;
///    VkChromaLocation                 suggestedXChromaOffset;
///    VkChromaLocation                 suggestedYChromaOffset;
///} VkAndroidHardwareBufferFormatProperties2ANDROID;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format`] is the Vulkan format corresponding to the Android hardware buffer’s format, or
///   `VK_FORMAT_UNDEFINED` if there is not an equivalent Vulkan format.
/// - [`external_format`] is an implementation-defined external format identifier for use with
///   [`ExternalFormatANDROID`]. It  **must**  not be zero.
/// - [`format_features`] describes the capabilities of this external format when used with an image
///   bound to memory imported from `buffer`.
/// - [`sampler_ycbcr_conversion_components`] is the component swizzle that  **should**  be used in
///   [`SamplerYcbcrConversionCreateInfo`].
/// - [`suggested_ycbcr_model`] is a suggested color model to use in the
///   [`SamplerYcbcrConversionCreateInfo`].
/// - [`suggested_ycbcr_range`] is a suggested numerical value range to use in
///   [`SamplerYcbcrConversionCreateInfo`].
/// - [`suggested_x_chroma_offset`] is a suggested X chroma offset to use in
///   [`SamplerYcbcrConversionCreateInfo`].
/// - [`suggested_y_chroma_offset`] is a suggested Y chroma offset to use in
///   [`SamplerYcbcrConversionCreateInfo`].
///# Description
///The bits reported in [`format_features`] **must**  include the bits reported in
///the corresponding fields of
///[`AndroidHardwareBufferFormatPropertiesANDROID`]::[`format_features`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID`
///# Related
/// - [`android_external_memory_android_hardware_buffer`]
/// - [`khr_format_feature_flags2`]
/// - [`ChromaLocation`]
/// - [`ComponentMapping`]
/// - [`Format`]
/// - [`FormatFeatureFlags2`]
/// - [`SamplerYcbcrModelConversion`]
/// - [`SamplerYcbcrRange`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAndroidHardwareBufferFormatProperties2ANDROID")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct AndroidHardwareBufferFormatProperties2ANDROID<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`format`] is the Vulkan format corresponding to the Android hardware
    ///buffer’s format, or `VK_FORMAT_UNDEFINED` if there is not an
    ///equivalent Vulkan format.
    pub format: Format,
    ///[`external_format`] is an implementation-defined external format
    ///identifier for use with [`ExternalFormatANDROID`].
    ///It  **must**  not be zero.
    pub external_format: u64,
    ///[`format_features`] describes the capabilities of this external format
    ///when used with an image bound to memory imported from `buffer`.
    pub format_features: FormatFeatureFlags2,
    ///[`sampler_ycbcr_conversion_components`] is the component swizzle that
    /// **should**  be used in [`SamplerYcbcrConversionCreateInfo`].
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    ///[`suggested_ycbcr_model`] is a suggested color model to use in the
    ///[`SamplerYcbcrConversionCreateInfo`].
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    ///[`suggested_ycbcr_range`] is a suggested numerical value range to use in
    ///[`SamplerYcbcrConversionCreateInfo`].
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    ///[`suggested_x_chroma_offset`] is a suggested X chroma offset to use in
    ///[`SamplerYcbcrConversionCreateInfo`].
    pub suggested_x_chroma_offset: ChromaLocation,
    ///[`suggested_y_chroma_offset`] is a suggested Y chroma offset to use in
    ///[`SamplerYcbcrConversionCreateInfo`].
    pub suggested_y_chroma_offset: ChromaLocation,
}
impl<'lt> Default for AndroidHardwareBufferFormatProperties2ANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES2_ANDROID,
            p_next: std::ptr::null_mut(),
            format: Default::default(),
            external_format: 0,
            format_features: Default::default(),
            sampler_ycbcr_conversion_components: Default::default(),
            suggested_ycbcr_model: Default::default(),
            suggested_ycbcr_range: Default::default(),
            suggested_x_chroma_offset: Default::default(),
            suggested_y_chroma_offset: Default::default(),
        }
    }
}
impl<'lt> AndroidHardwareBufferFormatProperties2ANDROID<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> Format {
        self.format
    }
    ///Gets the value of [`Self::external_format`]
    pub fn external_format(&self) -> u64 {
        self.external_format
    }
    ///Gets the value of [`Self::format_features`]
    pub fn format_features(&self) -> FormatFeatureFlags2 {
        self.format_features
    }
    ///Gets the value of [`Self::sampler_ycbcr_conversion_components`]
    pub fn sampler_ycbcr_conversion_components(&self) -> ComponentMapping {
        self.sampler_ycbcr_conversion_components
    }
    ///Gets the value of [`Self::suggested_ycbcr_model`]
    pub fn suggested_ycbcr_model(&self) -> SamplerYcbcrModelConversion {
        self.suggested_ycbcr_model
    }
    ///Gets the value of [`Self::suggested_ycbcr_range`]
    pub fn suggested_ycbcr_range(&self) -> SamplerYcbcrRange {
        self.suggested_ycbcr_range
    }
    ///Gets the value of [`Self::suggested_x_chroma_offset`]
    pub fn suggested_x_chroma_offset(&self) -> ChromaLocation {
        self.suggested_x_chroma_offset
    }
    ///Gets the value of [`Self::suggested_y_chroma_offset`]
    pub fn suggested_y_chroma_offset(&self) -> ChromaLocation {
        self.suggested_y_chroma_offset
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Gets a mutable reference to the value of [`Self::external_format`]
    pub fn external_format_mut(&mut self) -> &mut u64 {
        &mut self.external_format
    }
    ///Gets a mutable reference to the value of [`Self::format_features`]
    pub fn format_features_mut(&mut self) -> &mut FormatFeatureFlags2 {
        &mut self.format_features
    }
    ///Gets a mutable reference to the value of [`Self::sampler_ycbcr_conversion_components`]
    pub fn sampler_ycbcr_conversion_components_mut(&mut self) -> &mut ComponentMapping {
        &mut self.sampler_ycbcr_conversion_components
    }
    ///Gets a mutable reference to the value of [`Self::suggested_ycbcr_model`]
    pub fn suggested_ycbcr_model_mut(&mut self) -> &mut SamplerYcbcrModelConversion {
        &mut self.suggested_ycbcr_model
    }
    ///Gets a mutable reference to the value of [`Self::suggested_ycbcr_range`]
    pub fn suggested_ycbcr_range_mut(&mut self) -> &mut SamplerYcbcrRange {
        &mut self.suggested_ycbcr_range
    }
    ///Gets a mutable reference to the value of [`Self::suggested_x_chroma_offset`]
    pub fn suggested_x_chroma_offset_mut(&mut self) -> &mut ChromaLocation {
        &mut self.suggested_x_chroma_offset
    }
    ///Gets a mutable reference to the value of [`Self::suggested_y_chroma_offset`]
    pub fn suggested_y_chroma_offset_mut(&mut self) -> &mut ChromaLocation {
        &mut self.suggested_y_chroma_offset
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::format`]
    pub fn set_format(mut self, value: crate::vulkan1_0::Format) -> Self {
        self.format = value;
        self
    }
    ///Sets the value of [`Self::external_format`]
    pub fn set_external_format(mut self, value: u64) -> Self {
        self.external_format = value;
        self
    }
    ///Sets the value of [`Self::format_features`]
    pub fn set_format_features(mut self, value: crate::vulkan1_3::FormatFeatureFlags2) -> Self {
        self.format_features = value;
        self
    }
    ///Sets the value of [`Self::sampler_ycbcr_conversion_components`]
    pub fn set_sampler_ycbcr_conversion_components(mut self, value: crate::vulkan1_0::ComponentMapping) -> Self {
        self.sampler_ycbcr_conversion_components = value;
        self
    }
    ///Sets the value of [`Self::suggested_ycbcr_model`]
    pub fn set_suggested_ycbcr_model(mut self, value: crate::vulkan1_1::SamplerYcbcrModelConversion) -> Self {
        self.suggested_ycbcr_model = value;
        self
    }
    ///Sets the value of [`Self::suggested_ycbcr_range`]
    pub fn set_suggested_ycbcr_range(mut self, value: crate::vulkan1_1::SamplerYcbcrRange) -> Self {
        self.suggested_ycbcr_range = value;
        self
    }
    ///Sets the value of [`Self::suggested_x_chroma_offset`]
    pub fn set_suggested_x_chroma_offset(mut self, value: crate::vulkan1_1::ChromaLocation) -> Self {
        self.suggested_x_chroma_offset = value;
        self
    }
    ///Sets the value of [`Self::suggested_y_chroma_offset`]
    pub fn set_suggested_y_chroma_offset(mut self, value: crate::vulkan1_1::ChromaLocation) -> Self {
        self.suggested_y_chroma_offset = value;
        self
    }
}
impl Device {
    ///[vkGetAndroidHardwareBufferPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html) - Get Properties of External Memory Android Hardware Buffers
    ///# C Specifications
    ///To determine the memory parameters to use when importing an Android hardware
    ///buffer, call:
    ///```c
    ///// Provided by VK_ANDROID_external_memory_android_hardware_buffer
    ///VkResult vkGetAndroidHardwareBufferPropertiesANDROID(
    ///    VkDevice                                    device,
    ///    const struct AHardwareBuffer*               buffer,
    ///    VkAndroidHardwareBufferPropertiesANDROID*   pProperties);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that will be importing [`buffer`].
    /// - [`buffer`] is the Android hardware buffer which will be imported.
    /// - [`p_properties`] is a pointer to a [`AndroidHardwareBufferPropertiesANDROID`] structure in
    ///   which the properties of [`buffer`] are returned.
    ///# Description
    ///## Valid Usage
    /// - [`buffer`] **must**  be a valid Android hardware buffer object with at least one of the
    ///   `AHARDWAREBUFFER_USAGE_GPU_*` flags in its `AHardwareBuffer_Desc`::`usage`
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`buffer`] **must**  be a valid pointer to a valid [`AHardwareBuffer`] value
    /// - [`p_properties`] **must**  be a valid pointer to a
    ///   [`AndroidHardwareBufferPropertiesANDROID`] structure
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`
    ///# Related
    /// - [`android_external_memory_android_hardware_buffer`]
    /// - [`AndroidHardwareBufferPropertiesANDROID`]
    /// - [`Device`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_android_hardware_buffer_properties_android<'lt>(
        self: &Unique<Device>,
        buffer: &AHardwareBuffer,
        p_properties: Option<AndroidHardwareBufferPropertiesANDROID<'lt>>,
    ) -> VulkanResult<AndroidHardwareBufferPropertiesANDROID<'lt>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .android_external_memory_android_hardware_buffer()
            .and_then(|vtable| vtable.get_android_hardware_buffer_properties_android())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .android_external_memory_android_hardware_buffer()
            .and_then(|vtable| vtable.get_android_hardware_buffer_properties_android())
            .unwrap_unchecked();
        let mut p_properties = p_properties.unwrap_or_default();
        let _return = _function(self.as_raw(), buffer as *const AHardwareBuffer, &mut p_properties);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, {
                p_properties.p_next = std::ptr::null_mut();
                p_properties
            }),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetMemoryAndroidHardwareBufferANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html) - Get an Android hardware buffer for a memory object
    ///# C Specifications
    ///To export an Android hardware buffer referencing the payload of a Vulkan
    ///device memory object, call:
    ///```c
    ///// Provided by VK_ANDROID_external_memory_android_hardware_buffer
    ///VkResult vkGetMemoryAndroidHardwareBufferANDROID(
    ///    VkDevice                                    device,
    ///    const VkMemoryGetAndroidHardwareBufferInfoANDROID* pInfo,
    ///    struct AHardwareBuffer**                    pBuffer);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that created the device memory being exported.
    /// - [`p_info`] is a pointer to a [`MemoryGetAndroidHardwareBufferInfoANDROID`] structure
    ///   containing parameters of the export operation.
    /// - [`p_buffer`] will return an Android hardware buffer referencing the payload of the device
    ///   memory object.
    ///# Description
    ///Each call to [`get_memory_android_hardware_buffer_android`] **must**  return an
    ///Android hardware buffer with a new reference acquired in addition to the
    ///reference held by the [`DeviceMemory`].
    ///To avoid leaking resources, the application  **must**  release the reference by
    ///calling `AHardwareBuffer_release` when it is no longer needed.
    ///When called with the same handle in
    ///[`MemoryGetAndroidHardwareBufferInfoANDROID::memory`],
    ///[`get_memory_android_hardware_buffer_android`] **must**  return the same Android
    ///hardware buffer object.
    ///If the device memory was created by importing an Android hardware buffer,
    ///[`get_memory_android_hardware_buffer_android`] **must**  return that same Android
    ///hardware buffer object.
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_info`] **must**  be a valid pointer to a valid
    ///   [`MemoryGetAndroidHardwareBufferInfoANDROID`] structure
    /// - [`p_buffer`] **must**  be a valid pointer to a valid pointer to an [`AHardwareBuffer`]
    ///   value
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///# Related
    /// - [`android_external_memory_android_hardware_buffer`]
    /// - [`Device`]
    /// - [`MemoryGetAndroidHardwareBufferInfoANDROID`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_memory_android_hardware_buffer_android<'lt>(
        self: &Unique<Device>,
        p_info: &MemoryGetAndroidHardwareBufferInfoANDROID<'lt>,
    ) -> VulkanResult<*mut AHardwareBuffer> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .android_external_memory_android_hardware_buffer()
            .and_then(|vtable| vtable.get_memory_android_hardware_buffer_android())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .android_external_memory_android_hardware_buffer()
            .and_then(|vtable| vtable.get_memory_android_hardware_buffer_android())
            .unwrap_unchecked();
        let mut p_buffer = std::ptr::null_mut();
        let _return = _function(
            self.as_raw(),
            p_info as *const MemoryGetAndroidHardwareBufferInfoANDROID<'lt>,
            &mut p_buffer,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_buffer),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from
/// `VK_ANDROID_external_memory_android_hardware_buffer`
pub struct DeviceAndroidExternalMemoryAndroidHardwareBufferVTable {
    ///See [`FNGetAndroidHardwareBufferPropertiesAndroid`] for more information.
    pub get_android_hardware_buffer_properties_android: FNGetAndroidHardwareBufferPropertiesAndroid,
    ///See [`FNGetMemoryAndroidHardwareBufferAndroid`] for more information.
    pub get_memory_android_hardware_buffer_android: FNGetMemoryAndroidHardwareBufferAndroid,
}
impl DeviceAndroidExternalMemoryAndroidHardwareBufferVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            get_android_hardware_buffer_properties_android: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetAndroidHardwareBufferPropertiesANDROID").as_ptr(),
                ))
            },
            get_memory_android_hardware_buffer_android: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetMemoryAndroidHardwareBufferANDROID").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_android_hardware_buffer_properties_android`]. See
    /// [`FNGetAndroidHardwareBufferPropertiesAndroid`] for more information.
    pub fn get_android_hardware_buffer_properties_android(&self) -> FNGetAndroidHardwareBufferPropertiesAndroid {
        self.get_android_hardware_buffer_properties_android
    }
    ///Gets [`Self::get_memory_android_hardware_buffer_android`]. See
    /// [`FNGetMemoryAndroidHardwareBufferAndroid`] for more information.
    pub fn get_memory_android_hardware_buffer_android(&self) -> FNGetMemoryAndroidHardwareBufferAndroid {
        self.get_memory_android_hardware_buffer_android
    }
}
