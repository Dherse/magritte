

#[cfg(feature = "crossbeam-channel")]
pub type Receiver<T> = crossbeam_channel::Receiver<T>;

#[cfg(feature = "crossbeam-channel")]
pub type Sender<T> = crossbeam_channel::Sender<T>;

#[cfg(not(feature = "crossbeam-channel"))]
pub type Receiver<T> = std::sync::mpsc::Receiver<T>;

#[cfg(not(feature = "crossbeam-channel"))]
pub type Sender<T> = std::sync::mpsc::Sender<T>;

pub trait Rx<T>: Sized {
    type Tx: Tx<T> + Sized;

    fn unbounded() -> (Self::Tx, Self);

    fn recv(&self) -> Option<T>;
}

pub trait Tx<T> {
    fn send(&self, t: T) -> Option<()>;
}

impl<T> Rx<T> for std::sync::mpsc::Receiver<T> {
    type Tx = std::sync::mpsc::Sender<T>;

    fn recv(&self) -> Option<T> {
        self.recv().ok()
    }

    fn unbounded() -> (Self::Tx, Self) {
        std::sync::mpsc::channel()
    }
}

impl<T> Tx<T> for std::sync::mpsc::Sender<T> {
    fn send(&self, t: T) -> Option<()> {
        self.send(t).ok()
    }
}

#[cfg(feature = "crossbeam-channel")]
impl<T> Rx<T> for crossbeam_channel::Receiver<T> {
    type Tx = crossbeam_channel::Sender<T>;
    
    fn recv(&self) -> Option<T> {
        self.recv().ok()
    }

    fn unbounded() -> (Self::Tx, Self) {
        crossbeam_channel::unbounded()
    }
}

#[cfg(feature = "crossbeam-channel")]
impl<T> Tx<T> for crossbeam_channel::Sender<T> {
    fn send(&self, t: T) -> Option<()> {
        self.send(t).ok()
    }
}