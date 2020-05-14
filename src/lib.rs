use cid::CidGeneric;
use multihash::Code;

// re-export
pub use cid::Codec;
pub use cid::Version;
pub use cid::{Error, Result};

pub type Cid = CidGeneric<Codec, Code>;
