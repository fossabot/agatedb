use crate::db::Agate;
use crate::{Error, Result};
use bytes::{Bytes, BytesMut};

pub struct Snapshot {
    read_ts: u64,
    agate: Agate,
}

impl Snapshot {
    pub fn get(&self, key: &mut [u8]) -> Result<Option<Bytes>> {
        if key.is_empty() {
            return Err(Error::EmptyKey);
        }
        self.agate.get_with_ts(key, self.read_ts)
    }
}
