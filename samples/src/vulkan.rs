use std::{error::Error, sync::Arc};

use log::{debug, error, info, trace, Level};
use magritte::{
    cstr_ptr,
    entry::Entry,
    extensions::{ext_debug_utils::DebugUtilsMessengerEXT, khr_surface::SurfaceKHR},
    validation::{create_debug_utils_messenger, enable_validation, is_present, VALIDATION_LAYER_NAME},
    vulkan1_0::{
        ApplicationInfo, Device, DeviceCreateInfo, DeviceQueueCreateInfo, Instance, InstanceCreateInfo, PhysicalDevice,
        PhysicalDeviceFeatures, Queue, QueueFlags,
    },
    window::{create_surface, enable_required_extensions},
    AsRaw, DeviceExtensions, InstanceExtensions, Unique, Version,
};
use magritte_vma::Allocator;
use winit::window::Window;

use crate::AsCStr;

/// "Basic" Vulkan state
pub struct Vulkan {
    /// The entry into Vulkan: the library itself
    pub entry: Arc<Entry>,

    /// The Vulkan instance: version and instance-level extensions and layers
    pub instance: Unique<Instance>,

    /// The debug utils, we need to keep alive to keep getting messages.
    pub debug_utils: Option<Unique<DebugUtilsMessengerEXT>>,

    /// The physical device we will render on
    pub physical_device: Unique<PhysicalDevice>,

    /// The "connected" physical device
    pub device: Unique<Device>,

    /// The allocator
    pub allocator: Unique<Allocator>,

    /// The graphics queue we will render on
    pub graphics_queue: Unique<Queue>,

    /// The index of the queue family
    pub queue_family_index: u32,
}

impl Vulkan {
    pub fn new(
        window: &Window,
        instance_extensions: InstanceExtensions,
        device_extensions: DeviceExtensions,
        validation: bool,
    ) -> Result<(Self, Unique<SurfaceKHR>), Box<dyn Error>> {
        // First, we load the library
        let entry = Arc::new(Entry::new()?);

        // Let's check the maximum supports version: (optional)
        let version = unsafe { entry.enumerate_instance_version()? };

        debug!("Maximum supported Vulkan version: {}", version);

        // Let's also check what instance extensions are available: (optional)
        let extension_properties = unsafe { entry.enumerate_instance_extension_properties(None, None)? };

        extension_properties.into_iter().for_each(|ext| {
            debug!(
                "Instance extension: {} {}",
                ext.extension_name().as_cstr().to_string_lossy(),
                Version::from(ext.spec_version())
            )
        });

        // Let's also check what instance layers are available: (optional)
        let layer_properties = unsafe { entry.enumerate_instance_layer_properties(None)? };

        layer_properties.into_iter().for_each(|layer| {
            debug!(
                "Instance layer: {} {}",
                layer.layer_name().as_cstr().to_string_lossy(),
                Version::from(layer.spec_version())
            )
        });

        // We check if the validation layers are present (optional)
        let validation = if validation && unsafe { !is_present(&entry)? } {
            error!("Validation layers not present, cannot enable");

            false
        } else if validation {
            debug!("Validation layers are available and enabled");

            true
        } else {
            trace!("Validation layers are not enabled");

            false
        };

        // Here, we will create the extension set we need
        // We need a Vulkan 1.0 installation,
        // - we need to have the Vulkan validation layers (download the Vulkan SDK if it is not done).
        // - we need a swapchain to actually display things on screen
        // - we need an annoying set of extensions for showing the window, this is why Magritte comes with
        //   `enable_required_extensions` that will automatically deal with extensions for your window!
        let mut instance_extensions =
            enable_required_extensions(window, instance_extensions)?.enable_khr_get_physical_device_properties2();

        // If we have the validation layers, enable the extension (optional)
        if validation {
            instance_extensions = enable_validation(instance_extensions);

            trace!("We have added the extensions for debugging");
        }

        // Vulkan needs the extension names, so we create the list of names:
        let instance_extension_list = instance_extensions.extension_names();

        // Here, we create some application info that Vulkan can use.
        // I don't know exactly why they are used, but I would assume they are.
        // Here, we will give:
        // - The highest Vulkan version we support (Vulkan 1.0)
        // - The name of our application, we can make this easy using the `cstr` and `cstr_ptr` macros!
        // - The version of our application
        // - The name of our game engine (if any), we can make this easy using the `cstr` and `cstr_ptr`
        //   macros!
        // - The version of our game engine (if any)
        let app_info = ApplicationInfo::default()
            .set_api_version(instance_extensions.version().into())
            .set_application_name(cstr_ptr!("Magritte sample"))
            .set_application_version(Version::from((1, 0, 0)).into());

        // Here we will create the list of layers we wish to enable
        let mut instance_layer_list = Vec::new();

        // If we have the validation layers, add the layer (optional)
        if validation {
            instance_layer_list.push(VALIDATION_LAYER_NAME);
        }

        // Here we group up all of the information for creating an instance.
        // This can be extended using pointer chains to contain **many** things.
        let instance_create_info = InstanceCreateInfo::default()
            .set_application_info(&app_info)
            .set_pp_enabled_extension_names(&instance_extension_list)
            .set_pp_enabled_layer_names(&instance_layer_list);

        // Here we create the instance.
        // We give it the extra parameter `extensions` as it will keep it as a "metadata".
        let instance = unsafe { entry.create_instance(&instance_create_info, None, instance_extensions)? };

        // What is that `as_raw`??? It's simple, Magritte wraps Vulkan structures into a `Unique` which
        // helps to enforce most (**not** all) lifetimes. `as_raw` simply lets you get back to the
        // original Vulkan value.
        info!("We have created the instance: {:?}", instance.as_raw());

        // The final step in getting debug messages: creating the debug utils messenger.
        // This is done using a helper inside of Magritte.
        // We need to keep it alive to keep getting messages!
        let debug_utils = if validation {
            let debug = unsafe { create_debug_utils_messenger(&instance, Level::Trace, None)? };

            info!("We have created the debug messenger: {:?}", instance.as_raw());

            Some(debug)
        } else {
            None
        };

        // Here, we create the surface, literally the canvas on which we will paint.
        // As often for this simple but annoying tasks, Magritte provides a helper function.
        let (surface, _) = unsafe { create_surface(&instance, window, None)? };

        info!("We have created the surface: {:?}", surface.as_raw());

        // Here we get the list of all physical devices (GPUs) in the machine, can be
        // more than one!
        let (physical_devices, _) = unsafe { instance.enumerate_physical_devices(None)? };

        info!("We have {} physical devices", physical_devices.len());

        // We want to try and find a device that:
        // - Has a graphics submission queue (should be the case on most platforms)
        // - Is capable of supporting our surface
        // From this we get:
        // - The physical device we actually want to use
        // - The queue family index for graphics that we will layer use.
        // For a real game, you will **need** to improve this most basic algorithm.
        let (physical_device, queue_family_index) = physical_devices
            .into_iter()
            .find_map(|pdevice| unsafe {
                let families = pdevice.get_physical_device_queue_family_properties(None);
                families
                    .iter()
                    .enumerate()
                    .find_map(|(index, info)| {
                        let supports_graphics_and_surface = info.queue_flags().contains(QueueFlags::GRAPHICS)
                            && pdevice
                                .get_physical_device_surface_support_khr(Some(index as u32), surface.as_raw())
                                .unwrap();

                        supports_graphics_and_surface.then(|| index)
                    })
                    .map(|index| (pdevice, index as u32))
            })
            .expect("couldn't find a suitable device");

        info!(
            "We have chosen a physical device and queue family: {:?} and {}",
            physical_device.as_raw(),
            queue_family_index
        );

        // We get the properties of the device just so we can get its name.
        let properties = unsafe { physical_device.get_physical_device_properties() };

        info!(
            "Selected GPU: {}, max vulkan version: {}, driver version: {}",
            properties.device_name().as_cstr().to_string_lossy(),
            Version(properties.api_version()),
            Version(properties.driver_version())
        );

        // Enable the swapchain extension
        let mut device_extensions = device_extensions.enable_khr_swapchain();

        // Here we fetch the supported extensions that will be used to allocate memory
        // in a easier and more performant way using the Vulkan Memory Allocator.
        // Note that both the use of VMA and raw Vulkan allocations are covered in this
        // sample.
        Allocator::enable_extensions(&physical_device, &mut device_extensions, false)?;

        // We need to tell Vulkan that we want a queue from a certain family.
        // We got the family from the previous step. We will get one queue with max priority (1.0)
        let queue_info = DeviceQueueCreateInfo::default()
            .set_queue_family_index(queue_family_index)
            .set_queue_priorities(&[1.0]);

        // We get the names of the device extensions (if any)
        let device_extension_names = device_extensions.extension_names();

        // The physical device features we wish to enable, currently none
        let features = PhysicalDeviceFeatures::default();

        // We group up all of the information about the creation of the device.
        // This can be extended using pointer chains to contain **many** things.
        let device_create_info = DeviceCreateInfo::default()
            .set_queue_create_infos(std::slice::from_ref(&queue_info))
            .set_pp_enabled_extension_names(&device_extension_names)
            .set_enabled_features(&features);

        // Finally, we create the device.
        // You can imagine this step as "connecting" to the device, we are now ready to talk to it
        // and tell it what to allocate, do, deallocate, etc.
        let (device, _) = unsafe { physical_device.create_device(&device_create_info, None, device_extensions)? };

        info!("We have created a device: {:?}", device.as_raw());

        // Now, we will get the queue we created all the way back around line 180.
        // We know the family index, and we know the index within that family (being zero).
        // We can therefore get the queue out of the device.
        // Queues are very sensitive to synchronization, Magritte currently doesn't handle synchronization
        // automatically, be careful of this! Always read the documentation before dealing with queues.
        // Once you know the basics of queues, it will be alright!
        let graphics_queue = unsafe { device.get_device_queue(Some(queue_family_index), Some(0)) };

        info!("We have obtained the graphics queue: {:?}", graphics_queue.as_raw());

        // We create the allocator:
        let allocator = Allocator::new(&device, None, None)?;

        info!("Created the VMA allocator instance: {:?}", allocator.as_raw());

        // Fewww... after all of this work, we have the basic elements of a Vulkan-backed
        // game/application!
        Ok((
            Self {
                entry,
                instance,
                debug_utils,
                physical_device,
                device,
                allocator,
                graphics_queue,
                queue_family_index,
            },
            surface,
        ))
    }

    /// Get a reference to the vulkan's entry.
    pub fn entry(&self) -> &Entry {
        self.entry.as_ref()
    }

    /// Get a reference to the vulkan's instance.
    pub fn instance(&self) -> &Unique<Instance> {
        &self.instance
    }

    /// Get a reference to the vulkan's debug utils.
    pub fn debug_utils(&self) -> Option<&Unique<DebugUtilsMessengerEXT>> {
        self.debug_utils.as_ref()
    }

    /// Get a reference to the vulkan's physical device.
    pub fn physical_device(&self) -> &Unique<PhysicalDevice> {
        &self.physical_device
    }

    /// Get a reference to the vulkan's device.
    pub fn device(&self) -> &Unique<Device> {
        &self.device
    }

    /// Get a reference to the vulkan's graphics queue.
    pub fn graphics_queue(&self) -> &Unique<Queue> {
        &self.graphics_queue
    }

    /// Get a reference to the vulkan's queue family index.
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }

    /// Get a reference to the vulkan's allocator.
    pub fn allocator(&self) -> &Unique<Allocator> {
        &self.allocator
    }
}
