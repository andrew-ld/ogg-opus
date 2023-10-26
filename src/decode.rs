use std::io::{Read, Seek};

use crate::Error;

//--- Final range  things ------------------------------------------------------

#[cfg(test)]
use std::cell::RefCell;

#[cfg(test)]
thread_local! {
    static LAST_FINAL_RANGE: RefCell<u32> = RefCell::new(0);
}

#[cfg(test)]
pub(crate) fn get_final_range() -> u32 {
    LAST_FINAL_RANGE.with(|f| *f.borrow())
}

//--- Code ---------------------------------------------------------------------

pub struct PlayData {
    pub channels: u16,
}

pub(crate) const OPUS_MAGIC_HEADER: [u8; 8] = [b'O', b'p', b'u', b's', b'H', b'e', b'a', b'd'];

/**Reads audio from Ogg Opus, note: it only can read from the ones produced
by itself, this is not ready for anything more, third return is final range just
available while testing, otherwise it is a 0*/
pub fn decode<T: Read + Seek, const TARGET_SPS: u32>(
    _data: T,
) -> Result<(Vec<i16>, PlayData), Error> {
    unimplemented!()
}
