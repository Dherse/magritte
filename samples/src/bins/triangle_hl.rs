use std::{error::Error, ffi::CStr};

use clap::Parser;
use log::LevelFilter;
use magritte::{cstr, vulkan1_0::SampleCountFlagBits, InstanceExtensions, Version};
use magritte_hl::{context::Context, VulkanApplication};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

#[derive(Parser, Debug)]
struct Opts {
    /// Enables the Vulkan validation layers if they are present
    #[clap(short = 'l', long)]
    pub validation_layers: bool,

    /// The level of verbosity (1: INFO, 2: DEBUG, 3: TRACE)
    #[clap(short = 'v', long, parse(from_occurrences))]
    pub verbose: u8,

    /// The level of multisampling, must be a multiple of 2 (i.e 1, 2, 4, 8, 16).
    /// Some platforms even support very high MSAA such as 32 or 64 but that is rarer.
    #[clap(short = 'm', long, default_value_t = 1)]
    pub msaa: u32,

    /// The index of the physical device to use
    #[clap(short = 'g', long, default_value_t = 0)]
    pub gpu: u32,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();

    let msaa = unsafe { SampleCountFlagBits::from_bits_unchecked(opts.msaa) };

    // Initialization of logging
    pretty_env_logger::formatted_builder()
        .filter_level(match opts.verbose {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
        .init();

    // First we create the window and the event loop
    let event_loop = EventLoop::<()>::new();
    let window = WindowBuilder::new()
        .with_title("Magritte - Example")
        .with_inner_size(LogicalSize::new(1920.0_f64, 1080.0))
        .build(&event_loop)?;

    // Then we create the application instance
    let app = TriangleApplication::new(window, opts.validation_layers);

    // Then we create the Vulkan context
    let mut context = Context::new(app)?;

    Ok(())
}

struct TriangleApplication {
    window: Window,
    validation_layers: bool,
}

impl TriangleApplication {
    pub fn new(window: Window, validation_layers: bool) -> Self {
        Self {
            window,
            validation_layers,
        }
    }
}

impl VulkanApplication for TriangleApplication {
    fn name(&self) -> &CStr {
        cstr!("Triangle")
    }

    fn version(&self) -> Version {
        Version::new(1, 0, 0)
    }

    fn required_instance_extensions(&mut self, version: Version) -> InstanceExtensions {
        magritte::window::enable_required_extensions(&self.window, InstanceExtensions::from_version(version)).unwrap()
    }

    fn validation_layers(&mut self, available: bool) -> bool {
        self.validation_layers && available
    }

    fn window(&self) -> Option<&dyn magritte::window::HasRawWindowHandle> {
        Some(&self.window)
    }

    fn queues(&mut self, device: &magritte_hl::device::PhysicalDevice) -> Vec<magritte_hl::queue::QueueCreateInfo> {
        device
            .queue_families
            .iter()
            .find(|q| q.supports_graphics() && q.supports_surface)
            .map(|q| q.as_create_info(&[1.0], false))
            .into_iter()
            .collect()
    }
}
