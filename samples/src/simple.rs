use std::{borrow::Cow, error::Error, ffi::CStr};

use log::{debug, error, info, trace, warn};
use magritte::{
    cstr,
    entry::Entry,
    extensions::{
        ext_debug_utils::{
            DebugUtilsMessageSeverityFlagBitsEXT, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
            DebugUtilsMessengerCallbackDataEXT, DebugUtilsMessengerCreateInfoEXT,
        },
        khr_display::SurfaceTransformFlagsKHR,
        khr_surface::{CompositeAlphaFlagBitsKHR, PresentModeKHR, SurfaceTransformFlagBitsKHR},
        khr_swapchain::SwapchainCreateInfoKHR,
    },
    vulkan1_0::{
        ApplicationInfo, Bool32, CommandBufferAllocateInfo, CommandBufferLevel, CommandPoolCreateFlags,
        CommandPoolCreateInfo, DeviceCreateInfo, DeviceQueueCreateInfo, Extent2D, ImageUsageFlags, InstanceCreateInfo,
        PhysicalDeviceFeatures, QueueFlags, SharingMode, FALSE, ImageViewCreateInfo, ImageViewType, ComponentMapping, ComponentSwizzle, ImageSubresourceRange, ImageAspectFlags, FenceCreateInfo, FenceCreateFlags, SemaphoreCreateInfo, Instance, Device, Queue, CommandPool, CommandBuffer, Semaphore, Fence, PhysicalDevice,
    },
    AsRaw, Extensions, Version, SmallVec, Unique,
};

use winit::{event_loop::EventLoop, window::WindowBuilder};

pub fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let event_loop = EventLoop::<()>::new();
    let window = WindowBuilder::new()
        .with_title("Magritte - Example")
        .with_inner_size(winit::dpi::LogicalSize::new(f64::from(1920), f64::from(1080)))
        .build(&event_loop)?;

    let entry = Entry::new()?;

    let version = unsafe { entry.enumerate_instance_version() }.unwrap();

    info!("Maximum supported version: {}", version);

    let extensions = magritte::window::enable_required_extensions(&window, Extensions::from_version(Version::VULKAN1_0))?
        .enable_ext_debug_utils()
        .enable_khr_swapchain();

    let ext_list = extensions
        .instance_extension_names()
        .into_iter()
        .map(CStr::as_ptr)
        .collect::<Vec<_>>();

    let app_info = ApplicationInfo::default().set_api_version(version.into());

    let layer_names = [cstr!("VK_LAYER_KHRONOS_validation").as_ptr()];

    let create_info = InstanceCreateInfo::default()
        .set_application_info(&app_info)
        .set_pp_enabled_extension_names(&ext_list)
        .set_pp_enabled_layer_names(&layer_names[..]);

    let instance = unsafe { entry.create_instance(&create_info, None, extensions).unwrap() };

    let (physical_devices, _) = unsafe { instance.enumerate_physical_devices(None)? };

    let surface = unsafe { magritte::window::create_surface(&instance, &window, None)?.0 };

    let debug_info = DebugUtilsMessengerCreateInfoEXT::default()
        .set_message_severity(
            DebugUtilsMessageSeverityFlagsEXT::ERROR
                | DebugUtilsMessageSeverityFlagsEXT::WARNING
                | DebugUtilsMessageSeverityFlagsEXT::INFO,
        )
        .set_message_type(
            DebugUtilsMessageTypeFlagsEXT::GENERAL
                | DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .set_pfn_user_callback(Some(vulkan_debug_callback));

    let _debug_utils = unsafe { instance.create_debug_utils_messenger_ext(&debug_info, None)?.0 };

    let (pdevice, queue_family_index) = physical_devices
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
        .expect("couldn't find a device");

    info!("Found device: {:?}", pdevice.as_raw());

    let queue_info = DeviceQueueCreateInfo::default()
        .set_queue_family_index(queue_family_index)
        .set_queue_priorities(&[1.0]);

    let device_extensions = extensions
        .device_extension_names()
        .into_iter()
        .map(|c| c.as_ptr())
        .collect::<Vec<_>>();

    let features = PhysicalDeviceFeatures::default();
    let device_create_info = DeviceCreateInfo::default()
        .set_queue_create_infos(std::slice::from_ref(&queue_info))
        .set_pp_enabled_extension_names(&device_extensions)
        .set_enabled_features(&features);

    let (device, _) = unsafe { pdevice.create_device(&device_create_info, None)? };

    info!("Created device: {:?}", device.as_raw());

    let queue = unsafe { device.get_device_queue(Some(queue_family_index), Some(0)) };

    let (mut surface_formats, _) =
        unsafe { pdevice.get_physical_device_surface_formats_khr(Some(surface.as_raw()), None)? };

    let surface_format = surface_formats.remove(0);
    info!("Using format {:?}", surface_format);

    let (surface_capabilities, _) = unsafe { pdevice.get_physical_device_surface_capabilities_khr(surface.as_raw())? };

    let mut desired_image_count = surface_capabilities.min_image_count() + 1;
    if surface_capabilities.max_image_count() > 0 && desired_image_count > surface_capabilities.max_image_count() {
        desired_image_count = surface_capabilities.max_image_count();
    }

    let surface_resolution = match surface_capabilities.current_extent().width() {
        std::u32::MAX => Extent2D {
            width: 1920,
            height: 1080,
        },
        _ => surface_capabilities.current_extent(),
    };

    let pre_transform = if surface_capabilities
        .supported_transforms()
        .contains(SurfaceTransformFlagsKHR::IDENTITY)
    {
        SurfaceTransformFlagBitsKHR::IDENTITY
    } else {
        surface_capabilities.current_transform()
    };

    let (present_modes, _) =
        unsafe { pdevice.get_physical_device_surface_present_modes_khr(Some(surface.as_raw()), None)? };

    let present_mode = present_modes
        .iter()
        .cloned()
        .find(|&mode| mode == PresentModeKHR::MAILBOX)
        .unwrap_or(PresentModeKHR::FIFO);

    let swapchain_create_info = SwapchainCreateInfoKHR::default()
        .set_surface(surface.as_raw())
        .set_min_image_count(desired_image_count)
        .set_image_color_space(surface_format.color_space())
        .set_image_format(surface_format.format())
        .set_image_extent(surface_resolution)
        .set_image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
        .set_image_sharing_mode(SharingMode::EXCLUSIVE)
        .set_pre_transform(pre_transform)
        .set_composite_alpha(CompositeAlphaFlagBitsKHR::OPAQUE)
        .set_present_mode(present_mode)
        .set_clipped(true)
        .set_image_array_layers(1);

    let (swapchain, _) = unsafe { device.create_swapchain_khr(&swapchain_create_info, None)? };

    info!("Created swapchain: {:?}", swapchain.as_raw());

    let pool_create_info = CommandPoolCreateInfo::default()
        .set_flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .set_queue_family_index(queue_family_index);

    let (command_pool, _) = unsafe { device.create_command_pool(&pool_create_info, None)? };

    info!("Created command pool: {:?}", command_pool.as_raw());

    let command_buffer_allocate_info = CommandBufferAllocateInfo::default()
        .set_command_buffer_count(2)
        .set_command_pool(command_pool.as_raw())
        .set_level(CommandBufferLevel::PRIMARY);

    let (mut command_buffers, _) = unsafe { command_pool.allocate_command_buffers(&command_buffer_allocate_info)? };

    let setup_command_buffer = command_buffers.pop().unwrap();
    let draw_command_buffer = command_buffers.pop().unwrap();

    let (present_images, _) = unsafe { swapchain.get_swapchain_images_khr(swapchain.as_raw(), None)? };

    let present_image_views = present_images
        .iter()
        .map(|image| {
            let create_view_info = ImageViewCreateInfo::default()
                .set_view_type(ImageViewType::_2_D)
                .set_format(surface_format.format)
                .set_components(ComponentMapping {
                    r: ComponentSwizzle::R,
                    g: ComponentSwizzle::G,
                    b: ComponentSwizzle::B,
                    a: ComponentSwizzle::A,
                })
                .set_subresource_range(ImageSubresourceRange {
                    aspect_mask: ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                })
                .set_image(image.as_raw_image());

            unsafe {
                image.create_swapchain_image_view(
                    &create_view_info,
                    None,
                ).result()
            }
        })
        .collect::<Result<SmallVec<_>, _>>()?;

    let fence_create_info = FenceCreateInfo::default().set_flags(FenceCreateFlags::SIGNALED);

    let (draw_commands_reuse_fence, _) = unsafe {
        device.create_fence(&fence_create_info, None)?
    };

    let (setup_commands_reuse_fence, _) = unsafe {
        device.create_fence(&fence_create_info, None)?
    };

    let semaphore_create_info = SemaphoreCreateInfo::default();

    let (present_complete_semaphore, _) = unsafe {
        device.create_semaphore(&semaphore_create_info, None)?
    };

    let (rendering_complete_semaphore, _) = unsafe {
        device.create_semaphore(&semaphore_create_info, None)?
    };


    Ok(())
}

pub struct State<'a> {
    event_loop: EventLoop<()>,
    entry: &'a Entry,
    instance: &'a Unique<'a, 'static, Instance>,
}

impl<'state> State<'state> {
    pub fn new<'a>(
        event_loop: EventLoop<()>,
        entry: &'a Entry,
        instance: &'a Unique<'a, 'static, Instance>,
    ) -> State<'a> {
        State::<'a> {
            event_loop,
            entry,
            instance,
        }
    }
}

unsafe extern "system" fn vulkan_debug_callback<'lt>(
    message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
    message_type: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'lt>,
    _user_data: *mut std::os::raw::c_void,
) -> Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number: i32 = callback_data.message_id_number as i32;

    let message_id_name = if callback_data.message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.message_id_name).to_string_lossy()
    };

    let message = if callback_data.message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.message).to_string_lossy()
    };

    match message_severity {
        DebugUtilsMessageSeverityFlagBitsEXT::VERBOSE => trace!(
            "{:?} [{} ({})] : {}",
            message_type,
            message_id_name,
            &message_id_number.to_string(),
            message,
        ),
        DebugUtilsMessageSeverityFlagBitsEXT::INFO => debug!(
            "{:?} [{} ({})] : {}",
            message_type,
            message_id_name,
            &message_id_number.to_string(),
            message,
        ),
        DebugUtilsMessageSeverityFlagBitsEXT::WARNING => warn!(
            "{:?} [{} ({})] : {}",
            message_type,
            message_id_name,
            &message_id_number.to_string(),
            message,
        ),
        DebugUtilsMessageSeverityFlagBitsEXT::ERROR => error!(
            "{:?} [{} ({})] : {}",
            message_type,
            message_id_name,
            &message_id_number.to_string(),
            message,
        ),
        other => error!(
            "[UNKNOWN: {:0X}] {:?} [{} ({})] : {}",
            other.bits(),
            message_type,
            message_id_name,
            &message_id_number.to_string(),
            message,
        ),
    }

    FALSE
}
