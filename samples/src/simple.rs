use std::{borrow::Cow, error::Error, ffi::CStr};

use magritte::{
    cstr,
    entry::Entry,
    extensions::{
        ext_debug_utils::{
            DebugUtilsMessageSeverityFlagBitsEXT, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
            DebugUtilsMessengerCallbackDataEXT, DebugUtilsMessengerCreateInfoEXT,
        },
        khr_display::SurfaceTransformFlagsKHR, khr_surface::{PresentModeKHR, CompositeAlphaFlagsKHR, SurfaceTransformFlagBitsKHR, CompositeAlphaFlagBitsKHR}, khr_swapchain::SwapchainCreateInfoKHR,
    },
    vulkan1_0::{
        ApplicationInfo, Bool32, DeviceCreateInfo, DeviceQueueCreateInfo, Extent2D, InstanceCreateInfo,
        PhysicalDeviceFeatures, QueueFlags, FALSE, ImageUsageFlags, SharingMode, CommandPoolCreateInfo, CommandPoolCreateFlags, CommandBufferAllocateInfo, CommandBufferLevel,
    },
    AsRaw, Extensions, SmallVec, Unique, Version,
};

use winit::{event_loop::EventLoop, window::WindowBuilder};

pub fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::<()>::new();
    let window = WindowBuilder::new()
        .with_title("Magritte - Example")
        .with_inner_size(winit::dpi::LogicalSize::new(f64::from(1920), f64::from(1080)))
        .build(&event_loop)?;

    let entry = Entry::new()?;

    let version = unsafe { entry.enumerate_instance_version() }.unwrap();

    println!("Maximum supported version: {}", version);

    let extensions = magritte::window::enable_required_extensions(&window, Extensions::from_version(version))?
        .enable_ext_debug_utils()
        .enable_khr_swapchain();

    let mut ext_list = extensions
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
            DebugUtilsMessageSeverityFlagsEXT::DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_EXT
                | DebugUtilsMessageSeverityFlagsEXT::DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_EXT
                | DebugUtilsMessageSeverityFlagsEXT::DEBUG_UTILS_MESSAGE_SEVERITY_INFO_EXT,
        )
        .set_message_type(
            DebugUtilsMessageTypeFlagsEXT::DEBUG_UTILS_MESSAGE_TYPE_GENERAL_EXT
                | DebugUtilsMessageTypeFlagsEXT::DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_EXT
                | DebugUtilsMessageTypeFlagsEXT::DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_EXT,
        )
        .set_pfn_user_callback(Some(vulkan_debug_callback));

    let debug_utils = unsafe { instance.create_debug_utils_messenger_ext(&debug_info, None)?.0 };

    let (pdevice, queue_family_index) = physical_devices
        .into_iter()
        .find_map(|pdevice| unsafe {
            let families = pdevice.get_physical_device_queue_family_properties(None);
            families
                .iter()
                .enumerate()
                .find_map(|(index, info)| {
                    let supports_graphics_and_surface = info.queue_flags().contains(QueueFlags::QUEUE_GRAPHICS)
                        && pdevice
                            .get_physical_device_surface_support_khr(Some(index as u32), surface.as_raw())
                            .unwrap();

                    supports_graphics_and_surface.then(|| index)
                })
                .map(|index| (pdevice, index))
        })
        .expect("couldn't find a device");

    println!("Found device: {:?}", pdevice.as_raw());

    let queue_info = DeviceQueueCreateInfo::default()
        .set_queue_family_index(queue_family_index as u32)
        .set_queue_priorities(&[1.0]);

    let features = PhysicalDeviceFeatures::default();

    let device_extensions = extensions
        .device_extension_names()
        .into_iter()
        .map(|c| c.as_ptr())
        .collect::<Vec<_>>();
    let device_create_info = DeviceCreateInfo::default()
        .set_queue_create_infos(std::slice::from_ref(&queue_info))
        .set_pp_enabled_extension_names(&device_extensions)
        .set_enabled_features(&features);

    let (device, _) = unsafe { pdevice.create_device(&device_create_info, None)? };

    println!("Created device: {:?}", device.as_raw());

    let queue = unsafe { device.get_device_queue(Some(queue_family_index as u32), Some(0)) };

    let (mut surface_formats, _) =
        unsafe { pdevice.get_physical_device_surface_formats_khr(Some(surface.as_raw()), None)? };

    let surface_format = surface_formats.remove(0);
    println!("Using format {:?}", surface_format);

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
        .contains(SurfaceTransformFlagsKHR::SURFACE_TRANSFORM_IDENTITY_KHR)
    {
        SurfaceTransformFlagBitsKHR::SurfaceTransformIdentityKhr
    } else {
        surface_capabilities.current_transform()
    };

    let (present_modes, _) = unsafe {
        pdevice.get_physical_device_surface_present_modes_khr(Some(surface.as_raw()), None)?
    };

    let present_mode = present_modes
        .iter()
        .cloned()
        .find(|&mode| mode == PresentModeKHR::PresentModeMailboxKhr)
        .unwrap_or(PresentModeKHR::PresentModeFifoKhr);

    let swapchain_create_info = SwapchainCreateInfoKHR::default()
        .set_surface(surface.as_raw())
        .set_min_image_count(desired_image_count)
        .set_image_color_space(surface_format.color_space())
        .set_image_format(surface_format.format())
        .set_image_extent(surface_resolution)
        .set_image_usage(ImageUsageFlags::IMAGE_USAGE_COLOR_ATTACHMENT)
        .set_image_sharing_mode(SharingMode::Exclusive)
        .set_pre_transform(pre_transform)
        .set_composite_alpha(CompositeAlphaFlagBitsKHR::CompositeAlphaOpaqueKhr)
        .set_present_mode(present_mode)
        .set_clipped(true)
        .set_image_array_layers(1);

    let (swapchain, _) = unsafe { device.create_swapchain_khr(&swapchain_create_info, None)? };

    println!("Created swapchain: {:?}", swapchain.as_raw());

    let pool_create_info = CommandPoolCreateInfo::default()
        .set_flags(CommandPoolCreateFlags::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER)
        .set_queue_family_index(queue_family_index as u32);

    let (pool, _) = unsafe {
        device.create_command_pool(&pool_create_info, None)?
    };

    println!("Created command pool: {:?}", pool.as_raw());

    let command_buffer_allocate_info = CommandBufferAllocateInfo::default()
        .set_command_buffer_count(2)
        .set_command_pool(pool.as_raw())
        .set_level(CommandBufferLevel::Primary);

    let (mut command_buffers, _) = unsafe {
        device.allocate_command_buffers(&command_buffer_allocate_info, &pool)?
    };

    let setup_command_buffer = command_buffers.pop().unwrap();
    let draw_command_buffer = command_buffers.pop().unwrap();

    let (present_images, _) = unsafe {
        device.get_swapchain_images_khr(swapchain.as_raw(), None)?
    };

    /*let present_image_views = present_images
        .iter()
        .map(|image| {

        })*/

    Ok(())
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

    println!(
        "{:?}:\n{:?} [{} ({})] : {}\n",
        message_severity,
        message_type,
        message_id_name,
        &message_id_number.to_string(),
        message,
    );

    FALSE
}
