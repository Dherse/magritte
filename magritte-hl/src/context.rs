use std::{ffi::CStr, sync::Arc};

use log::{debug, trace, Level};
use magritte::{
    entry::Entry,
    extensions::ext_debug_utils::DebugUtilsMessengerEXT,
    validation::VALIDATION_LAYER_NAME_CSTR,
    vulkan1_0::{self, ApplicationInfo, Device, DeviceCreateInfo, Instance, InstanceCreateInfo, VulkanResultCodes},
    window::create_surface,
    AsCStr, AsRaw, Chain, DeviceExtensions, InstanceExtensions, SmallVec, Unique, Version,
};
use magritte_vma::Allocator;

use crate::{device::PhysicalDevice, queue::{Queues, Queue}, VulkanApplication};

#[derive(thiserror::Error, Debug)]
pub enum ContextError {
    #[error("Failed to load the Vulkan entry, reason: {0}")]
    EntryLoadingFailed(#[from] Box<dyn std::error::Error>),

    #[error("Failed to load the Vulkan version, reason: {0}")]
    VersionLoadFailed(VulkanResultCodes),

    #[error("The Vulkan version is too old for this application, expected minimum: {expected}, found: {found}")]
    VersionTooOld { expected: Version, found: Version },

    #[error("Failed to load the Vulkan instance extensions, reason: {0}")]
    InstanceExtensionsLoadingFailed(VulkanResultCodes),

    #[error("Failed to load the Vulkan instance layers, reason: {0}")]
    InstanceLayersLoadingFailed(VulkanResultCodes),

    #[error("Not all required instance extensions are available")]
    InstanceExtensionsNotSufficient {
        expected: InstanceExtensions,
        found: InstanceExtensions,
    },

    #[error("Failed to check if the validation layers are available, reason: {0}")]
    ValidationLayersCheckFailed(VulkanResultCodes),

    #[error("Validation layers were requested but not present")]
    ValidationLayersNotPresent,

    #[error("Validation features were requested but not present")]
    ValidationFeaturesNotPresent,

    #[error("Failed to create the Vulkan instance, reason: {0}")]
    InstanceCreationFailed(VulkanResultCodes),

    #[error("Failed to create the Vulkan validation layer instance, reason: {0}")]
    ValidationLayersCreationFailed(VulkanResultCodes),

    #[error("Failed to create the Vulkan surface, reason: {0}")]
    SurfaceCreationFailed(VulkanResultCodes),

    #[error("Failed to create the enumerate the Vulkan physical devices, reason: {0}")]
    PhysicalDeviceEnumerationFailed(VulkanResultCodes),

    #[error("Failed to create the Vulkan physical device, reason: {0}")]
    PhysicalDeviceCreationFailed(VulkanResultCodes),

    #[error("Failed to select the Vulkan physical device")]
    NoPhysicalDeviceSelected,

    #[error("Failed to enable the extensions required by the allocator, reason: {0}")]
    AllocatorExtensionEnableFailed(VulkanResultCodes),

    #[error("Not all required device extensions are available")]
    DeviceExtensionsNotSufficient {
        expected: DeviceExtensions,
        found: DeviceExtensions,
    },

    #[error("Failed to load the Vulkan device layers, reason: {0}")]
    DeviceLayersLoadingFailed(VulkanResultCodes),

    #[error("Failed to create the Vulkan device instance, reason: {0}")]
    DeviceCreationFailed(VulkanResultCodes),

    #[error("Failed to create the Vulkan memory allocator, reason: {0}")]
    AllocatorCreationFailed(VulkanResultCodes),

    #[error("Queue from family {family} with index {index} does not have a supported type")]
    UnsupportedQueueFamily {
        family: u32,
        index: u32
    },
}

pub struct Context<V: VulkanApplication> {
    /// The application that created this context
    pub application: V,

    /// The entry into Vulkan: the library itself
    pub entry: Arc<Entry>,

    /// The Vulkan instance: version and instance-level extensions and layers
    pub instance: Unique<Instance>,

    /// The debug utils, we need to keep alive to keep getting messages.
    pub debug_utils: Option<Unique<DebugUtilsMessengerEXT>>,

    /// The physical device we will render on
    pub physical_device: PhysicalDevice,

    /// The "connected" physical device
    pub device: Unique<Device>,

    /// The allocator
    pub allocator: Unique<Allocator>,

    /// The queues that are instantiated on the device
    pub queues: Queues,
}

impl<V: VulkanApplication> Context<V> {
    pub fn new(mut application: V) -> Result<Self, ContextError> {
        // First, we load the library
        let entry = Arc::new(application.load_entry()?);
        debug!("Loaded Vulkan library");

        // Get the maximum version supported by this application
        let version = unsafe {
            entry
                .enumerate_instance_version()
                .map_err(ContextError::VersionLoadFailed)?
        };
        debug!("Vulkan version: {}", version);

        let version = application.vulkan_version(version);
        debug!("Will use Vulkan version: {}", version);

        // Verify that we meet the minimum supported Vulkan version
        let expected = application.minimum_vulkan_version();
        if version < expected {
            return Err(ContextError::VersionTooOld {
                expected,
                found: version,
            });
        }

        // Let's also check what instance extensions are available: (optional)
        let extension_properties = unsafe {
            entry
                .enumerate_instance_extension_properties(None, None)
                .map_err(ContextError::InstanceExtensionsLoadingFailed)?
        };

        trace!("Found {} instance extensions", extension_properties.len());
        extension_properties.iter().for_each(|ext| {
            trace!(
                " - name: `{}`, version: {}",
                ext.extension_name().as_cstr().to_string_lossy(),
                Version::from(ext.spec_version())
            )
        });

        // We check that all of the required extensions are present
        let supported_extensions = InstanceExtensions::from_extension_properties(version, &extension_properties);
        let expected = application.required_instance_extensions(version);
        if supported_extensions < expected {
            return Err(ContextError::InstanceExtensionsNotSufficient {
                expected,
                found: supported_extensions,
            });
        }

        // We get the instance extensions that we want to enable
        let mut instance_extensions = application.instance_extensions(version, supported_extensions);

        // Let's also check what instance layers are available: (optional)
        let layer_properties = unsafe {
            entry
                .enumerate_instance_layer_properties(None)
                .map_err(ContextError::InstanceLayersLoadingFailed)?
        };

        trace!("Found {} instance layers", layer_properties.len());
        layer_properties.iter().for_each(|layer| {
            trace!(
                "- name: `{}`, version: {}",
                layer.layer_name().as_cstr().to_string_lossy(),
                Version::from(layer.spec_version())
            )
        });

        let validation_layers_available =
            unsafe { magritte::validation::is_present(&entry).map_err(ContextError::ValidationLayersCheckFailed)? };
        if validation_layers_available {
            debug!("Validation layers are available");
        } else {
            debug!("Validation layers are not available");
        }

        // We get the instance layers that we want to enable
        let mut instance_layers = application.instance_layers(version, &layer_properties);

        // Checks if the validation layers are available
        let validation_layers = application.validation_layers(validation_layers_available);
        if validation_layers && !validation_layers_available {
            return Err(ContextError::ValidationLayersNotPresent);
        } else if validation_layers {
            instance_layers.push(VALIDATION_LAYER_NAME_CSTR);
            instance_extensions = magritte::validation::enable_validation(instance_extensions);
            debug!("Enabling the validation layers");
        }

        let mut disabled_validation_features = vec![];

        let mut enabled_validation_features = vec![];

        let validation_features = application.validation_features(supported_extensions.ext_validation_features);
        if let Some((enabled, disabled)) = validation_features {
            if !enabled.is_empty() {
                if !supported_extensions.ext_validation_features {
                    return Err(ContextError::ValidationFeaturesNotPresent);
                }

                debug!("Enabling the validation features");
                instance_extensions = instance_extensions.enable_ext_validation_features();

                enabled_validation_features = enabled;
                disabled_validation_features = disabled;
            }
        }

        // Vulkan needs the extension names, so we create the list of names:
        let instance_extensions_list = instance_extensions.extension_names();

        // Vulkan needs the extension names, so we create the list of names:
        let instance_layers_list = instance_layers
            .into_iter()
            .map(|layer| layer.as_ptr())
            .collect::<Vec<_>>();

        let app_info = ApplicationInfo::default()
            .with_api_version(version.into())
            .with_application_name(application.name().as_ptr())
            .with_application_version(application.version().into())
            .with_engine_name(application.engine_name().map_or_else(std::ptr::null, CStr::as_ptr))
            .with_engine_version(application.engine_version().into());

        // Here we group up all of the information for creating an instance.
        // This can be extended using pointer chains to contain **many** things.
        let mut instance_create_info = InstanceCreateInfo::default()
            .with_application_info(&app_info)
            .with_pp_enabled_extension_names(&instance_extensions_list)
            .with_pp_enabled_layer_names(&instance_layers_list);

        let mut validation_features_info =
            magritte::extensions::ext_validation_features::ValidationFeaturesEXT::default()
                .with_enabled_validation_features(&enabled_validation_features)
                .with_disabled_validation_features(&disabled_validation_features);

        if !enabled_validation_features.is_empty() {
            instance_create_info = instance_create_info.chain(&mut validation_features_info);
        }

        // Here we create the instance.
        // We give it the extra parameter `extensions` as it will keep it as a "metadata".
        let instance = unsafe {
            entry
                .create_instance(&instance_create_info, None, instance_extensions)
                .map_err(ContextError::InstanceCreationFailed)?
        };
        debug!("Created the Vulkan instance: {:?}", instance.as_raw());

        let debug_utils = if validation_layers {
            Some(unsafe {
                magritte::validation::create_debug_utils_messenger(&instance, Level::Info, None)
                    .map_err(ContextError::ValidationLayersCreationFailed)?
            })
        } else {
            None
        };
        debug!(
            "Created the debug utils messenger: {:?}",
            debug_utils.as_ref().map(|v| v.as_raw())
        );

        let surface = if let Some(window) = application.window() {
            let surface = unsafe {
                create_surface(&instance, window, None)
                    .result()
                    .map_err(ContextError::SurfaceCreationFailed)?
            };
            debug!("Created the surface: {:?}", surface.as_raw());
            Some(surface)
        } else {
            None
        };

        let mut physical_devices = unsafe {
            instance
                .enumerate_physical_devices(None)
                .result()
                .map_err(ContextError::PhysicalDeviceEnumerationFailed)?
                .into_iter()
                .enumerate()
                .map(|(index, p)| PhysicalDevice::new(index, &instance, surface.as_ref(), p).result())
                .collect::<Result<SmallVec<_>, _>>()
                .map_err(ContextError::PhysicalDeviceCreationFailed)?
        };

        debug!("Found {} physical devices", physical_devices.len());
        physical_devices
            .iter()
            .for_each(|device| trace!("- name: `{}`, API version: {}", device.name, device.api_version));

        let physical_device = match application.select_physical_device(&physical_devices, surface.as_ref()) {
            Some(device) => {
                debug!("Selected the physical device: {}", device.name);

                physical_devices.swap_remove(device.index)
            },
            None => {
                return Err(ContextError::NoPhysicalDeviceSelected);
            },
        };

        // We check that all of the required extensions are present
        let supported_extensions = physical_device.extensions;
        let expected = application.required_device_extensions(version);
        if supported_extensions < expected {
            return Err(ContextError::DeviceExtensionsNotSufficient {
                expected,
                found: supported_extensions,
            });
        }

        // We get the instance extensions that we want to enable
        let mut device_extensions = application.device_extensions(version, supported_extensions);

        // Let's also check what instance layers are available: (optional)
        let layer_properties: SmallVec<_> = unsafe {
            physical_device
                .enumerate_device_layer_properties(None)
                .result()
                .map_err(ContextError::DeviceLayersLoadingFailed)?
        };

        trace!("Found {} device layers", layer_properties.len());
        layer_properties.iter().for_each(|layer| {
            trace!(
                "- name: `{}`, version: {}",
                layer.layer_name().as_cstr().to_string_lossy(),
                Version::from(layer.spec_version())
            )
        });

        let device_layers = application.device_layers(version, &layer_properties);
        let device_layer_ptrs = device_layers.iter().map(|s| s.as_ptr()).collect::<SmallVec<_>>();

        // Here we fetch the supported extensions that will be used to allocate memory
        // in a easier and more performant way using the Vulkan Memory Allocator.
        // Note that both the use of VMA and raw Vulkan allocations are covered in this
        // sample.
        Allocator::enable_extensions(&physical_device, &mut device_extensions, false)
            .map_err(ContextError::AllocatorExtensionEnableFailed)?;

        let device_extension_names = device_extensions.extension_names();

        let features = application.features(&physical_device);
        let queue_defs = application.queues(&physical_device);
        let queue_info = queue_defs
            .iter()
            .map(|q| q.into())
            .collect::<SmallVec<vulkan1_0::DeviceQueueCreateInfo>>();

        // We group up all of the information about the creation of the device.
        // This can be extended using pointer chains to contain **many** things.
        let device_create_info = application
            .device_create_info(&physical_device, DeviceCreateInfo::default())
            .with_queue_create_infos(&queue_info[..])
            .with_pp_enabled_layer_names(&device_layer_ptrs)
            .with_pp_enabled_extension_names(&device_extension_names)
            .with_enabled_features_raw(features.as_ref().map(|f| f as *const _).unwrap_or_else(std::ptr::null));

        // Finally, we create the device.
        // You can imagine this step as "connecting" to the device, we are now ready to talk to it
        // and tell it what to allocate, do, deallocate, etc.
        let device = unsafe {
            physical_device
                .create_device(&device_create_info, None, device_extensions)
                .result()
                .map_err(ContextError::DeviceCreationFailed)?
        };
        debug!("Device created: {:?}", device.as_raw());

        let allocator = Allocator::new(&device, None, None).map_err(ContextError::AllocatorCreationFailed)?;
        debug!("Allocator created: {:?}", allocator.as_raw());

        let mut queues = Queues::default();

        let mut len = 0;
        for queue_def in &queue_defs {
            for i in 0..queue_def.priorities.len() {
                let queue = unsafe { device.get_device_queue(Some(queue_def.family_index),  Some(i as _)) };

                let family = &physical_device.queue_families[queue_def.family_index as usize];
                let queue = Queue::new(
                    queue_def.family_index,
                    i as u32,
                    family.properties.queue_flags(),
                    queue
                );
                
                len += 1;
                if family.supports_graphics() && family.supports_surface() {
                    queues.present.push(queue);
                } else if family.supports_graphics() {
                    queues.graphics.push(queue);
                } else if family.supports_compute() {
                    queues.compute.push(queue);
                } else if family.supports_transfer() {
                    queues.transfer.push(queue);
                } else {
                    return Err(ContextError::UnsupportedQueueFamily {
                        family: queue_def.family_index,
                        index: i as u32,
                    });
                }
            }
        }

        trace!("Loaded {} queues", len);

        Ok(Self {
            application,
            entry,
            instance,
            debug_utils,
            physical_device,
            device,
            allocator,
            queues,
        })
    }

    /// Returns a reference to the entry of this [`Context<V>`].
    pub fn entry(&self) -> &Arc<Entry> {
        &self.entry
    }

    /// Returns a reference to the instance of this [`Context<V>`].
    pub fn instance(&self) -> &Unique<Instance> {
        &self.instance
    }

    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.physical_device
    }

    pub fn device(&self) -> &Unique<Device> {
        &self.device
    }

    pub fn allocator(&self) -> &Unique<Allocator> {
        &self.allocator
    }

    pub fn queues(&self) -> &Queues {
        &self.queues
    }
}

impl<V: VulkanApplication> std::ops::Deref for Context<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.application
    }
}

impl<V: VulkanApplication> std::ops::DerefMut for Context<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.application
    }
}

impl<V: VulkanApplication> AsRef<V> for Context<V> {
    fn as_ref(&self) -> &V {
        &*self
    }
}

impl<V: VulkanApplication> AsMut<V> for Context<V> {
    fn as_mut(&mut self) -> &mut V {
        &mut *self
    }
}
