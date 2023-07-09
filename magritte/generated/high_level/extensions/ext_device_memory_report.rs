pub use crate::common::extensions::ext_device_memory_report::{
    DeviceMemoryReportEventTypeEXT, DeviceMemoryReportFlagsEXT, EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME,
    EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{DeviceSize, ObjectType, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceDeviceMemoryReportFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    #[doc(alias = "deviceMemoryReport")]
    pub device_memory_report: bool,
}
impl PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    ///Get a reference to the `device_memory_report` field.
    pub fn device_memory_report(&self) -> &bool {
        &self.device_memory_report
    }
    ///Get a mutable reference to the `device_memory_report` field.
    pub fn device_memory_report_mut(&mut self) -> &mut bool {
        &mut self.device_memory_report
    }
    ///Sets the `device_memory_report` field.
    pub fn set_device_memory_report(&mut self, device_memory_report: bool) -> &mut Self {
        self.device_memory_report = device_memory_report;
        self
    }
    ///Sets the `device_memory_report` field in a builder way.
    pub fn with_device_memory_report(mut self, device_memory_report: bool) -> Self {
        self.device_memory_report = device_memory_report;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_device_memory_report::PhysicalDeviceDeviceMemoryReportFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_device_memory_report::PhysicalDeviceDeviceMemoryReportFeaturesEXT {
            s_type: StructureType::PhysicalDeviceDeviceMemoryReportFeaturesExt,
            p_next: std::ptr::null_mut(),
            device_memory_report: self.device_memory_report.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            device_memory_report: crate::conv::FromLowLevel::from_low_level(context, value.device_memory_report),
        }
    }
}
#[doc(alias = "VkDeviceMemoryReportCallbackDataEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceMemoryReportCallbackDataEXT {
    pub flags: DeviceMemoryReportFlagsEXT,
    #[doc(alias = "type")]
    pub type_: DeviceMemoryReportEventTypeEXT,
    #[doc(alias = "memoryObjectId")]
    pub memory_object_id: u64,
    pub size: DeviceSize,
    #[doc(alias = "objectType")]
    pub object_type: ObjectType,
    #[doc(alias = "objectHandle")]
    pub object_handle: u64,
    #[doc(alias = "heapIndex")]
    pub heap_index: u32,
}
impl DeviceMemoryReportCallbackDataEXT {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> DeviceMemoryReportFlagsEXT {
        self.flags
    }
    ///Get a reference to the `type_` field.
    pub fn type_(&self) -> DeviceMemoryReportEventTypeEXT {
        self.type_
    }
    ///Get a reference to the `memory_object_id` field.
    pub fn memory_object_id(&self) -> u64 {
        self.memory_object_id
    }
    ///Get a reference to the `size` field.
    pub fn size(&self) -> &DeviceSize {
        &self.size
    }
    ///Get a reference to the `object_type` field.
    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }
    ///Get a reference to the `object_handle` field.
    pub fn object_handle(&self) -> u64 {
        self.object_handle
    }
    ///Get a reference to the `heap_index` field.
    pub fn heap_index(&self) -> u32 {
        self.heap_index
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceMemoryReportCallbackDataEXT {
    type LowLevel = crate::native::extensions::ext_device_memory_report::DeviceMemoryReportCallbackDataEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_device_memory_report::DeviceMemoryReportCallbackDataEXT {
            s_type: StructureType::DeviceMemoryReportCallbackDataExt,
            p_next: std::ptr::null_mut(),
            flags: self.flags.into_low_level(context, bump),
            type_: self.type_.into_low_level(context, bump),
            memory_object_id: self.memory_object_id.into_low_level(context, bump),
            size: self.size.into_low_level(context, bump),
            object_type: self.object_type.into_low_level(context, bump),
            object_handle: self.object_handle.into_low_level(context, bump),
            heap_index: self.heap_index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceMemoryReportCallbackDataEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            type_: crate::conv::FromLowLevel::from_low_level(context, value.type_),
            memory_object_id: crate::conv::FromLowLevel::from_low_level(context, value.memory_object_id),
            size: crate::conv::FromLowLevel::from_low_level(context, value.size),
            object_type: crate::conv::FromLowLevel::from_low_level(context, value.object_type),
            object_handle: crate::conv::FromLowLevel::from_low_level(context, value.object_handle),
            heap_index: crate::conv::FromLowLevel::from_low_level(context, value.heap_index),
        }
    }
}
