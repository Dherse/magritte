use crate::vulkan1_0::{
    BaseInStructure, BaseOutStructure, DescriptorType, DeviceAddress, DeviceSize, ImageView, Sampler, StructureType,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION")]
pub const NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME")]
pub const NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NVX_image_view_handle");
///[VkImageViewHandleInfoNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewHandleInfoNVX.html) - Structure specifying the image view for handle queries
///# C Specifications
///The [`ImageViewHandleInfoNVX`] structure is defined as:
///```c
///// Provided by VK_NVX_image_view_handle
///typedef struct VkImageViewHandleInfoNVX {
///    VkStructureType     sType;
///    const void*         pNext;
///    VkImageView         imageView;
///    VkDescriptorType    descriptorType;
///    VkSampler           sampler;
///} VkImageViewHandleInfoNVX;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image_view`] is the image view to query.
/// - [`descriptor_type`] is the type of descriptor for which to query a handle.
/// - [`sampler`] is the sampler to combine with the image view when generating the handle.
///# Description
///## Valid Usage
/// - [`descriptor_type`] **must**  be `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`,
///   `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`
/// - [`sampler`] **must**  be a valid [`Sampler`] if [`descriptor_type`] is
///   `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`
/// - If descriptorType is `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` or
///   `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, the image that [`image_view`] was created from
///   **must**  have been created with the `VK_IMAGE_USAGE_SAMPLED_BIT` usage bit set
/// - If descriptorType is `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, the image that [`image_view`] was
///   created from  **must**  have been created with the `VK_IMAGE_USAGE_STORAGE_BIT` usage bit set
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX`
/// - [`p_next`] **must**  be `NULL`
/// - [`image_view`] **must**  be a valid [`ImageView`] handle
/// - [`descriptor_type`] **must**  be a valid [`DescriptorType`] value
/// - If [`sampler`] is not [`crate::utils::Handle::null`], [`sampler`] **must**  be a valid
///   [`Sampler`] handle
/// - Both of [`image_view`], and [`sampler`] that are valid handles of non-ignored parameters
///   **must**  have been created, allocated, or retrieved from the same [`Device`]
///# Related
/// - [`VK_NVX_image_view_handle`]
/// - [`DescriptorType`]
/// - [`ImageView`]
/// - [`Sampler`]
/// - [`StructureType`]
/// - [`GetImageViewHandleNVX`]
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
pub struct ImageViewHandleInfoNVX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`image_view`] is the image view to query.
    image_view: ImageView,
    ///[`descriptor_type`] is the type of descriptor for which to query a
    ///handle.
    descriptor_type: DescriptorType,
    ///[`sampler`] is the sampler to combine with the image view when
    ///generating the handle.
    sampler: Sampler,
}
impl<'lt> Default for ImageViewHandleInfoNVX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            image_view: Default::default(),
            descriptor_type: Default::default(),
            sampler: Default::default(),
        }
    }
}
impl<'lt> ImageViewHandleInfoNVX<'lt> {
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
    ///Gets the value of [`Self::image_view`]
    pub fn image_view(&self) -> ImageView {
        self.image_view
    }
    ///Gets the value of [`Self::descriptor_type`]
    pub fn descriptor_type(&self) -> DescriptorType {
        self.descriptor_type
    }
    ///Gets the value of [`Self::sampler`]
    pub fn sampler(&self) -> Sampler {
        self.sampler
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::image_view`]
    pub fn image_view_mut(&mut self) -> &mut ImageView {
        &mut self.image_view
    }
    ///Gets a mutable reference to the value of [`Self::descriptor_type`]
    pub fn descriptor_type_mut(&mut self) -> &mut DescriptorType {
        &mut self.descriptor_type
    }
    ///Gets a mutable reference to the value of [`Self::sampler`]
    pub fn sampler_mut(&mut self) -> &mut Sampler {
        &mut self.sampler
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
    ///Sets the raw value of [`Self::image_view`]
    pub fn set_image_view(&mut self, value: crate::vulkan1_0::ImageView) -> &mut Self {
        self.image_view = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_type`]
    pub fn set_descriptor_type(&mut self, value: crate::vulkan1_0::DescriptorType) -> &mut Self {
        self.descriptor_type = value;
        self
    }
    ///Sets the raw value of [`Self::sampler`]
    pub fn set_sampler(&mut self, value: crate::vulkan1_0::Sampler) -> &mut Self {
        self.sampler = value;
        self
    }
}
///[VkImageViewAddressPropertiesNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewAddressPropertiesNVX.html) - Structure specifying the image view for handle queries
///# C Specifications
///The [`ImageViewAddressPropertiesNVX`] structure is defined as:
///```c
///// Provided by VK_NVX_image_view_handle
///typedef struct VkImageViewAddressPropertiesNVX {
///    VkStructureType    sType;
///    void*              pNext;
///    VkDeviceAddress    deviceAddress;
///    VkDeviceSize       size;
///} VkImageViewAddressPropertiesNVX;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`device_address`] is the device address of the image view.
/// - [`size`] is the size in bytes of the image view device memory.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_NVX_image_view_handle`]
/// - [`DeviceAddress`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`GetImageViewAddressNVX`]
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
pub struct ImageViewAddressPropertiesNVX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`device_address`] is the device address of the image view.
    device_address: DeviceAddress,
    ///[`size`] is the size in bytes of the image view device memory.
    size: DeviceSize,
}
impl<'lt> Default for ImageViewAddressPropertiesNVX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            device_address: Default::default(),
            size: Default::default(),
        }
    }
}
impl<'lt> ImageViewAddressPropertiesNVX<'lt> {
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
    ///Gets the value of [`Self::device_address`]
    pub fn device_address(&self) -> DeviceAddress {
        self.device_address
    }
    ///Gets the value of [`Self::size`]
    pub fn size(&self) -> DeviceSize {
        self.size
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
    ///Gets a mutable reference to the value of [`Self::device_address`]
    pub fn device_address_mut(&mut self) -> &mut DeviceAddress {
        &mut self.device_address
    }
    ///Gets a mutable reference to the value of [`Self::size`]
    pub fn size_mut(&mut self) -> &mut DeviceSize {
        &mut self.size
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
    ///Sets the raw value of [`Self::device_address`]
    pub fn set_device_address(&mut self, value: crate::vulkan1_0::DeviceAddress) -> &mut Self {
        self.device_address = value;
        self
    }
    ///Sets the raw value of [`Self::size`]
    pub fn set_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.size = value;
        self
    }
}
