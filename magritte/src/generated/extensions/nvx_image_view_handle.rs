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
///Valid Usage
/// - [`descriptor_type`]**must** be `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`,
///   `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`
/// - [`sampler`]**must** be a valid [`Sampler`] if [`descriptor_type`] is
///   `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`
/// - If descriptorType is `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` or
///   `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, the image that [`image_view`] was created from
///   **must** have been created with the `VK_IMAGE_USAGE_SAMPLED_BIT` usage bit set
/// - If descriptorType is `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, the image that [`image_view`] was
///   created from **must** have been created with the `VK_IMAGE_USAGE_STORAGE_BIT` usage bit set
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX`
/// - [`p_next`]**must** be `NULL`
/// - [`image_view`]**must** be a valid [`ImageView`] handle
/// - [`descriptor_type`]**must** be a valid [`DescriptorType`] value
/// - If [`sampler`] is not [`crate::utils::Handle::null`], [`sampler`]**must** be a valid
///   [`Sampler`] handle
/// - Both of [`image_view`], and [`sampler`] that are valid handles of non-ignored parameters
///   **must** have been created, allocated, or retrieved from the same [`Device`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImageViewHandleInfoNVX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`image_view`] is the image view to query.
    image_view: ImageView,
    ///[`descriptor_type`] is the type of descriptor for which to query a
    ///handle.
    descriptor_type: DescriptorType,
    ///[`sampler`] is the sampler to combine with the image view when
    ///generating the handle.
    sampler: Sampler,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImageViewAddressPropertiesNVX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`device_address`] is the device address of the image view.
    device_address: DeviceAddress,
    ///[`size`] is the size in bytes of the image view device memory.
    size: DeviceSize,
}
