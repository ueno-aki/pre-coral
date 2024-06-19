use protocol_core::{Decoder, Encoder};
use protodef::{
    integer::Uvarint,
    slice::{decode_i32slice, encode_i32slice},
};

#[derive(Debug)]
pub struct LoginRequestTokens {
    pub certificate_chain: String,
    pub client_properties: String,
}
impl Encoder for LoginRequestTokens {
    fn encode<W: protodef::WriteBytesExt>(self, w: &mut W) -> anyhow::Result<()> {
        let Self {
            certificate_chain,
            client_properties,
        } = self;
        let buf = {
            let mut buf = Vec::new();
            encode_i32slice(certificate_chain, &mut buf)?;
            encode_i32slice(client_properties, &mut buf)?;
            buf
        };
        Uvarint::try_from(buf.len())?.encode(w)?;
        w.write_all(&buf)?;
        Ok(())
    }
}
impl Decoder for LoginRequestTokens {
    fn decode<R>(r: &mut R) -> anyhow::Result<Self>
    where
        R: protodef::ReadBytesExt,
        Self: Sized,
    {
        let _ = Uvarint::decode(r)?;
        let certificate_chain = String::from_utf8(decode_i32slice(r)?)?;
        let client_properties = String::from_utf8(decode_i32slice(r)?)?;
        Ok(Self {
            certificate_chain,
            client_properties,
        })
    }
}
