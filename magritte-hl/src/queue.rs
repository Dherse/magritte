use magritte::{
    extensions::khr_swapchain::PresentInfoKHR,
    vulkan1_0::{self, DeviceQueueCreateFlags, DeviceQueueCreateInfo, Fence, QueueFlags, SubmitInfo},
    AsRaw, SmallVec, Unique, VulkanResult,
};

/// A Wrapper around a [`vulkan1_0::Queue`] that provides a safe interface.
/// It also provide auxillary data such as the index, the type and the capabilities.
#[derive(Debug)]
pub struct Queue {
    /// The family index of the queue
    family_index: u32,

    /// The index of the queue
    index: u32,

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
            family_index,
            index,
            type_,
            queue,
            submit_info_cache: SmallVec::new(),
        }
    }
    
    /// Submit a list of commands to the queue.
    /// See [`vulkan1_0::Queue::queue_submit`] for more information.
    ///
    /// # Safety
    /// This function is safe because the usage of the queue is guaranteed to be single threaded due
    /// to the use of a mutable reference to self.
    pub fn submit<'lt>(&mut self, infos: &[SubmitInfo<'lt>], fence: Option<Unique<Fence>>) -> VulkanResult<()> {
        unsafe { self.queue.queue_submit(infos, fence.map(|f| f.as_raw())) }
    }

    /// Submit a list of commands to the queue.
    /// See [`vulkan1_0::Queue::queue_submit`] for more information.
    ///
    /// # Safety
    /// This function is safe because the usage of the queue is guaranteed to be single threaded due
    /// to the use of a mutable reference to self.
    pub fn submit_iter<'lt, I, V>(&mut self, infos: I, fence: Option<Unique<Fence>>) -> VulkanResult<()>
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

    /// Submit a present call to the queue.
    /// See [`vulkan1_0::Queue::queue_present_khr`] for more information.
    ///
    /// # Safety
    /// This function is safe because the usage of the queue is guaranteed to be single threaded due
    /// to the use of a mutable reference to self.
    pub fn present<'lt>(&mut self, present_info: PresentInfoKHR<'lt>) -> VulkanResult<()> {
        unsafe { self.queue.queue_present_khr(&present_info) }
    }
    /// Gets the family index of the queue family that owns this queue
    pub fn family_index(&self) -> u32 {
        self.family_index
    }

    /// Returns the index of this [`Queue`] within the queue family.
    pub fn index(&self) -> u32 {
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

#[derive(Debug, Default)]
pub struct Queues {
    /// The instantiated queue with present capabilities.
    /// May overlap with other categories of queues.
    pub present: SmallVec<Queue>,

    /// The list of instantiated queues with graphics capabilities.
    /// May overlap with other categories of queues.
    pub graphics: SmallVec<Queue>,

    /// The list of instantiated queues with transfer capabilities.
    /// May overlap with other categories of queues.
    pub transfer: SmallVec<Queue>,

    /// The list of instantiated queues with compute capabilities.
    /// May overlap with other categories of queues.
    pub compute: SmallVec<Queue>,
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
