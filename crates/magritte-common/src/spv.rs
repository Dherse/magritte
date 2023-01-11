use std::io::{self, Error, ErrorKind};

const SPV_MAGIC_NUMBER_LE: u32 = 0x07230203;
const SPV_MAGIC_NUMBER_BE: u32 = SPV_MAGIC_NUMBER_LE.swap_bytes();

#[track_caller]
pub fn read_spv<'a>(input: &'a [u8]) -> io::Result<Vec<u32>> {
    if input.len() % 4 != 0 {
        return Err(Error::new(ErrorKind::InvalidData, "SPIR-V length not divisible by 4"));
    }

    let mut words: Vec<_> = input
        .chunks(4)
        .map(|word| u32::from_le_bytes(word.try_into().unwrap()))
        .collect();

    match words.get(0) {
        Some(&SPV_MAGIC_NUMBER_LE) => (),
        Some(&SPV_MAGIC_NUMBER_BE) => {
            for word in &mut words {
                *word = word.swap_bytes();
            }
        },
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "SPIR-V invalid data")),
    }

    Ok(words)
}