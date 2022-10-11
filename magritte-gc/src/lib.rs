#![feature(drain_filter)]

mod channel;

use std::{collections::BTreeMap, thread::JoinHandle};

use channel::{Receiver, Rx, Sender, Tx};
use magritte::{
    handles::{Handles, ToBits},
    vulkan1_0::VulkanResultCodes,
};

#[cfg(not(feature = "parking_lot"))]
use std::sync::Mutex;

#[cfg(feature = "parking_lot")]
use parking_lot::Mutex;

#[cfg(all(feature = "log", feature = "tracing"))]
compile_error!("You must select only one of the two features `log` or `tracing`");

#[cfg(feature = "log")]
use log::{error, info, trace, warn};

#[cfg(feature = "tracing")]
use tracing::{error, info, trace, warn};

#[cfg(not(any(feature = "log", feature = "tracing")))]
macro_rules! error {
    ($($arg:tt)*) => { () };
}

#[cfg(not(any(feature = "log", feature = "tracing")))]
macro_rules! trace {
    ($($arg:tt)*) => { () };
}

#[cfg(not(any(feature = "log", feature = "tracing")))]
macro_rules! info {
    ($($arg:tt)*) => { () };
}

#[cfg(not(any(feature = "log", feature = "tracing")))]
macro_rules! warn {
    ($($arg:tt)*) => { () };
}

/// A simple dependency and reference counter based garbage collector for Vulkan handles.
pub struct MagritteGc {
    /// The handle to the garbage collection thread
    gc_thread_handle: Option<JoinHandle<Result<(), GcError>>>,

    /// The sender for passing data to the GC thread
    tx: Sender<Command>,
}

impl Drop for MagritteGc {
    #[allow(unused_variables)]
    fn drop(&mut self) {
        if let Err(e) = <Sender<_> as Tx<_>>::send(&self.tx, Command::Stop).ok_or(GcError::ChannelDied) {
            error!("Failed to send stop command to GC thread: {:?}", e);
        }

        if let Some(handle) = self.gc_thread_handle.take() {
            match handle.join() {
                Ok(r) => {
                    if let Err(e) = r {
                        error!("GC thread failed: {:?}", e);
                    }
                },
                Err(e) => error!("Failed to join GC thread: {:?}", e),
            }
        }
    }
}

impl MagritteGc {
    pub fn new() -> Self {
        let (tx, rx) = <Receiver<_> as Rx<_>>::unbounded();
        let gc_thread_handle = Some(std::thread::spawn(move || gc_thread(rx)));

        Self { gc_thread_handle, tx }
    }

    /// Runs one cycle of the garbage collector
    pub fn run(&self) -> Result<(), GcError> {
        self.tx.send(Command::Run).map_err(|_| GcError::ChannelDied)
    }

    /// Adds a new object to the garbage collector, if it was already added, it will be ignored
    pub fn track<T: Into<Handles>>(&self, handle: T) -> Result<(), GcError> {
        <Sender<_> as Tx<_>>::send(&self.tx, Command::Track(handle.into())).ok_or(GcError::ChannelDied)
    }

    /// Binds the lifetime of two objects together.
    /// If either object is not already registered within the garbage collector, they are ignored.
    /// This therefore means that you must ensure that both handles are registered for this call to
    /// do anything.
    pub fn bind<T: magritte::Handle, U: magritte::Handle>(&self, handle: T, parent: U) -> Result<(), GcError> {
        <Sender<_> as Tx<_>>::send(
            &self.tx,
            Command::Bind(handle.as_stored().to_bits(), parent.as_stored().to_bits()),
        )
        .ok_or(GcError::ChannelDied)
    }

    /// Stops the GC thread by sending it the `Stop` command and waiting for the thread to stop.
    ///
    /// # Safety
    /// This function is unsafe because it will drop every handle in the garbage collector.
    /// Which may incur calls to unsafe methods.
    pub unsafe fn stop(mut self) -> Result<(), GcError> {
        <Sender<_> as Tx<_>>::send(&self.tx, Command::Stop).ok_or(GcError::ChannelDied)?;

        if let Some(handle) = self.gc_thread_handle.take() {
            handle.join().map_err(|_| GcError::JoinFailed)??;
        }

        Ok(())
    }
}

enum Command {
    Run,
    Track(Handles),
    Bind(u64, u64),
    Stop,
}

fn gc_thread(rx: Receiver<Command>) -> Result<(), GcError> {
    let mut objects = BTreeMap::<u64, Handle>::new();
    let mut to_remove = Vec::new();
    loop {
        match <Receiver<_> as Rx<_>>::recv(&rx) {
            Some(msg) => match msg {
                Command::Run => {
                    trace!("Running garbage collection");

                    to_remove.clear();

                    for (handle, object) in objects.iter() {
                        if object.is_dead(&objects) {
                            to_remove.push(*handle);
                        }
                    }

                    for handle in &to_remove {
                        objects.remove(handle);
                    }
                },
                Command::Track(handle) => {
                    if !objects.contains_key(&handle.as_raw()) {
                        trace!("Tracking handle {:?}", handle);
                        objects.insert(handle.as_raw(), Handle::new(handle));
                    }
                },
                Command::Bind(this, other) => {
                    if objects.contains_key(&other) {
                        warn!(
                            "The parent handle {:?} is not registered with the garbage collector",
                            other
                        );
                        if let Some(handle) = objects.get_mut(&this) {
                            trace!("Binding handle {:X} to {:X}", this, other);
                            handle.bind(other);
                        } else {
                            warn!(
                                "The child handle {:?} is not registered with the garbage collector",
                                other
                            );
                        }
                    }
                },
                Command::Stop => {
                    info!("Stopping GC thread");
                    break;
                },
            },
            None => {
                error!("Unexpected closure of GC thread channel");
                return Err(GcError::ChannelDied);
            },
        }
    }

    Ok(())
}

struct Handle {
    handle: Handles,
    tracked_by: Mutex<Vec<u64>>,
}

impl Handle {
    fn new(handle: Handles) -> Self {
        Self {
            handle,
            tracked_by: Mutex::new(Vec::new()),
        }
    }

    fn bind(&mut self, other: u64) {
        self.tracked_by.get_mut().push(other);
    }

    fn is_dead(&self, objects: &BTreeMap<u64, Handle>) -> bool {
        #[cfg(feature = "parking_lot")]
        let mut lock = self.tracked_by.lock();

        #[cfg(not(feature = "parking_lot"))]
        let mut lock = self.tracked_by.lock().unwrap();

        lock.drain_filter(|value| !objects.contains_key(&*value));

        lock.is_empty()
            && self.handle.strong_count() == 1
            && if let Handles::Fence(f) = &self.handle {
                match f.status() {
                    magritte::VulkanResult::Success(VulkanResultCodes::NOT_READY, _) => false,
                    _ => true,
                }
            } else {
                false
            }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GcError {
    ChannelDied,

    JoinFailed,
}
