use anyhow::Result;
use protocol_core::{BinaryStream, Decoder, Encoder};
use protodef::{integer::Uvarint, ReadBytesExt, WriteBytesExt};

use crate::types::message_type::MessageType;

macro_rules! match_enum {
    ($src:expr, $type:ty = $($dst:ident)|*) => {
        match $src {
            $(
                <$type>::$dst => true,
            )*
            _ => false
        }
    };
}

#[derive(Debug, BinaryStream)]
pub struct Text {
    pub message_type: MessageType,
    pub localize: bool,
    #[proto(dependency(match_enum!(message_type, MessageType = Chat|Whisper|Announcement)))]
    pub source_name: Option<String>,
    pub message: String,
    #[proto(dependency(match_enum!(message_type, MessageType = Translate|Popup|JukeboxPopup)), encode_with(encode_param), decode_with(decode_param))]
    pub parameter_list: Option<Vec<String>>,
    pub sender_xuid: String,
    pub platform_id: String,
    pub filtered_message: String,
}

fn encode_param<W: WriteBytesExt>(param: Vec<String>, w: &mut W) -> Result<()> {
    Uvarint::try_from(param.len())?.encode(w)?;
    for v in param {
        String::encode(v, w)?;
    }
    Ok(())
}
fn decode_param<R: ReadBytesExt>(r: &mut R) -> Result<Vec<String>> {
    let mut vec = Vec::new();
    for _ in 0..Uvarint::decode(r)?.0 {
        vec.push(String::decode(r)?);
    }
    Ok(vec)
}
