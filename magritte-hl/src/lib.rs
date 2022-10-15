use std::{error::Error, ffi::CStr};

use device::PhysicalDevice;
use magritte::{
    cstr,
    entry::Entry,
    extensions::{
        ext_validation_features::{ValidationFeatureDisableEXT, ValidationFeatureEnableEXT},
        khr_surface::SurfaceKHR,
    },
    vulkan1_0::{DeviceCreateInfo, LayerProperties, PhysicalDeviceFeatures},
    window::HasRawWindowHandle,
    DeviceExtensions, InstanceExtensions, Unique, Version,
};
use queue::QueueCreateInfo;

pub mod context;
pub mod device;
pub mod queue;

pub trait VulkanApplication {
    /// Gets the application name
    fn name(&self) -> &CStr;

    /// Gets the application version
    fn version(&self) -> Version;

    /// Gets the game engine's name
    fn engine_name(&self) -> Option<&CStr> {
        Some(cstr!("Magritte"))
    }

    /// Gets the game engine's version
    fn engine_version(&self) -> Version {
        Version::new(0, 1, 0)
    }

    /// Loads the entry that will be used by the context
    fn load_entry(&mut self) -> Result<Entry, Box<dyn Error>> {
        Entry::new().map_err(Into::into)
    }

    /// Gets the minimum Vulkan version that this application supports
    fn minimum_vulkan_version(&self) -> Version {
        Version::VULKAN1_0
    }

    /// Gets the Vulkan version that this application wants to use.
    /// Based on the Vulkan version that is available
    fn vulkan_version(&mut self, supported: Version) -> Version {
        supported
    }

    /// Gets the instance extensions that this applications needs.
    /// Based on the Vulkan version that will be used.
    fn required_instance_extensions(&mut self, version: Version) -> InstanceExtensions {
        InstanceExtensions::from_version(version)
    }

    /// Gets the instance extensions that this applications wants to enable.
    /// Based on the Vulkan version that will be used and the available extensions.
    fn instance_extensions(&mut self, version: Version, _available: InstanceExtensions) -> InstanceExtensions {
        self.required_instance_extensions(version)
    }

    /// Gets the required instance layers based on the Vulkan version that will be used and the
    /// available layers.
    fn instance_layers<'a>(&mut self, _version: Version, _layers: &'a [LayerProperties]) -> Vec<&'a CStr> {
        vec![]
    }

    /// Whether the debug utilities should be enabled
    fn debug_utils(&mut self) -> bool {
        false
    }

    /// Whether the validation layers should be enabled
    fn validation_layers(&mut self, _available: bool) -> bool {
        false
    }

    /// What validation features should be enabled. [`None`] if the option should be disabled.
    fn validation_features(
        &mut self,
        _available: bool,
    ) -> Option<(Vec<ValidationFeatureEnableEXT>, Vec<ValidationFeatureDisableEXT>)> {
        None
    }

    /// Gets the window to enable surface creation
    fn window(&self) -> Option<&dyn HasRawWindowHandle> {
        None
    }

    /// Selects a physical device from the list of available devices.
    /// Provides the list of devices and the surface to check for support.
    ///
    /// Return [`None`] if no physical device is acceptable.
    fn select_physical_device<'a>(
        &mut self,
        devices: &'a [PhysicalDevice],
        surface: Option<&Unique<SurfaceKHR>>,
    ) -> Option<&'a PhysicalDevice> {
        if surface.is_some() {
            devices.iter().find(|device| device.supports_surface)
        } else {
            devices.first()
        }
    }

    /// Gets the instance extensions that this applications needs.
    /// Based on the Vulkan version that will be used.
    fn required_device_extensions(&mut self, version: Version) -> DeviceExtensions {
        DeviceExtensions::from_version(version)
    }

    /// Gets the device extensions that this applications wants to enable.
    /// Based on the Vulkan version that will be used and the available extensions.
    fn device_extensions(&mut self, version: Version, _available: DeviceExtensions) -> DeviceExtensions {
        self.required_device_extensions(version)
    }

    /// Gets the required device layers based on the Vulkan version that will be used and the
    /// available layers.
    fn device_layers<'a>(&mut self, _version: Version, _layers: &'a [LayerProperties]) -> Vec<&'a CStr> {
        vec![]
    }

    fn queues(&mut self, _device: &PhysicalDevice) -> Vec<QueueCreateInfo>;

    /// Gets the basic physical device features that this application requires.
    /// Can use [`None`] if the user whishes to provide the features using the [`VulkanApplication::device_create_info`] function.
    fn features(&mut self, _device: &PhysicalDevice) -> Option<PhysicalDeviceFeatures> {
        Some(PhysicalDeviceFeatures::default())
    }

    /// Allows the user to expand the device creation info with pointer chains.
    /// This is useful for providing a [`magritte::vulkan1_1::PhysicalDeviceFeatures2`].
    fn device_create_info<'a>(
        &'a mut self,
        _device: &PhysicalDevice,
        device_create_info: DeviceCreateInfo<'static>,
    ) -> DeviceCreateInfo<'a> {
        device_create_info
    }
}
