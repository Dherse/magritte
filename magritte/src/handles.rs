//! # Handles
//! Traits for dealing with handles at a higher level

pub trait Handle {
    /// The parent of this handle
    type Parent: Handle;

    /// The VTable of this handle
    type VTable;

    /// Gets the vtable
    fn vtable(&self) -> &Self::VTable;
}

impl Handle for () {
    type Parent = ();

    type VTable = ();

    fn vtable(&self) -> &() {
        &()
    }
}
