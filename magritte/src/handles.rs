//! # Handles
//! Traits for dealing with handles at a higher level

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use tracing::debug;

use crate::entry::{Entry, EntryVTable};

#[derive(Debug)]
pub struct Unique<'a, T: Handle> {
    parent: &'a T::Parent<'a>,
    vtable: T::VTable,
    metadata: T::Metadata,
    this: T,
}

impl<'a, T: Handle> Deref for Unique<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.this
    }
}

impl<'a, T: Handle> DerefMut for Unique<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.this
    }
}

unsafe impl<'a, T: Handle + Send> Send for Unique<'a, T> {}
unsafe impl<'a, T: Handle + Send> Sync for Unique<'a, T> {}

impl<'a, T: Handle> Unique<'a, T> {
    /// Creates a new unique pointer
    ///
    /// # Safety
    /// Cannot prove that the value is unique.
    #[inline]
    pub unsafe fn new<'b: 'a>(parent: &'a T::Parent<'b>, this: T, metadata: T::Metadata) -> Self {
        let vtable = this.load_vtable(parent, &metadata);

        Self {
            parent: std::mem::transmute(parent),
            this,
            metadata,
            vtable,
        }
    }

    /// Gets a reference to the parent
    #[inline]
    pub fn parent(&self) -> &'a T::Parent<'a> {
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

impl<'a, T: Handle> AsRaw for Unique<'a, T> {
    type Raw = T;

    fn as_raw(&self) -> Self::Raw {
        self.this
    }
}

impl<'a, T: Handle> Drop for Unique<'a, T> {
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

    /// Destroy tha handle, only called on `drop`
    /// The function is unsafe because it cannot be proven
    /// it has only be called once.
    unsafe fn destroy<'a>(self: &mut Unique<'a, Self>);

    /// Loads the V-Table of this handle.
    unsafe fn load_vtable<'a>(&self, parent: &Self::Parent<'a>, metadata: &Self::Metadata) -> Self::VTable;
}

impl Handle for () {
    type Parent<'a> = ();

    type VTable = ();

    type Metadata = ();

    #[inline]
    unsafe fn destroy<'a>(self: &mut Unique<'a, Self>) {}

    #[inline]
    unsafe fn load_vtable<'a>(&self, _: &Self::Parent<'a>, _: &Self::Metadata) -> Self::VTable {
        ()
    }
}

impl Handle for Entry {
    type Parent<'a> = ();

    type VTable = EntryVTable;

    type Metadata = ();

    #[inline]
    unsafe fn destroy<'a>(self: &mut Unique<'a, Self>) {}

    #[inline]
    unsafe fn load_vtable<'a>(&self, _: &Self::Parent<'a>, _: &Self::Metadata) -> Self::VTable {
        self.0
    }
}

pub trait AsRaw {
    type Raw;

    fn as_raw(&self) -> Self::Raw;
}