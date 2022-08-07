//! # Command buffers
//! TODO

use crate::{Unique, vulkan1_0::CommandBuffer};

pub trait CommonCommandBuffer {
    /// Gets the inner command buffer
    fn inner(&self) -> &Unique<CommandBuffer>;
}