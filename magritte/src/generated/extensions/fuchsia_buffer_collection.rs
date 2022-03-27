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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImportMemoryBufferCollectionFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *const BaseInStructure<'lt>,
    ///[`collection`] is the [`BufferCollectionFUCHSIA`] handle
    collection: BufferCollectionFUCHSIA,
    ///[`index`] the index of the buffer to import from [`collection`]
    index: u32,
}
impl<'lt> Default for ImportMemoryBufferCollectionFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            collection: Default::default(),
            index: 0,
        }
    }
}
impl<'lt> ImportMemoryBufferCollectionFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::index`]
    pub fn index_raw(&self) -> u32 {
        self.index
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::index`]
    pub fn set_index_raw(&mut self, value: u32) -> &mut Self {
        self.index = value;
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
    ///Gets the value of [`Self::collection`]
    pub fn collection(&self) -> BufferCollectionFUCHSIA {
        self.collection
    }
    ///Gets the value of [`Self::index`]
    pub fn index(&self) -> u32 {
        self.index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::collection`]
    pub fn collection_mut(&mut self) -> &mut BufferCollectionFUCHSIA {
        &mut self.collection
    }
    ///Gets a mutable reference to the value of [`Self::index`]
    pub fn index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::collection`]
    pub fn set_collection(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    ) -> &mut Self {
        self.collection = value;
        self
    }
    ///Sets the raw value of [`Self::index`]
    pub fn set_index(&mut self, value: u32) -> &mut Self {
        self.index = value;
        self
    }
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BufferCollectionImageCreateInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *const BaseInStructure<'lt>,
    ///[`collection`] is the [`BufferCollectionFUCHSIA`] handle
    collection: BufferCollectionFUCHSIA,
    ///[`index`] is the index of the buffer in the buffer collection from
    ///which the memory will be imported
    index: u32,
}
impl<'lt> Default for BufferCollectionImageCreateInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            collection: Default::default(),
            index: 0,
        }
    }
}
impl<'lt> BufferCollectionImageCreateInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::index`]
    pub fn index_raw(&self) -> u32 {
        self.index
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::index`]
    pub fn set_index_raw(&mut self, value: u32) -> &mut Self {
        self.index = value;
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
    ///Gets the value of [`Self::collection`]
    pub fn collection(&self) -> BufferCollectionFUCHSIA {
        self.collection
    }
    ///Gets the value of [`Self::index`]
    pub fn index(&self) -> u32 {
        self.index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::collection`]
    pub fn collection_mut(&mut self) -> &mut BufferCollectionFUCHSIA {
        &mut self.collection
    }
    ///Gets a mutable reference to the value of [`Self::index`]
    pub fn index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::collection`]
    pub fn set_collection(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    ) -> &mut Self {
        self.collection = value;
        self
    }
    ///Sets the raw value of [`Self::index`]
    pub fn set_index(&mut self, value: u32) -> &mut Self {
        self.index = value;
        self
    }
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BufferCollectionBufferCreateInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *const BaseInStructure<'lt>,
    ///[`collection`] is the [`BufferCollectionFUCHSIA`] handle
    collection: BufferCollectionFUCHSIA,
    ///[`index`] is the index of the buffer in the buffer collection from
    ///which the memory will be imported
    index: u32,
}
impl<'lt> Default for BufferCollectionBufferCreateInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            collection: Default::default(),
            index: 0,
        }
    }
}
impl<'lt> BufferCollectionBufferCreateInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::index`]
    pub fn index_raw(&self) -> u32 {
        self.index
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::index`]
    pub fn set_index_raw(&mut self, value: u32) -> &mut Self {
        self.index = value;
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
    ///Gets the value of [`Self::collection`]
    pub fn collection(&self) -> BufferCollectionFUCHSIA {
        self.collection
    }
    ///Gets the value of [`Self::index`]
    pub fn index(&self) -> u32 {
        self.index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::collection`]
    pub fn collection_mut(&mut self) -> &mut BufferCollectionFUCHSIA {
        &mut self.collection
    }
    ///Gets a mutable reference to the value of [`Self::index`]
    pub fn index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::collection`]
    pub fn set_collection(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    ) -> &mut Self {
        self.collection = value;
        self
    }
    ///Sets the raw value of [`Self::index`]
    pub fn set_index(&mut self, value: u32) -> &mut Self {
        self.index = value;
        self
    }
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
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BufferCollectionCreateInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *const BaseInStructure<'lt>,
    ///[`collection_token`] is a [`zx_handle_t`] containing the Sysmem
    ///client’s buffer collection token
    collection_token: zx_handle_t,
}
impl<'lt> Default for BufferCollectionCreateInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            collection_token: Default::default(),
        }
    }
}
impl<'lt> BufferCollectionCreateInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::collection_token`]
    pub fn collection_token_raw(&self) -> &zx_handle_t {
        &self.collection_token
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::collection_token`]
    pub fn set_collection_token_raw(&mut self, value: zx_handle_t) -> &mut Self {
        self.collection_token = value;
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
    ///Gets the value of [`Self::collection_token`]
    pub fn collection_token(&self) -> &zx_handle_t {
        &self.collection_token
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::collection_token`]
    pub fn collection_token_mut(&mut self) -> &mut zx_handle_t {
        &mut self.collection_token
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::collection_token`]
    pub fn set_collection_token(&mut self, value: crate::native::zx_handle_t) -> &mut Self {
        self.collection_token = value;
        self
    }
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
///the [`ImageConstraintsInfoFUCHSIA::format_constraints`] element
///chosen by Sysmem when [`SetBufferCollectionImageConstraintsFUCHSIA`] was
///called.
///The value of [`sysmem_color_space_index`] will be an index to one of the
///color spaces provided in the
///[`ImageFormatConstraintsInfoFUCHSIA::color_spaces`] array.The implementation must have
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BufferCollectionPropertiesFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *mut BaseOutStructure<'lt>,
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
impl<'lt> Default for BufferCollectionPropertiesFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            memory_type_bits: 0,
            buffer_count: 0,
            create_info_index: 0,
            sysmem_pixel_format: 0,
            format_features: Default::default(),
            sysmem_color_space_index: Default::default(),
            sampler_ycbcr_conversion_components: Default::default(),
            suggested_ycbcr_model: Default::default(),
            suggested_ycbcr_range: Default::default(),
            suggested_x_chroma_offset: Default::default(),
            suggested_y_chroma_offset: Default::default(),
        }
    }
}
impl<'lt> BufferCollectionPropertiesFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::memory_type_bits`]
    pub fn memory_type_bits_raw(&self) -> u32 {
        self.memory_type_bits
    }
    ///Gets the raw value of [`Self::buffer_count`]
    pub fn buffer_count_raw(&self) -> u32 {
        self.buffer_count
    }
    ///Gets the raw value of [`Self::create_info_index`]
    pub fn create_info_index_raw(&self) -> u32 {
        self.create_info_index
    }
    ///Gets the raw value of [`Self::sysmem_pixel_format`]
    pub fn sysmem_pixel_format_raw(&self) -> u64 {
        self.sysmem_pixel_format
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::memory_type_bits`]
    pub fn set_memory_type_bits_raw(&mut self, value: u32) -> &mut Self {
        self.memory_type_bits = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_count`]
    pub fn set_buffer_count_raw(&mut self, value: u32) -> &mut Self {
        self.buffer_count = value;
        self
    }
    ///Sets the raw value of [`Self::create_info_index`]
    pub fn set_create_info_index_raw(&mut self, value: u32) -> &mut Self {
        self.create_info_index = value;
        self
    }
    ///Sets the raw value of [`Self::sysmem_pixel_format`]
    pub fn set_sysmem_pixel_format_raw(&mut self, value: u64) -> &mut Self {
        self.sysmem_pixel_format = value;
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
    ///Gets the value of [`Self::memory_type_bits`]
    pub fn memory_type_bits(&self) -> u32 {
        self.memory_type_bits
    }
    ///Gets the value of [`Self::buffer_count`]
    pub fn buffer_count(&self) -> u32 {
        self.buffer_count
    }
    ///Gets the value of [`Self::create_info_index`]
    pub fn create_info_index(&self) -> u32 {
        self.create_info_index
    }
    ///Gets the value of [`Self::sysmem_pixel_format`]
    pub fn sysmem_pixel_format(&self) -> u64 {
        self.sysmem_pixel_format
    }
    ///Gets the value of [`Self::format_features`]
    pub fn format_features(&self) -> FormatFeatureFlags {
        self.format_features
    }
    ///Gets the value of [`Self::sysmem_color_space_index`]
    pub fn sysmem_color_space_index(&self) -> SysmemColorSpaceFUCHSIA<'lt> {
        self.sysmem_color_space_index
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
    ///Gets a mutable reference to the value of [`Self::memory_type_bits`]
    pub fn memory_type_bits_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::buffer_count`]
    pub fn buffer_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::create_info_index`]
    pub fn create_info_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::sysmem_pixel_format`]
    pub fn sysmem_pixel_format_mut(&mut self) -> &mut u64 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::format_features`]
    pub fn format_features_mut(&mut self) -> &mut FormatFeatureFlags {
        &mut self.format_features
    }
    ///Gets a mutable reference to the value of [`Self::sysmem_color_space_index`]
    pub fn sysmem_color_space_index_mut(&mut self) -> &mut SysmemColorSpaceFUCHSIA<'lt> {
        &mut self.sysmem_color_space_index
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
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::memory_type_bits`]
    pub fn set_memory_type_bits(&mut self, value: u32) -> &mut Self {
        self.memory_type_bits = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_count`]
    pub fn set_buffer_count(&mut self, value: u32) -> &mut Self {
        self.buffer_count = value;
        self
    }
    ///Sets the raw value of [`Self::create_info_index`]
    pub fn set_create_info_index(&mut self, value: u32) -> &mut Self {
        self.create_info_index = value;
        self
    }
    ///Sets the raw value of [`Self::sysmem_pixel_format`]
    pub fn set_sysmem_pixel_format(&mut self, value: u64) -> &mut Self {
        self.sysmem_pixel_format = value;
        self
    }
    ///Sets the raw value of [`Self::format_features`]
    pub fn set_format_features(&mut self, value: crate::vulkan1_0::FormatFeatureFlags) -> &mut Self {
        self.format_features = value;
        self
    }
    ///Sets the raw value of [`Self::sysmem_color_space_index`]
    pub fn set_sysmem_color_space_index(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA<'lt>,
    ) -> &mut Self {
        self.sysmem_color_space_index = value;
        self
    }
    ///Sets the raw value of [`Self::sampler_ycbcr_conversion_components`]
    pub fn set_sampler_ycbcr_conversion_components(&mut self, value: crate::vulkan1_0::ComponentMapping) -> &mut Self {
        self.sampler_ycbcr_conversion_components = value;
        self
    }
    ///Sets the raw value of [`Self::suggested_ycbcr_model`]
    pub fn set_suggested_ycbcr_model(&mut self, value: crate::vulkan1_1::SamplerYcbcrModelConversion) -> &mut Self {
        self.suggested_ycbcr_model = value;
        self
    }
    ///Sets the raw value of [`Self::suggested_ycbcr_range`]
    pub fn set_suggested_ycbcr_range(&mut self, value: crate::vulkan1_1::SamplerYcbcrRange) -> &mut Self {
        self.suggested_ycbcr_range = value;
        self
    }
    ///Sets the raw value of [`Self::suggested_x_chroma_offset`]
    pub fn set_suggested_x_chroma_offset(&mut self, value: crate::vulkan1_1::ChromaLocation) -> &mut Self {
        self.suggested_x_chroma_offset = value;
        self
    }
    ///Sets the raw value of [`Self::suggested_y_chroma_offset`]
    pub fn set_suggested_y_chroma_offset(&mut self, value: crate::vulkan1_1::ChromaLocation) -> &mut Self {
        self.suggested_y_chroma_offset = value;
        self
    }
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BufferConstraintsInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *const BaseInStructure<'lt>,
    ///No documentation found
    create_info: BufferCreateInfo<'lt>,
    ///[`required_format_features`] bitmask of [`FormatFeatureFlagBits`]
    ///required features of the buffers in the buffer collection
    required_format_features: FormatFeatureFlags,
    ///[`buffer_collection_constraints`] is used to supply parameters for the
    ///negotiation and allocation of the buffer collection
    buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'lt>,
}
impl<'lt> Default for BufferConstraintsInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            create_info: Default::default(),
            required_format_features: Default::default(),
            buffer_collection_constraints: Default::default(),
        }
    }
}
impl<'lt> BufferConstraintsInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::create_info`]
    pub fn create_info(&self) -> BufferCreateInfo<'lt> {
        self.create_info
    }
    ///Gets the value of [`Self::required_format_features`]
    pub fn required_format_features(&self) -> FormatFeatureFlags {
        self.required_format_features
    }
    ///Gets the value of [`Self::buffer_collection_constraints`]
    pub fn buffer_collection_constraints(&self) -> BufferCollectionConstraintsInfoFUCHSIA<'lt> {
        self.buffer_collection_constraints
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::create_info`]
    pub fn create_info_mut(&mut self) -> &mut BufferCreateInfo<'lt> {
        &mut self.create_info
    }
    ///Gets a mutable reference to the value of [`Self::required_format_features`]
    pub fn required_format_features_mut(&mut self) -> &mut FormatFeatureFlags {
        &mut self.required_format_features
    }
    ///Gets a mutable reference to the value of [`Self::buffer_collection_constraints`]
    pub fn buffer_collection_constraints_mut(&mut self) -> &mut BufferCollectionConstraintsInfoFUCHSIA<'lt> {
        &mut self.buffer_collection_constraints
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::create_info`]
    pub fn set_create_info(&mut self, value: crate::vulkan1_0::BufferCreateInfo<'lt>) -> &mut Self {
        self.create_info = value;
        self
    }
    ///Sets the raw value of [`Self::required_format_features`]
    pub fn set_required_format_features(&mut self, value: crate::vulkan1_0::FormatFeatureFlags) -> &mut Self {
        self.required_format_features = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_collection_constraints`]
    pub fn set_buffer_collection_constraints(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA<'lt>,
    ) -> &mut Self {
        self.buffer_collection_constraints = value;
        self
    }
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SysmemColorSpaceFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *const BaseInStructure<'lt>,
    ///[`color_space`] value of the Sysmem `ColorSpaceType`
    color_space: u32,
}
impl<'lt> Default for SysmemColorSpaceFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            color_space: 0,
        }
    }
}
impl<'lt> SysmemColorSpaceFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::color_space`]
    pub fn color_space_raw(&self) -> u32 {
        self.color_space
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::color_space`]
    pub fn set_color_space_raw(&mut self, value: u32) -> &mut Self {
        self.color_space = value;
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
    ///Gets the value of [`Self::color_space`]
    pub fn color_space(&self) -> u32 {
        self.color_space
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::color_space`]
    pub fn color_space_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::color_space`]
    pub fn set_color_space(&mut self, value: u32) -> &mut Self {
        self.color_space = value;
        self
    }
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
/// - [`color_space_count`] the element count of [`color_spaces`]
/// - [`color_spaces`] is a pointer to an array of [`SysmemColorSpaceFUCHSIA`] structs of size
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
/// - [`color_spaces`]**must** be a valid pointer to an array of [`color_space_count`] valid
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageFormatConstraintsInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *const BaseInStructure<'lt>,
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
    ///[`color_space_count`] the element count of [`color_spaces`]
    color_space_count: u32,
    ///[`color_spaces`] is a pointer to an array of
    ///[`SysmemColorSpaceFUCHSIA`] structs of size [`color_space_count`]
    color_spaces: *const SysmemColorSpaceFUCHSIA<'lt>,
}
impl<'lt> Default for ImageFormatConstraintsInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            image_create_info: Default::default(),
            required_format_features: Default::default(),
            flags: Default::default(),
            sysmem_pixel_format: 0,
            color_space_count: 0,
            color_spaces: std::ptr::null(),
        }
    }
}
impl<'lt> ImageFormatConstraintsInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::sysmem_pixel_format`]
    pub fn sysmem_pixel_format_raw(&self) -> u64 {
        self.sysmem_pixel_format
    }
    ///Gets the raw value of [`Self::color_space_count`]
    pub fn color_space_count_raw(&self) -> u32 {
        self.color_space_count
    }
    ///Gets the raw value of [`Self::color_spaces`]
    pub fn color_spaces_raw(&self) -> *const SysmemColorSpaceFUCHSIA<'lt> {
        self.color_spaces
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::sysmem_pixel_format`]
    pub fn set_sysmem_pixel_format_raw(&mut self, value: u64) -> &mut Self {
        self.sysmem_pixel_format = value;
        self
    }
    ///Sets the raw value of [`Self::color_space_count`]
    pub fn set_color_space_count_raw(&mut self, value: u32) -> &mut Self {
        self.color_space_count = value;
        self
    }
    ///Sets the raw value of [`Self::color_spaces`]
    pub fn set_color_spaces_raw(&mut self, value: *const SysmemColorSpaceFUCHSIA<'lt>) -> &mut Self {
        self.color_spaces = value;
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
    ///Gets the value of [`Self::image_create_info`]
    pub fn image_create_info(&self) -> ImageCreateInfo<'lt> {
        self.image_create_info
    }
    ///Gets the value of [`Self::required_format_features`]
    pub fn required_format_features(&self) -> FormatFeatureFlags {
        self.required_format_features
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> ImageFormatConstraintsFlagsFUCHSIA {
        self.flags
    }
    ///Gets the value of [`Self::sysmem_pixel_format`]
    pub fn sysmem_pixel_format(&self) -> u64 {
        self.sysmem_pixel_format
    }
    ///Gets the value of [`Self::color_space_count`]
    pub fn color_space_count(&self) -> u32 {
        self.color_space_count
    }
    ///Gets the value of [`Self::color_spaces`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn color_spaces(&self) -> &[SysmemColorSpaceFUCHSIA<'lt>] {
        std::slice::from_raw_parts(self.color_spaces, self.color_space_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::image_create_info`]
    pub fn image_create_info_mut(&mut self) -> &mut ImageCreateInfo<'lt> {
        &mut self.image_create_info
    }
    ///Gets a mutable reference to the value of [`Self::required_format_features`]
    pub fn required_format_features_mut(&mut self) -> &mut FormatFeatureFlags {
        &mut self.required_format_features
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ImageFormatConstraintsFlagsFUCHSIA {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::sysmem_pixel_format`]
    pub fn sysmem_pixel_format_mut(&mut self) -> &mut u64 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::color_space_count`]
    pub fn color_space_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::image_create_info`]
    pub fn set_image_create_info(&mut self, value: crate::vulkan1_0::ImageCreateInfo<'lt>) -> &mut Self {
        self.image_create_info = value;
        self
    }
    ///Sets the raw value of [`Self::required_format_features`]
    pub fn set_required_format_features(&mut self, value: crate::vulkan1_0::FormatFeatureFlags) -> &mut Self {
        self.required_format_features = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::ImageFormatConstraintsFlagsFUCHSIA,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::sysmem_pixel_format`]
    pub fn set_sysmem_pixel_format(&mut self, value: u64) -> &mut Self {
        self.sysmem_pixel_format = value;
        self
    }
    ///Sets the raw value of [`Self::color_space_count`]
    pub fn set_color_space_count(&mut self, value: u32) -> &mut Self {
        self.color_space_count = value;
        self
    }
    ///Sets the raw value of [`Self::color_spaces`]
    pub fn set_color_spaces(
        &mut self,
        value: &'lt [crate::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.color_spaces = value.as_ptr();
        self.color_space_count = len_;
        self
    }
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
/// - [`format_constraints_count`] is the number of elements in [`format_constraints`].
/// - [`format_constraints`] is a pointer to an array of [`ImageFormatConstraintsInfoFUCHSIA`]
///   structures of size [`format_constraints_count`] that is used to further constrain buffer
///   collection format selection for image-based buffer collections.
/// - [`buffer_collection_constraints`] is a [`BufferCollectionConstraintsInfoFUCHSIA`] structure
///   used to supply parameters for the negotiation and allocation for buffer-based buffer
///   collections.
/// - [`flags`] is a [`ImageConstraintsInfoFlagBitsFUCHSIA`] value specifying hints about the type
///   of memory Sysmem should allocate for the buffer collection.
///# Description
///Valid Usage
/// - All elements of [`format_constraints`]**must** have at least one bit set in its
///   [`ImageFormatConstraintsInfoFUCHSIA::required_format_features`]
/// - If [`format_constraints`]`::imageCreateInfo`::`usage` contains `VK_IMAGE_USAGE_SAMPLED_BIT`,
///   then [`format_constraints`]`::requiredFormatFeatures`**must** contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`
/// - If [`format_constraints`]`::imageCreateInfo`::`usage` contains `VK_IMAGE_USAGE_STORAGE_BIT`,
///   then [`format_constraints`]`::requiredFormatFeatures`**must** contain
///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT`
/// - If [`format_constraints`]`::imageCreateInfo`::`usage` contains
///   `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`, then
///   [`format_constraints`]`::requiredFormatFeatures`**must** contain
///   `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
/// - If [`format_constraints`]`::imageCreateInfo`::`usage` contains
///   `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, then
///   [`format_constraints`]`::requiredFormatFeatures`**must** contain
///   `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`format_constraints`]`::imageCreateInfo`::`usage` contains
///   `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`, then
///   [`format_constraints`]`::requiredFormatFeatures`**must** contain at least one of
///   `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT` or `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If the [`attachmentFragmentShadingRate` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate)
///   is enabled, and [`format_constraints`]`::imageCreateInfo`::`usage` contains
///   `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`, then
///   [`format_constraints`]`::requiredFormatFeatures`**must** contain
///   `VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
/// - [`format_constraints`]**must** be a valid pointer to an array of [`format_constraints_count`]
///   valid [`ImageFormatConstraintsInfoFUCHSIA`] structures
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageConstraintsInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`format_constraints_count`] is the number of elements in
    ///[`format_constraints`].
    format_constraints_count: u32,
    ///[`format_constraints`] is a pointer to an array of
    ///[`ImageFormatConstraintsInfoFUCHSIA`] structures of size
    ///[`format_constraints_count`] that is used to further constrain buffer
    ///collection format selection for image-based buffer collections.
    format_constraints: *const ImageFormatConstraintsInfoFUCHSIA<'lt>,
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
impl<'lt> Default for ImageConstraintsInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            format_constraints_count: 0,
            format_constraints: std::ptr::null(),
            buffer_collection_constraints: Default::default(),
            flags: Default::default(),
        }
    }
}
impl<'lt> ImageConstraintsInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::format_constraints_count`]
    pub fn format_constraints_count_raw(&self) -> u32 {
        self.format_constraints_count
    }
    ///Gets the raw value of [`Self::format_constraints`]
    pub fn format_constraints_raw(&self) -> *const ImageFormatConstraintsInfoFUCHSIA<'lt> {
        self.format_constraints
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::format_constraints_count`]
    pub fn set_format_constraints_count_raw(&mut self, value: u32) -> &mut Self {
        self.format_constraints_count = value;
        self
    }
    ///Sets the raw value of [`Self::format_constraints`]
    pub fn set_format_constraints_raw(&mut self, value: *const ImageFormatConstraintsInfoFUCHSIA<'lt>) -> &mut Self {
        self.format_constraints = value;
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
    ///Gets the value of [`Self::format_constraints_count`]
    pub fn format_constraints_count(&self) -> u32 {
        self.format_constraints_count
    }
    ///Gets the value of [`Self::format_constraints`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn format_constraints(&self) -> &[ImageFormatConstraintsInfoFUCHSIA<'lt>] {
        std::slice::from_raw_parts(self.format_constraints, self.format_constraints_count as usize)
    }
    ///Gets the value of [`Self::buffer_collection_constraints`]
    pub fn buffer_collection_constraints(&self) -> BufferCollectionConstraintsInfoFUCHSIA<'lt> {
        self.buffer_collection_constraints
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> ImageConstraintsInfoFlagsFUCHSIA {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::format_constraints_count`]
    pub fn format_constraints_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::buffer_collection_constraints`]
    pub fn buffer_collection_constraints_mut(&mut self) -> &mut BufferCollectionConstraintsInfoFUCHSIA<'lt> {
        &mut self.buffer_collection_constraints
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ImageConstraintsInfoFlagsFUCHSIA {
        &mut self.flags
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::format_constraints_count`]
    pub fn set_format_constraints_count(&mut self, value: u32) -> &mut Self {
        self.format_constraints_count = value;
        self
    }
    ///Sets the raw value of [`Self::format_constraints`]
    pub fn set_format_constraints(
        &mut self,
        value: &'lt [crate::extensions::fuchsia_buffer_collection::ImageFormatConstraintsInfoFUCHSIA<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.format_constraints = value.as_ptr();
        self.format_constraints_count = len_;
        self
    }
    ///Sets the raw value of [`Self::buffer_collection_constraints`]
    pub fn set_buffer_collection_constraints(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA<'lt>,
    ) -> &mut Self {
        self.buffer_collection_constraints = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFlagsFUCHSIA,
    ) -> &mut Self {
        self.flags = value;
        self
    }
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BufferCollectionConstraintsInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *const BaseInStructure<'lt>,
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
impl<'lt> Default for BufferCollectionConstraintsInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            min_buffer_count: 0,
            max_buffer_count: 0,
            min_buffer_count_for_camping: 0,
            min_buffer_count_for_dedicated_slack: 0,
            min_buffer_count_for_shared_slack: 0,
        }
    }
}
impl<'lt> BufferCollectionConstraintsInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::min_buffer_count`]
    pub fn min_buffer_count_raw(&self) -> u32 {
        self.min_buffer_count
    }
    ///Gets the raw value of [`Self::max_buffer_count`]
    pub fn max_buffer_count_raw(&self) -> u32 {
        self.max_buffer_count
    }
    ///Gets the raw value of [`Self::min_buffer_count_for_camping`]
    pub fn min_buffer_count_for_camping_raw(&self) -> u32 {
        self.min_buffer_count_for_camping
    }
    ///Gets the raw value of [`Self::min_buffer_count_for_dedicated_slack`]
    pub fn min_buffer_count_for_dedicated_slack_raw(&self) -> u32 {
        self.min_buffer_count_for_dedicated_slack
    }
    ///Gets the raw value of [`Self::min_buffer_count_for_shared_slack`]
    pub fn min_buffer_count_for_shared_slack_raw(&self) -> u32 {
        self.min_buffer_count_for_shared_slack
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::min_buffer_count`]
    pub fn set_min_buffer_count_raw(&mut self, value: u32) -> &mut Self {
        self.min_buffer_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_buffer_count`]
    pub fn set_max_buffer_count_raw(&mut self, value: u32) -> &mut Self {
        self.max_buffer_count = value;
        self
    }
    ///Sets the raw value of [`Self::min_buffer_count_for_camping`]
    pub fn set_min_buffer_count_for_camping_raw(&mut self, value: u32) -> &mut Self {
        self.min_buffer_count_for_camping = value;
        self
    }
    ///Sets the raw value of [`Self::min_buffer_count_for_dedicated_slack`]
    pub fn set_min_buffer_count_for_dedicated_slack_raw(&mut self, value: u32) -> &mut Self {
        self.min_buffer_count_for_dedicated_slack = value;
        self
    }
    ///Sets the raw value of [`Self::min_buffer_count_for_shared_slack`]
    pub fn set_min_buffer_count_for_shared_slack_raw(&mut self, value: u32) -> &mut Self {
        self.min_buffer_count_for_shared_slack = value;
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
    ///Gets the value of [`Self::min_buffer_count`]
    pub fn min_buffer_count(&self) -> u32 {
        self.min_buffer_count
    }
    ///Gets the value of [`Self::max_buffer_count`]
    pub fn max_buffer_count(&self) -> u32 {
        self.max_buffer_count
    }
    ///Gets the value of [`Self::min_buffer_count_for_camping`]
    pub fn min_buffer_count_for_camping(&self) -> u32 {
        self.min_buffer_count_for_camping
    }
    ///Gets the value of [`Self::min_buffer_count_for_dedicated_slack`]
    pub fn min_buffer_count_for_dedicated_slack(&self) -> u32 {
        self.min_buffer_count_for_dedicated_slack
    }
    ///Gets the value of [`Self::min_buffer_count_for_shared_slack`]
    pub fn min_buffer_count_for_shared_slack(&self) -> u32 {
        self.min_buffer_count_for_shared_slack
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::min_buffer_count`]
    pub fn min_buffer_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_buffer_count`]
    pub fn max_buffer_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::min_buffer_count_for_camping`]
    pub fn min_buffer_count_for_camping_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::min_buffer_count_for_dedicated_slack`]
    pub fn min_buffer_count_for_dedicated_slack_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::min_buffer_count_for_shared_slack`]
    pub fn min_buffer_count_for_shared_slack_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::min_buffer_count`]
    pub fn set_min_buffer_count(&mut self, value: u32) -> &mut Self {
        self.min_buffer_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_buffer_count`]
    pub fn set_max_buffer_count(&mut self, value: u32) -> &mut Self {
        self.max_buffer_count = value;
        self
    }
    ///Sets the raw value of [`Self::min_buffer_count_for_camping`]
    pub fn set_min_buffer_count_for_camping(&mut self, value: u32) -> &mut Self {
        self.min_buffer_count_for_camping = value;
        self
    }
    ///Sets the raw value of [`Self::min_buffer_count_for_dedicated_slack`]
    pub fn set_min_buffer_count_for_dedicated_slack(&mut self, value: u32) -> &mut Self {
        self.min_buffer_count_for_dedicated_slack = value;
        self
    }
    ///Sets the raw value of [`Self::min_buffer_count_for_shared_slack`]
    pub fn set_min_buffer_count_for_shared_slack(&mut self, value: u32) -> &mut Self {
        self.min_buffer_count_for_shared_slack = value;
        self
    }
}
///[VkBufferCollectionFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionFUCHSIA.html) - Opaque handle to a buffer collection object
///# C Specifications
///Fuchsia’s FIDL-based Sysmem service interoperates with Vulkan via the
///`[`VK_FUCHSIA_buffer_collection`]` extension.A buffer collection is a set of one or more buffers
/// which were allocated
///together as a group and which all have the same properties.
///These properties describe the buffers' internal representation, such as its
///dimensions and memory layout.
///This ensures that all of the buffers can be used interchangeably by tasks
///that require swapping among multiple buffers, such as double-buffered
///graphics rendering.On Fuchsia, the Sysmem service uses buffer collections as a core construct
///in its design.Buffer collections are represented by [`BufferCollectionFUCHSIA`]
///handles:
///```c
///// Provided by VK_FUCHSIA_buffer_collection
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkBufferCollectionFUCHSIA)
///```
///# Related
/// - [`VK_FUCHSIA_buffer_collection`]
/// - [`BufferCollectionBufferCreateInfoFUCHSIA`]
/// - [`BufferCollectionImageCreateInfoFUCHSIA`]
/// - [`ImportMemoryBufferCollectionFUCHSIA`]
/// - [`CreateBufferCollectionFUCHSIA`]
/// - [`DestroyBufferCollectionFUCHSIA`]
/// - [`GetBufferCollectionPropertiesFUCHSIA`]
/// - [`SetBufferCollectionBufferConstraintsFUCHSIA`]
/// - [`SetBufferCollectionImageConstraintsFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct BufferCollectionFUCHSIA(pub u64);
impl BufferCollectionFUCHSIA {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub const fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for BufferCollectionFUCHSIA {}
impl Default for BufferCollectionFUCHSIA {
    fn default() -> Self {
        Self::default()
    }
}
impl std::fmt::Pointer for BufferCollectionFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
impl std::fmt::Debug for BufferCollectionFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
