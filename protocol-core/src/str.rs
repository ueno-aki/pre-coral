use protodef::integer::Uvarint;

use crate::{Decoder, Encoder};

impl Encoder for String {
    fn encode<W: protodef::WriteBytesExt>(self, w: &mut W) -> anyhow::Result<()> {
        let bytes = self.as_bytes();
        Uvarint::try_from(bytes.len())?.encode(w)?;
        w.write_all(bytes)?;
        Ok(())
    }
}
impl Decoder for String {
    fn decode<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: protodef::ReadBytesExt,
        Self: Sized,
    {
        let len = Uvarint::decode(r)?.0 as usize;
        let mut buf = vec![0u8; len];
        r.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }
}
