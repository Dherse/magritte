use std::error::Error;

use magritte::{entry::Entry, Extensions};

pub fn main() -> Result<(), Box<dyn Error>> {
    let entry = Entry::new()?;

    let version = unsafe { entry.enumerate_instance_version() }.unwrap();

    println!("Maximum supported version: {}", version);

    let extensions = Extensions::from_version(version)
        .enable_khr_surface()
        .enable_khr_swapchain();

    println!("Device extensions: {:#?}", extensions.device_extension_names());
    println!("Instance extensions: {:#?}", extensions.instance_extension_names());

    Ok(())
}
