use std::time::Duration;

use crate::{vulkan1_0::Fence, Unique, VulkanResult};

impl Fence {
    /// Gets the status of the fence
    ///
    /// See [`crate::vulkan1_0::Device::get_fence_status`] for more information.
    pub fn status(self: &Unique<Self>) -> VulkanResult<()> {
        unsafe { self.device().get_fence_status(self.this) }
    }

    /// Waits for the fence to be completed.
    ///
    /// See [`crate::vulkan1_0::Device::wait_for_fences`] for more information.
    pub fn wait(self: &Unique<Self>) -> VulkanResult<()> {
        unsafe { self.device().wait_for_fences(&[self.this], true, None) }
    }

    /// Waits for the fence to be completed, with a timeout.
    ///
    /// See [`crate::vulkan1_0::Device::wait_for_fences`] for more information.
    ///
    /// # Return value
    /// Will return [`crate::vulkan1_0::VulkanResultCodes::TIMEOUT`] if the timeout was reached.
    pub fn wait_timeout(self: &Unique<Self>, timeout: Duration) -> VulkanResult<()> {
        unsafe {
            self.device()
                .wait_for_fences(&[self.this], true, Some(timeout.as_nanos() as _))
        }
    }
}
