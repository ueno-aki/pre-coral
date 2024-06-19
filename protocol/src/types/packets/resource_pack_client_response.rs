use anyhow::bail;
use protocol_core::{Decoder, Encoder};
use protodef::LE;

use crate::types::resource_pack_client_responce_status::ResponseStatus;

#[derive(Debug)]
pub struct ResourcePackClientResponse {
    pub response: ResponseStatus,
    pub downloading_packs: Vec<String>,
}

impl Encoder for ResourcePackClientResponse {
    fn encode<W: protodef::WriteBytesExt>(self, w: &mut W) -> anyhow::Result<()> {
        let Self {
            response: responce,
            downloading_packs,
        } = self;
        w.write_u8(responce as u8)?;
        w.write_u16::<LE>(downloading_packs.len().try_into()?)?;
        for dp in downloading_packs {
            dp.encode(w)?;
        }
        Ok(())
    }
}

impl Decoder for ResourcePackClientResponse {
    fn decode<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: protodef::ReadBytesExt,
        Self: Sized,
    {
        use ResponseStatus::*;
        let response = match r.read_u8()? {
            1 => Cancel,
            2 => Downloading,
            3 => DownloadingFinished,
            4 => ResourcePackStackFinished,
            n => bail!("Cannot convert {n} into ResponseStatus"),
        };
        let mut downloading_packs = Vec::new();
        let len = r.read_u16::<LE>()?;
        for _ in 0..len {
            downloading_packs.push(String::decode(r)?)
        }
        Ok(Self {
            response,
            downloading_packs,
        })
    }
}
