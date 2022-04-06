//! # Handles
//! Traits for dealing with handles at a higher level

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use tracing::debug;

use crate::entry::{Entry, EntryVTable};

#[derive(Debug)]
pub struct Unique<'a, 'b: 'a, T: Handle> {
    pub(crate) parent: &'a T::Parent<'b>,
    pub(crate) vtable: T::VTable,
    pub(crate) metadata: T::Metadata,
    pub(crate) this: T,
}

impl<'a, 'b: 'a, T: Handle> Deref for Unique<'a, 'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.this
    }
}

impl<'a, 'b: 'a, T: Handle> DerefMut for Unique<'a, 'b, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.this
    }
}

unsafe impl<'a, 'b: 'a, T: Handle + Send> Send for Unique<'a, 'b, T> {}
unsafe impl<'a, 'b: 'a, T: Handle + Send> Sync for Unique<'a, 'b, T> {}

impl<'a, 'b: 'a, T: Handle> Unique<'a, 'b, T> {
    /// Creates a new unique pointer
    ///
    /// # Safety
    /// Cannot prove that the value is unique.
    #[inline]
    pub unsafe fn new(parent: &'a T::Parent<'b>, this: T, metadata: T::Metadata) -> Self {
        let vtable = this.load_vtable(parent, &metadata);

        Self {
            parent,
            this,
            metadata,
            vtable,
        }
    }

    /// Gets a reference to the parent
    #[inline]
    pub fn parent(&self) -> &'a T::Parent<'b> {
        self.parent
    }

    /// Gets the V-Table of this handle
    #[inline]
    pub fn vtable(&self) -> &T::VTable {
        &self.vtable
    }

    /// Gets a reference to the metadata of this handle
    #[inline]
    pub fn metadata(&self) -> &T::Metadata {
        &self.metadata
    }

    /// Gets a mutable reference to the metadata of this handle
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut T::Metadata {
        &mut self.metadata
    }
}

impl<'a, 'b: 'a, T: Handle> AsRaw for Unique<'a, 'b, T> {
    type Raw = T;

    fn as_raw(&self) -> Self::Raw {
        self.this
    }
}

impl<'a, 'b: 'a, T: Handle> Drop for Unique<'a, 'b, T> {
    fn drop(&mut self) {
        debug!("Dropping {}", std::any::type_name::<T>());

        unsafe {
            T::destroy(self);
        }
    }
}
pub trait Handle: Copy {
    /// The parent of this handle
    type Parent<'a>: Send + Sync + 'a;

    /// The associated V-Table
    type VTable: Send + Sync;

    /// The metadata of this handle
    type Metadata: Send + Sync;

    /// The raw contained type
    type Raw;

    /// Destroy tha handle, only called on `drop`
    /// The function is unsafe because it cannot be proven
    /// it has only be called once.
    unsafe fn destroy<'a, 'b>(self: &mut Unique<'a, 'b, Self>);

    #[doc(hidden)]
    fn as_raw(self) -> Self::Raw;

    #[doc(hidden)]
    unsafe fn from_raw(this: Self::Raw) -> Self;

    /// Loads the V-Table of this handle.
    unsafe fn load_vtable<'a>(&self, parent: &Self::Parent<'a>, metadata: &Self::Metadata) -> Self::VTable;

    #[inline]
    #[doc(hidden)]
    unsafe fn coerce<T: Handle<Raw = Self::Raw>>(self) -> T {
        T::from_raw(self.as_raw())
    }
}

impl Handle for () {
    type Parent<'a> = ();

    type VTable = ();

    type Metadata = ();

    type Raw = ();


    #[inline]
    unsafe fn destroy<'a, 'b>(self: &mut Unique<'a, 'b, Self>) {}

    #[inline]
    unsafe fn load_vtable<'a>(&self, _: &Self::Parent<'a>, _: &Self::Metadata) -> Self::VTable {
        ()
    }

    fn as_raw(self) -> Self::Raw {
        ()
    }

    unsafe fn from_raw(_: Self::Raw) -> Self {
        ()
    }
}

impl Handle for Entry {
    type Parent<'a> = ();

    type VTable = EntryVTable;

    type Metadata = ();

    type Raw = ();

    #[inline]
    unsafe fn destroy<'a, 'b>(self: &mut Unique<'a, 'b, Self>) {}

    #[inline]
    unsafe fn load_vtable<'a>(&self, _: &Self::Parent<'a>, _: &Self::Metadata) -> Self::VTable {
        self.0
    }

    fn as_raw(self) -> Self::Raw {
        ()
    }

    unsafe fn from_raw(_: Self::Raw) -> Self {
        unimplemented!();
    }
}

pub trait AsRaw {
    type Raw;

    fn as_raw(&self) -> Self::Raw;
}
