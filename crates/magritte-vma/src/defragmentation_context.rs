use magritte::{vulkan1_0::VulkanResultCodes, AsRaw, Unique};

use crate::{
    allocator::Allocator,
    ffi::{
        vmaBeginDefragmentationPass, vmaEndDefragmentation, vmaEndDefragmentationPass, DefragmentationMove,
        DefragmentationStats,
    },
    pool::Pool,
};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct DefragmentationContextHandle(pub *mut ());
impl DefragmentationContextHandle {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(std::ptr::null_mut())
    }

    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }

    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> *mut () {
        self.0
    }
}

pub struct DefragmentationContext {
    allocator: Unique<Allocator>,
    _pool: Option<Unique<Pool>>,
    handle: DefragmentationContextHandle,
    destroyed: bool,
}

impl DefragmentationContext {
    pub(crate) unsafe fn new(
        allocator: Unique<Allocator>,
        pool: Option<Unique<Pool>>,
        handle: DefragmentationContextHandle,
    ) -> Self {
        Self {
            allocator,
            _pool: pool,
            handle,
            destroyed: false,
        }
    }

    /// Ends defragmentation process.
    pub fn end(mut self) -> DefragmentationStats {
        let mut out = unsafe { std::mem::zeroed() };

        unsafe {
            vmaEndDefragmentation(self.allocator.as_raw(), self.as_raw(), &mut out);
        }

        self.destroyed = true;

        out
    }

    /// Runs a single defragmentation pass.
    /// Return `Ok(true)` if there are more passes to be executed.
    /// Return `Ok(false)` if defragmentation is done.
    ///
    /// In the provided function, you should:
    /// 1. Create a new buffer/image in the place pointed by [`DefragmentationMove::dstMemory`] +
    ///    [`DefragmentationMove::dstOffset`]
    /// 2. Copy data from the [DefragmentationMove::srcAllocation`] e.g. using `vkCmdCopyBuffer`,
    ///    `vkCmdCopyImage`.
    /// 3. Make sure these commands finished executing on the GPU.
    /// 4. Destroy the old buffer/image.
    pub fn run_pass<F>(&mut self, fun: F) -> Result<bool, VulkanResultCodes>
    where
        F: FnOnce(&mut [DefragmentationMove]) -> Result<(), VulkanResultCodes>,
    {
        let mut out = unsafe { std::mem::zeroed() };
        let res = unsafe { vmaBeginDefragmentationPass(self.allocator.as_raw(), self.as_raw(), &mut out) };

        if res != VulkanResultCodes::SUCCESS {
            return Err(res);
        }

        fun(unsafe { std::slice::from_raw_parts_mut(out.moves, out.move_count as usize) })?;

        let res = unsafe { vmaEndDefragmentationPass(self.allocator.as_raw(), self.as_raw(), &mut out) };

        match res {
            VulkanResultCodes::SUCCESS => Ok(false),
            VulkanResultCodes::INCOMPLETE => Ok(true),
            other => Err(other),
        }
    }
}

impl AsRaw for DefragmentationContext {
    type Raw = DefragmentationContextHandle;

    fn as_raw(&self) -> Self::Raw {
        self.handle
    }
}

impl Drop for DefragmentationContext {
    fn drop(&mut self) {
        if self.destroyed {
            return;
        }

        unsafe {
            vmaEndDefragmentation(self.allocator.as_raw(), self.handle, std::ptr::null_mut());
        }
    }
}
