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
///If the [`AllocateMemory`] command succeeds, the implementation  **must**
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImportAndroidHardwareBufferInfoANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`buffer`] is the Android hardware buffer to import.
    buffer: *mut AHardwareBuffer,
}
impl<'lt> Default for ImportAndroidHardwareBufferInfoANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    pub fn buffer_raw(&self) -> &*mut AHardwareBuffer {
        &self.buffer
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::buffer`]
    pub fn set_buffer_raw(&mut self, value: *mut AHardwareBuffer) -> &mut Self {
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
    ///Sets the raw value of [`Self::buffer`]
    pub fn set_buffer(
        &mut self,
        value: &'lt mut crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer,
    ) -> &mut Self {
        self.buffer = value as *mut _;
        self
    }
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AndroidHardwareBufferUsageANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`android_hardware_buffer_usage`] returns the Android hardware buffer
    ///usage flags.
    android_hardware_buffer_usage: u64,
}
impl<'lt> Default for AndroidHardwareBufferUsageANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            android_hardware_buffer_usage: 0,
        }
    }
}
impl<'lt> AndroidHardwareBufferUsageANDROID<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
        &mut getter
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
    ///Sets the raw value of [`Self::android_hardware_buffer_usage`]
    pub fn set_android_hardware_buffer_usage(&mut self, value: u64) -> &mut Self {
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AndroidHardwareBufferPropertiesANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`allocation_size`] is the size of the external memory
    allocation_size: DeviceSize,
    ///[`memory_type_bits`] is a bitmask containing one bit set for every
    ///memory type which the specified Android hardware buffer  **can**  be imported
    ///as.
    memory_type_bits: u32,
}
impl<'lt> Default for AndroidHardwareBufferPropertiesANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            allocation_size: Default::default(),
            memory_type_bits: 0,
        }
    }
}
impl<'lt> AndroidHardwareBufferPropertiesANDROID<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
        &mut getter
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
    ///Sets the raw value of [`Self::allocation_size`]
    pub fn set_allocation_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.allocation_size = value;
        self
    }
    ///Sets the raw value of [`Self::memory_type_bits`]
    pub fn set_memory_type_bits(&mut self, value: u32) -> &mut Self {
        self.memory_type_bits = value;
        self
    }
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`memory`] is the memory object from which the Android hardware buffer
    ///will be exported.
    memory: DeviceMemory,
}
impl<'lt> Default for MemoryGetAndroidHardwareBufferInfoANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    ///Sets the raw value of [`Self::memory`]
    pub fn set_memory(&mut self, value: crate::vulkan1_0::DeviceMemory) -> &mut Self {
        self.memory = value;
        self
    }
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`format`] is the Vulkan format corresponding to the Android hardware
    ///buffer’s format, or `VK_FORMAT_UNDEFINED` if there is not an
    ///equivalent Vulkan format.
    format: Format,
    ///[`external_format`] is an implementation-defined external format
    ///identifier for use with [`ExternalFormatANDROID`].
    ///It  **must**  not be zero.
    external_format: u64,
    ///[`format_features`] describes the capabilities of this external format
    ///when used with an image bound to memory imported from `buffer`.
    format_features: FormatFeatureFlags,
    ///[`sampler_ycbcr_conversion_components`] is the component swizzle that
    /// **should**  be used in [`SamplerYcbcrConversionCreateInfo`].
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
impl<'lt> Default for AndroidHardwareBufferFormatPropertiesANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
        &mut getter
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
    ///Sets the raw value of [`Self::format`]
    pub fn set_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.format = value;
        self
    }
    ///Sets the raw value of [`Self::external_format`]
    pub fn set_external_format(&mut self, value: u64) -> &mut Self {
        self.external_format = value;
        self
    }
    ///Sets the raw value of [`Self::format_features`]
    pub fn set_format_features(&mut self, value: crate::vulkan1_0::FormatFeatureFlags) -> &mut Self {
        self.format_features = value;
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
///   [`GetAndroidHardwareBufferPropertiesANDROID`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExternalFormatANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`external_format`] is an implementation-defined identifier for the
    ///external format
    external_format: u64,
}
impl<'lt> Default for ExternalFormatANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            external_format: 0,
        }
    }
}
impl<'lt> ExternalFormatANDROID<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
        &mut getter
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
    ///Sets the raw value of [`Self::external_format`]
    pub fn set_external_format(&mut self, value: u64) -> &mut Self {
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AndroidHardwareBufferFormatProperties2ANDROID<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`format`] is the Vulkan format corresponding to the Android hardware
    ///buffer’s format, or `VK_FORMAT_UNDEFINED` if there is not an
    ///equivalent Vulkan format.
    format: Format,
    ///[`external_format`] is an implementation-defined external format
    ///identifier for use with [`ExternalFormatANDROID`].
    ///It  **must**  not be zero.
    external_format: u64,
    ///[`format_features`] describes the capabilities of this external format
    ///when used with an image bound to memory imported from `buffer`.
    format_features: FormatFeatureFlags2,
    ///[`sampler_ycbcr_conversion_components`] is the component swizzle that
    /// **should**  be used in [`SamplerYcbcrConversionCreateInfo`].
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
impl<'lt> Default for AndroidHardwareBufferFormatProperties2ANDROID<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
        &mut getter
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
    ///Sets the raw value of [`Self::format`]
    pub fn set_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.format = value;
        self
    }
    ///Sets the raw value of [`Self::external_format`]
    pub fn set_external_format(&mut self, value: u64) -> &mut Self {
        self.external_format = value;
        self
    }
    ///Sets the raw value of [`Self::format_features`]
    pub fn set_format_features(&mut self, value: crate::vulkan1_3::FormatFeatureFlags2) -> &mut Self {
        self.format_features = value;
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
