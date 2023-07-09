pub use crate::common::extensions::nvx_image_view_handle::{
    NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME, NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{DescriptorType, DeviceAddress, DeviceSize, ImageView, Sampler, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkImageViewHandleInfoNVX")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageViewHandleInfoNVX {
    #[doc(alias = "imageView")]
    pub image_view: ImageView,
    #[doc(alias = "descriptorType")]
    pub descriptor_type: DescriptorType,
    pub sampler: Option<Sampler>,
}
impl ImageViewHandleInfoNVX {
    ///Get a reference to the `image_view` field.
    pub fn image_view(&self) -> &ImageView {
        &self.image_view
    }
    ///Get a reference to the `descriptor_type` field.
    pub fn descriptor_type(&self) -> DescriptorType {
        self.descriptor_type
    }
    ///Get a reference to the `sampler` field.
    pub fn sampler(&self) -> &Option<Sampler> {
        &self.sampler
    }
    ///Get a mutable reference to the `image_view` field.
    pub fn image_view_mut(&mut self) -> &mut ImageView {
        &mut self.image_view
    }
    ///Get a mutable reference to the `descriptor_type` field.
    pub fn descriptor_type_mut(&mut self) -> &mut DescriptorType {
        &mut self.descriptor_type
    }
    ///Get a mutable reference to the `sampler` field.
    pub fn sampler_mut(&mut self) -> &mut Option<Sampler> {
        &mut self.sampler
    }
    ///Sets the `image_view` field.
    pub fn set_image_view(&mut self, image_view: ImageView) -> &mut Self {
        self.image_view = image_view;
        self
    }
    ///Sets the `descriptor_type` field.
    pub fn set_descriptor_type(&mut self, descriptor_type: DescriptorType) -> &mut Self {
        self.descriptor_type = descriptor_type;
        self
    }
    ///Sets the `sampler` field.
    pub fn set_sampler(&mut self, sampler: Option<Sampler>) -> &mut Self {
        self.sampler = sampler;
        self
    }
    ///Sets the `image_view` field in a builder way.
    pub fn with_image_view(mut self, image_view: ImageView) -> Self {
        self.image_view = image_view;
        self
    }
    ///Sets the `descriptor_type` field in a builder way.
    pub fn with_descriptor_type(mut self, descriptor_type: DescriptorType) -> Self {
        self.descriptor_type = descriptor_type;
        self
    }
    ///Sets the `sampler` field in a builder way.
    pub fn with_sampler(mut self, sampler: Option<Sampler>) -> Self {
        self.sampler = sampler;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageViewHandleInfoNVX {
    type LowLevel = crate::native::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX {
            s_type: StructureType::ImageViewHandleInfoNvx,
            p_next: std::ptr::null(),
            image_view: self.image_view.into_low_level(context, bump),
            descriptor_type: self.descriptor_type.into_low_level(context, bump),
            sampler: self
                .sampler
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageViewHandleInfoNVX {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            image_view: crate::conv::FromLowLevel::from_low_level(context, value.image_view),
            descriptor_type: crate::conv::FromLowLevel::from_low_level(context, value.descriptor_type),
            sampler: if value.sampler == crate::native::vulkan1_0::Sampler::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.sampler))
            },
        }
    }
}
#[doc(alias = "VkImageViewAddressPropertiesNVX")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageViewAddressPropertiesNVX {
    #[doc(alias = "deviceAddress")]
    pub device_address: DeviceAddress,
    pub size: DeviceSize,
}
impl ImageViewAddressPropertiesNVX {
    ///Get a reference to the `device_address` field.
    pub fn device_address(&self) -> &DeviceAddress {
        &self.device_address
    }
    ///Get a reference to the `size` field.
    pub fn size(&self) -> &DeviceSize {
        &self.size
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageViewAddressPropertiesNVX {
    type LowLevel = crate::native::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX {
            s_type: StructureType::ImageViewAddressPropertiesNvx,
            p_next: std::ptr::null_mut(),
            device_address: self.device_address.into_low_level(context, bump),
            size: self.size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageViewAddressPropertiesNVX {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            device_address: crate::conv::FromLowLevel::from_low_level(context, value.device_address),
            size: crate::conv::FromLowLevel::from_low_level(context, value.size),
        }
    }
}
