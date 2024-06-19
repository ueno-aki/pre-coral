pub use uuid::*;

use crate::{Decoder, Encoder};

impl Encoder for Uuid {
    fn encode<W: protodef::WriteBytesExt>(self, w: &mut W) -> anyhow::Result<()> {
        w.write_all(self.as_bytes())?;
        Ok(())
    }
}
impl Decoder for Uuid {
    fn decode<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: protodef::ReadBytesExt,
        Self: Sized,
    {
        let mut buf = [0; 16];
        r.read_exact(&mut buf)?;
        Ok(Uuid::from_bytes(buf))
    }
}
