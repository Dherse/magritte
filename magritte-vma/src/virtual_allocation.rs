use magritte::{Unique, Handle};

use crate::allocator::VmaAllocator;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct VmaVirtualAllocation(pub u64);
impl VmaVirtualAllocation {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for VmaVirtualAllocation {}
impl Default for VmaVirtualAllocation {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for VmaVirtualAllocation {
    type Parent = Unique<VmaAllocator>;
    type VTable = ();
    type Metadata = ();
    type Raw = u64;
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
    unsafe fn destroy(self: &mut Unique<Self>) {
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}