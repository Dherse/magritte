use std::thread::JoinHandle;

use crossbeam_channel::{bounded, Sender};
use log::error;
use magritte::{
    extensions::khr_swapchain::PresentInfoKHR,
    vulkan1_0::{Device, Fence, Queue as VkQueue, SubmitInfo, VulkanResultCodes},
    AsRaw, Unique,
};

/// Helper for ensuring synchronization of the queue.
pub struct Queue {
    /// The join handle to await the queue thread.
    thread: Option<JoinHandle<()>>,

    /// The sender to queue actions in the queue thread
    sender: Sender<QueueMessage>,

    /// The underlying Vulkan queue
    queue: Unique<VkQueue>,

    /// The index of the queue family
    pub queue_family_index: u32,
}

impl Drop for Queue {
    fn drop(&mut self) {
        if let Err(e) = self.sender.send(QueueMessage::Stop) {
            error!("Failed to stop queue thread: {}", e);
            return;
        }

        if let Err(_) = self.thread.take().unwrap().join() {
            error!("Failed to join queue thread");
        }
    }
}

impl Queue {
    /// Creates a new Queue thread and a new queue channel to use the queue
    pub fn new(queue: Unique<VkQueue>, queue_family_index: u32) -> Self {
        let (sender, receiver) = bounded(1);

        let queue_ = queue.clone();
        let thread = std::thread::spawn(move || {
            while let Ok(message) = receiver.recv() {
                match message {
                    QueueMessage::Submit(info, fence, back) => {
                        let result = unsafe { queue.queue_submit(&info, Some(fence)).result() };

                        if let Err(e) = back.send(result) {
                            error!("Failed to send back message: {}. Stopping.", e);
                            break;
                        }
                    },
                    QueueMessage::Present(info, back) => {
                        let result = unsafe { queue.queue_present_khr(&info).result() };

                        if let Err(e) = back.send(result) {
                            error!("Failed to send back message: {}. Stopping.", e);
                            break;
                        }
                    },
                    QueueMessage::Stop => break,
                }
            }

            unsafe {
                // Wait for the queue to be done doing things
                queue.queue_wait_idle();
            }
        });

        Self {
            thread: Some(thread),
            sender,
            queue: queue_,
            queue_family_index,
        }
    }

    /// Submits some command buffers to the queue for execution.
    /// See [`VkQueue::queue_submit`] for more info.
    pub fn submit<'lt>(&self, submit_info: &[SubmitInfo<'lt>], fence: &Unique<Fence>) -> Result<(), VulkanResultCodes> {
        let (forth, back) = bounded(0);

        self.sender
            .send(QueueMessage::Submit(
                unsafe { std::mem::transmute(submit_info) },
                fence.as_raw(),
                forth,
            ))
            .expect("Queue thread is dead");

        back.recv().expect("failed to wait for queue")
    }

    /// Presents an image in a swapchain.
    /// See [`VkQueue::queue_present_khr`] for more info.
    pub fn present<'lt>(&self, present_info: PresentInfoKHR<'lt>) -> Result<(), VulkanResultCodes> {
        let (forth, back) = bounded(0);

        self.sender
            .send(QueueMessage::Present(
                unsafe { std::mem::transmute(present_info) },
                forth,
            ))
            .expect("Queue thread is dead");

        back.recv().unwrap()
    }

    /// Get a reference to the queue's queue family index.
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }

    /// Gets the device the queue was allocated from
    pub fn device(&self) -> &Unique<Device> {
        self.queue.device()
    }
}

enum QueueMessage {
    /// Submit some command buffers to the queue.
    Submit(
        &'static [SubmitInfo<'static>],
        Fence,
        Sender<Result<(), VulkanResultCodes>>,
    ),

    /// Present to the swapchain
    Present(PresentInfoKHR<'static>, Sender<Result<(), VulkanResultCodes>>),

    /// Stop the queue thread (on [`Drop`])
    Stop,
}

// This is okay because all of the usages of the structures are contained
// within this module where we enforce **all** lifetimes!
unsafe impl Send for QueueMessage {}
