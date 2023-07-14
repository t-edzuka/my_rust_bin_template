// crate prelude

pub use crate::error::Error;

// Result<T, Error> alias
pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtyp pattern, 
// mostly used for external trait implementation like From/TryFrom conversions.
pub struct W<T>(pub T);

// format as f
pub use std::format as f;

