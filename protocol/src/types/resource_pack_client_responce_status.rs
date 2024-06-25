use from_num::from_num;
use protocol_core::{Decoder, Encoder};

#[derive(Debug)]
#[from_num(u8)]
pub enum ResponseStatus {
    Cancel = 1,
    Downloading = 2,
    DownloadingFinished = 3,
    ResourcePackStackFinished = 4,
}

impl Encoder for ResponseStatus {
    fn encode<W: protodef::WriteBytesExt>(self, w: &mut W) -> anyhow::Result<()> {
        w.write_u8(self as u8)?;
        Ok(())
    }
}

impl Decoder for ResponseStatus {
    fn decode<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: protodef::ReadBytesExt,
        Self: Sized,
    {
        ResponseStatus::from_u8(r.read_u8()?)
    }
}
