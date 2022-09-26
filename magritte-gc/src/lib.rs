#![feature(drain_filter)]

mod channel;

use std::{thread::JoinHandle, collections::BTreeMap};

use channel::{Sender, Receiver, Rx, Tx};
use magritte::{handles::Handles};

#[cfg(not(feature = "parking_lot"))]
use std::sync::Mutex;

#[cfg(feature = "parking_lot")]
use parking_lot::Mutex;

#[cfg(all(feature = "log", feature = "tracing"))]
compile_error!("You must select only one of the two features `log` or `tracing`");

#[cfg(feature = "log")]
use log::{error, info, trace};

#[cfg(feature = "tracing")]
use tracing::{error, info, trace};

#[cfg(not(any(feature = "log", feature = "tracing")))]
macro_rules! error {
    ($($arg:tt)*) => {
        
    };
}

#[cfg(not(any(feature = "log", feature = "tracing")))]
macro_rules! trace {
    ($($arg:tt)*) => {
        
    };
}

#[cfg(not(any(feature = "log", feature = "tracing")))]
macro_rules! warn {
    ($($arg:tt)*) => {
        
    };
}

#[cfg(not(any(feature = "log", feature = "tracing")))]
macro_rules! info {
    ($($arg:tt)*) => {
        
    };
}

pub struct MagritteGc {
    /// The handle to the garbage collection thread
    gc_thread_handle: Option<JoinHandle<Result<(), GcError>>>,

    /// The sender for passing data to the GC thread
    tx: Sender<Command>,
}

impl Drop for MagritteGc {
    #[allow(unused_variables)]
    fn drop(&mut self) {
        self.tx.send(Command::Stop).unwrap();
        if let Err(e) = self.gc_thread_handle.take().unwrap().join().unwrap() {
            error!("Error while stopping the garbage collection thread: {:?}", e);
        }
    }
}

impl MagritteGc {
    pub fn new() -> Self {
        let(tx, rx) = <Receiver<_> as Rx<_>>::unbounded();
        let gc_thread_handle = Some(std::thread::spawn(move || gc_thread(rx)));

        Self {
            gc_thread_handle,
            tx
        }
    }

    pub fn run(&self) -> Result<(), GcError> {
        self.tx.send(Command::Run).map_err(|_| GcError::ChannelDied)
    }

    pub fn track<T: Into<Handles>>(&self, handle: T) -> Result<(), GcError> {
        <Sender<_> as Tx<_>>::send(&self.tx, Command::Track(handle.into())).ok_or(GcError::ChannelDied)
    }

    pub fn bind<T: Into<Handles>, U: Into<Handles>>(&self, handle: T, parent: U) -> Result<(), GcError> {
        <Sender<_> as Tx<_>>::send(&self.tx, Command::Bind(handle.into().as_raw(), parent.into().as_raw())).ok_or(GcError::ChannelDied)
    }

    pub fn stop(self) -> Result<(), GcError> {
        <Sender<_> as Tx<_>>::send(&self.tx, Command::Stop).ok_or(GcError::ChannelDied)
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
    loop {
        match <Receiver<_> as Rx<_>>::recv(&rx) {
            Some(msg) => match msg {
                Command::Run => {
                    trace!("Running garbage collection");
                    let mut to_remove = Vec::new();

                    for (handle, object) in objects.iter() {
                        if object.is_dead(&objects) {
                            to_remove.push(*handle);
                        }
                    }

                    for handle in to_remove {
                        objects.remove(&handle);
                    }
                },
                Command::Track(handle) => {
                    trace!("Tracking handle {:?}", handle);
                    objects.insert(handle.as_raw(), Handle::new(handle));
                },
                Command::Bind(this, other) => {
                    trace!("Binding handle {:X} to {:X}", this, other);
                    if let Some(handle) = objects.get_mut(&this) {
                        handle.bind(other);
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

        lock.drain_filter(|value| {
            objects.contains_key(&*value)
        });

        lock.is_empty() && self.handle.strong_count() == 1
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GcError {
    ChannelDied
}