use protocol_core::{
    slice_codec::{decode_uvarint_slice, encode_uvarint_slice},
    BinaryStream,
};

use crate::types::message_type::MessageType;

macro_rules! match_enum {
    ($type:ty => $src:tt == $($dst:ident)|*) => {
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
    #[proto(dependency(match_enum!(MessageType => message_type == Chat|Whisper|Announcement)))]
    pub source_name: Option<String>,
    pub message: String,
    #[proto(dependency(match_enum!(MessageType => message_type == Translate|Popup|JukeboxPopup)), encode_with(encode_uvarint_slice), decode_with(decode_uvarint_slice))]
    pub parameter_list: Option<Vec<String>>,
    pub sender_xuid: String,
    pub platform_id: String,
    pub filtered_message: String,
}
