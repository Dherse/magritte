#[path = "triangle/pipeline.rs"]
mod pipeline;

use std::{error::Error, mem::ManuallyDrop, time::Instant};

use bytemuck::{Pod, Zeroable};
use clap::Parser;
use log::{error, info, trace, LevelFilter};
use magritte::{
    extensions::khr_swapchain::PresentInfoKHR,
    vulkan1_0::{
        BufferUsageFlags, ClearColorValue, ClearDepthStencilValue, ClearValue, IndexType, Offset2D, PipelineBindPoint,
        PipelineStageFlags, Rect2D, RenderPassBeginInfo, SampleCountFlagBits, Semaphore, SemaphoreCreateInfo,
        SubpassContents,
    },
    AsRaw, DeviceExtensions, InstanceExtensions, SmallVec, Unique,
};

use magritte_samples::{
    buffer::Buffer, cache::PipelineCache, commands::Commands, depth::Depth, queue::Queue, render_target::RenderTarget,
    renderpass::RenderPass, surface::Surface, vulkan::Vulkan,
};
use pipeline::Pipeline;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::pipeline::PipelineShaders;

#[derive(Clone, Copy, Pod, Zeroable, Default)]
#[repr(C)]
pub struct Vertex {
    position: [f32; 4],
    color: [u8; 4],
}

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

    // We initialize the basics, from which we get a `Vulkan` and a `SurfaceKHR` protected in a
    // `Unique`. This is **not** part of Magritte, go see `Vulkan` to see how it works.
    let (vulkan, surface) = Vulkan::new(
        &window,
        InstanceExtensions::vulkan1_0(),
        DeviceExtensions::vulkan1_0(),
        opts.validation_layers,
        opts.gpu as usize,
    )?;

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
    let depth = Depth::new(&vulkan, &commands, &surface, msaa)?;

    let mut renderer = Renderer::new(vulkan, surface, commands, depth, msaa)?;

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
                if !renderer.rendering_enabled() {
                    return;
                }

                trace!("Drawing...");

                let frame = renderer.commands_mut().next_frame();
                if let Ok(swap_frame) = renderer
                    .surface()
                    .acquire_next_image(&renderer.present_complete_semaphores()[frame])
                {
                    let clear_values = [
                        ClearValue {
                            color: ClearColorValue { float32: [0.0; 4] },
                        },
                        ClearValue {
                            depth_stencil: ClearDepthStencilValue { depth: 1.0, stencil: 0 },
                        },
                    ];

                    let render_pass_begin_info = RenderPassBeginInfo::default()
                        .with_render_pass(renderer.renderpass().renderpass().as_raw())
                        .with_framebuffer(renderer.renderpass().framebuffers[swap_frame].as_raw())
                        .with_render_area(Rect2D {
                            offset: Offset2D { x: 0, y: 0 },
                            extent: renderer.surface().extent(),
                        })
                        .with_clear_values(&clear_values);

                    if let Err(e) = renderer.commands().record_and_submit_draw(
                        &[PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
                        &[renderer.present_complete_semaphores()[frame].as_raw()],
                        &[renderer.rendering_complete_semaphores()[frame].as_raw()],
                        |cmd| {
                            unsafe {
                                cmd.cmd_begin_render_pass(&render_pass_begin_info, SubpassContents::INLINE);

                                cmd.cmd_bind_pipeline(
                                    PipelineBindPoint::GRAPHICS,
                                    renderer.pipeline().pipeline().as_raw(),
                                );

                                cmd.cmd_bind_vertex_buffers(
                                    Some(0),
                                    Some(&[renderer.vertex_buffer().buffer().as_raw()]),
                                    &[0],
                                );

                                cmd.cmd_bind_index_buffer(
                                    renderer.index_buffer().buffer().as_raw(),
                                    0,
                                    IndexType::UINT32,
                                );

                                cmd.cmd_draw_indexed(Some(3), Some(1), Some(0), Some(0), Some(0));

                                cmd.cmd_end_render_pass();
                            }

                            Ok(())
                        },
                    ) {
                        error!("Failed to render: {:?}", e);
                        return;
                    }

                    let wait_semaphore = renderer.rendering_complete_semaphores()[frame].as_raw();
                    let swapchain = renderer.surface().swapchain().as_raw();
                    let image_index = swap_frame as u32;

                    if let Err(e) = renderer.commands().queue().present(
                        PresentInfoKHR::default()
                            .with_wait_semaphores(std::slice::from_ref(&wait_semaphore))
                            .with_swapchains(std::slice::from_ref(&swapchain))
                            .with_image_indices(std::slice::from_ref(&image_index)),
                    ) {
                        error!("Failed to present: {:?}", e);
                        return;
                    }
                }
            },
            _ => {},
        }
    })
}

pub struct Renderer {
    /// Is rendering enabled
    rendering_enabled: bool,

    /// The vulkan instance
    vulkan: Vulkan,

    /// The surface and swapchain
    surface: Surface,

    /// The command buffers and synchronization primitives
    commands: ManuallyDrop<Commands>,

    /// The depth buffer
    depth: Depth,

    msaa: SampleCountFlagBits,

    /// The optional multisampled image to be used as a render target
    msaa_images: Option<SmallVec<RenderTarget>>,

    /// The pipeline cache
    cache: PipelineCache,

    /// The pre-compiled pipeline shaders
    shaders: PipelineShaders,

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
    pub fn new(
        vulkan: Vulkan,
        surface: Surface,
        commands: Commands,
        depth: Depth,
        msaa: SampleCountFlagBits,
    ) -> Result<Self, Box<dyn Error>> {
        let msaa_images = if msaa.bits() <= 1 {
            None
        } else {
            let mut out = SmallVec::default();
            for _ in 0..surface.image_count() {
                out.push(RenderTarget::new(
                    &vulkan,
                    &commands,
                    &surface,
                    vulkan.allocator(),
                    surface.format(),
                    msaa,
                )?);
            }
            Some(out)
        };

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

        // Create the renderpass
        let renderpass = RenderPass::new(&vulkan, &surface, &depth, msaa, msaa_images.as_ref())?;

        // Create the index buffer
        let index_buffer = Buffer::new(&vulkan, BufferUsageFlags::INDEX_BUFFER, &[0u32, 1, 2])?;

        // Create the vertex buffer
        let vertex_buffer = Buffer::new(
            &vulkan,
            BufferUsageFlags::VERTEX_BUFFER,
            &[
                Vertex {
                    position: [-1.0, 1.0, 0.0, 1.0],
                    color: [0, 255, 0, 255],
                },
                Vertex {
                    position: [1.0, 1.0, 0.0, 1.0],
                    color: [255, 0, 0, 255],
                },
                Vertex {
                    position: [0.0, -1.0, 0.0, 1.0],
                    color: [0, 0, 255, 255],
                },
            ],
        )?;

        // Create the pipeline cache from a file
        let cache = PipelineCache::new(&vulkan, "./triangle_pipeline_cache.dat")?;

        // Pre-compile the shaders
        let shaders = PipelineShaders::new(&vulkan)?;

        // Create the pipeline
        let pipeline = Pipeline::new(&vulkan, &renderpass, &surface, &shaders, Some(&cache), msaa)?;

        Ok(Self {
            rendering_enabled: true,
            vulkan,
            surface,
            commands: ManuallyDrop::new(commands),
            msaa,
            msaa_images,
            depth,
            cache,
            shaders,
            renderpass,
            present_complete_semaphores,
            rendering_complete_semaphores,
            index_buffer,
            vertex_buffer,
            pipeline,
        })
    }

    pub fn resize(&mut self, window: &Window) -> Result<(), Box<dyn Error>> {
        // On windows, when minimizing, a resize event is produces with size of (0, 0).
        // This can cause the application to crash if we reallocate our buffers with that size.
        // For this reason, we need a guard to protect us against this scenario.
        // Instead, we will draw with a suboptimal swapchain.
        // In this example, we disable the rendering.
        if window.inner_size().width == 0 || window.inner_size().height == 0 {
            self.set_rendering_enabled(false);
            return Ok(());
        }

        self.set_rendering_enabled(true);

        let start = Instant::now();

        // We clear the command buffers
        self.commands_mut().wait_and_reset_all()?;

        info!("Reset all commands buffer at {:?}", start.elapsed());

        // Resize the surface
        self.surface.resize(&self.vulkan)?;

        info!("Recreated surface at {:?}", start.elapsed());

        // We created a new depth buffer
        self.depth = Depth::new(self.vulkan(), self.commands(), self.surface(), self.msaa())?;

        info!("Recreated depth texture at {:?}", start.elapsed());

        // We update the MSAA resolve images
        self.msaa_images = if self.msaa().bits() <= 1 {
            None
        } else {
            let mut out = SmallVec::default();
            for _ in 0..self.surface().image_count() {
                out.push(RenderTarget::new(
                    self.vulkan(),
                    self.commands(),
                    self.surface(),
                    self.vulkan().allocator(),
                    self.surface().format(),
                    self.msaa(),
                )?);
            }

            info!("Recreated MSAA textures at {:?}", start.elapsed());

            Some(out)
        };

        // We update the framebuffers
        self.renderpass
            .resize(&self.surface, &self.depth, self.msaa_images.as_ref())?;

        info!("Recreated renderpass at {:?}", start.elapsed());

        // We recreate the pipeline
        self.pipeline = Pipeline::new(
            self.vulkan(),
            self.renderpass(),
            self.surface(),
            self.shaders(),
            Some(self.cache()),
            self.msaa(),
        )?;

        info!("Recreated pipeline at {:?}", start.elapsed());

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

    /// Get a reference to the renderer's renderpass.
    pub fn renderpass(&self) -> &RenderPass {
        &self.renderpass
    }

    /// Get a mutable reference to the renderer's renderpass.
    pub fn renderpass_mut(&mut self) -> &mut RenderPass {
        &mut self.renderpass
    }

    /// Get a reference to the renderer's index buffer.
    pub fn index_buffer(&self) -> &Buffer<u32> {
        &self.index_buffer
    }

    /// Get a reference to the renderer's vertex buffer.
    pub fn vertex_buffer(&self) -> &Buffer<Vertex> {
        &self.vertex_buffer
    }

    /// Get a reference to the renderer's pipeline.
    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    /// Get the renderer's rendering enabled.
    pub fn rendering_enabled(&self) -> bool {
        self.rendering_enabled
    }

    /// Set the renderer's rendering enabled.
    pub fn set_rendering_enabled(&mut self, rendering_enabled: bool) {
        self.rendering_enabled = rendering_enabled;
    }

    /// Get a reference to the renderer's cache.
    #[must_use]
    pub fn cache(&self) -> &PipelineCache {
        &self.cache
    }

    /// Get a reference to the renderer's shaders.
    #[must_use]
    pub fn shaders(&self) -> &PipelineShaders {
        &self.shaders
    }

    /// Get a reference to the renderer's msaa images.
    #[must_use]
    pub fn msaa_images(&self) -> Option<&SmallVec<RenderTarget>> {
        self.msaa_images.as_ref()
    }

    /// Get the renderer's msaa.
    #[must_use]
    pub fn msaa(&self) -> SampleCountFlagBits {
        self.msaa
    }
}
