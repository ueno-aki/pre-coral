use protocol_core::{Decoder, Encoder};
use protodef::{
    integer::Uvarint,
    slice::{read_i32buffer, write_i32buffer},
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
            write_i32buffer(certificate_chain, &mut buf)?;
            write_i32buffer(client_properties, &mut buf)?;
            buf
        };
        Uvarint::encode(buf.len().try_into()?, w)?;
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
        let certificate_chain = String::from_utf8(read_i32buffer(r)?)?;
        let client_properties = String::from_utf8(read_i32buffer(r)?)?;
        Ok(Self {
            certificate_chain,
            client_properties,
        })
    }
}
