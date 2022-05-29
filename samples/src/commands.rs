use log::{error, info, trace};
use magritte::{
    vulkan1_0::{
        CommandBuffer, CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferLevel, CommandBufferResetFlags,
        CommandBufferUsageFlags, CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo, Fence, FenceCreateFlags,
        FenceCreateInfo, PipelineStageFlags, Semaphore, SubmitInfo, VulkanResultCodes,
    },
    AsRaw, SmallVec, Unique,
};

use crate::queue::Queue;

/// Helper for command pool and buffers.
/// This structure contains the command pool and two command buffers
/// required for the execution of this sample. Note that this helper is
/// mostly here to enforce synchronization of command buffers until
/// Magritte has a built-in solution for this non-trivial problem.
pub struct Commands {
    /// The queue on which these command buffers will be executed
    queue: Queue,

    /// The command pool from which we allocate the command buffers
    command_pool: Unique<CommandPool>,

    /// The command buffer used when setting up resources for our sample
    setup_command_buffer: Unique<CommandBuffer>,

    /// The fence that guards the setup command buffer
    setup_reuse_fence: Unique<Fence>,

    /// The command buffer used when drawing things on the screen
    draw_command_buffers: SmallVec<Unique<CommandBuffer>>,

    /// The fences that guard the draw command buffers
    draw_reuse_fences: SmallVec<Unique<Fence>>,

    /// The total number of frames
    frame_count: usize,

    /// The current frame index in range `0..self.frame_count`
    frame_index: usize,
}

impl Drop for Commands {
    fn drop(&mut self) {
        if let Err(e) = self.wait_and_reset_all() {
            error!("Failed to wait and reset all: {:?}", e);
        }
    }
}

impl Commands {
    pub fn wait_and_reset_all(&mut self) -> Result<(), VulkanResultCodes> {
        // First we wait on **all** the fences to make sure they are done executing
        unsafe {
            self.setup_reuse_fence.device().wait_for_fences(
                &[self.setup_reuse_fence.as_raw()],
                true,
                Some(std::u64::MAX),
            );

            // self.setup_reuse_fence.device().reset_fences(&[ self.setup_reuse_fence.as_raw() ])?;
        }

        unsafe {
            let fences = self
                .draw_reuse_fences
                .iter()
                .map(AsRaw::as_raw)
                .collect::<SmallVec<_>>();
            self.setup_reuse_fence
                .device()
                .wait_for_fences(&fences, true, Some(std::u64::MAX));
            // self.setup_reuse_fence.device().reset_fences(&fences)?;
        }

        // Then we reset all of the command buffers releasing all of their ownerships

        for cmd in self
            .draw_command_buffers
            .iter()
            .chain(std::iter::once(&self.setup_command_buffer))
        {
            unsafe {
                cmd.reset_command_buffer(CommandBufferResetFlags::RELEASE_RESOURCES)?;
            }
        }

        Ok(())
    }

    pub fn new(queue: Queue, frame_count: usize) -> Result<Self, VulkanResultCodes> {
        // Here we gather the relevant information about the command pool we wish to
        // create.
        // Currently, we tell it:
        // - The queue family we want it to be associated with; therefore we can this pool for more than one
        //   queue as long as they are part of the same family. to learn more, it might be worth it to read
        //   up on it or to list the queue families, their properties and their number in your computer.
        //   Mine for example has two graphics queues, two transfer queues but only one compute queue.
        // - We tell Vulkan that we will be resetting the command buffers allocated from this pool. This
        //   means that those command buffers are reused. It is perfectly possible to reallocate a buffer
        //   every time you need one. On some platforms this may even be faster, but it's unlikely.
        //
        // As with most structures in Vulkan, you can extend it using its pointer chain, something that
        // is not covered in this simple sample.
        let pool_create_info = CommandPoolCreateInfo::default()
            .with_queue_family_index(queue.queue_family_index())
            .with_flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        // Here we will create a command pool.
        // A command pool allows use to allocate command buffers with which we will
        // tell the GPU what work we want it to do.
        let (command_pool, _) = unsafe { queue.device().create_command_pool(&pool_create_info, None)? };

        info!("Created command pool: {:?}", command_pool.as_raw());

        // Now we gether information for what command buffers we want to allocate:
        // - We want two command buffers
        // - We tell it the command pool we want to allocate from
        // - We tell it the level of the command buffer, primary means that we can submit it directly to the
        //   GPU, otherwise we would need to call it from a primary command buffer.
        let command_buffer_allocate_info = CommandBufferAllocateInfo::default()
            .with_command_buffer_count(1 + frame_count as u32)
            .with_command_pool(command_pool.as_raw())
            .with_level(CommandBufferLevel::PRIMARY);

        // We can now allocate the command buffers using the previously gathered information.
        // We will allocate more than one draw command buffer and more than one fence, the goal is
        // to use one of these buffers and one of these fences per frame to try and decrease the wait time
        // between frames.
        // ⚠ Note that the command pool defined in the `CommandBufferAllocateInfo` and the pool we are
        // allocating from **must** be the same!
        let (mut command_buffers, _) = unsafe { command_pool.allocate_command_buffers(&command_buffer_allocate_info)? };

        let setup_command_buffer = command_buffers.pop().unwrap();
        let draw_command_buffers = command_buffers;

        info!("Allocated {} command buffers", frame_count + 1);

        // Now we will allocate all of the fences we need
        let fence_create_info = FenceCreateInfo::default().with_flags(FenceCreateFlags::SIGNALED);

        let (setup_reuse_fence, _) = unsafe { queue.device().create_fence(&fence_create_info, None)? };

        let draw_reuse_fences = (0..frame_count)
            .map(|_| unsafe { queue.device().create_fence(&fence_create_info, None).result() })
            .collect::<Result<SmallVec<_>, _>>()?;

        info!("Created {} fences", frame_count + 1);

        Ok(Self {
            queue,
            command_pool,
            setup_command_buffer,
            setup_reuse_fence,
            draw_command_buffers,
            draw_reuse_fences,
            frame_count,
            frame_index: 0,
        })
    }

    /// Get a reference to the commands's queue.
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    /// Get a reference to the commands's command pool.
    #[inline]
    pub fn command_pool(&self) -> &Unique<CommandPool> {
        &self.command_pool
    }

    /// Get a reference to the commands's setup command buffer.
    #[inline]
    pub fn setup_command_buffer(&self) -> &Unique<CommandBuffer> {
        &self.setup_command_buffer
    }

    /// Get a reference to the commands's setup reuse fence.
    #[inline]
    pub fn setup_reuse_fence(&self) -> &Unique<Fence> {
        &self.setup_reuse_fence
    }

    /// Get a reference to the commands's draw command buffers.
    #[inline]
    pub fn draw_command_buffers(&self) -> &SmallVec<Unique<CommandBuffer>> {
        &self.draw_command_buffers
    }

    /// Get a reference to the commands's draw reuse fences.
    #[inline]
    pub fn draw_reuse_fences(&self) -> &SmallVec<Unique<Fence>> {
        &self.draw_reuse_fences
    }

    /// Get a reference to the commands's frame count.
    #[inline]
    pub fn frame_count(&self) -> usize {
        self.frame_count
    }

    /// Get a reference to the commands's frame index.
    #[inline]
    pub fn frame_index(&self) -> usize {
        self.frame_index
    }

    /// Increases the frame index by one.
    /// Wraps to zero when '>= self.frame_count`
    #[inline]
    pub fn next_frame(&mut self) -> usize {
        self.frame_index = (self.frame_index + 1) % self.frame_count;

        self.frame_index
    }

    pub fn record_and_submit_setup<F>(
        &self,
        wait_mask: &[PipelineStageFlags],
        wait_semaphores: &[Semaphore],
        signal_semaphores: &[Semaphore],
        function: F,
    ) -> Result<(), VulkanResultCodes>
    where
        F: FnOnce(&Unique<CommandBuffer>) -> Result<(), VulkanResultCodes>,
    {
        Self::record_and_submit(
            self.setup_command_buffer(),
            self.setup_reuse_fence(),
            self.queue(),
            wait_mask,
            wait_semaphores,
            signal_semaphores,
            function,
        )
    }

    pub fn record_and_submit_draw<F>(
        &self,
        wait_mask: &[PipelineStageFlags],
        wait_semaphores: &[Semaphore],
        signal_semaphores: &[Semaphore],
        function: F,
    ) -> Result<(), VulkanResultCodes>
    where
        F: FnOnce(&Unique<CommandBuffer>) -> Result<(), VulkanResultCodes>,
    {
        Self::record_and_submit(
            &self.draw_command_buffers()[self.frame_index()],
            &self.draw_reuse_fences()[self.frame_index()],
            self.queue(),
            wait_mask,
            wait_semaphores,
            signal_semaphores,
            function,
        )
    }

    fn record_and_submit<F>(
        command_buffer: &Unique<CommandBuffer>,
        wait_fence: &Unique<Fence>,
        queue: &Queue,
        wait_mask: &[PipelineStageFlags],
        wait_semaphores: &[Semaphore],
        signal_semaphores: &[Semaphore],
        function: F,
    ) -> Result<(), VulkanResultCodes>
    where
        F: FnOnce(&Unique<CommandBuffer>) -> Result<(), VulkanResultCodes>,
    {
        unsafe {
            // We wait for the reuse fence
            command_buffer
                .device()
                .wait_for_fences(&[wait_fence.as_raw()], true, Some(std::u64::MAX))?;

            trace!("Waited for reuse fence");

            // We reset the reuse fence
            command_buffer.device().reset_fences(&[wait_fence.as_raw()])?;

            trace!("Reuse fence reset");

            // We reset the command buffer releasing the ownership of all of its resources
            command_buffer.reset_command_buffer(CommandBufferResetFlags::RELEASE_RESOURCES)?;

            trace!("Command buffer reset");

            // We tell Vulkan that we will begin recording commands for a one time submission, we
            // will not be reusing the commands
            let command_buffer_begin_info =
                CommandBufferBeginInfo::default().with_flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);

            // We begin a new command buffer, i.e we set it in a mode where it can record commands
            command_buffer.begin_command_buffer(&command_buffer_begin_info)?;

            trace!("Began command buffer recording");

            function(command_buffer)?;

            trace!("Command scheduled");

            // We stop recording commands in the command buffer
            command_buffer.end_command_buffer()?;

            trace!("Finished command buffer recording");

            let raw = command_buffer.as_raw();
            let submit_info = SubmitInfo::default()
                .with_wait_semaphores(wait_semaphores)
                .with_wait_dst_stage_mask(wait_mask)
                .with_command_buffers(std::slice::from_ref(&raw))
                .with_signal_semaphores(signal_semaphores);

            queue.submit(&[submit_info], wait_fence)?;
        }

        Ok(())
    }
}
