use magritte::{
    extensions::khr_swapchain::PresentInfoKHR,
    vulkan1_0::{
        self, CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo, DeviceQueueCreateFlags,
        DeviceQueueCreateInfo, Fence, QueueFlags, SubmitInfo,
    },
    vulkan1_3::SubmitInfo2,
    AsRaw, SmallVec, Unique, VulkanResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(C)]
pub struct QueueIndex(pub u32, pub u32);

impl QueueIndex {
    pub const fn family(&self) -> u32 {
        self.0
    }

    pub const fn index(&self) -> u32 {
        self.1
    }
}

impl From<&Queue> for QueueIndex {
    fn from(a: &Queue) -> Self {
        a.index
    }
}

impl From<(u32, u32)> for QueueIndex {
    fn from(a: (u32, u32)) -> Self {
        Self(a.0, a.1)
    }
}

/// A Wrapper around a [`vulkan1_0::Queue`] that provides a safe interface.
/// It also provide auxillary data such as the index, the type and the capabilities.
#[derive(Debug)]
pub struct Queue {
    /// The family index of the queue
    index: QueueIndex,

    /// The type of the queue
    type_: QueueFlags,

    /// The instance of the queue itself
    queue: Unique<vulkan1_0::Queue>,

    /// The cache that is used to avoid allocations when using iterators
    submit_info_cache: SmallVec<SubmitInfo<'static>>,
}

impl Queue {
    pub(crate) fn new(family_index: u32, index: u32, type_: QueueFlags, queue: Unique<vulkan1_0::Queue>) -> Self {
        Self {
            index: QueueIndex(family_index, index),
            type_,
            queue,
            submit_info_cache: SmallVec::new(),
        }
    }

    /// Creates a command pool within the queue family of this queue.
    pub fn create_command_pool(&self, flags: CommandPoolCreateFlags) -> VulkanResult<Unique<CommandPool>> {
        unsafe {
            self.queue.device().create_command_pool(
                &CommandPoolCreateInfo::default()
                    .with_flags(flags)
                    .with_queue_family_index(self.index().family()),
                None,
            )
        }
    }

    /// Submit a list of commands to the queue.
    /// See [`vulkan1_0::Queue::queue_submit`] for more information.
    pub unsafe fn submit<'lt>(&mut self, infos: &[SubmitInfo<'lt>], fence: Option<&Unique<Fence>>) -> VulkanResult<()> {
        unsafe { self.queue.queue_submit(infos, fence.map(|f| f.as_raw())) }
    }

    /// Submit a list of commands to the queue.
    /// See [`vulkan1_0::Queue::queue_submit`] for more information.
    pub unsafe fn submit2<'lt>(
        &mut self,
        infos: &[SubmitInfo2<'lt>],
        fence: Option<&Unique<Fence>>,
    ) -> VulkanResult<()> {
        unsafe { self.queue.queue_submit2(infos, fence.map(|f| f.as_raw())) }
    }

    /// Submit a list of commands to the queue.
    /// See [`vulkan1_0::Queue::queue_submit`] for more information.
    pub unsafe fn submit_iter<'lt, I, V>(&mut self, infos: I, fence: Option<&Unique<Fence>>) -> VulkanResult<()>
    where
        I: IntoIterator<Item = V>,
        V: Into<SubmitInfo<'lt>>,
    {
        self.submit_info_cache
            .extend(infos.into_iter().map(|v| unsafe { std::mem::transmute(v.into()) }));

        let res = unsafe {
            self.queue
                .queue_submit(&self.submit_info_cache, fence.map(|f| f.as_raw()))
        };

        self.submit_info_cache.clear();

        res
    }

    /// Waits for the queue to be idle. That means that the queue has finished all the operations
    /// that are outstanding.
    pub fn wait_idle(&mut self) -> VulkanResult<()> {
        unsafe { self.queue.queue_wait_idle() }
    }

    /// Bind device memory to a sparse resource object
    pub unsafe fn bind_sparse(
        &mut self,
        p_bind_info: &[vulkan1_0::BindSparseInfo],
        fence: Option<&Unique<Fence>>,
    ) -> VulkanResult<()> {
        unsafe { self.queue.queue_bind_sparse(p_bind_info, fence.map(|f| f.as_raw())) }
    }

    /// Submit a present call to the queue.
    /// See [`vulkan1_0::Queue::queue_present_khr`] for more information.
    ///
    /// # Safety
    /// This function is safe because the usage of the queue is guaranteed to be single threaded due
    /// to the use of a mutable reference to self.
    pub fn present<'lt>(&mut self, present_info: PresentInfoKHR<'lt>) -> VulkanResult<()> {
        unsafe { self.queue.queue_present_khr(&present_info) }
    }

    /// Returns the index of this [`Queue`].
    pub fn index(&self) -> QueueIndex {
        self.index
    }

    /// Returns the type of this [`Queue`].
    pub fn type_(&self) -> QueueFlags {
        self.type_
    }

    /// Gets the underlying Vulkan queue.
    ///
    /// # Safety
    /// This function is unsafe as it allows the user to break the safety guarantees of this
    /// library. This happens because the user can clone the queue and use it concurrently with
    /// this instance.
    pub unsafe fn queue(&self) -> &Unique<vulkan1_0::Queue> {
        &self.queue
    }
}

pub struct QueueCreateInfo {
    /// The family index of the queue
    pub family_index: u32,

    /// The priorities of the queues in this family
    pub priorities: Vec<f32>,

    /// Whether the queue should be created with the [`DeviceQueueCreateFlags::PROTECTED`] flag.
    pub protected: bool,
}

impl<'a> Into<DeviceQueueCreateInfo<'a>> for &'a QueueCreateInfo {
    fn into(self) -> DeviceQueueCreateInfo<'a> {
        DeviceQueueCreateInfo::default()
            .with_flags(if self.protected {
                DeviceQueueCreateFlags::PROTECTED
            } else {
                DeviceQueueCreateFlags::empty()
            })
            .with_queue_family_index(self.family_index)
            .with_queue_priorities(&self.priorities)
    }
}
