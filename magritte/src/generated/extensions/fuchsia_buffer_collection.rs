use crate::{
    native::zx_handle_t,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, BufferCreateInfo, ComponentMapping, FormatFeatureFlags, ImageCreateInfo,
        StructureType,
    },
    vulkan1_1::{ChromaLocation, SamplerYcbcrModelConversion, SamplerYcbcrRange},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION")]
pub const FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME")]
pub const FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_FUCHSIA_buffer_collection");
///[VkImportMemoryBufferCollectionFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryBufferCollectionFUCHSIA.html) - Structure to specify the Sysmem buffer to import
///# C Specifications
///The [`ImportMemoryBufferCollectionFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkImportMemoryBufferCollectionFUCHSIA {
///    VkStructureType              sType;
///    const void*                  pNext;
///    VkBufferCollectionFUCHSIA    collection;
///    uint32_t                     index;
///} VkImportMemoryBufferCollectionFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
/// - [`index`] the index of the buffer to import from [`collection`]
///# Description
///Valid Usage
/// - [`index`]**must** be less than the value retrieved as
///   [`BufferCollectionPropertiesFUCHSIA`]:bufferCount
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA`
/// - [`collection`]**must** be a valid [`BufferCollectionFUCHSIA`] handle
///# Related
/// - [`VK_FUCHSIA_buffer_collection`]
/// - [`BufferCollectionFUCHSIA`]
/// - [`StructureType`]
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
pub struct ImportMemoryBufferCollectionFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *mut BaseInStructure<'lt>,
    ///[`collection`] is the [`BufferCollectionFUCHSIA`] handle
    collection: BufferCollectionFUCHSIA,
    ///[`index`] the index of the buffer to import from [`collection`]
    index: u32,
}
///[VkBufferCollectionImageCreateInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionImageCreateInfoFUCHSIA.html) - Create a VkBufferCollectionFUCHSIA-compatible VkImage
///# C Specifications
///The [`BufferCollectionImageCreateInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkBufferCollectionImageCreateInfoFUCHSIA {
///    VkStructureType              sType;
///    const void*                  pNext;
///    VkBufferCollectionFUCHSIA    collection;
///    uint32_t                     index;
///} VkBufferCollectionImageCreateInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
/// - [`index`] is the index of the buffer in the buffer collection from which the memory will be
///   imported
///# Description
///Valid Usage
/// - [`index`]**must** be less than [`BufferCollectionPropertiesFUCHSIA::buffer_count`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA`
/// - [`collection`]**must** be a valid [`BufferCollectionFUCHSIA`] handle
///# Related
/// - [`VK_FUCHSIA_buffer_collection`]
/// - [`BufferCollectionFUCHSIA`]
/// - [`StructureType`]
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
pub struct BufferCollectionImageCreateInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *mut BaseInStructure<'lt>,
    ///[`collection`] is the [`BufferCollectionFUCHSIA`] handle
    collection: BufferCollectionFUCHSIA,
    ///[`index`] is the index of the buffer in the buffer collection from
    ///which the memory will be imported
    index: u32,
}
///[VkBufferCollectionBufferCreateInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionBufferCreateInfoFUCHSIA.html) - Create a VkBufferCollectionFUCHSIA-compatible VkBuffer
///# C Specifications
///The [`BufferCollectionBufferCreateInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkBufferCollectionBufferCreateInfoFUCHSIA {
///    VkStructureType              sType;
///    const void*                  pNext;
///    VkBufferCollectionFUCHSIA    collection;
///    uint32_t                     index;
///} VkBufferCollectionBufferCreateInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`collection`] is the [`BufferCollectionFUCHSIA`] handle
/// - [`index`] is the index of the buffer in the buffer collection from which the memory will be
///   imported
///# Description
///Valid Usage
/// - [`index`]**must** be less than [`BufferCollectionPropertiesFUCHSIA::buffer_count`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA`
/// - [`collection`]**must** be a valid [`BufferCollectionFUCHSIA`] handle
///# Related
/// - [`VK_FUCHSIA_buffer_collection`]
/// - [`BufferCollectionFUCHSIA`]
/// - [`StructureType`]
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
pub struct BufferCollectionBufferCreateInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *mut BaseInStructure<'lt>,
    ///[`collection`] is the [`BufferCollectionFUCHSIA`] handle
    collection: BufferCollectionFUCHSIA,
    ///[`index`] is the index of the buffer in the buffer collection from
    ///which the memory will be imported
    index: u32,
}
///[VkBufferCollectionCreateInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionCreateInfoFUCHSIA.html) - Structure specifying desired parameters to create the buffer collection
///# C Specifications
///The [`BufferCollectionCreateInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkBufferCollectionCreateInfoFUCHSIA {
///    VkStructureType    sType;
///    const void*        pNext;
///    zx_handle_t        collectionToken;
///} VkBufferCollectionCreateInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`collection_token`] is a [`zx_handle_t`] containing the Sysmem client’s buffer collection
///   token
///# Description
///Valid Usage
/// - [`collection_token`]**must** be a valid [`zx_handle_t`] to a Zircon channel allocated from
///   Sysmem (`fuchsia.sysmem.Allocator`/AllocateSharedCollection) with `ZX_DEFAULT_CHANNEL_RIGHTS`
///   rights
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_FUCHSIA_buffer_collection`]
/// - [`StructureType`]
/// - [`CreateBufferCollectionFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BufferCollectionCreateInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *mut BaseInStructure<'lt>,
    ///[`collection_token`] is a [`zx_handle_t`] containing the Sysmem
    ///client’s buffer collection token
    collection_token: zx_handle_t,
}
///[VkBufferCollectionPropertiesFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionPropertiesFUCHSIA.html) - Structure specifying the negotiated format chosen by Sysmem
///# C Specifications
///The [`BufferCollectionPropertiesFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkBufferCollectionPropertiesFUCHSIA {
///    VkStructureType                  sType;
///    void*                            pNext;
///    uint32_t                         memoryTypeBits;
///    uint32_t                         bufferCount;
///    uint32_t                         createInfoIndex;
///    uint64_t                         sysmemPixelFormat;
///    VkFormatFeatureFlags             formatFeatures;
///    VkSysmemColorSpaceFUCHSIA        sysmemColorSpaceIndex;
///    VkComponentMapping               samplerYcbcrConversionComponents;
///    VkSamplerYcbcrModelConversion    suggestedYcbcrModel;
///    VkSamplerYcbcrRange              suggestedYcbcrRange;
///    VkChromaLocation                 suggestedXChromaOffset;
///    VkChromaLocation                 suggestedYChromaOffset;
///} VkBufferCollectionPropertiesFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`memory_type_bits`] is a bitmask containing one bit set for every memory type which the
///   buffer collection can be imported as buffer collection
/// - [`buffer_count`] is the number of buffers in the collection
/// - [`create_info_index`] as described in [Sysmem chosen create infos](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sysmem-chosen-create-infos)
/// - [`sysmem_pixel_format`] is the Sysmem `PixelFormatType` as defined in
///   `fuchsia.sysmem/image_formats.fidl`
/// - [`format_features`] is a bitmask of [`FormatFeatureFlagBits`] shared by the buffer collection
/// - [`sysmem_color_space_index`] is a [`SysmemColorSpaceFUCHSIA`] struct specifying the color
///   space
/// - [`sampler_ycbcr_conversion_components`] is a [`ComponentMapping`] struct specifying the
///   component mapping
/// - [`suggested_ycbcr_model`] is a [`SamplerYcbcrModelConversion`] value specifying the suggested
///   Y′C<sub>B</sub>C<sub>R</sub> model
/// - [`suggested_ycbcr_range`] is a [`SamplerYcbcrRange`] value specifying the suggested
///   Y′C<sub>B</sub>C<sub>R</sub> range
/// - [`suggested_x_chroma_offset`] is a [`ChromaLocation`] value specifying the suggested X chroma
///   offset
/// - [`suggested_y_chroma_offset`] is a [`ChromaLocation`] value specifying the suggested Y chroma
///   offset
///# Description
///`sysmemColorSpace` is only set for image-based buffer collections where
///the constraints were specified using [`ImageConstraintsInfoFUCHSIA`] in
///a call to [`SetBufferCollectionImageConstraintsFUCHSIA`].For image-based buffer collections,
/// [`create_info_index`] will identify both
///the [`ImageConstraintsInfoFUCHSIA`]`::pImageCreateInfos` element and
///the [`ImageConstraintsInfoFUCHSIA::p_format_constraints`] element
///chosen by Sysmem when [`SetBufferCollectionImageConstraintsFUCHSIA`] was
///called.
///The value of [`sysmem_color_space_index`] will be an index to one of the
///color spaces provided in the
///[`ImageFormatConstraintsInfoFUCHSIA::p_color_spaces`] array.The implementation must have
/// [`format_features`] with all bits set that
///were set in
///[`ImageFormatConstraintsInfoFUCHSIA::required_format_features`], by
///the call to [`SetBufferCollectionImageConstraintsFUCHSIA`], at
///[`create_info_index`] (other bits could be set as well).Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
/// - [`format_features`]**must** be a valid combination of [`FormatFeatureFlagBits`] values
/// - [`format_features`]**must** not be `0`
/// - [`sysmem_color_space_index`]**must** be a valid [`SysmemColorSpaceFUCHSIA`] structure
/// - [`sampler_ycbcr_conversion_components`]**must** be a valid [`ComponentMapping`] structure
/// - [`suggested_ycbcr_model`]**must** be a valid [`SamplerYcbcrModelConversion`] value
/// - [`suggested_ycbcr_range`]**must** be a valid [`SamplerYcbcrRange`] value
/// - [`suggested_x_chroma_offset`]**must** be a valid [`ChromaLocation`] value
/// - [`suggested_y_chroma_offset`]**must** be a valid [`ChromaLocation`] value
///# Related
/// - [`VK_FUCHSIA_buffer_collection`]
/// - [`ChromaLocation`]
/// - [`ComponentMapping`]
/// - [`FormatFeatureFlags`]
/// - [`SamplerYcbcrModelConversion`]
/// - [`SamplerYcbcrRange`]
/// - [`StructureType`]
/// - [`SysmemColorSpaceFUCHSIA`]
/// - [`GetBufferCollectionPropertiesFUCHSIA`]
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
pub struct BufferCollectionPropertiesFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *const BaseOutStructure<'lt>,
    ///[`memory_type_bits`] is a bitmask containing one bit set for every
    ///memory type which the buffer collection can be imported as buffer
    ///collection
    memory_type_bits: u32,
    ///[`buffer_count`] is the number of buffers in the collection
    buffer_count: u32,
    ///[`create_info_index`] as described in [Sysmem chosen create infos](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sysmem-chosen-create-infos)
    create_info_index: u32,
    ///[`sysmem_pixel_format`] is the Sysmem `PixelFormatType` as defined in
    ///`fuchsia.sysmem/image_formats.fidl`
    sysmem_pixel_format: u64,
    ///[`format_features`] is a bitmask of [`FormatFeatureFlagBits`]
    ///shared by the buffer collection
    format_features: FormatFeatureFlags,
    ///[`sysmem_color_space_index`] is a [`SysmemColorSpaceFUCHSIA`] struct
    ///specifying the color space
    sysmem_color_space_index: SysmemColorSpaceFUCHSIA<'lt>,
    ///[`sampler_ycbcr_conversion_components`] is a [`ComponentMapping`]
    ///struct specifying the component mapping
    sampler_ycbcr_conversion_components: ComponentMapping,
    ///[`suggested_ycbcr_model`] is a [`SamplerYcbcrModelConversion`] value
    ///specifying the suggested Y′C<sub>B</sub>C<sub>R</sub> model
    suggested_ycbcr_model: SamplerYcbcrModelConversion,
    ///[`suggested_ycbcr_range`] is a [`SamplerYcbcrRange`] value
    ///specifying the suggested Y′C<sub>B</sub>C<sub>R</sub> range
    suggested_ycbcr_range: SamplerYcbcrRange,
    ///[`suggested_x_chroma_offset`] is a [`ChromaLocation`] value
    ///specifying the suggested X chroma offset
    suggested_x_chroma_offset: ChromaLocation,
    ///[`suggested_y_chroma_offset`] is a [`ChromaLocation`] value
    ///specifying the suggested Y chroma offset
    suggested_y_chroma_offset: ChromaLocation,
}
///[VkBufferConstraintsInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferConstraintsInfoFUCHSIA.html) - Structure buffer-based buffer collection constraints
///# C Specifications
///The [`BufferConstraintsInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkBufferConstraintsInfoFUCHSIA {
///    VkStructureType                             sType;
///    const void*                                 pNext;
///    VkBufferCreateInfo                          createInfo;
///    VkFormatFeatureFlags                        requiredFormatFeatures;
///    VkBufferCollectionConstraintsInfoFUCHSIA    bufferCollectionConstraints;
///} VkBufferConstraintsInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - `pBufferCreateInfo` a pointer to a [`BufferCreateInfo`] struct describing the buffer
///   attributes for the buffer collection
/// - [`required_format_features`] bitmask of [`FormatFeatureFlagBits`] required features of the
///   buffers in the buffer collection
/// - [`buffer_collection_constraints`] is used to supply parameters for the negotiation and
///   allocation of the buffer collection
///# Description
///Valid Usage
/// -    The [`required_format_features`] bitmask of [`FormatFeatureFlagBits`]**must** be chosen from among the buffer compatible format features listed in [buffer compatible format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#buffer-compatible-format-features)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
/// - [`create_info`]**must** be a valid [`BufferCreateInfo`] structure
/// - [`required_format_features`]**must** be a valid combination of [`FormatFeatureFlagBits`]
///   values
/// - [`buffer_collection_constraints`]**must** be a valid
///   [`BufferCollectionConstraintsInfoFUCHSIA`] structure
///# Related
/// - [`VK_FUCHSIA_buffer_collection`]
/// - [`BufferCollectionConstraintsInfoFUCHSIA`]
/// - [`BufferCreateInfo`]
/// - [`FormatFeatureFlags`]
/// - [`StructureType`]
/// - [`SetBufferCollectionBufferConstraintsFUCHSIA`]
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
pub struct BufferConstraintsInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *mut BaseInStructure<'lt>,
    ///No documentation found
    create_info: BufferCreateInfo<'lt>,
    ///[`required_format_features`] bitmask of [`FormatFeatureFlagBits`]
    ///required features of the buffers in the buffer collection
    required_format_features: FormatFeatureFlags,
    ///[`buffer_collection_constraints`] is used to supply parameters for the
    ///negotiation and allocation of the buffer collection
    buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'lt>,
}
///[VkSysmemColorSpaceFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSysmemColorSpaceFUCHSIA.html) - Structure describing the buffer collections color space
///# C Specifications
///The [`SysmemColorSpaceFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkSysmemColorSpaceFUCHSIA {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           colorSpace;
///} VkSysmemColorSpaceFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`color_space`] value of the Sysmem `ColorSpaceType`
///# Description
///Valid Usage
/// - [`color_space`]**must** be a `ColorSpaceType` as defined in
///   `fuchsia.sysmem/image_formats.fidl`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_FUCHSIA_buffer_collection`]
/// - [`BufferCollectionPropertiesFUCHSIA`]
/// - [`ImageFormatConstraintsInfoFUCHSIA`]
/// - [`StructureType`]
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
pub struct SysmemColorSpaceFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *mut BaseInStructure<'lt>,
    ///[`color_space`] value of the Sysmem `ColorSpaceType`
    color_space: u32,
}
///[VkImageFormatConstraintsInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsInfoFUCHSIA.html) - Structure image-based buffer collection constraints
///# C Specifications
///The [`ImageFormatConstraintsInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkImageFormatConstraintsInfoFUCHSIA {
///    VkStructureType                         sType;
///    const void*                             pNext;
///    VkImageCreateInfo                       imageCreateInfo;
///    VkFormatFeatureFlags                    requiredFormatFeatures;
///    VkImageFormatConstraintsFlagsFUCHSIA    flags;
///    uint64_t                                sysmemPixelFormat;
///    uint32_t                                colorSpaceCount;
///    const VkSysmemColorSpaceFUCHSIA*        pColorSpaces;
///} VkImageFormatConstraintsInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`image_create_info`] is the [`ImageCreateInfo`] used to create a [`Image`] that is to use
///   memory from the [`BufferCollectionFUCHSIA`]
/// - [`required_format_features`] is a bitmask of [`FormatFeatureFlagBits`] specifying required
///   features of the buffers in the buffer collection
/// - [`flags`] is reserved for future use
/// - [`sysmem_pixel_format`] is a `PixelFormatType` value from the
///   `fuchsia.sysmem/image_formats.fidl` FIDL interface
/// - [`color_space_count`] the element count of [`p_color_spaces`]
/// - [`p_color_spaces`] is a pointer to an array of [`SysmemColorSpaceFUCHSIA`] structs of size
///   [`color_space_count`]
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
/// - [`image_create_info`]**must** be a valid [`ImageCreateInfo`] structure
/// - [`required_format_features`]**must** be a valid combination of [`FormatFeatureFlagBits`]
///   values
/// - [`required_format_features`]**must** not be `0`
/// - [`flags`]**must** be `0`
/// - [`p_color_spaces`]**must** be a valid pointer to an array of [`color_space_count`] valid
///   [`SysmemColorSpaceFUCHSIA`] structures
/// - [`color_space_count`]**must** be greater than `0`
///# Related
/// - [`VK_FUCHSIA_buffer_collection`]
/// - [`FormatFeatureFlags`]
/// - [`ImageConstraintsInfoFUCHSIA`]
/// - [`ImageCreateInfo`]
/// - [`ImageFormatConstraintsFlagsFUCHSIA`]
/// - [`StructureType`]
/// - [`SysmemColorSpaceFUCHSIA`]
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
pub struct ImageFormatConstraintsInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *mut BaseInStructure<'lt>,
    ///[`image_create_info`] is the [`ImageCreateInfo`] used to create a
    ///[`Image`] that is to use memory from the
    ///[`BufferCollectionFUCHSIA`]
    image_create_info: ImageCreateInfo<'lt>,
    ///[`required_format_features`] is a bitmask of
    ///[`FormatFeatureFlagBits`] specifying required features of the
    ///buffers in the buffer collection
    required_format_features: FormatFeatureFlags,
    ///[`flags`] is reserved for future use
    flags: ImageFormatConstraintsFlagsFUCHSIA,
    ///[`sysmem_pixel_format`] is a `PixelFormatType` value from the
    ///`fuchsia.sysmem/image_formats.fidl` FIDL interface
    sysmem_pixel_format: u64,
    ///[`color_space_count`] the element count of [`p_color_spaces`]
    color_space_count: u32,
    ///[`p_color_spaces`] is a pointer to an array of
    ///[`SysmemColorSpaceFUCHSIA`] structs of size [`color_space_count`]
    p_color_spaces: *mut SysmemColorSpaceFUCHSIA<'lt>,
}
///[VkImageConstraintsInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFUCHSIA.html) - Structure of image-based buffer collection constraints
///# C Specifications
///The [`ImageConstraintsInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkImageConstraintsInfoFUCHSIA {
///    VkStructureType                               sType;
///    const void*                                   pNext;
///    uint32_t                                      formatConstraintsCount;
///    const VkImageFormatConstraintsInfoFUCHSIA*    pFormatConstraints;
///    VkBufferCollectionConstraintsInfoFUCHSIA      bufferCollectionConstraints;
///    VkImageConstraintsInfoFlagsFUCHSIA            flags;
///} VkImageConstraintsInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format_constraints_count`] is the number of elements in [`p_format_constraints`].
/// - [`p_format_constraints`] is a pointer to an array of [`ImageFormatConstraintsInfoFUCHSIA`]
///   structures of size [`format_constraints_count`] that is used to further constrain buffer
///   collection format selection for image-based buffer collections.
/// - [`buffer_collection_constraints`] is a [`BufferCollectionConstraintsInfoFUCHSIA`] structure
///   used to supply parameters for the negotiation and allocation for buffer-based buffer
///   collections.
/// - [`flags`] is a [`ImageConstraintsInfoFlagBitsFUCHSIA`] value specifying hints about the type
///   of memory Sysmem should allocate for the buffer collection.
///# Description
///Valid Usage
/// - All elements of [`p_format_constraints`]**must** have at least one bit set in its
///   [`ImageFormatConstraintsInfoFUCHSIA::required_format_features`]
/// - If [`p_format_constraints`]`::imageCreateInfo`::`usage` contains `VK_IMAGE_USAGE_SAMPLED_BIT`,
///   then [`p_format_constraints`]`::requiredFormatFeatures`**must** contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`
/// - If [`p_format_constraints`]`::imageCreateInfo`::`usage` contains `VK_IMAGE_USAGE_STORAGE_BIT`,
///   then [`p_format_constraints`]`::requiredFormatFeatures`**must** contain
///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT`
/// - If [`p_format_constraints`]`::imageCreateInfo`::`usage` contains
///   `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`, then
///   [`p_format_constraints`]`::requiredFormatFeatures`**must** contain
///   `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
/// - If [`p_format_constraints`]`::imageCreateInfo`::`usage` contains
///   `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, then
///   [`p_format_constraints`]`::requiredFormatFeatures`**must** contain
///   `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`p_format_constraints`]`::imageCreateInfo`::`usage` contains
///   `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`, then
///   [`p_format_constraints`]`::requiredFormatFeatures`**must** contain at least one of
///   `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT` or `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If the [`attachmentFragmentShadingRate` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate)
///   is enabled, and [`p_format_constraints`]`::imageCreateInfo`::`usage` contains
///   `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`, then
///   [`p_format_constraints`]`::requiredFormatFeatures`**must** contain
///   `VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
/// - [`p_format_constraints`]**must** be a valid pointer to an array of
///   [`format_constraints_count`] valid [`ImageFormatConstraintsInfoFUCHSIA`] structures
/// - [`buffer_collection_constraints`]**must** be a valid
///   [`BufferCollectionConstraintsInfoFUCHSIA`] structure
/// - [`flags`]**must** be a valid combination of [`ImageConstraintsInfoFlagBitsFUCHSIA`] values
/// - [`format_constraints_count`]**must** be greater than `0`
///# Related
/// - [`VK_FUCHSIA_buffer_collection`]
/// - [`BufferCollectionConstraintsInfoFUCHSIA`]
/// - [`ImageConstraintsInfoFlagsFUCHSIA`]
/// - [`ImageFormatConstraintsInfoFUCHSIA`]
/// - [`StructureType`]
/// - [`SetBufferCollectionImageConstraintsFUCHSIA`]
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
pub struct ImageConstraintsInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`format_constraints_count`] is the number of elements in
    ///[`p_format_constraints`].
    format_constraints_count: u32,
    ///[`p_format_constraints`] is a pointer to an array of
    ///[`ImageFormatConstraintsInfoFUCHSIA`] structures of size
    ///[`format_constraints_count`] that is used to further constrain buffer
    ///collection format selection for image-based buffer collections.
    p_format_constraints: *mut ImageFormatConstraintsInfoFUCHSIA<'lt>,
    ///[`buffer_collection_constraints`] is a
    ///[`BufferCollectionConstraintsInfoFUCHSIA`] structure used to supply
    ///parameters for the negotiation and allocation for buffer-based buffer
    ///collections.
    buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'lt>,
    ///[`flags`] is a [`ImageConstraintsInfoFlagBitsFUCHSIA`] value
    ///specifying hints about the type of memory Sysmem should allocate for the
    ///buffer collection.
    flags: ImageConstraintsInfoFlagsFUCHSIA,
}
///[VkBufferCollectionConstraintsInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionConstraintsInfoFUCHSIA.html) - Structure of general buffer collection constraints
///# C Specifications
///The [`BufferCollectionConstraintsInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///typedef struct VkBufferCollectionConstraintsInfoFUCHSIA {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           minBufferCount;
///    uint32_t           maxBufferCount;
///    uint32_t           minBufferCountForCamping;
///    uint32_t           minBufferCountForDedicatedSlack;
///    uint32_t           minBufferCountForSharedSlack;
///} VkBufferCollectionConstraintsInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`min_buffer_count`] is the minimum number of buffers available in the collection
/// - [`max_buffer_count`] is the maximum number of buffers allowed in the collection
/// - [`min_buffer_count_for_camping`] is the per-participant minimum buffers for camping
/// - [`min_buffer_count_for_dedicated_slack`] is the per-participant minimum buffers for dedicated
///   slack
/// - [`min_buffer_count_for_shared_slack`] is the per-participant minimum buffers for shared slack
///# Description
///Sysmem uses all buffer count parameters in combination to determine the
///number of buffers it will allocate.
///Sysmem defines buffer count constraints in
///`fuchsia.sysmem/constraints.fidl`.*Camping* as referred to by [`min_buffer_count_for_camping`],
/// is the number of
///buffers that should be available for the participant that are not for
///transient use.
///This number of buffers is required for the participant to logically operate.*Slack* as referred
/// to by [`min_buffer_count_for_dedicated_slack`] and
///[`min_buffer_count_for_shared_slack`], refers to the number of buffers desired
///by participants for optimal performance.
///[`min_buffer_count_for_dedicated_slack`] refers to the current participant.
///[`min_buffer_count_for_shared_slack`] refers to buffer slack for all
///participants in the collection.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_FUCHSIA_buffer_collection`]
/// - [`BufferConstraintsInfoFUCHSIA`]
/// - [`ImageConstraintsInfoFUCHSIA`]
/// - [`StructureType`]
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
pub struct BufferCollectionConstraintsInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *mut BaseInStructure<'lt>,
    ///[`min_buffer_count`] is the minimum number of buffers available in the
    ///collection
    min_buffer_count: u32,
    ///[`max_buffer_count`] is the maximum number of buffers allowed in the
    ///collection
    max_buffer_count: u32,
    ///[`min_buffer_count_for_camping`] is the per-participant minimum buffers
    ///for camping
    min_buffer_count_for_camping: u32,
    ///[`min_buffer_count_for_dedicated_slack`] is the per-participant minimum
    ///buffers for dedicated slack
    min_buffer_count_for_dedicated_slack: u32,
    ///[`min_buffer_count_for_shared_slack`] is the per-participant minimum
    ///buffers for shared slack
    min_buffer_count_for_shared_slack: u32,
}
