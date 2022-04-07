use std::{error::Error, sync::Arc};

use magritte::Extensions;

use magritte_samples::{surface::Surface, vulkan::Vulkan};
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};

pub fn main() -> Result<(), Box<dyn Error>> {
    // Initialization of logging
    pretty_env_logger::init();

    // First we create the window and the event loop
    let event_loop = EventLoop::<()>::new();
    let window = WindowBuilder::new()
        .with_title("Magritte - Example")
        .with_inner_size(LogicalSize::new(1920.0_f64, 1080.0))
        .build(&event_loop)?;

    // We initialize the basics, from which we get a `Vulkan` and a `SurfaceKHR` protected in a
    // `Unique`. This is **not** part of Magritte, go see `Vulkan` to see how it works.
    let (vulkan, surface) = Vulkan::new(&window, Extensions::default(), true)?;

    // We wrap the Vulkan into an arc
    let vulkan = Arc::new(vulkan);

    // Now that we have the basic state and the surface, we will create all additional
    // state required to get a swapchain!
    // This is **not** part of Magritte, go see `Surface` to see how it works.
    let surface = Surface::new(&vulkan, surface)?;

    Ok(())
}
