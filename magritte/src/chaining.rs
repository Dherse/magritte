use crate::vulkan1_0::BaseOutStructure;

pub unsafe trait Chain<'other, T>: Sized {
    type Out: 'other;

    #[must_use]
    fn chain(self, new: &'other mut T) -> Self::Out;
}

#[inline]
#[doc(hidden)]
pub unsafe fn insert_ptr_in_chain<'lt>(mut host: *mut BaseOutStructure<'lt>, mut addition: *mut BaseOutStructure<'lt>) {
    let old = (*host).next;
    (*addition).next = old;
    (*host).next = addition;
}
