use std::ops::Deref;

use magritte::{
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{self, Instance, PhysicalDeviceType, QueueFamilyProperties, QueueFlags, VulkanResultCodes},
    vulkan1_1::{PhysicalDeviceFeatures2, PhysicalDeviceProperties2},
    vulkan1_2::{
        PhysicalDeviceVulkan11Features, PhysicalDeviceVulkan11Properties, PhysicalDeviceVulkan12Features,
        PhysicalDeviceVulkan12Properties,
    },
    vulkan1_3::{PhysicalDeviceVulkan13Features, PhysicalDeviceVulkan13Properties},
    AsCStr, AsRaw, Chain, DeviceExtensions, SmallVec, Unique, Version, VulkanResult,
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

    /// The Vulkan 1.0 properties of the device
    pub properties: PhysicalDeviceProperties2<'static>,

    /// The Vulkan 1.1 features of the device
    pub features: PhysicalDeviceFeatures2<'static>,

    /// The extended Vulkan 1.1 properties of the device
    pub properties1_1: Option<PhysicalDeviceVulkan11Properties<'static>>,

    /// The extended Vulkan 1.1 features of the device
    pub features1_1: Option<PhysicalDeviceVulkan11Features<'static>>,

    /// The extended Vulkan 1.2 properties of the device
    pub properties1_2: Option<PhysicalDeviceVulkan12Properties<'static>>,

    /// The extended Vulkan 1.2 features of the device
    pub features1_2: Option<PhysicalDeviceVulkan12Features<'static>>,

    /// The extended Vulkan 1.3 properties of the device
    pub properties1_3: Option<PhysicalDeviceVulkan13Properties<'static>>,

    /// The extended Vulkan 1.3 features of the device
    pub features1_3: Option<PhysicalDeviceVulkan13Features<'static>>,

    /// The extensions that the device supports
    pub extensions: DeviceExtensions,
}

impl PhysicalDevice {
    pub fn new(
        index: usize,
        version: Version,
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

        let mut properties1_1 = (version >= Version::VULKAN1_2).then(PhysicalDeviceVulkan11Properties::default);
        let mut features1_1 = (version >= Version::VULKAN1_2).then(PhysicalDeviceVulkan11Features::default);

        let mut properties1_2 = (version >= Version::VULKAN1_2).then(PhysicalDeviceVulkan12Properties::default);
        let mut features1_2 = (version >= Version::VULKAN1_2).then(PhysicalDeviceVulkan12Features::default);

        let mut properties1_3 = (version >= Version::VULKAN1_3).then(PhysicalDeviceVulkan13Properties::default);
        let mut features1_3 = (version >= Version::VULKAN1_3).then(PhysicalDeviceVulkan13Features::default);

        let properties = PhysicalDeviceProperties2::default()
            .chain_opt(properties1_1.as_mut())
            .chain_opt(properties1_2.as_mut())
            .chain_opt(properties1_3.as_mut());
        let features = PhysicalDeviceFeatures2::default()
            .chain_opt(features1_1.as_mut())
            .chain_opt(features1_2.as_mut())
            .chain_opt(features1_3.as_mut());

        let properties = unsafe { device.get_physical_device_properties2(Some(properties)) };
        let features = unsafe { device.get_physical_device_features2(Some(features)) };

        let extensions = unsafe { device.enumerate_device_extension_properties(None, None).result()? };

        VulkanResult::Success(
            VulkanResultCodes::SUCCESS,
            Self {
                index,
                device,
                name: properties
                    .properties
                    .device_name()
                    .as_cstr()
                    .to_str()
                    .unwrap()
                    .to_string(),
                supports_surface: queues.iter().any(|q| q.supports_surface),
                queue_families: queues,
                api_version: Version::from(properties.properties.api_version()),
                device_type: properties.properties.device_type(),
                properties,
                features,
                extensions: DeviceExtensions::from_extension_properties(instance.metadata().version(), &extensions),
                properties1_3: properties1_3.map(|p| p.make_static()),
                features1_3: features1_3.map(|p| p.make_static()),
                properties1_2: properties1_2.map(|p| p.make_static()),
                features1_2: features1_2.map(|p| p.make_static()),
                properties1_1: properties1_1.map(|p| p.make_static()),
                features1_1: features1_1.map(|p| p.make_static()),
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
