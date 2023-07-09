pub use crate::common::extensions::ext_pci_bus_info::{EXT_PCI_BUS_INFO_EXTENSION_NAME, EXT_PCI_BUS_INFO_SPEC_VERSION};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDevicePCIBusInfoPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePciBusInfoPropertiesEXT {
    #[doc(alias = "pciDomain")]
    pub pci_domain: u32,
    #[doc(alias = "pciBus")]
    pub pci_bus: u32,
    #[doc(alias = "pciDevice")]
    pub pci_device: u32,
    #[doc(alias = "pciFunction")]
    pub pci_function: u32,
}
impl PhysicalDevicePciBusInfoPropertiesEXT {
    ///Get a reference to the `pci_domain` field.
    pub fn pci_domain(&self) -> u32 {
        self.pci_domain
    }
    ///Get a reference to the `pci_bus` field.
    pub fn pci_bus(&self) -> u32 {
        self.pci_bus
    }
    ///Get a reference to the `pci_device` field.
    pub fn pci_device(&self) -> u32 {
        self.pci_device
    }
    ///Get a reference to the `pci_function` field.
    pub fn pci_function(&self) -> u32 {
        self.pci_function
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePciBusInfoPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_pci_bus_info::PhysicalDevicePciBusInfoPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_pci_bus_info::PhysicalDevicePciBusInfoPropertiesEXT {
            s_type: StructureType::PhysicalDevicePciBusInfoPropertiesExt,
            p_next: std::ptr::null_mut(),
            pci_domain: self.pci_domain.into_low_level(context, bump),
            pci_bus: self.pci_bus.into_low_level(context, bump),
            pci_device: self.pci_device.into_low_level(context, bump),
            pci_function: self.pci_function.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePciBusInfoPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            pci_domain: crate::conv::FromLowLevel::from_low_level(context, value.pci_domain),
            pci_bus: crate::conv::FromLowLevel::from_low_level(context, value.pci_bus),
            pci_device: crate::conv::FromLowLevel::from_low_level(context, value.pci_device),
            pci_function: crate::conv::FromLowLevel::from_low_level(context, value.pci_function),
        }
    }
}
