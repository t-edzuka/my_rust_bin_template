
use std::fs::DirEntry;

use crate::prelude::*;

impl TryFrom<W<&DirEntry>> for String {
    type Error = Error;
    fn try_from(val: W<&DirEntry>) -> Result<String> {
        val.0
        .path()
        .to_str()
        .map(String::from)
        .ok_or(Error::NotSpecified(f!("Not specified {:?}", val.0)))
    }
}