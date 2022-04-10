use magritte::{Handle, Unique};

use crate::{ffi::VmaVulkanFunctions, allocator::VmaAllocator, pool::VmaPool};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct VmaDefragmentationContext(pub *mut ());
impl VmaDefragmentationContext {
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

unsafe impl Send for VmaDefragmentationContext {}
impl Default for VmaDefragmentationContext {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for VmaDefragmentationContext {
    type Parent = Unique<VmaAllocator>;
    type VTable = ();
    type Metadata = Option<Unique<VmaPool>>;
    type Raw = *mut ();

    #[inline]
    fn as_raw(self) -> Self::Raw {
        self.0
    }

    #[inline]
    unsafe fn from_raw(this: Self::Raw) -> Self {
        Self(this)
    }

    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {}

    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {
    }
}