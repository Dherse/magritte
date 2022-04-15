//![VK_NVX_image_view_handle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NVX_image_view_handle.html) - device extension
//!# Description
//!This extension allows applications to query an opaque handle from an image
//!view for use as a sampled image or storage image.
//!This provides no direct functionality itself.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Eric Werness [ewerness-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NVX_image_view_handle]
//!   @ewerness-nv%0A<<Here describe the issue or question you have about the
//!   VK_NVX_image_view_handle extension>>)
//!# New functions & commands
//! - [`get_image_view_address_nvx`]
//! - [`get_image_view_handle_nvx`]
//!# New structures
//! - [`ImageViewAddressPropertiesNVX`]
//! - [`ImageViewHandleInfoNVX`]
//!# New constants
//! - [`NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME`]
//! - [`NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX`  -
//!   `VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX`
//!# Version History
//! - Revision 2, 2020-04-03 (Piers Daniell)  - Add [`get_image_view_address_nvx`]
//! - Revision 1, 2018-12-07 (Eric Werness)  - Internal revisions
//!# Other info
//! * 2020-04-03
//! * - Eric Werness, NVIDIA  - Jeff Bolz, NVIDIA  - Daniel Koch, NVIDIA
//!# Related
//! - [`ImageViewAddressPropertiesNVX`]
//! - [`ImageViewHandleInfoNVX`]
//! - [`get_image_view_address_nvx`]
//! - [`get_image_view_handle_nvx`]
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
        BaseInStructure, BaseOutStructure, DescriptorType, Device, DeviceAddress, DeviceSize, ImageView, Sampler,
        StructureType, VulkanResultCodes,
    },
    AsRaw, Unique, VulkanResult,
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
///[vkGetImageViewHandleNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewHandleNVX.html) - Get the handle for an image view for a specific descriptor type
///# C Specifications
///To get the handle for an image view, call:
///```c
///// Provided by VK_NVX_image_view_handle
///uint32_t vkGetImageViewHandleNVX(
///    VkDevice                                    device,
///    const VkImageViewHandleInfoNVX*             pInfo);
///```
/// # Parameters
/// - [`device`] is the logical device that owns the image view.
/// - [`p_info`] describes the image view to query and type of handle.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_info`] **must**  be a valid pointer to a valid [`ImageViewHandleInfoNVX`] structure
/// # Related
/// - [`nvx_image_view_handle`]
/// - [`Device`]
/// - [`ImageViewHandleInfoNVX`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetImageViewHandleNVX")]
pub type FNGetImageViewHandleNvx =
    Option<for<'lt> unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX<'lt>) -> u32>;
///[vkGetImageViewAddressNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewAddressNVX.html) - Get the device address of an image view
///# C Specifications
///To get the device address for an image view, call:
///```c
///// Provided by VK_NVX_image_view_handle
///VkResult vkGetImageViewAddressNVX(
///    VkDevice                                    device,
///    VkImageView                                 imageView,
///    VkImageViewAddressPropertiesNVX*            pProperties);
///```
/// # Parameters
/// - [`device`] is the logical device that owns the image view.
/// - [`image_view`] is a handle to the image view.
/// - [`p_properties`] contains the device address and size when the call returns.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`image_view`] **must**  be a valid [`ImageView`] handle
/// - [`p_properties`] **must**  be a valid pointer to a [`ImageViewAddressPropertiesNVX`] structure
/// - [`image_view`] **must**  have been created, allocated, or retrieved from [`device`]
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_UNKNOWN`
/// # Related
/// - [`nvx_image_view_handle`]
/// - [`Device`]
/// - [`ImageView`]
/// - [`ImageViewAddressPropertiesNVX`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetImageViewAddressNVX")]
pub type FNGetImageViewAddressNvx = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        image_view: ImageView,
        p_properties: *mut ImageViewAddressPropertiesNVX<'lt>,
    ) -> VulkanResultCodes,
>;
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image_view`] is the image view to query.
/// - [`descriptor_type`] is the type of descriptor for which to query a handle.
/// - [`sampler`] is the sampler to combine with the image view when generating the handle.
/// # Description
/// ## Valid Usage
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
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX`
/// - [`p_next`] **must**  be `NULL`
/// - [`image_view`] **must**  be a valid [`ImageView`] handle
/// - [`descriptor_type`] **must**  be a valid [`DescriptorType`] value
/// - If [`sampler`] is not [`crate::Handle::null`], [`sampler`] **must**  be a valid [`Sampler`]
///   handle
/// - Both of [`image_view`], and [`sampler`] that are valid handles of non-ignored parameters
///   **must**  have been created, allocated, or retrieved from the same [`Device`]
/// # Related
/// - [`nvx_image_view_handle`]
/// - [`DescriptorType`]
/// - [`ImageView`]
/// - [`Sampler`]
/// - [`StructureType`]
/// - [`get_image_view_handle_nvx`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageViewHandleInfoNVX")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ImageViewHandleInfoNVX<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`image_view`] is the image view to query.
    pub image_view: ImageView,
    ///[`descriptor_type`] is the type of descriptor for which to query a
    ///handle.
    pub descriptor_type: DescriptorType,
    ///[`sampler`] is the sampler to combine with the image view when
    ///generating the handle.
    pub sampler: Sampler,
}
impl<'lt> Default for ImageViewHandleInfoNVX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMAGE_VIEW_HANDLE_INFO_NVX,
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
    ///Sets the value of [`Self::image_view`]
    pub fn set_image_view(mut self, value: crate::vulkan1_0::ImageView) -> Self {
        self.image_view = value;
        self
    }
    ///Sets the value of [`Self::descriptor_type`]
    pub fn set_descriptor_type(mut self, value: crate::vulkan1_0::DescriptorType) -> Self {
        self.descriptor_type = value;
        self
    }
    ///Sets the value of [`Self::sampler`]
    pub fn set_sampler(mut self, value: crate::vulkan1_0::Sampler) -> Self {
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`device_address`] is the device address of the image view.
/// - [`size`] is the size in bytes of the image view device memory.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX`
/// - [`p_next`] **must**  be `NULL`
/// # Related
/// - [`nvx_image_view_handle`]
/// - [`DeviceAddress`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`get_image_view_address_nvx`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageViewAddressPropertiesNVX")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ImageViewAddressPropertiesNVX<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`device_address`] is the device address of the image view.
    pub device_address: DeviceAddress,
    ///[`size`] is the size in bytes of the image view device memory.
    pub size: DeviceSize,
}
impl<'lt> Default for ImageViewAddressPropertiesNVX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX,
            p_next: std::ptr::null_mut(),
            device_address: Default::default(),
            size: Default::default(),
        }
    }
}
impl<'lt> ImageViewAddressPropertiesNVX<'lt> {
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
    ///Sets the value of [`Self::device_address`]
    pub fn set_device_address(mut self, value: crate::vulkan1_0::DeviceAddress) -> Self {
        self.device_address = value;
        self
    }
    ///Sets the value of [`Self::size`]
    pub fn set_size(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.size = value;
        self
    }
}
impl Device {
    ///[vkGetImageViewHandleNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewHandleNVX.html) - Get the handle for an image view for a specific descriptor type
    ///# C Specifications
    ///To get the handle for an image view, call:
    ///```c
    ///// Provided by VK_NVX_image_view_handle
    ///uint32_t vkGetImageViewHandleNVX(
    ///    VkDevice                                    device,
    ///    const VkImageViewHandleInfoNVX*             pInfo);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that owns the image view.
    /// - [`p_info`] describes the image view to query and type of handle.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_info`] **must**  be a valid pointer to a valid [`ImageViewHandleInfoNVX`] structure
    /// # Related
    /// - [`nvx_image_view_handle`]
    /// - [`Device`]
    /// - [`ImageViewHandleInfoNVX`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetImageViewHandleNVX")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_image_view_handle_nvx<'lt>(self: &Unique<Device>, p_info: &ImageViewHandleInfoNVX<'lt>) -> u32 {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nvx_image_view_handle()
            .and_then(|vtable| vtable.get_image_view_handle_nvx())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nvx_image_view_handle()
            .and_then(|vtable| vtable.get_image_view_handle_nvx())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_info as *const ImageViewHandleInfoNVX<'lt>);
        _return
    }
}
impl Device {
    ///[vkGetImageViewAddressNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewAddressNVX.html) - Get the device address of an image view
    ///# C Specifications
    ///To get the device address for an image view, call:
    ///```c
    ///// Provided by VK_NVX_image_view_handle
    ///VkResult vkGetImageViewAddressNVX(
    ///    VkDevice                                    device,
    ///    VkImageView                                 imageView,
    ///    VkImageViewAddressPropertiesNVX*            pProperties);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that owns the image view.
    /// - [`image_view`] is a handle to the image view.
    /// - [`p_properties`] contains the device address and size when the call returns.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`image_view`] **must**  be a valid [`ImageView`] handle
    /// - [`p_properties`] **must**  be a valid pointer to a [`ImageViewAddressPropertiesNVX`]
    ///   structure
    /// - [`image_view`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_UNKNOWN`
    /// # Related
    /// - [`nvx_image_view_handle`]
    /// - [`Device`]
    /// - [`ImageView`]
    /// - [`ImageViewAddressPropertiesNVX`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetImageViewAddressNVX")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_image_view_address_nvx<'lt>(
        self: &Unique<Device>,
        image_view: ImageView,
        p_properties: Option<ImageViewAddressPropertiesNVX<'lt>>,
    ) -> VulkanResult<ImageViewAddressPropertiesNVX<'lt>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nvx_image_view_handle()
            .and_then(|vtable| vtable.get_image_view_address_nvx())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nvx_image_view_handle()
            .and_then(|vtable| vtable.get_image_view_address_nvx())
            .unwrap_unchecked();
        let mut p_properties = p_properties.unwrap_or_default();
        let _return = _function(self.as_raw(), image_view, &mut p_properties);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, {
                p_properties.p_next = std::ptr::null_mut();
                p_properties
            }),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_NVX_image_view_handle`
pub struct DeviceNvxImageViewHandleVTable {
    ///See [`FNGetImageViewHandleNvx`] for more information.
    pub get_image_view_handle_nvx: FNGetImageViewHandleNvx,
    ///See [`FNGetImageViewAddressNvx`] for more information.
    pub get_image_view_address_nvx: FNGetImageViewAddressNvx,
}
impl DeviceNvxImageViewHandleVTable {
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
            get_image_view_handle_nvx: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetImageViewHandleNVX").as_ptr()))
            },
            get_image_view_address_nvx: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetImageViewAddressNVX").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_image_view_handle_nvx`]. See [`FNGetImageViewHandleNvx`] for more
    /// information.
    pub fn get_image_view_handle_nvx(&self) -> FNGetImageViewHandleNvx {
        self.get_image_view_handle_nvx
    }
    ///Gets [`Self::get_image_view_address_nvx`]. See [`FNGetImageViewAddressNvx`] for more
    /// information.
    pub fn get_image_view_address_nvx(&self) -> FNGetImageViewAddressNvx {
        self.get_image_view_address_nvx
    }
}
