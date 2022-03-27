use crate::{
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, ComponentMapping, DeviceMemory, DeviceSize, Format, FormatFeatureFlags,
        StructureType,
    },
    vulkan1_1::{ChromaLocation, SamplerYcbcrModelConversion, SamplerYcbcrRange},
    vulkan1_3::FormatFeatureFlags2,
};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
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
///If the [`AllocateMemory`] command succeeds, the implementation **must**
///acquire a reference to the imported hardware buffer, which it **must** release
///when the device memory object is freed.
///If the command fails, the implementation **must** not retain a reference.Valid Usage
/// - If [`buffer`] is not `NULL`, Android hardware buffers **must** be supported for import, as
///   reported by [`ExternalImageFormatProperties`] or [`ExternalBufferProperties`]
/// -    If [`buffer`] is not `NULL`, it **must** be a valid Android hardware buffer object with `AHardwareBuffer_Desc`::`usage` compatible with Vulkan as described in [Android Hardware Buffers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
/// - [`buffer`]**must** be a valid pointer to an [`AHardwareBuffer`] value
///# Related
/// - [`VK_ANDROID_external_memory_android_hardware_buffer`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImportAndroidHardwareBufferInfoANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`buffer`] is the Android hardware buffer to import.
    buffer: *const AHardwareBuffer,
}
///[VkAndroidHardwareBufferUsageANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferUsageANDROID.html) - Struct containing Android hardware buffer usage flags
///# C Specifications
///To obtain optimal Android hardware buffer usage flags for specific image
///creation parameters, add a [`AndroidHardwareBufferUsageANDROID`]
///structure to the [`p_next`] chain of a [`ImageFormatProperties2`]
///structure passed to [`GetPhysicalDeviceImageFormatProperties2`].
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
///The [`android_hardware_buffer_usage`] field **must** include Android hardware
///buffer usage flags listed in the
///[AHardwareBuffer Usage
///Equivalence](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-usage) table when the corresponding Vulkan image usage or image
///creation flags are included in the `usage` or `flags` fields of
///[`PhysicalDeviceImageFormatInfo2`].
///It **must** include at least one GPU usage flag
///(`AHARDWAREBUFFER_USAGE_GPU_*`), even if none of the corresponding Vulkan
///usages or flags are requested.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID`
///# Related
/// - [`VK_ANDROID_external_memory_android_hardware_buffer`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AndroidHardwareBufferUsageANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`android_hardware_buffer_usage`] returns the Android hardware buffer
    ///usage flags.
    android_hardware_buffer_usage: u64,
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
///   specified Android hardware buffer **can** be imported as.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`AndroidHardwareBufferFormatProperties2ANDROID`] or
///   [`AndroidHardwareBufferFormatPropertiesANDROID`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
///# Related
/// - [`VK_ANDROID_external_memory_android_hardware_buffer`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`GetAndroidHardwareBufferPropertiesANDROID`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AndroidHardwareBufferPropertiesANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`allocation_size`] is the size of the external memory
    allocation_size: DeviceSize,
    ///[`memory_type_bits`] is a bitmask containing one bit set for every
    ///memory type which the specified Android hardware buffer **can** be imported
    ///as.
    memory_type_bits: u32,
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
///Valid Usage
/// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID`**must** have been
///   included in [`ExportMemoryAllocateInfo::handle_types`] when [`memory`] was created
/// - If the [`p_next`] chain of the [`MemoryAllocateInfo`] used to allocate [`memory`] included a
///   [`MemoryDedicatedAllocateInfo`] with non-`NULL``image` member, then that `image`**must**
///   already be bound to [`memory`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
/// - [`p_next`]**must** be `NULL`
/// - [`memory`]**must** be a valid [`DeviceMemory`] handle
///# Related
/// - [`VK_ANDROID_external_memory_android_hardware_buffer`]
/// - [`DeviceMemory`]
/// - [`StructureType`]
/// - [`GetMemoryAndroidHardwareBufferANDROID`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`memory`] is the memory object from which the Android hardware buffer
    ///will be exported.
    memory: DeviceMemory,
}
///[VkAndroidHardwareBufferFormatPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferFormatPropertiesANDROID.html) - Structure describing the image format properties of an Android hardware buffer
///# C Specifications
///To obtain format properties of an Android hardware buffer, include a
///[`AndroidHardwareBufferFormatPropertiesANDROID`] structure in the
///[`p_next`] chain of the [`AndroidHardwareBufferPropertiesANDROID`]
///structure passed to [`GetAndroidHardwareBufferPropertiesANDROID`].
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
///   [`ExternalFormatANDROID`]. It **must** not be zero.
/// - [`format_features`] describes the capabilities of this external format when used with an image
///   bound to memory imported from `buffer`.
/// - [`sampler_ycbcr_conversion_components`] is the component swizzle that **should** be used in
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
///table](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-formats), then [`format`]**must** have the equivalent Vulkan format listed in
///the table.
///Otherwise, [`format`]**may** be `VK_FORMAT_UNDEFINED`, indicating the
///Android hardware buffer **can** only be used with an external format.The [`format_features`]
/// member **must** include
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT` and at least one of
///`VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT` or
///`VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`, and **should** include
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` and
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`.Android hardware buffers
/// with the same external format **must** have the same
///support for `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`,
///`VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT`,
///`VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`,
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`,
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT`,
///and
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT`.
///in [`format_features`].
///Other format features **may** differ between Android hardware buffers that have
///the same external format.
///This allows applications to use the same [`SamplerYcbcrConversion`]
///object (and samplers and pipelines created from them) for any Android
///hardware buffers that have the same external format.If [`format`] is not `VK_FORMAT_UNDEFINED`,
/// then the value of
///[`sampler_ycbcr_conversion_components`]**must** be valid when used as the
///`components` member of [`SamplerYcbcrConversionCreateInfo`] with
///that format.
///If [`format`] is `VK_FORMAT_UNDEFINED`, all members of
///[`sampler_ycbcr_conversion_components`]**must** be the
///[identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings).Implementations **may** not always be able to determine the color model,
///numerical range, or chroma offsets of the image contents, so the values in
///[`AndroidHardwareBufferFormatPropertiesANDROID`] are only suggestions.
///Applications **should** treat these values as sensible defaults to use in the
///absence of more reliable information obtained through some other means.
///If the underlying physical device is also usable via OpenGL ES with the
///[`GL_OES_EGL_image_external`](https://www.khronos.org/registry/OpenGL/extensions/OES/OES_EGL_image_external.txt)
///extension, the implementation **should** suggest values that will produce
///similar sampled values as would be obtained by sampling the same external
///image via `samplerExternalOES` in OpenGL ES using equivalent sampler
///parameters.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID`
///# Related
/// - [`VK_ANDROID_external_memory_android_hardware_buffer`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`format`] is the Vulkan format corresponding to the Android hardware
    ///buffer’s format, or `VK_FORMAT_UNDEFINED` if there is not an
    ///equivalent Vulkan format.
    format: Format,
    ///[`external_format`] is an implementation-defined external format
    ///identifier for use with [`ExternalFormatANDROID`].
    ///It **must** not be zero.
    external_format: u64,
    ///[`format_features`] describes the capabilities of this external format
    ///when used with an image bound to memory imported from `buffer`.
    format_features: FormatFeatureFlags,
    ///[`sampler_ycbcr_conversion_components`] is the component swizzle that
    ///**should** be used in [`SamplerYcbcrConversionCreateInfo`].
    sampler_ycbcr_conversion_components: ComponentMapping,
    ///[`suggested_ycbcr_model`] is a suggested color model to use in the
    ///[`SamplerYcbcrConversionCreateInfo`].
    suggested_ycbcr_model: SamplerYcbcrModelConversion,
    ///[`suggested_ycbcr_range`] is a suggested numerical value range to use in
    ///[`SamplerYcbcrConversionCreateInfo`].
    suggested_ycbcr_range: SamplerYcbcrRange,
    ///[`suggested_x_chroma_offset`] is a suggested X chroma offset to use in
    ///[`SamplerYcbcrConversionCreateInfo`].
    suggested_x_chroma_offset: ChromaLocation,
    ///[`suggested_y_chroma_offset`] is a suggested Y chroma offset to use in
    ///[`SamplerYcbcrConversionCreateInfo`].
    suggested_y_chroma_offset: ChromaLocation,
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
///Otherwise, the `image` will have the specified external format.Valid Usage
/// - [`external_format`]**must** be `0` or a value returned in the [`external_format`] member of
///   [`AndroidHardwareBufferFormatPropertiesANDROID`] by an earlier call to
///   [`GetAndroidHardwareBufferPropertiesANDROID`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID`
///# Related
/// - [`VK_ANDROID_external_memory_android_hardware_buffer`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalFormatANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`external_format`] is an implementation-defined identifier for the
    ///external format
    external_format: u64,
}
///[VkAndroidHardwareBufferFormatProperties2ANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferFormatProperties2ANDROID.html) - Structure describing the image format properties of an Android hardware buffer
///# C Specifications
///The format properties of an Android hardware buffer **can** be obtained by
///including a [`AndroidHardwareBufferFormatProperties2ANDROID`] structure
///in the [`p_next`] chain of the
///[`AndroidHardwareBufferPropertiesANDROID`] structure passed to
///[`GetAndroidHardwareBufferPropertiesANDROID`].
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
///   [`ExternalFormatANDROID`]. It **must** not be zero.
/// - [`format_features`] describes the capabilities of this external format when used with an image
///   bound to memory imported from `buffer`.
/// - [`sampler_ycbcr_conversion_components`] is the component swizzle that **should** be used in
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
///The bits reported in [`format_features`]**must** include the bits reported in
///the corresponding fields of
///[`AndroidHardwareBufferFormatPropertiesANDROID`]::[`format_features`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID`
///# Related
/// - [`VK_ANDROID_external_memory_android_hardware_buffer`]
/// - [`VK_KHR_format_feature_flags2`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AndroidHardwareBufferFormatProperties2ANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`format`] is the Vulkan format corresponding to the Android hardware
    ///buffer’s format, or `VK_FORMAT_UNDEFINED` if there is not an
    ///equivalent Vulkan format.
    format: Format,
    ///[`external_format`] is an implementation-defined external format
    ///identifier for use with [`ExternalFormatANDROID`].
    ///It **must** not be zero.
    external_format: u64,
    ///[`format_features`] describes the capabilities of this external format
    ///when used with an image bound to memory imported from `buffer`.
    format_features: FormatFeatureFlags2,
    ///[`sampler_ycbcr_conversion_components`] is the component swizzle that
    ///**should** be used in [`SamplerYcbcrConversionCreateInfo`].
    sampler_ycbcr_conversion_components: ComponentMapping,
    ///[`suggested_ycbcr_model`] is a suggested color model to use in the
    ///[`SamplerYcbcrConversionCreateInfo`].
    suggested_ycbcr_model: SamplerYcbcrModelConversion,
    ///[`suggested_ycbcr_range`] is a suggested numerical value range to use in
    ///[`SamplerYcbcrConversionCreateInfo`].
    suggested_ycbcr_range: SamplerYcbcrRange,
    ///[`suggested_x_chroma_offset`] is a suggested X chroma offset to use in
    ///[`SamplerYcbcrConversionCreateInfo`].
    suggested_x_chroma_offset: ChromaLocation,
    ///[`suggested_y_chroma_offset`] is a suggested Y chroma offset to use in
    ///[`SamplerYcbcrConversionCreateInfo`].
    suggested_y_chroma_offset: ChromaLocation,
}
