use std::{
    ffi::{c_void, c_char},
    mem::MaybeUninit, hint::unreachable_unchecked,
};

use uuid::Uuid;

use crate::native::vulkan1_0::{TRUE, FALSE};

pub unsafe trait IntoLowLevel {
    type LowLevel;

    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel;
}

macro_rules ! impl_into_low_level_for_primitive {
    ($($t:ty),*) => {
        $(
            unsafe impl IntoLowLevel for $t {
                type LowLevel = $t;

                unsafe fn into_low_level(&self, context: &std::sync::Arc<crate::context::Context>, bump: &bumpalo::Bump) -> Self::LowLevel {
                    *self
                }
            }

            unsafe impl FromLowLevel for $t {
                unsafe fn from_low_level(_: &std::sync::Arc<crate::context::Context>, value: Self::LowLevel) -> Self {
                    value
                }
            }
        )*
    };
}

impl_into_low_level_for_primitive!(
    u8,
    u16,
    u32,
    u64,
    i8,
    i16,
    i32,
    i64,
    f32,
    f64,
    usize,
    isize,
    Uuid,
    *const c_void,
    *mut c_void
);

unsafe impl IntoLowLevel for bool {
    type LowLevel = u32;

    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self as u32
    }
}

unsafe impl<T: IntoLowLevel, const N: usize> IntoLowLevel for [T; N] {
    type LowLevel = [T::LowLevel; N];

    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut uninit = MaybeUninit::<T::LowLevel>::uninit_array::<N>();

        for i in 0..N {
            uninit[i] = MaybeUninit::new(self[i].into_low_level(context, bump));
        }

        MaybeUninit::array_assume_init(uninit)
    }
}

unsafe impl IntoLowLevel for String {
    type LowLevel = *const c_char;

    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        struct ZeroAppendedIterator<'a>(std::slice::Iter<'a, u8>, bool);

        impl<'a> Iterator for ZeroAppendedIterator<'a> {
            type Item = u8;

            fn next(&mut self) -> Option<Self::Item> {
                match self.0.next() {
                    Some(x) => Some(*x),
                    None => {
                        if self.1 {
                            None
                        } else {
                            self.1 = true;
                            Some(0)
                        }
                    },
                }
            }
        }

        impl<'a> ExactSizeIterator for ZeroAppendedIterator<'a> {
            fn len(&self) -> usize {
                self.0.len() + 1
            }
        }

        match memchr::memchr(b'\0', self.as_bytes()) {
            Some(_) => panic!("`String` contains a null byte"),
            None => {},
        }

        let bytes = bump.alloc_slice_fill_iter(ZeroAppendedIterator(self.as_bytes().iter(), false));

        bytes.as_ptr().cast()
    }
}

pub unsafe trait FromLowLevel: IntoLowLevel {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as IntoLowLevel>::LowLevel,
    ) -> Self;
}



unsafe impl FromLowLevel for bool {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as IntoLowLevel>::LowLevel,
    ) -> Self {
        debug_assert!(value == TRUE || value == FALSE);

        match value {
            TRUE => true,
            FALSE => false,
            _ => unreachable_unchecked()
        }
    }
}

unsafe impl<T: IntoLowLevel> IntoLowLevel for Option<T> {
    type LowLevel = <T as IntoLowLevel>::LowLevel;

    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        unreachable!("`Option` is not a low-level type")
    }
}

unsafe impl<T: FromLowLevel> FromLowLevel for Option<T> {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as IntoLowLevel>::LowLevel,
    ) -> Self {
        Some(T::from_low_level(context, value))
    }
}

unsafe impl<T: FromLowLevel, const N: usize> FromLowLevel for [T; N] {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut uninit = MaybeUninit::<T>::uninit_array::<N>();

        for i in 0..N {
            uninit[i] = MaybeUninit::new(T::from_low_level(context, value[i]));
        }

        MaybeUninit::array_assume_init(uninit)
    }
}

unsafe impl FromLowLevel for String {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as IntoLowLevel>::LowLevel,
    ) -> Self {
        let cstr = std::ffi::CStr::from_ptr(value.cast());

        cstr.to_string_lossy().into_owned()
    }
}