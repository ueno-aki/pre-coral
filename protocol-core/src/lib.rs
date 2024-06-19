mod bool;
mod integer;
mod str;
mod uuid;

use anyhow::Result;
use protodef::{ReadBytesExt, WriteBytesExt};

pub use protocol_macros::BinaryStream;

pub trait Encoder {
    fn encode<W: WriteBytesExt>(self, w: &mut W) -> Result<()>;
}

pub trait Decoder {
    fn decode<R>(r: &mut R) -> Result<Self>
    where
        R: ReadBytesExt,
        Self: Sized;
}
