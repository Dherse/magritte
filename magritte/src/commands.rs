//! # Command buffers
//! TODO

use crate::{vulkan1_0::CommandBuffer, Unique};

pub trait CommonCommandBuffer {
    /// Gets the inner command buffer
    fn inner(&self) -> &Unique<CommandBuffer>;
}
