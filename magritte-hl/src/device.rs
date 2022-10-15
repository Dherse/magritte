use std::ops::Deref;

use magritte::{
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{
        self, Instance, PhysicalDeviceProperties, PhysicalDeviceType, QueueFamilyProperties, QueueFlags,
        VulkanResultCodes,
    },
    AsCStr, AsRaw, DeviceExtensions, SmallVec, Unique, Version, VulkanResult,
};

use crate::queue::QueueCreateInfo;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueueFamily {
    /// Index of the queue family
    pub index: u32,

    /// Does the queue family support the surface?
    pub supports_surface: bool,

    /// The properties of the queue family
    pub properties: QueueFamilyProperties,
}

impl QueueFamily {
    pub fn new(
        index: u32,
        device: &Unique<vulkan1_0::PhysicalDevice>,
        surface: Option<&Unique<SurfaceKHR>>,
        properties: QueueFamilyProperties,
    ) -> VulkanResult<Self> {
        let supports_surface = surface
            .map(|surface| unsafe { device.get_physical_device_surface_support_khr(Some(index), surface.as_raw()) })
            .map_or(Ok(None), |v| v.result().map(Some))?
            .unwrap_or(false);

        VulkanResult::Success(
            VulkanResultCodes::SUCCESS,
            Self {
                index,
                supports_surface,
                properties,
            },
        )
    }

    pub fn as_create_info(&self, priorities: &[f32], protected: bool) -> QueueCreateInfo {
        QueueCreateInfo {
            family_index: self.index,
            priorities: priorities.to_vec(),
            protected,
        }
    }

    #[inline]
    pub fn supports_graphics(&self) -> bool {
        self.properties.queue_flags.contains(QueueFlags::GRAPHICS)
    }

    #[inline]
    pub fn supports_compute(&self) -> bool {
        self.properties.queue_flags.contains(QueueFlags::COMPUTE)
    }

    #[inline]
    pub fn supports_transfer(&self) -> bool {
        self.properties.queue_flags.contains(QueueFlags::TRANSFER)
    }

    #[inline]
    pub fn supports_surface(&self) -> bool {
        self.supports_surface
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PhysicalDevice {
    /// The index of the device in the list of devices
    pub index: usize,

    /// The handle to the Vulkan physical device.
    pub device: Unique<vulkan1_0::PhysicalDevice>,

    /// The name of the device
    pub name: String,

    /// The queue families that this device supports
    pub queue_families: SmallVec<QueueFamily>,

    /// Does a queue support the surface?
    pub supports_surface: bool,

    /// The API version this device supports
    pub api_version: Version,

    /// The type of device
    pub device_type: PhysicalDeviceType,

    /// The basic (Vulkan 1.0) properties of the device
    pub properties: PhysicalDeviceProperties,

    /// The extensions that the device supports
    pub extensions: DeviceExtensions,
}

impl PhysicalDevice {
    pub fn new(
        index: usize,
        instance: &Unique<Instance>,
        surface: Option<&Unique<SurfaceKHR>>,
        device: Unique<vulkan1_0::PhysicalDevice>,
    ) -> VulkanResult<Self> {
        let queues = unsafe {
            device
                .get_physical_device_queue_family_properties(None)
                .into_iter()
                .enumerate()
                .map(|(i, q)| QueueFamily::new(i as u32, &device, surface, q).result())
                .collect::<Result<SmallVec<_>, _>>()?
        };

        let properties = unsafe { device.get_physical_device_properties() };

        let extensions = unsafe { device.enumerate_device_extension_properties(None, None).result()? };

        VulkanResult::Success(
            VulkanResultCodes::SUCCESS,
            Self {
                index,
                device,
                name: properties.device_name().as_cstr().to_str().unwrap().to_string(),
                supports_surface: queues.iter().any(|q| q.supports_surface),
                queue_families: queues,
                api_version: Version::from(properties.api_version()),
                device_type: properties.device_type(),
                properties,
                extensions: DeviceExtensions::from_extension_properties(instance.metadata().version(), &extensions),
            },
        )
    }
}

impl Deref for PhysicalDevice {
    type Target = Unique<vulkan1_0::PhysicalDevice>;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl AsRef<Unique<vulkan1_0::PhysicalDevice>> for PhysicalDevice {
    fn as_ref(&self) -> &Unique<vulkan1_0::PhysicalDevice> {
        &self.device
    }
}
