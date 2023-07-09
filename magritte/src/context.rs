use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    }, collections::HashSet,
};

use parking_lot::Mutex;

use crate::native::vulkan1_0::VulkanResultCodes;

thread_local! {
    pub(crate) static CONTEXT: RefCell<Option<Arc<Context>>> = RefCell::new(None);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectId(pub(crate) uuid::Uuid);

impl ObjectId {
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("vulkan error: {0:?}")]
    Vulkan(VulkanResultCodes),
}

#[cfg(all(feature = "native", feature = "wasm"))]
compile_error!("Only one of the features `native` and `wasm` can be enabled at the same time.");

#[cfg(all(feature = "native", not(feature = "wasm")))]
pub type Context = crate::native::api::Api;

#[cfg(all(feature = "wasm", not(feature = "native")))]
pub type Context = crate::wasm::Api;

#[doc(hidden)]
#[derive(Debug)]
pub(crate) struct Container<H> {
    handle: H,
    parent: Option<ObjectId>,
    ref_counter: AtomicUsize,
    fences: Mutex<HashSet<ObjectId>>,
}

impl<H: PartialEq> PartialEq for Container<H> {
    fn eq(&self, other: &Self) -> bool {
        self.handle() == other.handle() && self.ref_count() == other.ref_count()
    }
}

impl<H> Container<H> {
    pub(crate) fn new(handle: H) -> Self {
        Self {
            handle,
            parent: None,
            ref_counter: AtomicUsize::new(1),
            fences: Mutex::new(HashSet::new()),
        }
    }

    pub(crate) fn with_parent(handle: H, parent: ObjectId) -> Self{
        Self {
            handle,
            parent: Some(parent),
            ref_counter: AtomicUsize::new(1),
            fences: Mutex::new(HashSet::new()),
        }
    }

    pub(crate) fn handle(&self) -> &H {
        &self.handle
    }

    pub(crate) fn parent(&self) -> Option<ObjectId> {
        self.parent
    }

    pub(crate) fn set_parent(&mut self, parent: ObjectId) {
        self.parent = Some(parent);
    }

    pub(crate) fn ref_count(&self) -> usize {
        self.ref_counter.load(Ordering::Acquire)
    }

    pub(crate) fn inc_ref(&self) {
        self.ref_counter.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn dec_ref(&self) -> usize {
        self.ref_counter.fetch_sub(1, Ordering::Release)
    }

    pub(crate) fn fences(&self) -> parking_lot::MutexGuard<'_, HashSet<ObjectId>> {
        self.fences.lock()
    }

    pub(crate) fn push_fence(&self, fence: ObjectId) {
        self.fences().insert(fence);
    }

    pub(crate) fn pop_fence(&self, fence: ObjectId) -> bool {
        self.fences().remove(&fence)
    }

    pub(crate) fn acquire(&self) {
        #[cfg(not(sanitize = "thread"))]
        std::sync::atomic::fence(Ordering::Acquire);

        #[cfg(sanitize = "thread")]
        self.ref_counter.load(Ordering::Acquire);
    }
}
