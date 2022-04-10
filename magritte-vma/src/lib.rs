#![feature(arbitrary_self_types)]

#[doc(hidden)]
pub mod ffi;

pub mod allocator;
pub mod pool;
pub mod allocation;
pub mod defragmentation_context;
pub mod virtual_allocation;
pub mod virtual_block;
pub mod flags;