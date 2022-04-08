use crate::vulkan1_0::BaseOutStructure;

pub unsafe trait Chain<'lt, T>: Sized {
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'lt mut T) -> Self {
        unsafe {
            insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'lt>,
                new as *mut T as *mut BaseOutStructure<'lt>,
            );
        }

        self
    }
}

#[inline]
pub unsafe fn insert_ptr_in_chain<'lt>(
    mut host: *mut BaseOutStructure<'lt>,
    mut addition: *mut BaseOutStructure<'lt>,
) {
    let old = (*host).next;
    (*addition).next = old;
    (*host).next = addition;
}