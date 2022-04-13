//! # Handles
//! Traits for dealing with handles at a higher level

use std::{
    fmt::Debug,
    ops::Deref,
    ptr::NonNull,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

#[cfg(not(sanitize = "thread"))]
macro_rules! acquire {
    ($x:expr) => {
        std::sync::atomic::fence(Ordering::Acquire)
    };
}

// ThreadSanitizer does not support memory fences. To avoid false positive
// reports in Arc / Weak implementation use atomic loads for synchronization
// instead.
#[cfg(sanitize = "thread")]
macro_rules! acquire {
    ($x:expr) => {
        $x.load(Ordering::Acquire)
    };
}

use crate::entry::{Entry, EntryVTable};

pub(crate) struct Inner<T: Handle> {
    pub(crate) parent: T::Parent,
    pub(crate) vtable: T::VTable,
    pub(crate) metadata: T::Metadata,
    pub(crate) strong: AtomicUsize,
}

#[derive(Debug)]
pub struct Unique<T: Handle> {
    pub(crate) inner: NonNull<Inner<T>>,
    pub(crate) this: T,
}

impl<T: Handle> Deref for Unique<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.this
    }
}

unsafe impl<T: Handle + Send> Send for Unique<T> {}
unsafe impl<T: Handle + Send> Sync for Unique<T> {}

impl<T: Handle> Unique<T> {
    /// Creates a new unique pointer
    ///
    /// # Safety
    /// Cannot prove that the value is unique.
    #[inline]
    pub unsafe fn new(parent: &T::Parent, this: T, metadata: T::Metadata) -> Self {
        let vtable = this.load_vtable(parent, &metadata);

        let inner = Box::new(Inner {
            parent: parent.clone(),
            vtable,
            metadata,
            strong: AtomicUsize::new(1),
        });

        Self {
            inner: NonNull::new_unchecked(Box::leak(inner) as *mut Inner<T>),
            this,
        }
    }

    /// Gets the inner storage of this value
    #[inline]
    fn inner(&self) -> &Inner<T> {
        unsafe { self.inner.as_ref() }
    }

    /// Gets a reference to the parent
    #[inline]
    pub fn parent(&self) -> &T::Parent {
        &self.inner().parent
    }

    /// Gets the V-Table of this handle
    #[inline]
    pub fn vtable(&self) -> &T::VTable {
        &self.inner().vtable
    }

    /// Gets a reference to the metadata of this handle
    #[inline]
    pub fn metadata(&self) -> &T::Metadata {
        &self.inner().metadata
    }
}

impl<T: Handle> AsRaw for Unique<T>
where
    T: Copy,
{
    type Raw = T;

    fn as_raw(&self) -> Self::Raw {
        self.this
    }
}

impl<T: Handle> Drop for Unique<T> {
    fn drop(&mut self) {
        if self.inner().strong.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        acquire!(self.inner().strong);

        unsafe {
            T::destroy(self);
            Box::from_raw(self.inner.as_ptr());
        }
    }
}

impl<T: Handle> Clone for Unique<T> {
    fn clone(&self) -> Self {
        self.inner().strong.fetch_add(1, Ordering::Relaxed);

        Self {
            inner: self.inner.clone(),
            this: self.this.clone(),
        }
    }
}

pub trait Handle: Clone {
    /// The parent of this handle
    type Parent: Send + Sync + Clone;

    /// The associated V-Table
    type VTable: Send + Sync;

    /// The metadata of this handle
    type Metadata: Send + Sync;

    /// The contained type
    type Storage;

    /// Destroy tha handle, only called on `drop`
    /// The function is unsafe because it cannot be proven
    /// it has only be called once.
    unsafe fn destroy(self: &mut Unique<Self>);

    #[doc(hidden)]
    fn as_stored(self) -> Self::Storage;

    #[doc(hidden)]
    unsafe fn from_stored(this: Self::Storage) -> Self;

    /// Loads the V-Table of this handle.
    unsafe fn load_vtable(&self, parent: &Self::Parent, metadata: &Self::Metadata) -> Self::VTable;

    #[inline]
    #[doc(hidden)]
    unsafe fn coerce<T: Handle<Storage = Self::Storage>>(self) -> T {
        T::from_stored(self.as_stored())
    }
}

impl Handle for () {
    type Parent = ();

    type VTable = ();

    type Metadata = ();

    type Storage = ();

    #[inline]
    unsafe fn destroy(self: &mut Unique<Self>) {}

    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {
        ()
    }

    fn as_stored(self) -> Self::Storage {
        ()
    }

    unsafe fn from_stored(_: Self::Storage) -> Self {
        ()
    }
}

impl Handle for Arc<Entry> {
    type Parent = ();

    type VTable = EntryVTable;

    type Metadata = ();

    type Storage = ();

    #[inline]
    unsafe fn destroy(self: &mut Unique<Self>) {}

    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {
        self.0
    }

    fn as_stored(self) -> Self::Storage {
        ()
    }

    unsafe fn from_stored(_: Self::Storage) -> Self {
        unimplemented!();
    }
}

pub trait AsRaw {
    type Raw;

    fn as_raw(&self) -> Self::Raw;
}
