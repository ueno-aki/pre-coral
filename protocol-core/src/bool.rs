use anyhow::bail;

use crate::{Decoder, Encoder};

impl Encoder for bool {
    fn encode<W: protodef::WriteBytesExt>(self, w: &mut W) -> anyhow::Result<()> {
        w.write_u8(self as u8)?;
        Ok(())
    }
}
impl Decoder for bool {
    fn decode<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: protodef::ReadBytesExt,
        Self: Sized,
    {
        match r.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            n => bail!("cannot decode {n} as a bool"),
        }
    }
}
