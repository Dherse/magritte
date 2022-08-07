use std::thread::JoinHandle;



pub struct MagritteGc {
    /// The handle to the garbage collection thread
    gc_thread_handle: JoinHandle<Result<(), GcError>>,

    
}

#[derive(Debug, Clone, Copy)]
pub enum GcError {

}