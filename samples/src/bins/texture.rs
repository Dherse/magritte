#[path = "texture/pipeline.rs"]
mod pipeline;

use std::{error::Error, mem::ManuallyDrop, time::Instant};

use bytemuck::{Pod, Zeroable};
use log::{error, info, trace};
use magritte::{
    vulkan1_0::{
        BorderColor, BufferUsageFlags, CompareOp, Filter, Sampler, SamplerAddressMode, SamplerCreateInfo,
        SamplerMipmapMode, Semaphore, SemaphoreCreateInfo,
    },
    window::create_surface,
    DeviceExtensions, InstanceExtensions, SmallVec, Unique,
};
use magritte_samples::{
    buffer::Buffer, commands::Commands, depth::Depth, queue::Queue, renderpass::RenderPass, surface::Surface,
    vulkan::Vulkan,
};
use pipeline::Pipeline;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[derive(Clone, Copy, Pod, Zeroable, PartialEq, Default)]
#[repr(C)]
pub struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
}

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
    let (vulkan, surface) = Vulkan::new(&window, InstanceExtensions::default(), DeviceExtensions::default(), true)?;

    // Now that we have the basic state and the surface, we will create all additional
    // state required to get a swapchain!
    // This is **not** part of Magritte, go see `Surface` to see how it works.
    let surface = Surface::new(&vulkan, surface)?;

    // Here we create a helper thread that will **always** be synchronized with the
    // queue, making it much easier to use. In addition, this thread is asynchronous, meaning
    // we could await in a completely different place or even implement `Queue` using
    // asynchronous channels and functions!
    let queue = Queue::new(vulkan.graphics_queue().clone(), vulkan.queue_family_index());

    // Here, we will create the command pool and the command buffers. We use a little helper
    // to contain all of the state and deal with the initialization of the command buffers
    // when we want to record commands in them. This makes it easier and hides some of the redundant
    // code.
    let commands = Commands::new(queue, surface.swapchain_images().len())?;

    // Here we create a depth buffer we will later use.
    let depth = Depth::new(&vulkan, &commands, &surface)?;

    let mut renderer = Renderer::new(vulkan, surface, commands, depth)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(_) => {
                    if let Err(e) = renderer.resize(&window) {
                        error!("Failed to resize: {:?}", e);
                    }
                },
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                _ => {},
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                trace!("Drawing...");
            },
            _ => {},
        }
    });
}

pub struct Renderer {
    /// The vulkan instance
    vulkan: Vulkan,

    /// The surface and swapchain
    surface: Surface,

    /// The command buffers and synchronization primitives
    commands: ManuallyDrop<Commands>,

    /// The depth buffer
    depth: Depth,

    /// The renderpass
    renderpass: RenderPass,

    /// The graphics pipeline
    pipeline: Pipeline,

    /// The semaphores signaling that we have presented the image
    present_complete_semaphores: SmallVec<Unique<Semaphore>>,

    /// The semaphores signaling that we have completed rendering
    rendering_complete_semaphores: SmallVec<Unique<Semaphore>>,

    // The index buffer we will render
    index_buffer: Buffer<u32>,

    // The vertex buffer we will render
    vertex_buffer: Buffer<Vertex>,

    // The sampler for sampling the texture
    sampler: Unique<Sampler>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        // As a caveat, we need to drop the command buffers first to enforce that they release all of their
        // locks! Magritte doesn't track usage in command buffers, therefore you will have to do it
        // manually in some places. The goal of Magritte is to decrease the necessity of these
        // manual operations, but they cannot trivially all be removed!
        unsafe {
            ManuallyDrop::drop(&mut self.commands);
        }
    }
}

impl Renderer {
    /// Creates a new renderer
    pub fn new(vulkan: Vulkan, surface: Surface, commands: Commands, depth: Depth) -> Result<Self, Box<dyn Error>> {
        let mut present_complete_semaphores = SmallVec::with_capacity(surface.image_count());
        let mut rendering_complete_semaphores = SmallVec::with_capacity(surface.image_count());

        let semaphore_create_info = SemaphoreCreateInfo::default();
        for _ in 0..surface.image_count() {
            present_complete_semaphores.push(unsafe {
                vulkan
                    .device()
                    .create_semaphore(&semaphore_create_info, None)
                    .result()?
            });

            rendering_complete_semaphores.push(unsafe {
                vulkan
                    .device()
                    .create_semaphore(&semaphore_create_info, None)
                    .result()?
            });
        }

        info!("Created {} semaphores", surface.image_count() * 2);

        let renderpass = RenderPass::new(&vulkan, &surface, &depth)?;

        let pipeline = Pipeline::new(&vulkan, &renderpass, &surface)?;

        // Create the index buffer
        let index_buffer = Buffer::new(&vulkan, BufferUsageFlags::INDEX_BUFFER, &[0u32, 1, 2, 2, 3, 0])?;

        // Create the vertex buffer
        let vertex_buffer = Buffer::new(
            &vulkan,
            BufferUsageFlags::VERTEX_BUFFER,
            &[
                Vertex {
                    position: [-0.5, -0.5],
                    uv: [0.0, 0.0],
                },
                Vertex {
                    position: [-0.5, 0.5],
                    uv: [0.0, 1.0],
                },
                Vertex {
                    position: [0.5, 0.5],
                    uv: [1.0, 1.0],
                },
                Vertex {
                    position: [0.5, -0.5],
                    uv: [1.0, 0.0],
                },
            ],
        )?;

        let sampler_create_info = SamplerCreateInfo::default()
            .set_mag_filter(Filter::LINEAR)
            .set_min_filter(Filter::LINEAR)
            .set_mipmap_mode(SamplerMipmapMode::NEAREST)
            .set_address_mode_u(SamplerAddressMode::REPEAT)
            .set_address_mode_v(SamplerAddressMode::REPEAT)
            .set_address_mode_w(SamplerAddressMode::REPEAT)
            .set_max_anisotropy(1.0)
            .set_border_color(BorderColor::FLOAT_OPAQUE_BLACK)
            .set_compare_op(CompareOp::NEVER);

        let (sampler, _) = unsafe { vulkan.device().create_sampler(&sampler_create_info, None)? };

        Ok(Self {
            vulkan,
            surface,
            commands: ManuallyDrop::new(commands),
            depth,
            present_complete_semaphores,
            rendering_complete_semaphores,
            renderpass,
            pipeline,
            index_buffer,
            vertex_buffer,
            sampler,
        })
    }

    pub fn resize(&mut self, window: &Window) -> Result<(), Box<dyn Error>> {
        if window.inner_size().width == 0 || window.inner_size().height == 0 {
            return Ok(());
        }

        let start = Instant::now();

        // We clear the command buffers
        self.commands_mut().wait_and_reset_all()?;

        let (surface, _) = unsafe { create_surface(self.vulkan().instance(), window, None)? };

        // Resize the surface
        self.surface = Surface::new(self.vulkan(), surface)?;

        // We created a new depth buffer
        self.depth = Depth::new(self.vulkan(), self.commands(), self.surface())?;

        // We update the framebuffers
        self.renderpass.resize(&self.surface, &self.depth)?;

        // We recreate the pipeline
        self.pipeline = Pipeline::new(self.vulkan(), self.renderpass(), self.surface())?;

        info!("Resizing took {:?}", start.elapsed());

        Ok(())
    }

    /// Get a reference to the renderer's vulkan.
    #[inline]
    pub fn vulkan(&self) -> &Vulkan {
        &self.vulkan
    }

    /// Get a reference to the renderer's surface.
    #[inline]
    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    /// Get a mutable reference to the renderer's surface.
    pub fn surface_mut(&mut self) -> &mut Surface {
        &mut self.surface
    }

    /// Get a reference to the renderer's commands.
    #[inline]
    pub fn commands(&self) -> &Commands {
        &*self.commands
    }

    /// Get a mutable reference to the renderer's commands.
    pub fn commands_mut(&mut self) -> &mut Commands {
        &mut *self.commands
    }

    /// Get a reference to the renderer's depth.
    pub fn depth(&self) -> &Depth {
        &self.depth
    }

    /// Get a mutable reference to the renderer's depth.
    pub fn depth_mut(&mut self) -> &mut Depth {
        &mut self.depth
    }

    /// Get a reference to the renderer's present complete semaphores.
    pub fn present_complete_semaphores(&self) -> &SmallVec<Unique<Semaphore>> {
        &self.present_complete_semaphores
    }

    /// Get a reference to the renderer's rendering complete semaphores.
    pub fn rendering_complete_semaphores(&self) -> &SmallVec<Unique<Semaphore>> {
        &self.rendering_complete_semaphores
    }

    /// Get a reference to the renderer's index buffer.
    pub fn index_buffer(&self) -> &Buffer<u32> {
        &self.index_buffer
    }

    /// Get a reference to the renderer's vertex buffer.
    pub fn vertex_buffer(&self) -> &Buffer<Vertex> {
        &self.vertex_buffer
    }

    /// Get a reference to the renderer's renderpass.
    pub fn renderpass(&self) -> &RenderPass {
        &self.renderpass
    }

    /// Get a reference to the renderer's pipeline.
    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    /// Get a reference to the renderer's sampler.
    pub fn sampler(&self) -> &Unique<Sampler> {
        &self.sampler
    }
}
