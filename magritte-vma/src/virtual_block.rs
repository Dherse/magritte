use magritte::{Handle, Unique};

use crate::{allocator::VmaAllocator, pool::VmaPool};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct VmaVirtualBlock(pub *mut ());
impl VmaVirtualBlock {
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

unsafe impl Send for VmaVirtualBlock {}
impl Default for VmaVirtualBlock {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for VmaVirtualBlock {
    type Parent = Unique<VmaAllocator>;
    type VTable = ();
    type Metadata = Option<Unique<VmaPool>>;
    type Storage = *mut ();

    #[inline]
    fn as_stored(self) -> Self::Storage {
        self.0
    }

    #[inline]
    unsafe fn from_stored(this: Self::Storage) -> Self {
        Self(this)
    }

    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {}

    #[inline]
    unsafe fn load_vtable(&self, parent: &Self::Parent, _: &Self::Metadata) -> Self::VTable {
        ()
    }
}
