use std::error::Error;

use magritte::entry::Entry;

pub fn main() -> Result<(), Box<dyn Error>> {
    let entry = Entry::new()?;

    let version = unsafe { entry.enumerate_instance_version() }.unwrap();
    println!("{:?}", version);

    Ok(())
}
